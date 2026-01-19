use germterm::{
    color::{Color, ColorGradient, GradientStop, lerp},
    crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind},
    draw::{Pos, draw_octad, draw_text, fill_screen},
    engine::{Engine, end_frame, exit_cleanup, init, start_frame},
    fps_counter::draw_fps_counter,
    input::poll_input,
    particle::{
        ParticleColor, ParticleEmitter, ParticleEmitterShape, ParticleSpec, spawn_particles,
    },
};
use rand::{Rng, rngs::ThreadRng};

use std::{f32::consts::PI, io};

pub const TERM_COLS: u16 = 80;
pub const TERM_ROWS: u16 = 24;

fn main() -> io::Result<()> {
    let mut engine: Engine = Engine::new(TERM_COLS, TERM_ROWS)
        .title("particles")
        .limit_fps(0);

    init(&mut engine)?;
    'game_loop: loop {
        start_frame(&mut engine);
        fill_screen(&mut engine, Color::BLACK);

        for event in poll_input() {
            if let Event::Key(KeyEvent {
                code: KeyCode::Char('q'),
                ..
            }) = event
            {
                break 'game_loop;
            }

            if let Event::Key(KeyEvent {
                code: KeyCode::Char('w'),
                kind: KeyEventKind::Press,
                ..
            }) = event
            {
                let mut rng: ThreadRng = rand::rng();

                let spec: ParticleSpec = ParticleSpec {
                    gravity_scale: rng.random_range(0.04..0.07),
                    speed: 5.0..=rng.random_range(30.0..120.0),
                    lifetime_sec: 2.0,
                    // color: ParticleColor::Solid(Color::ORANGE.with_alpha(127)),
                    color: ParticleColor::Gradient(ColorGradient::new(vec![
                        GradientStop::new(0.0, Color::WHITE),
                        GradientStop::new(0.13, Color(rng.random()).with_alpha(255)),
                        GradientStop::new(1.0, Color(rng.random()).with_alpha(0)),
                    ])),
                    ..Default::default()
                };
                let emitter: ParticleEmitter = ParticleEmitter {
                    // shape: ParticleEmitterShape::Cone {
                    //     direction_deg: -90.0,
                    //     width_deg: 75.0,
                    // },
                    shape: ParticleEmitterShape::Circle,
                    count: rng.random_range(25..200),
                    ..Default::default()
                };

                let x_a: f32 = TERM_COLS as f32 * 0.3;
                let y_a: f32 = TERM_ROWS as f32 * 0.3;
                let x_b: f32 = TERM_COLS as f32 * 0.7;
                let y_b: f32 = TERM_ROWS as f32 * 0.7;

                spawn_particles(
                    &mut engine,
                    rng.random_range(x_a..=x_b),
                    rng.random_range(y_a..=y_b),
                    &spec,
                    &emitter,
                );
            }
        }

        draw_fps_counter(&mut engine, Pos::new(0, 0));

        end_frame(&mut engine)?;
    }

    exit_cleanup(&mut engine)?;
    Ok(())
}
