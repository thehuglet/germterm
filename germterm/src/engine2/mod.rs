pub mod buffer;
pub mod draw;
pub mod frame;
pub mod timer;
pub mod widget;

use crate::{
    cell::Cell,
    engine2::{
        buffer::{Buffer, paired::PairedBuffer, slice::SubBuffer},
        draw::{Position, Rect, Size},
        layer::{LayerIndex, Layers},
        timer::{DefaultTimer, FrameTimer, TimerWrapper},
        widget::{FrameContext, Widget},
    },
};

pub struct DrawCall<'a> {
    pub pos: Position,
    pub cell: &'a Cell,
}

pub struct Engine<Timed: FrameTimer, Buf> {
    timer: TimerWrapper<Timed>,
    buffer: Buf,
}

impl<Timed: FrameTimer, Buf: Buffer> Engine<Timed, Buf> {
    pub fn buffer(&self) -> &Buf {
        &self.buffer
    }

    pub fn buffer_mut(&mut self) -> &mut Buf {
        &mut self.buffer
    }
}

impl<Timed: FrameTimer, Layered: Layers> Engine<Timed, Layered> {
    pub fn draw_at(&mut self, index: LayerIndex, area: Rect, widget: impl Widget<Timed::Delta>) {
        let li = self.buffer.current_layer_index();
        self.buffer.set_layer(index);
        self.draw(area, widget);

        self.buffer.set_layer(li);
    }
}

impl<Timed: FrameTimer, Buf: Buffer> Engine<Timed, Buf> {
    fn draw(&mut self, area: Rect, mut widget: impl Widget<Timed::Delta>) {
        let fc = FrameContext {
            delta: self.timer.previous_delta,
            buffer: &mut SubBuffer::new(&mut self.buffer, area),
        };

        widget.draw(fc);
    }
}
