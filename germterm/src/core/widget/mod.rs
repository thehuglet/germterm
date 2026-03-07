//! Widgets for rendering UI components to the screen.
//!
//! This module provides traits for building terminal UI components.
//! Use [`SimpleWidget`] for static content and [`Widget`] when you need
//! animation or timing information.
//!
//! # Examples
//!
//! Static widget that doesn't need timing:
//! ```rust,ignore
//! struct Label { text: String }
//!
//! impl SimpleWidget for Label {
//!     fn draw(&self, ctx: FrameContext<'_, impl Buffer>) {
//!         // Draw a static widget
//!     }
//! }
//! ```
//!
//! Animated widget that uses timing:
//! ```rust,ignore
//! struct Spinner { /* ... */ }
//!
//! impl Widget for Spinner {
//!     fn draw(&self, ctx: FrameContext<'_, impl Buffer, f32>) {
//!         let delta = ctx.delta();
//!         // Use delta for animation
//!     }
//! }
//! ```

pub mod block;
pub mod text;

use crate::core::{
    DisplayWidth,
    buffer::Buffer,
    timer::{NoDelta, TimerDelta},
};

/// A widget for static content that doesn't require timing information.
///
/// Automatically implements [`Widget<T>`] for any `T`, allowing simple widgets
/// to be used anywhere a [`Widget`] is expected.
pub trait SimpleWidget {
    /// Draw the widget to the buffer.
    fn draw(&self, ctx: FrameContext<'_, impl Buffer>);
}

impl<W: SimpleWidget, T: TimerDelta> Widget<T> for W {
    fn draw(&self, mut ctx: FrameContext<'_, impl Buffer, T>) {
        SimpleWidget::draw(
            self,
            FrameContext {
                total_time: NoDelta::new(),
                delta: NoDelta::new(),
                display_width: ctx.display_width,
                buffer: ctx.buffer_mut(),
            },
        );
    }
}

/// A widget that can be rendered to the screen.
///
/// Use `NoDelta` for static content or `Delta` (`f32`) when you need
/// animation timing. Defaults to `Delta`.
///
/// If implementing a static widget (`Widget<NoDelta>`), [`SimpleWidget`] should be implemented instead.
/// SimpleWidget has blanket impls that simplify the use in container widgets.
///
/// # Type Parameters
///
/// - `Delta`: Timing information available during rendering.
///   - `NoDelta`: No timing (static widgets)
///   - `Delta` (`f32`): Frame delta time in seconds (animated widgets)
pub trait Widget<Delta: TimerDelta = crate::core::timer::Delta> {
    /// Draw the widget to the buffer.
    fn draw(&self, ctx: FrameContext<'_, impl Buffer, Delta>);
}

/// Rendering context passed to widgets during drawing, carrying the buffer,
/// timing data, and display-width info.
pub struct FrameContext<'a, Buf: Buffer + ?Sized, Delta = NoDelta> {
    pub(crate) total_time: Delta,
    pub(crate) delta: Delta,
    pub(crate) buffer: &'a mut Buf,
    pub(crate) display_width: DisplayWidth,
}

impl<Buf: Buffer + ?Sized, Delta: TimerDelta> FrameContext<'_, Buf, Delta> {
    /// Returns the elapsed time since the application started.
    #[inline(always)]
    pub fn total_time(&self) -> Delta {
        self.total_time
    }

    /// Returns the time elapsed since the last frame.
    #[inline(always)]
    pub fn delta(&self) -> Delta {
        self.delta
    }
}

impl<'a, Buf: Buffer + ?Sized, Delta> FrameContext<'a, Buf, Delta> {
    /// Creates a new `FrameContext`.
    pub fn new(
        total_time: Delta,
        delta: Delta,
        buffer: &'a mut Buf,
        display_width: DisplayWidth,
    ) -> Self {
        Self {
            total_time,
            delta,
            buffer,
            display_width,
        }
    }

    /// Returns a shared reference to the underlying buffer.
    #[inline(always)]
    pub fn buffer(&self) -> &Buf {
        self.buffer
    }

    /// Returns a mutable reference to the underlying buffer.
    #[inline(always)]
    pub fn buffer_mut(&mut self) -> &mut Buf {
        self.buffer
    }
}
