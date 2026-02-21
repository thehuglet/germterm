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

pub struct Timer<Timer: FrameTimer> {
    pub(crate) timer: Timer,
    pub(crate) total_time: Timer::Delta,
    pub(crate) delta: Timer::Delta,
}

impl<T: FrameTimer> Timer<T> {
    pub(crate) fn update(&mut self) {
        let delta = self.timer.delta();
        self.total_time = TimerDelta::total(self.total_time, delta);
        self.delta = delta;
    }
}

pub type Delta = f32;
// TODO: maybe seal this trait??
pub trait TimerDelta: Copy {
    fn total(lhs: Self, rhs: Self) -> Self;
}
impl TimerDelta for Delta {
    fn total(lhs: Self, rhs: Self) -> Self {
        lhs + rhs
    }
}
impl TimerDelta for NoDelta {
    fn total(_lhs: Self, _rhs: Self) -> Self {
        Self::new()
    }
}
