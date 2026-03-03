use std::marker::PhantomData;

use crate::{
    core::{
        DisplayWidth,
        buffer::slice::SubBuffer,
        draw::{Position, Rect},
        timer::NoDelta,
        widget::{
            FrameContext, Widget,
            text::{LineWidth, span::Span},
        },
    },
    style::{Stylable, Style},
};

/// A widget that renders a single line composed of one or more [`Span`]s.
///
/// Each span carries its own [`Style`], so a single `Line` can display
/// multiple colors, backgrounds, and text attributes on one row.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Line<'a, Spans = &'a [Span<'a>]> {
    spans: Spans,
    style: Style,
    __p: PhantomData<&'a ()>,
}

impl<'a, Spans: AsRef<[Span<'a>]>> LineWidth for Line<'a, Spans> {
    fn width(&self, display_width: &DisplayWidth) -> u16 {
        let mut sum: u16 = 0;
        #[cold]
        fn cold() {}
        for span in self.spans.as_ref() {
            let w = span.width(display_width);
            sum = sum.saturating_add(w);
            if sum == u16::MAX {
                cold();
                break;
            }
        }

        sum
    }
}

impl<'a, Spans> Line<'a, Spans>
where
    Spans: AsRef<[Span<'a>]>,
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
            .as_ref()
            .iter()
            .fold(0, |len, s| len + s.content().len())
    }
}

impl<'a, Spans> Widget for Line<'a, Spans>
where
    Spans: AsRef<[Span<'a>]>,
{
    fn draw(&self, ctx: &mut FrameContext<'_, impl crate::core::buffer::Buffer>) {
        let buf = ctx.buffer_mut();
        let sz = buf.size();

        if sz.area() == 0 {
            return;
        }

        let mut offset = 0;
        for span in self.spans.as_ref().iter() {
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
