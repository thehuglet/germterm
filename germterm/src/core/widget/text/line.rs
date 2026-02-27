use std::marker::PhantomData;

use crate::{
    core::{
        buffer::slice::SubBuffer,
        draw::{Position, Rect},
        timer::NoDelta,
        widget::{FrameContext, Widget, text::span::Span},
    },
    style::{Stylable, Style},
};

/// A widget that renders a single line composed of one or more [`Span`]s.
///
/// Each span carries its own [`Style`], so a single `Line` can display
/// multiple colors, backgrounds, and text attributes on one row.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Line<'a, Spans = Vec<Span<'a>>> {
    spans: Spans,
    style: Style,
    __p: PhantomData<&'a ()>,
}

impl<'a, Spans> Line<'a, Spans>
where
    Spans: IntoIterator<Item = &'a Span<'a>>,
    Spans: Clone,
{
    /// Creates a new `Line` from a mutable slice of [`Span`]s and an
    /// optional base [`Style`].
    pub fn new(spans: Spans) -> Self {
        Self {
            spans,
            style: Style::EMPTY,
            __p: PhantomData,
        }
    }

    pub fn width(&self) -> usize {
        self.spans
            .clone()
            .into_iter()
            .fold(0, |len, s| len + s.content().len())
    }
}

impl<'a, Spans> Widget<NoDelta> for Line<'a, Spans>
where
    Spans: IntoIterator<Item = &'a Span<'a>>,
    Spans: Clone,
{
    fn draw(&mut self, ctx: &mut FrameContext<'_, impl crate::core::buffer::Buffer, NoDelta>) {
        let buf = ctx.buffer_mut();
        let sz = buf.size();

        if sz.area() == 0 {
            return;
        }

        let mut offset = 0;
        for span in self.spans.clone().into_iter() {
            offset = span
                .as_borrowed()
                .with_style(self.style.merged(span.style()))
                .fill_cells(
                    &mut SubBuffer::new(buf, Rect::new(Position::new(offset, 0), sz)),
                    sz.width - offset,
                )
                .saturating_add(offset);
            if offset >= sz.width {
                break;
            }
        }

        for x in offset..sz.width {
            buf.get_cell_mut(Position::new(x, 0))
                .style
                .merge(self.style);
        }
    }
}

impl<'a, Spans> Stylable for Line<'a, Spans> {
    fn style(&self) -> Style {
        self.style
    }

    fn set_style(&mut self, style: Style) {
        self.style = style;
    }
}
