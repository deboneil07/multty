use crossterm::event::KeyCode;

use super::Editor;
use super::util::{boundary_backward, boundary_forward};

impl Editor {
    pub(super) fn current_row_len(&self) -> usize {
        self.rows.get(self.cursor_y).map_or(0, |row| row.len())
    }

    pub(super) fn clamp_cursor(&mut self) {
        if self.rows.is_empty() {
            self.rows.push(String::new());
        }

        self.cursor_y = self.cursor_y.min(self.rows.len() - 1);

        let max_x = self.current_row_len();
        if self.cursor_x > max_x {
            self.cursor_x = max_x;
        }
    }

    pub(super) fn move_cursor(&mut self, key: KeyCode) {
        let last_row = self.rows.len().saturating_sub(1);

        match key {
            KeyCode::Up if self.cursor_y > 0 => self.cursor_y -= 1,
            KeyCode::Down if self.cursor_y < last_row => self.cursor_y += 1,
            KeyCode::Left if self.cursor_x > 0 => {
                let x = boundary_backward(&self.rows[self.cursor_y], self.cursor_x - 1);
                self.cursor_x = x;
            }
            KeyCode::Right if self.cursor_x < self.current_row_len() => {
                let x = boundary_forward(&self.rows[self.cursor_y], self.cursor_x + 1);
                self.cursor_x = x;
            }
            _ => {}
        }

        self.clamp_cursor();
    }

    fn text_area_rows(&self) -> usize {
        self.screen_rows.saturating_sub(1) as usize
    }

    pub(super) fn scroll(&mut self) {
        let text_rows = self.text_area_rows();

        if self.cursor_y < self.row_offset {
            self.row_offset = self.cursor_y;
        }
        if text_rows > 0 && self.cursor_y >= self.row_offset + text_rows {
            self.row_offset = self.cursor_y + 1 - text_rows;
        }

        if self.cursor_x < self.col_offset {
            self.col_offset = self.cursor_x;
        }

        let text_cols = self.screen_cols as usize;
        if text_cols > 0 && self.cursor_x >= self.col_offset + text_cols {
            self.col_offset = self.cursor_x + 1 - text_cols;
        }
    }
}
