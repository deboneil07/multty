use std::io;

use crossterm::terminal;

mod cursor;
mod file;
mod input;
mod render;
mod text;
mod util;

pub struct Editor {
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
    pub fn new() -> io::Result<Self> {
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
            status_message: None,
        })
    }

    pub fn set_status(&mut self, message: impl Into<String>) {
        self.status_message = Some(message.into());
    }

    fn clear_status(&mut self) {
        self.status_message = None;
    }
}
