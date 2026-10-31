use std::io::{self, Stdout, Write};

use crossterm::{
    cursor, queue,
    terminal::{self, Clear, ClearType},
};

use super::Editor;
use super::util::{boundary_forward, fit_width};

impl Editor {
    fn status_line(&self) -> String {
        let file_info = self.filename.as_deref().unwrap_or("[No Name]");

        let mut line = format!(
            "miniedit | {} | Lines: {} | Pos: ({}, {})",
            file_info,
            self.rows.len(),
            self.cursor_x,
            self.cursor_y
        );

        if self.modified {
            line.push_str(" | [Modified]");
        }

        line.push_str(" | Ctrl+Q to exit");

        if let Some(message) = &self.status_message {
            line.push_str(" | ");
            line.push_str(message);
        }

        line
    }

    pub fn refresh_screen(&mut self, stdout: &mut Stdout) -> io::Result<()> {
        let (cols, rows) = terminal::size()?;
        self.screen_cols = cols;
        self.screen_rows = rows;
        self.scroll();

        let width = cols as usize;
        let status_row = rows.saturating_sub(1);

        queue!(
            stdout,
            cursor::Hide,
            cursor::MoveTo(0, 0),
            Clear(ClearType::All)
        )?;

        for screen_row in 0..rows {
            queue!(
                stdout,
                cursor::MoveTo(0, screen_row),
                Clear(ClearType::UntilNewLine)
            )?;

            if screen_row == status_row {
                let line = self.status_line();
                stdout.write_all(fit_width(&line, width).as_bytes())?;
                continue;
            }

            let file_row = screen_row as usize + self.row_offset;
            if file_row >= self.rows.len() {
                stdout.write_all(b"~")?;
                continue;
            }

            let row = &self.rows[file_row];
            if self.col_offset < row.len() {
                let visible = &row[boundary_forward(row, self.col_offset)..];
                stdout.write_all(fit_width(visible, width).as_bytes())?;
            }
        }

        let screen_cols = width.saturating_sub(1);
        let screen_rows = status_row.saturating_sub(1) as usize;

        let screen_cursor_x = self
            .cursor_x
            .saturating_sub(self.col_offset)
            .min(screen_cols) as u16;
        let screen_cursor_y = self
            .cursor_y
            .saturating_sub(self.row_offset)
            .min(screen_rows) as u16;

        queue!(
            stdout,
            cursor::MoveTo(screen_cursor_x, screen_cursor_y),
            cursor::Show
        )?;
        stdout.flush()?;

        Ok(())
    }
}
