use crate::{
    color::{Color, ColorRgb},
    draw::{Layer, draw_rect, erase_rect, fill_screen},
    fps_counter::{FpsCounter, update_fps_counter},
    fps_limiter::{self, FpsLimiter, wait_for_next_frame},
    frame::{
        DrawCall, Frame, compose_frame_buffer, copy_frame_buffer, diff_frame_buffers,
        draw_to_terminal,
    },
    particle::{ParticleState, update_and_draw_particles},
    rich_text::RichText,
};
use crossterm::{cursor, event, execute, terminal};
use std::{
    io::{self},
    time::Duration,
};

pub struct Engine {
    pub delta_time: f32,
    pub game_time: f32,
    pub stdout: io::Stdout,
    pub fps_counter: FpsCounter,
    pub default_blending_color: Color,
    pub base_bg_color: Color,
    pub(crate) max_layer_index: usize,
    pub(crate) frame: Frame,
    pub(crate) fps_limiter: FpsLimiter,
    pub(crate) particle_state: Vec<ParticleState>,
    title: &'static str,
}

impl Engine {
    pub fn new(cols: u16, rows: u16) -> Self {
        Self {
            delta_time: 0.01667,
            game_time: 0.0,
            title: "my-awesome-terminal",
            stdout: io::stdout(),
            max_layer_index: 0,
            frame: Frame::new(cols, rows),
            fps_limiter: FpsLimiter::new(60, 0.001, 0.002),
            fps_counter: FpsCounter::new(0.3),
            particle_state: Vec::with_capacity(512),
            default_blending_color: {
                match termbg::rgb(Duration::from_millis(100)) {
                    Ok(rgb) => Color::new(rgb.r as u8, rgb.g as u8, rgb.b as u8, 255),
                    Err(_) => Color::BLACK,
                }
            },
            base_bg_color: Color::NO_COLOR,
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

/// Overrides the default blending color.
///
/// Only use this if you need to support terminals where the background color cannot
/// be reliably auto-detected by `termbg`. Otherwise, it's best to leave this alone.
pub fn override_default_blending_color(engine: &mut Engine, color: ColorRgb) {
    engine.default_blending_color = color.into();
}

pub fn init(engine: &mut Engine) -> io::Result<()> {
    let layer_count = engine.max_layer_index + 1;
    if engine.frame.layered_draw_queue.len() < layer_count {
        engine
            .frame
            .layered_draw_queue
            .resize_with(layer_count, Vec::new);
    }

    terminal::enable_raw_mode()?;
    execute!(
        engine.stdout,
        terminal::EnterAlternateScreen,
        terminal::DisableLineWrap,
        terminal::SetTitle(engine.title),
        event::EnableMouseCapture,
        cursor::Hide,
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

    engine.frame.flat_draw_queue.clear();

    let mut lowest_possible_layer = Layer::new(engine, 0);
    fill_screen(&mut lowest_possible_layer, Color::NO_COLOR);
}

pub fn end_frame(engine: &mut Engine) -> io::Result<()> {
    update_and_draw_particles(engine);

    for layer in engine.frame.layered_draw_queue.iter_mut() {
        engine.frame.flat_draw_queue.append(layer);
    }

    compose_frame_buffer(
        &mut engine.frame.current_frame_buffer,
        &engine.frame.flat_draw_queue,
        engine.frame.cols,
        engine.frame.rows,
        engine.default_blending_color,
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

    engine.game_time += engine.delta_time;
    Ok(())
}
