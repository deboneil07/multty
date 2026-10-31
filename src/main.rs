use std::env;
use std::io::{self, stdout};

use crossterm::event::{self, Event};

mod editor;
mod terminalguard;

use editor::Editor;
use terminalguard::{TerminalGuard, setup_panic_hook};

fn main() -> io::Result<()> {
    setup_panic_hook();

    let _guard = TerminalGuard::new()?;
    let mut out = stdout();
    let mut editor = Editor::new()?;

    if let Some(path) = env::args().nth(1)
        && let Err(error) = editor.open(&path)
    {
        editor.set_status(format!("Error opening {}: {}", path, error));
    }

    loop {
        editor.refresh_screen(&mut out)?;

        if let Event::Key(key_event) = event::read()?
            && editor.handle_key(key_event.code, key_event.modifiers)
        {
            break;
        }
    }

    Ok(())
}
