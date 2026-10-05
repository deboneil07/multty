use crossterm::{event::{self, Event, KeyCode, KeyModifiers}, terminal::{self, disable_raw_mode,size, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen, Clear, ClearType}, cursor, execute, queue};
use std::io::{self, stdout, Stdout, Write};
use std::panic;
use std::env;
use std::fs;

// TerminalGuard

struct TerminalGuard;

impl TerminalGuard {
  fn new() -> io::Result<Self> {
    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen)?;
    Ok(Self)
  }
}

impl Drop for TerminalGuard {
  fn drop(&mut self) {
    let _ = execute!(stdout(), LeaveAlternateScreen, cursor::Show);
    let _ = disable_raw_mode();
  }
}

fn setup_panic_hook() {
  let original_hook = panic::take_hook();
  panic::set_hook(Box::new(move |panic_info|  {
    let _ = execute!(stdout(), LeaveAlternateScreen, cursor::Show);
    let _ = disable_raw_mode();
    original_hook(panic_info);
  }));
}


//  EDITOR

struct Editor {
  cursor_x: usize,
  cursor_y: usize,
  screen_cols: u16,
  screen_rows: u16,
  rows: Vec<String>,
  filename: Option<String>,
  row_offset: usize,
  col_offset: usize,
  modified: bool,
  status_message: Option<String>,
}

impl Editor {
  fn new() -> io::Result<Self> {
    let (cols, rows) = terminal::size()?;
    Ok(Self {
      cursor_x: 0,
      cursor_y: 0,
      screen_cols: cols,
      screen_rows: rows,
      rows: vec![String::new()],
      filename: None,
      row_offset: 0,
      col_offset: 0,
      modified: false,
      status_message: None
    })
  }

  fn open(&mut self, path: &str) -> io::Result<()> {
    let content = fs::read_to_string(path)?;
    let lines: Vec<String> = content.lines().map(String::from).collect();
    if !lines.is_empty() {
      self.rows = lines;
      }
    self.filename = Some(path.to_string());
    self.modified = false;
    self.status_message = Some(format!("Opened {}", path));
    Ok(())
  }

  fn save(&mut self) -> io::Result<String> {
    let path = match &self.filename {
      Some(p) => p.clone(),
      None => {
        let default_name: String = "untitled.txt".to_string();
        self.filename = Some(default_name.clone());
        default_name
      }
    };

    let mut content = self.rows.join("\n");;
    content.push('\n');

    fs::write(&path, content.as_bytes())?;
    self.modified = false;

    let msg = format!("saved {} bytes to {}", content.len(), path);
    self.status_message = Some(msg.clone());
    Ok(msg)
  }

  fn current_row_len(&self) -> usize {
    self.rows.get(self.cursor_y).map_or(0, |row| row.len())
  }

  fn snap_cursor_to_row(&mut self) {
    let max_x = self.current_row_len();
    if self.cursor_x > max_x {
      self.cursor_x = max_x;
    }
  }

  fn text_area_rows(&self) -> usize {
    self.screen_rows.saturating_sub(1) as usize
  }

  fn scroll(&mut self) {
    let text_rows = self.text_area_rows();

    if self.cursor_y < self.row_offset {
      self.row_offset = self.cursor_y;
    }

    if self.cursor_y >= self.row_offset + text_rows {
      self.row_offset = self.cursor_y - text_rows + 1;
    }

    if self.cursor_x < self.col_offset {
      self.col_offset = self.cursor_x;
    }
    
    let text_cols = self.screen_cols as usize;
    if self.cursor_x >= self.col_offset + text_cols {
      self.col_offset = self.cursor_x - text_cols + 1;
    }
  }

  fn insert(&mut self, c: char) {
    if self.rows.is_empty() {
      self.rows.push(String::new());
    }

    let row = &mut self.rows[self.cursor_y];
    row.insert(self.cursor_x, c);
    self.cursor_x += 1;
    self.modified = true;
  }

  fn insert_newline(&mut self) {
    if self.rows.is_empty() {
      self.rows.push(String::new());
    }

    let remainder = self.rows[self.cursor_y].split_off(self.cursor_x);
    self.rows.insert(self.cursor_y + 1, remainder);

    self.cursor_y += 1;
    self.cursor_x = 0;
    self.modified = true;
    self.status_message = None;
  }

  fn delete(&mut self) {
    if self.cursor_x > 0 {
      let row = &mut self.rows[self.cursor_y];
      row.remove(self.cursor_x - 1);
      self.cursor_x -=1;
      self.modified = true;
    } else if (self.cursor_y > 0) {
      let current_row = self.rows.remove(self.cursor_y);
      self.cursor_y -= 1;
      self.cursor_x = self.rows[self.cursor_y].len();
      self.rows[self.cursor_y].push_str(&current_row);
      self.modified = true;
      }
  }

  fn move_cursor(&mut self, key: KeyCode) {
    // let max_x = (self.screen_cols.saturating_sub(1)) as usize;
    // let max_y = (self.screen_cols.saturating_sub(1)) as usize;
    let screen_limit = (self.screen_rows.saturating_sub(2)) as usize;
    let content_limit = self.rows.len().saturating_sub(1);
    //let max_y = screen_limit.min(content_limit);
    let max_y = self.rows.len().saturating_sub(1);


    match key {
      KeyCode::Up => {
        if self.cursor_y > 0 {
          self.cursor_y -= 1;
          self.snap_cursor_to_row();
          }
      }
      KeyCode::Down => {
        if self.cursor_y < max_y {
          self.cursor_y += 1;
          self.snap_cursor_to_row();
        }
      }
      KeyCode::Left => {
        if self.cursor_x > 0 {
          self.cursor_x -= 1;
          }
      }
      KeyCode::Right => {
        if self.cursor_x < self.current_row_len() {
          self.cursor_x += 1;
        }
      }
      _ => {}
    }
  }

  fn refresh_screen(&mut self, stdout: &mut Stdout) -> io::Result<()> {
    let (cols, rows) = terminal::size()?;
    self.screen_cols = cols;
    self.screen_rows = rows;

    self.scroll();

    queue!(stdout, cursor::Hide, cursor::MoveTo(0, 0))?;

    for r in 0..rows{
      queue!(stdout, Clear(ClearType::CurrentLine))?;

      if r == rows - 1 {
        let file_info = self.filename.as_deref().unwrap_or("[No Name]");
        let modified_tag = if self.modified { "[Modified]" } else {""};
        let status = format!("miniedit | {}| Lines: {} | Pos: ({}, {}) | Press Ctrl+Q to exit. | {}", file_info, self.rows.len(), self.cursor_x, self.cursor_y, modified_tag);
        let len = status.len().min(cols as usize);

        print!("{}", &status[..len]);
      } else{
        let file_row_idx = r as usize + self.row_offset;
        if file_row_idx < self.rows.len() {
          let row = &self.rows[file_row_idx];
          if self.col_offset < row.len() {
            let visible_slice = &row[self.col_offset..];
            let len = visible_slice.len().min(cols as usize);
            print!("{}", &visible_slice[..len]);
          }
          } else {
            print!("~");
          }
        }

      if r <  rows - 1 {
        print!("\r\n");
      }
    }

    let screen_cursor_x = (self.cursor_x - self.col_offset) as u16;
    let screen_cursor_y = (self.cursor_y - self.row_offset) as u16;

    queue!(stdout, cursor::MoveTo(screen_cursor_x, screen_cursor_y), cursor::Show)?;
    stdout.flush()?;
    Ok(())
  }
}

fn main() -> io::Result<()> {
  setup_panic_hook();

  let _guard = TerminalGuard::new()?;
  let mut out = stdout();
  let mut editor = Editor::new()?;

  let args: Vec<String> = env::args().collect();
  if let Some(path) = args.get(1) {
    editor.open(path)?;
  }

   loop {
    editor.refresh_screen(&mut out)?;
    if let Event::Key(key_event) = event::read()? {


      //if (key_event.code == KeyCode::Char('c') && key_event.modifiers.contains(KeyModifiers::CONTROL)) {
      //  break;
      //}

      //editor.move_cursor(key_event.code);

      match (key_event.code, key_event.modifiers) {
        (KeyCode::Char('q'), KeyModifiers::CONTROL) => break,
        (KeyCode::Char(c), KeyModifiers::NONE) | (KeyCode::Char(c), KeyModifiers::SHIFT) => {
          editor.insert(c);
        }
        (KeyCode::Char('s'), KeyModifiers::CONTROL) => {
          if let Err(e) = editor.save() {
            editor.status_message = Some(format!("Error saving file: {}", e));
          }
        }

        (KeyCode::Enter, _) => editor.insert_newline(),

        (KeyCode::Backspace, _) => {
          editor.delete();
        }

        (code, _) => {
          editor.move_cursor(code);
        }
      }
    }
   }
  Ok(())
}
