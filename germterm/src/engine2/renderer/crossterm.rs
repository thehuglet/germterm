use std::io::{self, Write};

use crossterm::{cursor, event, execute, terminal};

use crate::engine2::renderer::Renderer;

struct CrosstermRenderer<W: Write> {
    out: W,
}

// TODO: make the init/restore customizable
impl<W: Write> Renderer for CrosstermRenderer<W> {
    type Error = std::io::Error;

    fn start_frame(&mut self) -> Result<(), Self::Error> {
        terminal::enable_raw_mode()?;
        execute!(
            &mut self.out,
            terminal::EnterAlternateScreen,
            terminal::DisableLineWrap,
            cursor::Hide,
            event::EnableMouseCapture,
        )
    }

    fn init(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn restore(&mut self) -> Result<(), Self::Error> {
        terminal::disable_raw_mode()?;
        execute!(
            &mut self.out,
            terminal::LeaveAlternateScreen,
            terminal::EnableLineWrap,
            cursor::Show,
            event::DisableMouseCapture,
        )
    }

    fn end_frame(&mut self) -> Result<(), Self::Error> {
        self.out.flush()
    }
    fn render<'a>(
        &mut self,
        calls: impl Iterator<Item = crate::engine2::DrawCall<'a>>,
    ) -> Result<(), Self::Error> {
        todo!()
    }
}
