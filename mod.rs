pub mod attributes;
pub mod color;
pub mod fps_limiter;
pub mod input;
pub mod screen;

use std::io;

use crossterm::{cursor, event, execute, terminal};

use crate::engine::{
    fps_limiter::{FpsLimiter, limit_fps, wait_for_next_frame},
    screen::Screen,
};

pub struct Engine {
    pub delta_time: f32,
    title: &'static str,
    stdout: io::Stdout,
    screen: Screen,
    fps_limiter: FpsLimiter,
}

impl Engine {
    pub fn new(cols: u16, rows: u16) -> Self {
        Engine {
            delta_time: 0.01667,
            title: "my-game",
            stdout: io::stdout(),
            screen: Screen::new(cols, rows),
            fps_limiter: FpsLimiter::new(60, 0.001, 0.002),
        }
    }

    pub fn title(mut self, value: &'static str) -> Self {
        self.title = value;
        self
    }

    pub fn limit_fps(mut self, value: u32) -> Self {
        limit_fps(&mut self.fps_limiter, value);
        self
    }
}

pub fn init(engine: &mut Engine) -> io::Result<()> {
    terminal::enable_raw_mode()?;
    execute!(
        engine.stdout,
        terminal::EnterAlternateScreen,
        terminal::DisableLineWrap,
        terminal::SetTitle("term-slots"),
        event::EnableMouseCapture,
        cursor::Hide,
        terminal::SetSize(engine.screen.cols, engine.screen.rows)
    )?;
    Ok(())
}

pub fn exit_cleanup(engine: &mut Engine) {}

pub fn start_frame(engine: &mut Engine) {
    engine.delta_time = wait_for_next_frame(&mut engine.fps_limiter);
}

pub fn end_frame(engine: &mut Engine) {}
