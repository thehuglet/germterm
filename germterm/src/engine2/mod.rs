pub mod buffer;
pub mod draw;
pub mod frame;
pub mod renderer;
pub mod timer;
pub mod widget;

use crate::{
    cell::Cell,
    engine2::{
        buffer::{Buffer, slice::SubBuffer},
        draw::{Position, Rect},
        timer::{FrameTimer, Timer},
        widget::{FrameContext, Widget},
    },
};

pub struct DrawCall<'a> {
    pub pos: Position,
    pub cell: &'a Cell,
}

pub struct Engine<Timed: FrameTimer, Buf> {
    timer: Timer<Timed>,
    buffer: Buf,
}

impl<Timed: FrameTimer, Buf: Buffer> Engine<Timed, Buf> {
    /// Creates a new `Engine` with the given timer and buffer.
    pub fn new(timer: Timed, buffer: Buf) -> Self
    where
        Timed::Delta: Default,
    {
        Self {
            timer: Timer {
                timer,
                total_time: Default::default(),
                delta: Default::default(),
            },
            buffer,
        }
    }

    /// Returns the delta time from the most recently completed frame.
    ///
    /// On the very first frame this returns `Timed::Delta::default()`.
    pub fn delta(&self) -> Timed::Delta {
        self.timer.delta
    }

    pub fn buffer(&self) -> &Buf {
        &self.buffer
    }

    pub fn buffer_mut(&mut self) -> &mut Buf {
        &mut self.buffer
    }
}

impl<Timed: FrameTimer, Buf: Buffer> Engine<Timed, Buf> {
    fn draw(&mut self, area: Rect, mut widget: impl Widget<Timed::Delta>) {
        let fc = FrameContext {
            total_time: self.timer.total_time,
            delta: self.timer.delta,
            buffer: &mut SubBuffer::new(&mut self.buffer, area),
        };

        widget.draw(fc);
    }
}

impl<Timed: FrameTimer, Buf: Buffer + buffer::Drawer> Engine<Timed, Buf> {
    /// Runs the engine loop.
    ///
    /// Initializes the terminal, then repeatedly calls `update` until it returns `true`,
    /// at which point the loop breaks and the terminal is restored.
    ///
    /// Each iteration follows this order:
    /// 1. `Buffer::start_frame` - clear/prepare the buffer for new draw commands.
    /// 2. `update(&mut engine)` - caller draws into the buffer; return `true` to stop.
    /// 3. `Renderer::start_frame` -> `Renderer::render(draw_calls)` - diff the buffer
    ///    and emit only changed cells to the renderer.
    /// 4. `Buffer::end_frame` - swap the current and previous frames so the
    ///    just-rendered frame becomes the baseline for the next diff.
    /// 5. `Renderer::end_frame` - flush/complete the rendered frame.
    ///
    /// # Errors
    ///
    /// Returns an [`io::Error`] if terminal initialization, rendering, or cleanup fails.
    pub fn run<R, F>(&mut self, renderer: &mut R, mut update: F) -> Result<(), R::Error>
    where
        R: renderer::Renderer,
        F: FnMut(&mut Self) -> bool,
    {
        renderer.init()?;

        // Ideally we would catch panics and restore but that means [`std::panic::catch_unwind`]
        // must be used.
        //
        // Since this is intended to be in the core of the library pefer to use core features
        let res = || -> Result<(), R::Error> {
            loop {
                self.buffer.start_frame();

                let should_exit = update(self);

                if let err @ Err(_) = renderer.start_frame() {
                    return err;
                }
                if let err @ Err(_) = renderer.render(self.buffer.draw()) {
                    return err;
                }

                self.buffer.end_frame();
                if let err @ Err(_) = renderer.end_frame() {
                    return err;
                }

                self.timer.update();
                if should_exit {
                    break;
                }
            }

            Ok(())
        }();

        res.and(renderer.restore())?;

        Ok(())
    }
}
