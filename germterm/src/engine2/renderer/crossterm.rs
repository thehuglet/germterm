use std::io::{self, Write};

use crossterm::{cursor, event, execute, terminal};

struct CrosstermRenderer<W: Write> {
    out: W,
}

impl<W: Write> CrosstermRenderer<W> {
    pub fn exit(&mut self) -> io::Result<()> {
        terminal::disable_raw_mode()?;
        execute!(
            &mut self.out,
            terminal::LeaveAlternateScreen,
            terminal::EnableLineWrap,
            cursor::Show,
            event::DisableMouseCapture,
        )?;
        Ok(())
    }
}
