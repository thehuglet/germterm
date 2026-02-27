use std::borrow::Cow;

use crate::{
    core::{
        buffer::Buffer,
        draw::Position,
        timer::NoDelta,
        widget::{FrameContext, Widget},
    },
    style::Style,
};

#[derive(Clone)]
pub struct Span<'a> {
    pub(crate) content: Cow<'a, str>,
    style: Style,
}

#[derive(Clone, Copy, Debug, Hash)]
pub enum SpanError {
    ControlCharacter { at: usize },
}

impl<'a> Span<'a> {
    pub fn new(content: impl Into<Cow<'a, str>>) -> Result<Self, SpanError> {
        let content: Cow<'a, str> = content.into();
        let s = &*content;
        let mut offset = 0;
        while offset < s.len() {
            if s.as_bytes()[offset].is_ascii_control() {
                return Err(SpanError::ControlCharacter { at: offset });
            }
            offset += 1;
        }

        Ok(Self {
            content,
            style: Style::EMPTY,
        })
    }

    pub const fn new_unchecked(content: &'static str) -> Self {
        let content = Cow::Borrowed(content);
        Self {
            content,
            style: Style::EMPTY,
        }
    }

    pub fn set_content(mut self, content: impl Into<Cow<'a, str>>) -> Self {
        self.content = content.into();
        self
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn style(&self) -> Style {
        self.style
    }

    pub fn set_style(mut self, style: Style) -> Self {
        self.style = style;

        self
    }

    /// Fills the cells in the provided buffer as much as possible without exceeding `limit` cells.
    ///
    /// This is mainly intended to be called from other [`Widget`]'s where they would account for
    /// line wrapping themselves. In other words this is a primitive text drawer in widget form.
    pub fn fill_cells<Buf: Buffer>(&self, buf: &mut Buf, limit: u16) -> u16 {
        let limit = limit as u32;
        let sz = buf.size();
        let mut chars = self.content.chars();
        let mut written = 0;
        for y in 0..sz.height {
            for x in 0..sz.width {
                let c = buf.get_cell_mut(Position::new(x, y));
                written = sz.width as u32 * y as u32 + x as u32;
                // TODO: add cell merging once cell styling is stored
                if let Some(ch) = chars.next() {
                    c.style.merge(self.style);
                    c.ch = ch;
                    if written >= limit {
                        break;
                    }
                } else {
                    break;
                }
            }
        }

        written as u16
    }

    pub fn as_borrowed(&'a self) -> Span<'a> {
        Self {
            content: Cow::Borrowed(self.content.as_ref()),
            style: self.style,
        }
    }
}

impl<'a> Widget<NoDelta> for Span<'a> {
    fn draw(&mut self, ctx: &mut FrameContext<'_, impl crate::core::buffer::Buffer, NoDelta>) {
        self.fill_cells(ctx.buffer, ctx.buffer.size().width);
    }
}
