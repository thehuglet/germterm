use std::time::Instant;

/// A source of frame delta time.
///
/// Implementors produce a [`Delta`](Timer::Delta) value on each call to
/// [`delta`](Timer::delta), representing the time elapsed since the previous
/// call. The associated type allows both real timers (which return `f32` or
/// `f64` seconds) and the no-op [`NoTimer`] (which returns [`NoTimer`]
/// itself) to share the same trait.
pub trait Timer {
    /// The type of value produced by [`delta`](Timer::delta).
    ///
    /// For real timers this is typically `f32` or `f64` (elapsed seconds).
    /// For [`NoTimer`] this is [`NoTimer`] itself.
    type Delta: Default;

    /// Returns the time elapsed since the last call to this method, expressed
    /// as a value of type [`Delta`](Timer::Delta).
    ///
    /// Calling this method updates the timer's internal state, so successive
    /// calls return the time between *those* calls, not since construction.
    fn delta(&mut self) -> Self::Delta;
}

/// A wall-clock timer that measures elapsed time between frames.
///
/// `DefaultTimer` records the [`Instant`] at which it was last polled and
/// returns the elapsed time in seconds as an `f32` on each call to
/// [`delta`](Timer::delta).
pub struct DefaultTimer {
    previous_time: Instant,
}

impl DefaultTimer {
    /// Creates a new `DefaultTimer`, capturing the current instant as the
    /// starting point for the first delta measurement.
    pub fn new() -> Self {
        Self {
            previous_time: Instant::now(),
        }
    }
}

impl Default for DefaultTimer {
    fn default() -> Self {
        Self::new()
    }
}

impl Timer for DefaultTimer {
    type Delta = f32;

    /// Returns the number of seconds elapsed since the last call as an `f32`,
    /// and updates the internal timestamp to now.
    fn delta(&mut self) -> Self::Delta {
        let now = Instant::now();
        let delta = now.duration_since(self.previous_time).as_secs_f32();
        self.previous_time = now;
        delta
    }
}

/// A zero-cost placeholder for widgets that do not require a timer.
///
/// `NoTimer` implements [`Timer`] with `Delta = NoTimer`, so it can be used
/// anywhere a `Timer` is expected without introducing any runtime overhead.
/// The [`Widget`](crate::engine2::widget::Widget) trait defaults its `Delta`
/// parameter to `NoTimer`, meaning most widgets need not mention timers at all.
#[repr(transparent)]
#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct NoTimer;

impl Timer for NoTimer {
    type Delta = NoTimer;

    /// Returns [`NoTimer`] immediately. No time measurement is performed.
    fn delta(&mut self) -> NoTimer {
        NoTimer
    }
}

/// Wraps a [`Timer`] and caches the most recent delta value.
pub(crate) struct TimerWrapper<T: Timer> {
    pub(crate) timer: T,
    /// The delta value produced by the most recent call to [`Timer::delta`].
    pub(crate) previous_delta: T::Delta,
}

impl<T: Timer> TimerWrapper<T> {
    /// Creates a new `TimerWrapper` with the given timer and an initial cached
    /// delta (typically [`Default::default`]).
    pub const fn new(timer: T, previous_delta: T::Delta) -> Self {
        Self {
            timer,
            previous_delta,
        }
    }
}

/// Marker trait for numeric delta types that can be passed through a
/// [`FrameContext`](crate::engine2::widget::FrameContext).
///
/// Implemented for `f32` and `f64`. This bound is used to restrict
/// [`FrameContext::delta`](crate::engine2::widget::FrameContext::delta) to
/// contexts where a real numeric delta is available, excluding [`NoTimer`].
pub trait TimerDelta: Clone + Copy {}
impl TimerDelta for f32 {}
impl TimerDelta for f64 {}
