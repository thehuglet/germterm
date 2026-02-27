pub mod set;

use std::marker::PhantomData;

use bitflags::bitflags;

use crate::core::{
    buffer::{Buffer, slice::SubBuffer},
    draw::{Position, Rect},
    timer::TimerDelta,
    widget::{FrameContext, Widget, block::set::BlockSet},
};

bitflags! {
    pub struct BorderSides: u8 {
        const TOP = 0b00000001;
        const RIGHT = 0b00000010;
        const BOTTOM = 0b00000100;
        const LEFT = 0b00001000;
    }
}

pub struct Block<D: TimerDelta, W: Widget<D>, B> {
    widget: W,
    set: B,
    sides: BorderSides,
    _timer: PhantomData<D>,
}

impl<D: TimerDelta, W: Widget<D>, B> Block<D, W, B> {
    pub fn new(widget: W, set: B) -> Self {
        Self {
            widget,
            set,
            sides: BorderSides::all(),
            _timer: PhantomData,
        }
    }
}

impl<D: TimerDelta, W: Widget<D>, B: BlockSet> Widget<D> for Block<D, W, B> {
    fn draw(&mut self, ctx: &mut FrameContext<'_, impl Buffer, D>) {
        let size = ctx.buffer().size();
        if size.area() == 0 {
            return;
        }

        let delta = ctx.delta;
        let total_time = ctx.total_time;
        let buf = ctx.buffer_mut();
        let x_end = size.width.saturating_sub(1).max(1);
        let left_offset = self.sides.contains(BorderSides::LEFT) as u16;
        let right_offset = self.sides.contains(BorderSides::RIGHT) as u16;
        let top_offset = self.sides.contains(BorderSides::TOP) as u16;
        let bottom_offset = self.sides.contains(BorderSides::BOTTOM) as u16;

        // top left corner
        if self.sides.contains(BorderSides::LEFT) && self.sides.contains(BorderSides::TOP) {
            let cur = buf.get_cell_mut(Position::ZERO);
            cur.ch = self
                .set
                .top_left(&cur.ch.to_string())
                .chars()
                .next()
                .unwrap_or_default();
        }

        // top side
        if self.sides.contains(BorderSides::TOP) && size.width > 2 {
            for x in 1..x_end {
                let cur = buf.get_cell_mut(Position { x, y: 0 });
                cur.ch = self
                    .set
                    .top(&cur.ch.to_string())
                    .chars()
                    .next()
                    .unwrap_or_default();
            }
        }

        // top right corner
        if self.sides.contains(BorderSides::RIGHT)
            && self.sides.contains(BorderSides::TOP)
            && size.width > 1
        {
            let cur = buf.get_cell_mut(Position {
                x: size.width - 1,
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
        if size.height > 2 {
            let h_end = size.height.saturating_sub(1).max(1);
            // Left side
            if self.sides.contains(BorderSides::LEFT) {
                for y in 1..h_end {
                    let cur = buf.get_cell_mut(Position { x: 0, y });
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
                for y in 1..h_end {
                    let cur = buf.get_cell_mut(Position {
                        x: size.width - 1,
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
        if self.sides.contains(BorderSides::BOTTOM)
            && self.sides.contains(BorderSides::LEFT)
            && size.height > 1
        {
            let cur = buf.get_cell_mut(Position {
                x: 0,
                y: size.height - 1,
            });
            cur.ch = self
                .set
                .bottom_left(&cur.ch.to_string())
                .chars()
                .next()
                .unwrap_or_default();
        }

        // bottom
        if self.sides.contains(BorderSides::BOTTOM) && size.width > 2 {
            let y = size.height - 1;
            for x in 1..x_end {
                let cur = buf.get_cell_mut(Position { x, y });
                cur.ch = self
                    .set
                    .bottom(&cur.ch.to_string())
                    .chars()
                    .next()
                    .unwrap_or_default();
            }
        }

        // bottom right
        if self.sides.contains(BorderSides::BOTTOM)
            && self.sides.contains(BorderSides::RIGHT)
            && size.width > 1
        {
            let cur = buf.get_cell_mut(Position {
                x: size.width - 1,
                y: size.height - 1,
            });
            cur.ch = self
                .set
                .bottom_right(&cur.ch.to_string())
                .chars()
                .next()
                .unwrap_or_default();
        }

        if size.width > 2 && size.height > 2 {
            self.widget.draw(&mut FrameContext {
                total_time,
                delta,
                buffer: &mut SubBuffer::new(
                    buf,
                    Rect::from_xywh(
                        left_offset,
                        top_offset,
                        size.width - (left_offset + right_offset),
                        size.height - (top_offset + bottom_offset),
                    ),
                ),
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{
        buffer::paired::PairedBuffer, draw::Size, timer::NoDelta,
        widget::block::set::SimpleBorderSet,
    };

    struct EmptyWidget;
    impl Widget<NoDelta> for EmptyWidget {
        fn draw(&mut self, _ctx: &mut FrameContext<'_, impl Buffer, NoDelta>) {}
    }

    #[test]
    fn test_block_draw_borders() {
        let mut buf = PairedBuffer::new(Size::new(3, 3));
        let mut block = Block::new(EmptyWidget, SimpleBorderSet::ASCII);
        let mut ctx = FrameContext {
            total_time: NoDelta::new(),
            delta: NoDelta::new(),
            buffer: &mut buf,
        };

        block.draw(&mut ctx);

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

        struct SpyWidget {
            drawn: Rc<Cell<bool>>,
        }
        impl Widget<NoDelta> for SpyWidget {
            fn draw(&mut self, _ctx: &mut FrameContext<'_, impl Buffer, NoDelta>) {
                self.drawn.set(true);
            }
        }

        let mut buf = PairedBuffer::new(Size::new(5, 5));
        let drawn = Rc::new(Cell::new(false));
        let spy = SpyWidget {
            drawn: drawn.clone(),
        };
        let mut block = Block::new(spy, SimpleBorderSet::ASCII);
        let mut ctx = FrameContext {
            total_time: NoDelta::new(),
            delta: NoDelta::new(),
            buffer: &mut buf,
        };

        block.draw(&mut ctx);

        assert!(drawn.get());
    }

    // TODO: add more tests
}
