pub mod block;

use crate::engine2::{
    buffer::Buffer,
    draw::Size,
    timer::{FrameTimer, NoTimer, TimerDelta},
};

pub trait Widget<Delta: TimerDelta = NoTimer> {
    fn draw(&mut self, ctx: FrameContext<'_, impl Buffer, Delta>);
}

impl<W: Widget> Widget<f32> for W {
    fn draw(&mut self, ctx: FrameContext<'_, impl Buffer, f32>) {
        W::draw(
            self,
            FrameContext {
                delta: NoTimer::new(),
                buffer: ctx.buffer,
            },
        );
    }
}

pub struct FrameContext<'a, Buf: Buffer + ?Sized, Delta = NoTimer> {
    pub(crate) delta: Delta,
    pub(crate) buffer: &'a mut Buf,
}

impl<Buf: Buffer + ?Sized, Delta: TimerDelta> FrameContext<'_, Buf, Delta> {
    #[inline(always)]
    pub fn delta(&self) -> Delta {
        self.delta
    }
}

impl<Buf: Buffer + ?Sized, Delta> FrameContext<'_, Buf, Delta> {
    #[inline(always)]
    pub fn buffer(&self) -> &Buf {
        self.buffer
    }

    #[inline(always)]
    pub fn buffer_mut(&mut self) -> &mut Buf {
        self.buffer
    }
}
