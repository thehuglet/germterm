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
impl NoDelta {
    pub(crate) const fn new() -> Self {
        Self(PhantomData)
    }
}
impl TimerDelta for NoDelta {}

pub struct Timer<Timer: FrameTimer> {
    pub(crate) timer: Timer,
    pub(crate) total_time: Timer::Delta,
    pub(crate) delta: Timer::Delta,
}

pub type Delta = f32;
// TODO: maybe seal this trait??
pub trait TimerDelta: Copy {}
impl TimerDelta for Delta {}
impl TimerDelta for NoTimer {}
