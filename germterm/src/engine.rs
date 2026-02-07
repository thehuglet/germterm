//! Core engine orchestration and frame management.
//!
//! This module ties together the terminal, frame, drawing layers, FPS, and particle state.
//! It provides the primary functions needed to initialize the terminal, start and end the frame, and render output.
//! Essentially, this is the central "body" that coordinates everything.

use crate::{
    color::{Color, ColorRgb},
    draw::{Layer, fill_screen},
    fps_counter::{FpsCounter, update_fps_counter},
    fps_limiter::{self, FpsLimiter, wait_for_next_frame},
    frame::{Frame, compose_frame_buffer, copy_frame_buffer, diff_frame_buffers, draw_to_terminal},
    particle::{ParticleState, update_and_draw_particles},
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
    pub(crate) default_blending_color: Color,
    pub(crate) fps_counter: FpsCounter,
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

/// This function should be called once after constructing the [`Engine`] and defining the [`Layer`]s,
/// and before entering the main update loop to initialize the engine.
///
/// # Panics
/// This function will not panic directly, but misusing it by defining a [`Layer`] after
/// [`init`] has been called, and then referencing the layer will likely cause a panic.
///
/// # Example
/// ```rust,no_run
/// # use germterm::{draw::{Layer}, engine::{Engine, init}};
/// let mut engine = Engine::new(40, 20);
/// let mut layer = Layer::new(&mut engine, 0);
/// init(&mut engine);
/// ```
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
        terminal::SetTitle(engine.title),
        event::EnableMouseCapture,
        cursor::Hide,
    )?;
    Ok(())
}

/// Cleans up the terminal state and exits the altenate screen.
///
/// Not calling ['exit_cleanup'] before exiting the program
/// will result in a messed up terminal state. (Be nice, clean up after yourself!)
pub fn exit_cleanup(engine: &mut Engine) -> io::Result<()> {
    terminal::disable_raw_mode()?;
    execute!(
        engine.stdout,
        terminal::LeaveAlternateScreen,
        terminal::EnableLineWrap,
        cursor::Show,
        event::DisableMouseCapture
    )?;
    Ok(())
}

/// Prepares a fresh frame state.
///
/// This function should be called once at the start of each frame inside the update loop.
///
/// Drawing should only happen after this is called for predictable results.
pub fn start_frame(engine: &mut Engine) {
    engine.delta_time = wait_for_next_frame(&mut engine.fps_limiter);
    update_fps_counter(&mut engine.fps_counter, engine.delta_time);

    let mut lowest_possible_layer = Layer::new(engine, 0);
    fill_screen(&mut lowest_possible_layer, Color::NO_COLOR);
}

/// Renders the contents to the terminal and ends the frame.
///
/// This function should be called once at the end of each frame inside the update loop.
///
/// No drawing should be happening after this function is called in the update loop.
pub fn end_frame(engine: &mut Engine) -> io::Result<()> {
    update_and_draw_particles(engine);

    compose_frame_buffer(
        &mut engine.frame.current_frame_buffer,
        engine.frame.layered_draw_queue.iter_mut().flat_map(|v| v.drain(..)),
        engine.frame.cols,
        engine.frame.rows,
        engine.default_blending_color,
    );
    let diff_products = diff_frame_buffers(
        &engine.frame.current_frame_buffer,
        &engine.frame.old_frame_buffer,
        engine.frame.cols,
    );
    draw_to_terminal(&mut engine.stdout, diff_products)?;
    copy_frame_buffer(
        &mut engine.frame.old_frame_buffer,
        &engine.frame.current_frame_buffer,
    );

    engine.game_time += engine.delta_time;
    Ok(())
}
