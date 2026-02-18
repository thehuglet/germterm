use std::{marker::PhantomData, time::Instant};

pub trait FrameTimer {
    type Delta: TimerDelta;
    fn delta(&mut self) -> Self::Delta;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DefaultTimer {
    prev: Instant,
}

impl FrameTimer for DefaultTimer {
    type Delta = f32;
    fn delta(&mut self) -> Self::Delta {
        let now = Instant::now();
        let elapsed = now.duration_since(self.prev).as_secs_f32();
        self.prev = now;
        elapsed
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NoTimer(PhantomData<()>);

impl NoTimer {
    pub(crate) const fn new() -> Self {
        NoTimer(PhantomData)
    }
}

impl FrameTimer for NoTimer {
    type Delta = NoDelta;
    fn delta(&mut self) -> Self::Delta {
        NoDelta(PhantomData)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NoDelta(PhantomData<()>);
impl TimerDelta for NoDelta {}

pub struct Timer<Timer: FrameTimer> {
    pub(crate) timer: Timer,
    pub(crate) delta: Timer::Delta,
}

pub trait TimerDelta: Copy {}
impl TimerDelta for f32 {}
impl TimerDelta for NoTimer {}
