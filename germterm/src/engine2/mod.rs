pub mod buffer;
pub mod draw;
pub mod frame;
pub mod renderer;
pub mod timer;
pub mod widget;

use crossterm::{cursor, event, execute, terminal};
use std::io;

use crate::{
    cell::Cell,
    engine2::{
        buffer::{Buffer, paired::PairedBuffer, slice::SubBuffer},
        draw::{Position, Rect, Size},
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
    /// Creates a new `Engine` with the given timer and buffer.
    pub fn new(timer: Timed, buffer: Buf) -> Self {
        Self {
            timer: TimerWrapper::new(timer, Default::default()),
            buffer,
        }
    }

    /// Returns the delta time from the most recently completed frame.
    ///
    /// On the very first frame this returns `Timed::Delta::default()`.
    pub fn delta(&self) -> Timed::Delta {
        self.timer.previous_delta
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
            delta: self.timer.previous_delta,
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
    /// 1. `Buffer::start_frame` — clear/prepare the buffer for new draw commands.
    /// 2. `update(engine)` — caller draws into the buffer; return `true` to stop.
    /// 3. `Renderer::start_frame` → `Renderer::render(draw_calls)` — emit draw calls from the current frame.
    /// 4. `Buffer::end_frame` — finalise the buffer (diff + swap).
    /// 5. `Renderer::end_frame` — flush/complete the rendered frame.
    ///
    /// # Errors
    ///
    /// Returns an [`io::Error`] if terminal initialization, rendering, or cleanup fails.
    pub fn run<R, F>(&mut self, renderer: &mut R, mut update: F) -> io::Result<()>
    where
        R: renderer::Renderer<Error = io::Error>,
        F: FnMut(&mut Self) -> bool,
    {
        let mut stdout = io::stdout();

        terminal::enable_raw_mode()?;
        execute!(
            stdout,
            terminal::EnterAlternateScreen,
            event::EnableMouseCapture,
            cursor::Hide,
        )?;

        loop {
            self.buffer.start_frame();

            let should_exit = update(self);

            renderer.start_frame()?;
            renderer.render(self.buffer.draw())?;

            self.buffer.end_frame();
            renderer.end_frame()?;

            // Tick the timer after the full frame (update + render) so that
            // `delta()` reflects real frame time rather than just update time.
            self.timer.previous_delta = self.timer.timer.delta();

            if should_exit {
                break;
            }
        }

        terminal::disable_raw_mode()?;
        execute!(
            stdout,
            terminal::LeaveAlternateScreen,
            terminal::EnableLineWrap,
            cursor::Show,
            event::DisableMouseCapture,
        )?;

        Ok(())
    }
}
