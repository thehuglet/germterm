use super::Engine;
use std::time::Instant;

pub trait Timer {
    fn delta(&mut self) -> f32;
}

pub struct DefaultTimer {
    previous_time: Instant,
}

impl DefaultTimer {
    pub fn new() -> Self {
        Self {
            previous_time: Instant::now(),
        }
    }
}

impl Timer for DefaultTimer {
    fn delta(&mut self) -> f32 {
        let now = Instant::now();
        let delta = now.duration_since(self.previous_time).as_secs_f32();
        self.previous_time = now;
        delta
    }
}

/// Marker trait for anything that implements [`Timer`] and `()`.
pub trait TimerMarker {
    type Data;
}

impl TimerMarker for () {
    type Data = ();
}

impl<T: Timer> TimerMarker for T {
    type Data = f32;
}

pub(crate) struct TimerWrapper<T: TimerMarker> {
    pub(crate) timer: T,
    pub(crate) previous_delta: <T as TimerMarker>::Data,
}

impl<T: TimerMarker> TimerWrapper<T> {
    pub(crate) fn new(timer: T, previous_delta: T::Data) -> Self {
        Self {
            timer,
            previous_delta,
        }
    }
}
