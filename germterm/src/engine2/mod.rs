mod buffer;
mod draw;
mod frame;
mod timer;
mod widget;

use crate::{
    cell::Cell,
    engine2::{
        buffer::{paired::PairedBuffer, Buffer},
        draw::{Position, Size},
        timer::{DefaultTimer, TimerMarker, TimerWrapper},
    },
};

pub struct DrawCall<'a> {
    pub pos: Position,
    pub cell: &'a Cell,
}

pub struct Engine<Timed: TimerMarker, Buf: Buffer> {
    timer: TimerWrapper<Timed>,
    buffer: Buf,
}

impl<Timed: TimerMarker, Buf: Buffer> Engine<Timed, Buf> {
    pub fn buffer(&self) -> &Buf {
        &self.buffer
    }

    pub fn buffer_mut(&mut self) -> &mut Buf {
        &mut self.buffer
    }
}

impl Engine<DefaultTimer, PairedBuffer> {
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            timer: TimerWrapper::new(DefaultTimer::new(), 0.0),
            buffer: PairedBuffer::new(Size::new(width, height)),
        }
    }
}
