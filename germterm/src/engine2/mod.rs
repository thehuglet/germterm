pub mod buffer;
pub mod draw;
pub mod frame;
pub mod timer;
pub mod widget;

use crate::{
    cell::Cell,
    engine2::{
        buffer::{Buffer, slice::SubBuffer},
        draw::{Position, Rect},
        timer::{FrameTimer, Timer},
        widget::{FrameContext, Widget},
    },
};

pub struct DrawCall<'a> {
    pub pos: Position,
    pub cell: &'a Cell,
}

pub struct Engine<Timed: FrameTimer, Buf> {
    timer: Timer<Timed>,
    buffer: Buf,
}

impl<Timed: FrameTimer, Buf: Buffer> Engine<Timed, Buf> {
    pub fn buffer(&self) -> &Buf {
        &self.buffer
    }

    pub fn buffer_mut(&mut self) -> &mut Buf {
        &mut self.buffer
    }
}

impl<Timed: FrameTimer, Buf: Buffer> Engine<Timed, Buf> {
    fn draw(&mut self, area: Rect, mut widget: impl Widget<Timed::Delta>) {
        let fc = FrameContext {
            total_time: self.timer.total_time,
            delta: self.timer.delta,
            buffer: &mut SubBuffer::new(&mut self.buffer, area),
        };

        widget.draw(fc);
    }
}
