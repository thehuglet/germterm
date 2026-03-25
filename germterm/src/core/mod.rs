pub mod buffer;
pub mod draw;
pub mod renderer;
pub mod timer;
pub mod widget;

use std::{io::Write, ops::ControlFlow};

use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};

use crate::{
    cell::Cell,
    core::{
        buffer::{Buffer, diffed::DiffedBuffers, flat::FlatBuffer, slice::SubBuffer},
        draw::{Position, Rect, Size},
        renderer::crossterm::CrosstermRenderer,
        timer::{DefaultTimer, FrameTimer, Timer},
        widget::{FrameContext, Widget},
    },
};

#[derive(Clone, Debug)]
pub struct DrawCall<'a> {
    pub pos: Position,
    pub cell: &'a Cell,
}

#[derive(Clone, Copy, Debug, Hash)]
pub struct DisplayWidth {
    char_width: fn(char) -> u16,
    str_width: fn(&str) -> u16,
}

impl DisplayWidth {
    #[inline(always)]
    pub fn char_width(self, ch: char) -> u16 {
        (self.char_width)(ch)
    }

    #[inline(always)]
    pub fn str_width(self, s: &str) -> u16 {
        (self.str_width)(s)
    }
}

impl Default for DisplayWidth {
    fn default() -> Self {
        Self {
            char_width: |c| UnicodeWidthChar::width(c).unwrap_or(0) as u16,
            str_width: |s| UnicodeWidthStr::width(s) as u16,
        }
    }
}

pub struct Engine<Timed: FrameTimer, Buf> {
    timer: Timer<Timed>,
    buffer: Buf,
    display_width: DisplayWidth,
}

impl<Timed: FrameTimer, Buf: Buffer> Engine<Timed, Buf> {
    /// Creates a new `Engine` with the given timer and buffer.
    pub fn new(timer: Timed, buffer: Buf, display_width: DisplayWidth) -> Self
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
            display_width,
        }
    }

    /// Returns the delta time from the most recently completed frame.
    ///
    /// On the very first frame this returns `Timed::Delta::default()`.
    pub fn delta(&self) -> Timed::Delta {
        self.timer.delta
    }

    pub fn total_time(&self) -> Timed::Delta {
        self.timer.total_time
    }

    pub fn buffer(&self) -> &Buf {
        &self.buffer
    }

    pub fn buffer_mut(&mut self) -> &mut Buf {
        &mut self.buffer
    }
}

impl<Timed: FrameTimer, Buf: Buffer> Engine<Timed, Buf> {
    pub fn draw(&mut self, area: Rect, widget: impl Widget<Timed::Delta>) {
        let fc = FrameContext {
            total_time: self.timer.total_time,
            delta: self.timer.delta,
            buffer: &mut SubBuffer::new(&mut self.buffer, area),
            display_width: self.display_width,
        };

        widget.draw(fc);
    }
}

impl<Timed: FrameTimer, Buf: Buffer + buffer::Drawer> Engine<Timed, Buf> {
    /// Runs the engine loop.
    ///
    /// Initializes the terminal, then repeatedly calls `update` until it returns
    /// [`ControlFlow::Break`], at which point the loop exits, the terminal is
    /// restored, and the break value is returned.
    ///
    /// Each iteration follows this order:
    /// 1. `Buffer::start_frame` - clear/prepare the buffer for new draw commands.
    /// 2. `update(&mut engine)` - caller draws into the buffer; return
    ///    `ControlFlow::Break(value)` to stop.
    /// 3. `Renderer::start_frame` -> `Renderer::render(draw_calls)` - diff the
    ///    buffer and emit only changed cells to the renderer.
    /// 4. `Buffer::end_frame` - swap the current and previous frames so the
    ///    just-rendered frame becomes the baseline for the next diff.
    /// 5. `Renderer::end_frame` - flush/complete the rendered frame.
    ///
    /// # Errors
    ///
    /// Returns a renderer error if terminal initialization, rendering, or cleanup
    /// fails.
    pub fn run<R, Bre>(
        &mut self,
        renderer: &mut R,
        mut update: impl FnMut(&mut Self) -> ControlFlow<Bre>,
    ) -> Result<Bre, R::Error>
    where
        R: renderer::Renderer,
    {
        renderer.init()?;

        // Ideally we would catch panics and restore but that means
        // [`std::panic::catch_unwind`] must be used.
        //
        // Since this is intended to be in the core of the library prefer to use
        // core features.
        let res = || -> Result<Bre, R::Error> {
            loop {
                self.buffer.start_frame();

                let should_exit = update(self);

                renderer.start_frame()?;
                renderer.render(self.buffer.draw())?;

                self.buffer.end_frame();
                renderer.end_frame()?;

                self.timer.update();

                if let ControlFlow::Break(bre) = should_exit {
                    break Ok(bre);
                }
            }
        }();

        match renderer.restore() {
            Ok(()) => res,
            Err(err) => res.map_err(|_| err),
        }
    }
}

pub fn run(
    w: &mut impl Write,
    size: Size,
    update: impl FnMut(
        &mut Engine<DefaultTimer, DiffedBuffers<FlatBuffer>>,
    ) -> ControlFlow<std::io::Result<()>>,
) -> std::io::Result<()> {
    let mut eng = Engine::new(
        DefaultTimer::new(),
        DiffedBuffers::new(size, FlatBuffer::new(size), FlatBuffer::new(size)),
        Default::default(),
    );

    eng.run(&mut CrosstermRenderer::new(w), update)?
}
