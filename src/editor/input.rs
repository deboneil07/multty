use crossterm::event::{KeyCode, KeyModifiers};

use super::Editor;

impl Editor {
    pub fn handle_key(&mut self, code: KeyCode, modifiers: KeyModifiers) -> bool {
        self.clear_status();

        match (code, modifiers) {
            (KeyCode::Char('q'), KeyModifiers::CONTROL) => return true,
            (KeyCode::Char('s'), KeyModifiers::CONTROL) => {
                if let Err(error) = self.save() {
                    self.set_status(format!("Error saving file: {}", error));
                }
            }
            (KeyCode::Char(c), KeyModifiers::NONE | KeyModifiers::SHIFT) => self.insert(c),
            (KeyCode::Enter, _) => self.insert_newline(),
            (KeyCode::Backspace, _) => self.delete(),
            (other, KeyModifiers::NONE | KeyModifiers::SHIFT) => self.move_cursor(other),
            _ => {}
        }

        false
    }
}
