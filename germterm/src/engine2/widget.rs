use crate::engine2::{
    buffer::Buffer,
    timer::{Timer, TimerMarker},
};

pub trait Widget<Timed: TimerMarker = ()> {
    fn draw(&mut self, ctx: FrameContext<'_, impl Buffer, Timed>);
}

// Compatibility: widgets that don't need a timer (Widget<Buf, ()>)
// automatically work in timed contexts (Widget<Buf, Timed: Timer>).
// The timer data is simply discarded.
impl<W, Timed> Widget<Timed> for W
where
    W: Widget<()>,
    Timed: Timer,
{
    fn draw(&mut self, ctx: FrameContext<'_, impl Buffer, Timed>) {
        <W as Widget<()>>::draw(
            self,
            FrameContext {
                delta: (),
                buffer: ctx.buffer,
            },
        );
    }
}

pub struct FrameContext<'a, Buf: Buffer + ?Sized, Timed: TimerMarker = ()> {
    pub(crate) delta: Timed::Data,
    pub(crate) buffer: &'a mut Buf,
}

impl <Buf: Buffer +?Sized, Timed: Timer>FrameContext<'_, Buf, Timed> {
    pub fn delta(&self) -> f32 {
        self.delta
    }
}
