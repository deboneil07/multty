use super::Editor;
use super::util::{boundary_backward, boundary_forward};

impl Editor {
    pub(super) fn insert(&mut self, c: char) {
        self.clamp_cursor();

        let row = &self.rows[self.cursor_y];
        let index = boundary_forward(row, self.cursor_x.min(row.len()));

        self.rows[self.cursor_y].insert(index, c);
        self.cursor_x = index + c.len_utf8();
        self.modified = true;
    }

    pub(super) fn insert_newline(&mut self) {
        self.clamp_cursor();

        let row = &self.rows[self.cursor_y];
        let index = boundary_forward(row, self.cursor_x.min(row.len()));

        let remainder = self.rows[self.cursor_y].split_off(index);
        self.rows.insert(self.cursor_y + 1, remainder);

        self.cursor_y += 1;
        self.cursor_x = 0;
        self.modified = true;
    }

    pub(super) fn delete(&mut self) {
        self.clamp_cursor();

        if self.cursor_x > 0 {
            let row = &self.rows[self.cursor_y];
            let end = self.cursor_x.min(row.len());
            let start = boundary_backward(row, end - 1);

            self.rows[self.cursor_y].replace_range(start..end, "");
            self.cursor_x = start;
            self.modified = true;
        } else if self.cursor_y > 0 {
            let remainder = self.rows.remove(self.cursor_y);

            self.cursor_y -= 1;
            self.cursor_x = self.rows[self.cursor_y].len();
            self.rows[self.cursor_y].push_str(&remainder);
            self.modified = true;
        }
    }
}
