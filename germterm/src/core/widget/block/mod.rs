pub mod set;
pub mod title;

use bitflags::bitflags;

use crate::core::{
    DisplayWidth,
    buffer::{Buffer, slice::SubBuffer},
    draw::{Position, Rect, Size},
    timer::TimerDelta,
    widget::{
        FrameContext, Widget,
        block::{
            set::BlockSet,
            title::{Title, TitleAlignment, TitlePosition},
        },
        text::{LineWidth, line::Line},
    },
};

bitflags! {
    pub struct BorderSides: u8 {
        const TOP = 0b00000001;
        const RIGHT = 0b00000010;
        const BOTTOM = 0b00000100;
        const LEFT = 0b00001000;
    }
}

pub struct Block<'a, B, T = Line<'a>> {
    set: B,
    sides: BorderSides,
    titles: &'a [Title<T>],
}

impl<'a, B> Block<'a, B> {
    pub fn new(set: B) -> Self {
        Self {
            set,
            sides: BorderSides::all(),
            titles: &[],
        }
    }
}

impl<'a, B, T> Block<'a, B, T> {
    pub fn with_titles<T2>(self, titles: &'a [Title<T2>]) -> Block<'a, B, T2> {
        Block {
            set: self.set,
            sides: self.sides,
            titles,
        }
    }
}

impl<'a, B: BlockSet, T: LineWidth> Block<'a, B, T> {
    pub fn inner_area(&self, sz: Size, display_width: &DisplayWidth) -> Rect {
        let set = &self.set;
        let left_offset =
            self.sides.contains(BorderSides::LEFT) as u16 * set.left_width(display_width) as u16;
        let right_offset =
            self.sides.contains(BorderSides::RIGHT) as u16 * set.right_width(display_width) as u16;
        let horizontal_offset = left_offset.saturating_add(right_offset);
        let top_offset = self.sides.contains(BorderSides::TOP) as u16;
        let bottom_offset = self.sides.contains(BorderSides::BOTTOM) as u16;
        let vertical_offset = top_offset.saturating_add(bottom_offset);

        // just return the whole area if the area will just be filled with borders
        //
        // the content inside takes priority over the border if needed
        if (sz.width <= horizontal_offset) || sz.height <= vertical_offset {
            return Rect::new(Position::ZERO, sz);
        }

        Rect::new(
            Position::new(left_offset, top_offset),
            Size::new(
                sz.width.saturating_sub(horizontal_offset),
                sz.height.saturating_sub(vertical_offset),
            ),
        )
    }

    fn render_titles<Buf: Buffer, D: TimerDelta>(
        &self,
        ctx: &mut FrameContext<'_, Buf, D>,
        titles: impl Iterator<Item = &'a Title<T>>,
        y_pos: u16,
        left_offset: u16,
        right_offset: u16,
    ) where
        T: Widget<D>,
    {
        let size = ctx.buffer().size();
        let free_width = size
            .width
            .saturating_sub(left_offset)
            .saturating_sub(right_offset);
        if free_width == 0 {
            return;
        }
        for title in titles {
            let title_width = title.inner().width(&ctx.display_width);
            let total_time = ctx.total_time;
            let delta = ctx.delta;
            let display_width = ctx.display_width;
            let mut sub;
            match title.alignment() {
                TitleAlignment::Left => {
                    sub = SubBuffer::new(
                        ctx.buffer_mut(),
                        Rect::new(
                            Position::new(left_offset, y_pos),
                            Size::new(title_width.min(free_width), 1),
                        ),
                    );
                }
                TitleAlignment::Center => {
                    sub = SubBuffer::new(
                        ctx.buffer_mut(),
                        Rect::new(
                            Position::new(size.width.saturating_sub(title_width) / 2, y_pos),
                            Size::new(title_width.min(free_width), 1),
                        ),
                    );
                }
                TitleAlignment::Right => {
                    sub = SubBuffer::new(
                        ctx.buffer_mut(),
                        Rect::new(
                            Position::new(size.width.saturating_sub(title_width), y_pos),
                            Size::new(title_width.min(free_width), 1),
                        ),
                    );
                }
            }

            title.inner().draw(FrameContext {
                total_time,
                delta,
                buffer: &mut sub,
                display_width,
            });
        }
    }
}

impl<'a, D: TimerDelta, B: BlockSet, T: Widget<D> + LineWidth> Widget<D> for Block<'a, B, T> {
    fn draw(&self, mut ctx: FrameContext<'_, impl Buffer, D>) {
        let size = ctx.buffer().size();

        let left_offset = self.sides.contains(BorderSides::LEFT) as u16;
        let right_offset = self.sides.contains(BorderSides::RIGHT) as u16;
        let horizontal_offset = left_offset.saturating_add(right_offset);
        let x_end = size.width.saturating_sub(right_offset);
        let top_offset = self.sides.contains(BorderSides::TOP) as u16;
        let bottom_offset = self.sides.contains(BorderSides::BOTTOM) as u16;
        let vertical_offset = top_offset.saturating_add(bottom_offset);
        if size.width <= horizontal_offset || size.height <= vertical_offset || size.area() == 0 {
            return;
        }

        // top left corner
        if self.sides.contains(BorderSides::LEFT)
            && self.sides.contains(BorderSides::TOP)
            && size.width > 0
        {
            let cur = ctx.buffer_mut().get_cell_mut(Position::ZERO);
            cur.ch = self
                .set
                .top_left(&cur.ch.to_string())
                .chars()
                .next()
                .unwrap_or_default();
        }

        // top side
        if self.sides.contains(BorderSides::TOP) && size.width > horizontal_offset {
            for x in left_offset..x_end {
                let cur = ctx.buffer_mut().get_cell_mut(Position { x, y: 0 });
                cur.ch = self
                    .set
                    .top(&cur.ch.to_string())
                    .chars()
                    .next()
                    .unwrap_or_default();
            }

            // Draw the top titles
            let top_titles = self
                .titles
                .as_ref()
                .iter()
                .filter(|title| title.position() == TitlePosition::Top);
            self.render_titles(&mut ctx, top_titles, 0, left_offset, right_offset);
        }

        // top right corner
        if self.sides.contains(BorderSides::RIGHT)
            && self.sides.contains(BorderSides::TOP)
            && size.width > 2
        {
            let cur = ctx.buffer_mut().get_cell_mut(Position {
                x: size.width.saturating_sub(1),
                y: 0,
            });
            cur.ch = self
                .set
                .top_right(&cur.ch.to_string())
                .chars()
                .next()
                .unwrap_or_default();
        }

        // LR sides
        if size.height > vertical_offset {
            let h_end = size.height.saturating_sub(bottom_offset).max(1);
            // Left side
            if self.sides.contains(BorderSides::LEFT) {
                for y in top_offset..h_end {
                    let cur = ctx.buffer_mut().get_cell_mut(Position::new(0, y));
                    cur.ch = self
                        .set
                        .left(&cur.ch.to_string())
                        .chars()
                        .next()
                        .unwrap_or_default();
                }
            }

            // Right side
            if self.sides.contains(BorderSides::RIGHT) {
                for y in top_offset..h_end {
                    let cur = ctx.buffer_mut().get_cell_mut(Position {
                        x: size.width.saturating_sub(right_offset),
                        y,
                    });
                    cur.ch = self
                        .set
                        .right(&cur.ch.to_string())
                        .chars()
                        .next()
                        .unwrap_or_default();
                }
            }
        }

        // bottom left
        if self.sides.contains(BorderSides::BOTTOM) && self.sides.contains(BorderSides::LEFT) {
            let cur = ctx
                .buffer_mut()
                .get_cell_mut(Position::new(0, size.height.saturating_sub(1)));
            cur.ch = self
                .set
                .bottom_left(&cur.ch.to_string())
                .chars()
                .next()
                .unwrap_or_default();
        }

        // bottom
        if self.sides.contains(BorderSides::BOTTOM) {
            let y = size.height.saturating_sub(1);
            for x in left_offset..x_end {
                let cur = ctx.buffer_mut().get_cell_mut(Position { x, y });
                cur.ch = self
                    .set
                    .bottom(&cur.ch.to_string())
                    .chars()
                    .next()
                    .unwrap_or_default();
            }

            let bottom_titles = self
                .titles
                .as_ref()
                .iter()
                .filter(|title| title.position() == TitlePosition::Bottom);

            self.render_titles(&mut ctx, bottom_titles, y, left_offset, right_offset);
        }

        // bottom right
        if self.sides.contains(BorderSides::BOTTOM) && self.sides.contains(BorderSides::RIGHT) {
            let cur = ctx.buffer_mut().get_cell_mut(Position {
                x: size.width.saturating_sub(1),
                y: size.height.saturating_sub(1),
            });
            cur.ch = self
                .set
                .bottom_right(&cur.ch.to_string())
                .chars()
                .next()
                .unwrap_or_default();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{
        DisplayWidth, buffer::paired::PairedBuffer, draw::Size, timer::NoDelta,
        widget::block::set::SimpleBorderSet,
    };

    #[test]
    fn test_block_draw_borders() {
        let mut buf = PairedBuffer::new(Size::new(3, 3));
        let block = Block::new(SimpleBorderSet::ASCII);
        let ctx = FrameContext {
            total_time: NoDelta::new(),
            delta: NoDelta::new(),
            buffer: &mut buf,
            display_width: DisplayWidth::default(),
        };

        block.draw(ctx);

        // Ascii style: top_left='+', top='-', top_right='+', left='|', right='|', bottom_left='+', bottom='-', bottom_right='+'
        assert_eq!(buf.get_cell(Position::new(0, 0)).ch, '+');
        assert_eq!(buf.get_cell(Position::new(1, 0)).ch, '-');
        assert_eq!(buf.get_cell(Position::new(2, 0)).ch, '+');
        assert_eq!(buf.get_cell(Position::new(0, 1)).ch, '|');
        assert_eq!(buf.get_cell(Position::new(2, 1)).ch, '|');
        assert_eq!(buf.get_cell(Position::new(0, 2)).ch, '+');
        assert_eq!(buf.get_cell(Position::new(1, 2)).ch, '-');
        assert_eq!(buf.get_cell(Position::new(2, 2)).ch, '+');
    }

    #[test]
    fn test_block_draws_inner_widget() {
        // TODO: check the buffer results are correct
        use std::cell::Cell;
        use std::rc::Rc;

        let mut buf = PairedBuffer::new(Size::new(5, 5));
        let drawn = Rc::new(Cell::new(false));
        let block = Block::new(SimpleBorderSet::ASCII);
        let ctx = FrameContext {
            total_time: NoDelta::new(),
            delta: NoDelta::new(),
            buffer: &mut buf,
            display_width: DisplayWidth::default(),
        };

        block.draw(ctx);

        assert!(drawn.get());
    }

    // TODO: add more tests
}
