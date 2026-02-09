mod buffer;
mod frame;
mod timer;

use crate::{
    cell::Cell,
    engine2::{
        buffer::{Buffer, paired::PairedBuffer},
        timer::{DefaultTimer, Timer, TimerMarker, TimerWrapper},
    },
};

#[derive(Clone, Copy, Debug)]
pub struct Position {
    pub x: u16,
    pub y: u16,
}

pub struct DrawCall<'a> {
    pub x: u16,
    pub y: u16,
    pub cell: &'a Cell,
}

pub struct Engine<Timed: TimerMarker, Buf: Buffer> {
    timer: TimerWrapper<Timed>,
    buffer: Buf,
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
