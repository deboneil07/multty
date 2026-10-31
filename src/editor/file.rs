use std::fs;
use std::io;

use super::Editor;

impl Editor {
    pub fn open(&mut self, path: &str) -> io::Result<()> {
        let content = fs::read_to_string(path)?;
        let mut lines: Vec<String> = content.lines().map(String::from).collect();

        if lines.is_empty() {
            lines.push(String::new());
        }

        self.rows = lines;
        self.filename = Some(path.to_string());
        self.cursor_x = 0;
        self.cursor_y = 0;
        self.row_offset = 0;
        self.col_offset = 0;
        self.modified = false;
        self.set_status(format!("Opened {}", path));

        Ok(())
    }

    pub fn save(&mut self) -> io::Result<String> {
        let path = match &self.filename {
            Some(name) => name.clone(),
            None => String::from("untitled.txt"),
        };
        self.filename = Some(path.clone());

        let mut content = self.rows.join("\n");
        content.push('\n');

        fs::write(&path, content.as_bytes())?;
        self.modified = false;

        let message = format!("Saved {} bytes to {}", content.len(), path);
        self.set_status(message.clone());

        Ok(message)
    }
}
