use crate::{
    fps_counter::{FpsCounter, update_fps_counter},
    fps_limiter::{self, FpsLimiter, wait_for_next_frame},
    frame::{Frame, compose_frame_buffer, copy_frame_buffer, diff_frame_buffers, draw_to_terminal},
    particle::{ParticleState, update_and_draw_particles},
};
use crossterm::{cursor, event, execute, terminal};
use std::io::{self};

pub struct Engine {
    pub delta_time: f32,
    pub game_time: f32,
    title: &'static str,
    stdout: io::Stdout,
    pub(crate) frame: Frame,
    pub(crate) fps_limiter: FpsLimiter,
    pub(crate) fps_counter: FpsCounter,
    pub(crate) particle_state: Vec<ParticleState>,
}

impl Engine {
    pub fn new(cols: u16, rows: u16) -> Self {
        Self {
            delta_time: 0.01667,
            game_time: 0.0,
            title: "my-awesome-terminal",
            stdout: io::stdout(),
            frame: Frame::new(cols, rows),
            fps_limiter: FpsLimiter::new(60, 0.001, 0.002),
            fps_counter: FpsCounter::new(0.08),
            particle_state: Vec::with_capacity(512),
        }
    }

    pub fn title(mut self, value: &'static str) -> Self {
        self.title = value;
        self
    }

    /// A value of `0` will result in uncapped FPS.
    pub fn limit_fps(mut self, value: u32) -> Self {
        fps_limiter::limit_fps(&mut self.fps_limiter, value);
        self
    }
}

pub fn init(engine: &mut Engine) -> io::Result<()> {
    terminal::enable_raw_mode()?;
    execute!(
        engine.stdout,
        terminal::EnterAlternateScreen,
        terminal::DisableLineWrap,
        terminal::SetTitle(engine.title),
        event::EnableMouseCapture,
        cursor::Hide,
        terminal::SetSize(engine.frame.cols, engine.frame.rows)
    )?;
    Ok(())
}

pub fn exit_cleanup(engine: &mut Engine) -> io::Result<()> {
    terminal::disable_raw_mode()?;
    execute!(
        engine.stdout,
        terminal::LeaveAlternateScreen,
        cursor::Show,
        event::DisableMouseCapture
    )?;
    Ok(())
}

pub fn start_frame(engine: &mut Engine) {
    engine.delta_time = wait_for_next_frame(&mut engine.fps_limiter);
    update_fps_counter(&mut engine.fps_counter, engine.delta_time);
}

pub fn end_frame(engine: &mut Engine) -> io::Result<()> {
    // Particles are always drawn on top due to engine limitations
    update_and_draw_particles(
        &mut engine.particle_state,
        &mut engine.frame.draw_queue,
        engine.delta_time,
        engine.game_time,
    );

    compose_frame_buffer(
        &mut engine.frame.current_frame_buffer,
        &engine.frame.draw_queue,
        engine.frame.cols,
        engine.frame.rows,
    );
    diff_frame_buffers(
        &mut engine.frame.diff_products,
        &engine.frame.current_frame_buffer,
        &engine.frame.old_frame_buffer,
        engine.frame.cols,
    );
    draw_to_terminal(&mut engine.stdout, &engine.frame.diff_products)?;
    copy_frame_buffer(
        &mut engine.frame.old_frame_buffer,
        &engine.frame.current_frame_buffer,
    );

    engine.frame.draw_queue.clear();
    engine.game_time += engine.delta_time;

    Ok(())
}
