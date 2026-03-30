use std::marker::PhantomData;

use crate::{
    core::{
        DisplayWidth,
        buffer::slice::SubBuffer,
        draw::{Position, Rect, gfx::text::WrittenTracker},
        widget::{
            FrameContext, SimpleWidget,
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

    pub fn fill_cells(
        &self,
        ctx: FrameContext<'_, impl crate::core::buffer::Buffer>,
    ) -> WrittenTracker {
        let sz = ctx.buffer.size();

        if sz.area() == 0 {
            return WrittenTracker::default();
        }

        let mut wt = WrittenTracker::default();
        for span in self.spans.as_ref().iter() {
            // Cannot underflow as its checked at the end of the iteration and we break if it does
            let allowed_width = sz.width - wt.cells;

            let swt = span
                .as_borrowed()
                .with_style(self.style.merged(span.style()))
                .fill_cells(
                    FrameContext::new(
                        ctx.total_time,
                        ctx.delta,
                        &mut SubBuffer::new(ctx.buffer, Rect::new(Position::new(wt.cells, 0), sz)),
                        ctx.display_width,
                    ),
                    allowed_width,
                );
            wt.cells += swt.cells;
            wt.bytes += swt.bytes;

            if wt.cells >= sz.width {
                break;
            }
        }

        // Set the style for the cells that are untouched in our `Line`
        for x in wt.cells..sz.width {
            ctx.buffer
                .get_cell_mut(Position::new(x, 0))
                .style_mut()
                .merge(self.style);
        }

        wt
    }
}

impl<'a, Spans> SimpleWidget for Line<'a, Spans>
where
    Spans: AsRef<[Span<'a>]>,
{
    fn draw(&self, ctx: FrameContext<'_, impl crate::core::buffer::Buffer>) {
        let sz = ctx.buffer.size();

        if sz.area() == 0 {
            return;
        }

        let mut offset = 0;
        for span in self.spans.as_ref().iter() {
            // Cannot underflow as its checked at the end of the iteration and we break if it does
            let allowed_width = sz.width - offset;
            offset = span
                .as_borrowed()
                .with_style(self.style.merged(span.style()))
                .fill_cells(
                    FrameContext::new(
                        ctx.total_time,
                        ctx.delta,
                        &mut SubBuffer::new(ctx.buffer, Rect::new(Position::new(offset, 0), sz)),
                        ctx.display_width,
                    ),
                    allowed_width,
                )
                .cells
                .saturating_add(offset);
            if offset >= sz.width {
                break;
            }
        }

        // Set the style for the cells that are untouched in our `Line`
        for x in offset..sz.width {
            ctx.buffer
                .get_cell_mut(Position::new(x, 0))
                .style_mut()
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
