use crate::engine2::{
    buffer::Buffer,
    draw::Size,
    timer::{NoTimer, TimerDelta},
};

pub trait Widget<Delta = NoTimer> {
    fn draw(&mut self, ctx: FrameContext<'_, impl Buffer, Delta>);
}

// A non-timer widget can be used wherever an f64-timed widget is expected.
// It can also be used where a f32-timed widget is expected due to the blanket impl below.
impl<W: Widget<NoTimer>> Widget<f64> for W {
    fn draw(&mut self, ctx: FrameContext<'_, impl Buffer, f64>) {
        <W as Widget<NoTimer>>::draw(
            self,
            FrameContext {
                delta: NoTimer,
                size: ctx.size,
                buffer: ctx.buffer,
            },
        );
    }
}

// An f64-timed widget can be used wherever an f32-timed widget is expected.
// Conversion is lossless: f32 is widened to f64.
impl<W: Widget<f64>> Widget<f32> for W {
    fn draw(&mut self, ctx: FrameContext<'_, impl Buffer, f32>) {
        <W as Widget<f64>>::draw(
            self,
            FrameContext {
                delta: ctx.delta as f64,
                size: ctx.size,
                buffer: ctx.buffer,
            },
        );
    }
}

pub struct FrameContext<'a, Buf: Buffer + ?Sized, Delta = NoTimer> {
    pub(crate) delta: Delta,
    pub(crate) size: Size,
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

#[cfg(test)]
mod tests {
    use crate::engine2::widget::Widget;

    struct SampleWidget;
    impl Widget for SampleWidget {
        fn draw(
            &mut self,
            _ctx: super::FrameContext<
                '_,
                impl crate::engine2::buffer::Buffer,
                crate::engine2::timer::NoTimer,
            >,
        ) {
            todo!()
        }
    }
}
