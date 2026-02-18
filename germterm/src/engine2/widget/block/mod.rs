pub mod set;

use std::marker::PhantomData;

use crate::engine2::{
    buffer::{Buffer, slice::SubBuffer},
    draw::{Position, Rect},
    timer::TimerDelta,
    widget::{FrameContext, Widget, block::set::BlockSet},
};

pub struct Block<D: TimerDelta, W: Widget<D>, B> {
    pub widget: W,
    pub border_set: B,
    _timer: PhantomData<D>,
}

impl<D: TimerDelta, W: Widget<D>, B> Block<D, W, B> {
    pub fn new(widget: W, border_set: B) -> Self {
        Self {
            widget,
            border_set,
            _timer: PhantomData,
        }
    }
}

impl<D: TimerDelta, W: Widget<D>, B: BlockSet> Widget<D> for Block<D, W, B> {
    fn draw(&mut self, mut ctx: FrameContext<'_, impl Buffer, D>) {
        let size = ctx.buffer().size();
        if size.width == 0 || size.height == 0 {
            return;
        }

        let delta = ctx.delta;
        let total_time = ctx.total_time;
        let buf = ctx.buffer_mut();

        // top left corner
        {
            let cur = buf.get_cell_mut(Position::ZERO);
            cur.ch = self
                .border_set
                .top_left(&cur.ch.to_string())
                .chars()
                .next()
                .unwrap_or_default();
        }

        // top side
        if size.width > 2 {
            for x in 1..size.width {
                let cur = buf.get_cell_mut(Position { x, y: 0 });
                cur.ch = self
                    .border_set
                    .top(&cur.ch.to_string())
                    .chars()
                    .next()
                    .unwrap_or_default();
            }
        }

        // top right corner
        if size.width > 1 {
            let cur = buf.get_cell_mut(Position {
                x: size.width - 1,
                y: 0,
            });
            cur.ch = self
                .border_set
                .top_right(&cur.ch.to_string())
                .chars()
                .next()
                .unwrap_or_default();
        }

        // LR sides
        if size.height > 2 {
            // Left side
            for y in 1..size.height {
                let cur = buf.get_cell_mut(Position { x: 0, y });
                cur.ch = self
                    .border_set
                    .left(&cur.ch.to_string())
                    .chars()
                    .next()
                    .unwrap_or_default();
            }

            // Right side
            for y in 1..size.height {
                let cur = buf.get_cell_mut(Position {
                    x: size.width - 1,
                    y,
                });
                cur.ch = self
                    .border_set
                    .right(&cur.ch.to_string())
                    .chars()
                    .next()
                    .unwrap_or_default();
            }
        }

        // bottom left
        if size.height > 1 {
            let cur = buf.get_cell_mut(Position {
                x: 0,
                y: size.height - 1,
            });
            cur.ch = self
                .border_set
                .bottom_left(&cur.ch.to_string())
                .chars()
                .next()
                .unwrap_or_default();
        }

        // bottom
        if size.width > 2 {
            let y = size.height - 1;
            for x in 1..size.width {
                let cur = buf.get_cell_mut(Position { x, y });
                cur.ch = self
                    .border_set
                    .bottom(&cur.ch.to_string())
                    .chars()
                    .next()
                    .unwrap_or_default();
            }
        }

        // bottom right
        if size.width > 1 {
            let cur = buf.get_cell_mut(Position {
                x: size.width - 1,
                y: size.height - 1,
            });
            cur.ch = self
                .border_set
                .bottom_right(&cur.ch.to_string())
                .chars()
                .next()
                .unwrap_or_default();
        }

        if size.width > 2 && size.height > 2 {
            self.widget.draw(FrameContext {
                total_time,
                delta,
                buffer: &mut SubBuffer::new(
                    buf,
                    Rect::from_xywh(1, 1, size.width - 2, size.height - 2),
                ),
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine2::buffer::paired::PairedBuffer;
    use crate::engine2::draw::Size;
    use crate::engine2::timer::NoTimer;
    use crate::engine2::widget::block::set::SimpleBorderSet;

    struct EmptyWidget;
    impl Widget<NoTimer> for EmptyWidget {
        fn draw(&mut self, _ctx: FrameContext<'_, impl Buffer, crate::engine2::timer::NoTimer>) {}
    }

    #[test]
    fn test_block_draw_borders() {
        let mut buf = PairedBuffer::new(Size::new(3, 3));
        let mut block = Block::new(EmptyWidget, SimpleBorderSet::ASCII);
        let ctx = FrameContext {
            total_time: NoTimer::new(),
            delta: NoTimer::new(),
            buffer: &mut buf,
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

        struct SpyWidget {
            drawn: Rc<Cell<bool>>,
        }
        impl Widget<NoTimer> for SpyWidget {
            fn draw(&mut self, _ctx: FrameContext<'_, impl Buffer, NoTimer>) {
                self.drawn.set(true);
            }
        }

        let mut buf = PairedBuffer::new(Size::new(5, 5));
        let drawn = Rc::new(Cell::new(false));
        let spy = SpyWidget {
            drawn: drawn.clone(),
        };
        let mut block = Block::new(spy, SimpleBorderSet::ASCII);
        let ctx = FrameContext {
            total_time: NoTimer::new(),
            delta: NoTimer::new(),
            buffer: &mut buf,
        };

        block.draw(ctx);

        assert!(drawn.get());
    }
}
