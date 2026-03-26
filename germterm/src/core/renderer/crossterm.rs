use std::io::Write;

use crossterm::{cursor, event, execute, queue, style, terminal};

use crate::{
    color::Color,
    core::{DrawCall, renderer::Renderer},
    style::Attributes,
};

pub struct CrosstermRenderer<W: Write> {
    out: W,
}

impl<W: Write> CrosstermRenderer<W> {
    pub fn new(out: W) -> Self {
        Self { out }
    }
}

// TODO: make the init/restore customizable
impl<W: Write> Renderer for CrosstermRenderer<W> {
    type Error = std::io::Error;

    fn start_frame(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn init(&mut self) -> Result<(), Self::Error> {
        terminal::enable_raw_mode()?;
        execute!(
            &mut self.out,
            terminal::EnterAlternateScreen,
            terminal::DisableLineWrap,
            cursor::Hide,
            event::EnableMouseCapture,
        )
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

    fn render<'a>(&mut self, calls: impl Iterator<Item = DrawCall<'a>>) -> Result<(), Self::Error> {
        // TODO: Based on the crossterm source there are a few optimizations we can do here
        // specifically when writing fg/bg
        //
        // Track the last cursor position to skip MoveTo when cells are
        // consecutive on the same row, and the last style to skip redundant
        // Reset+SetStyle pairs when adjacent cells share styling.
        let mut last_pos: Option<(u16, u16)> = None;
        let mut last_style: Option<style::ContentStyle> = None;

        for DrawCall { pos, cell } in calls {
            fn conv(c: Color) -> style::Color {
                style::Color::Rgb {
                    r: c.r(),
                    g: c.g(),
                    b: c.b(),
                }
            }
            // Build the crossterm style from the cell's colors and attributes.
            let fg = cell.style.fg().map(conv);
            let bg = cell.style.bg().map(conv);

            let ct_attrs = [
                (Attributes::BOLD, style::Attribute::Bold),
                (Attributes::ITALIC, style::Attribute::Italic),
                (Attributes::UNDERLINED, style::Attribute::Underlined),
                (Attributes::HIDDEN, style::Attribute::Hidden),
            ]
            .iter()
            .fold(style::Attributes::none(), |acc, &(flag, attr)| {
                if cell.style.attributes().contains(flag) {
                    acc | attr
                } else {
                    acc
                }
            });

            let cell_style = style::ContentStyle {
                foreground_color: fg,
                background_color: bg,
                underline_color: None,
                attributes: ct_attrs,
            };

            // Only move the cursor if we are not already at the right position.
            // Consecutive cells on the same row advance the cursor automatically
            // after each printed character, so we only emit MoveTo when needed.
            if last_pos.is_none_or(|(lx, ly)| ly != pos.y || lx + 1 != pos.x) {
                queue!(&mut self.out, cursor::MoveTo(pos.x, pos.y))?;
            }

            // Only emit Reset+SetStyle when the style actually changes.
            if last_style.as_ref() != Some(&cell_style) {
                queue!(
                    &mut self.out,
                    style::SetAttribute(style::Attribute::Reset),
                    style::SetStyle(cell_style),
                )?;
                last_style = Some(cell_style);
            }

            queue!(&mut self.out, style::Print(cell.ch))?;

            last_pos = Some((pos.x, pos.y));
        }

        Ok(())
    }
}
