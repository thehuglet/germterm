pub mod block;

use crate::engine2::{
    buffer::Buffer,
    timer::{NoTimer, TimerDelta},
};

pub trait Widget<Delta: TimerDelta = NoTimer> {
    fn draw(&mut self, ctx: FrameContext<'_, impl Buffer, Delta>);
}

impl<W: Widget> Widget<f32> for W {
    fn draw(&mut self, ctx: FrameContext<'_, impl Buffer, f32>) {
        W::draw(
            self,
            FrameContext {
                total_time: NoTimer::new(),
                delta: NoTimer::new(),
                buffer: ctx.buffer,
            },
        );
    }
}

pub struct FrameContext<'a, Buf: Buffer + ?Sized, Delta = NoTimer> {
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
