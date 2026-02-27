pub mod block;
pub mod text;

use crate::core::{
    buffer::Buffer,
    timer::{Delta, NoDelta, TimerDelta},
};

pub trait Widget<Delta: TimerDelta = NoDelta> {
    fn draw(&mut self, ctx: &mut FrameContext<'_, impl Buffer, Delta>);
}

impl<W: Widget> Widget<Delta> for W {
    fn draw(&mut self, ctx: &mut FrameContext<'_, impl Buffer, Delta>) {
        W::draw(
            self,
            &mut FrameContext {
                total_time: NoDelta::new(),
                delta: NoDelta::new(),
                buffer: ctx.buffer,
            },
        );
    }
}

pub struct FrameContext<'a, Buf: Buffer + ?Sized, Delta = NoDelta> {
    pub(crate) total_time: Delta,
    pub(crate) delta: Delta,
    pub(crate) buffer: &'a mut Buf,
}

impl<Buf: Buffer + ?Sized, Delta: TimerDelta> FrameContext<'_, Buf, Delta> {
    #[inline(always)]
    pub fn total_time(&self) -> Delta {
        self.total_time
    }

    #[inline(always)]
    pub fn delta(&self) -> Delta {
        self.delta
    }
}

impl<'a, Buf: Buffer + ?Sized, Delta> FrameContext<'a, Buf, Delta> {
    /// Creates a new `FrameContext`.
    pub fn new(total_time: Delta, delta: Delta, buffer: &'a mut Buf) -> Self {
        Self {
            total_time,
            delta,
            buffer,
        }
    }

    #[inline(always)]
    pub fn buffer(&self) -> &Buf {
        self.buffer
    }

    #[inline(always)]
    pub fn buffer_mut(&mut self) -> &mut Buf {
        self.buffer
    }
}
