mod buffer;
mod draw;
mod frame;
mod timer;

use crate::{
    cell::Cell,
    engine2::{
        buffer::{Buffer, ErrorOutOfBoundsAxises, paired::PairedBuffer},
        timer::{DefaultTimer, Timer, TimerMarker, TimerWrapper},
    },
};

#[derive(Clone, Copy, Debug)]
pub struct Position {
    pub x: u16,
    pub y: u16,
}

#[derive(Clone, Copy, Debug)]
pub struct Size {
    pub width: u16,
    pub height: u16,
}

impl Size {
    fn contains(&self, pos: Position) -> Result<(), ErrorOutOfBoundsAxises> {
        let err = match pos {
            Position { x, y } if x >= self.width && y >= self.height => ErrorOutOfBoundsAxises::XY,
            Position { x, y } if y >= self.height => ErrorOutOfBoundsAxises::Y,
            Position { x, y } if x >= self.width => ErrorOutOfBoundsAxises::X,
            _ => return Ok(()),
        };

        Err(err)
    }
}

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
            buffer: PairedBuffer::new(width, height),
        }
    }
}

trait TimedEngine {
    fn next_delta(&mut self) -> f32;
    fn current_delta(&self) -> f32;
}

impl<T: Timer, Buf: Buffer> TimedEngine for Engine<T, Buf> {
    fn next_delta(&mut self) -> f32 {
        self.timer.timer.delta()
    }

    fn current_delta(&self) -> f32 {
        self.timer.previous_delta
    }
}
