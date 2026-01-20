use std::io;

use germterm::{
    color::Color,
    crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind},
    draw::{draw_text, fill_screen},
    engine::{Engine, end_frame, exit_cleanup, init, start_frame},
    fps_counter::draw_fps_counter,
    input::poll_input,
    particle::{ParticleEmitter, ParticleSpec, spawn_particles},
};

pub const TERM_COLS: u16 = 40;
pub const TERM_ROWS: u16 = 20;

fn main() -> io::Result<()> {
    let mut engine: Engine = Engine::new(TERM_COLS, TERM_ROWS)
        .title("particle-benchmark")
        .limit_fps(0);

    init(&mut engine)?;

    'update_loop: loop {
        start_frame(&mut engine);
        fill_screen(&mut engine, Color::BLACK);

        for event in poll_input() {
            match event {
                Event::Key(KeyEvent {
                    code: KeyCode::Char('q'),
                    ..
                }) => break 'update_loop,
                Event::Key(KeyEvent {
                    code: KeyCode::Char('w'),
                    kind: KeyEventKind::Press,
                    ..
                }) => {
                    spawn_particles(
                        &mut engine,
                        TERM_COLS as f32 * 0.5,
                        TERM_ROWS as f32 * 0.5,
                        &ParticleSpec {
                            lifetime_sec: 10.0,
                            speed: 0.5..=50.0,
                            gravity_scale: 0.02,
                            ..Default::default()
                        },
                        &ParticleEmitter {
                            count: 100_000,
                            ..Default::default()
                        },
                    );
                }
                _ => (),
            }
        }

        draw_fps_counter(&mut engine, 0, 0);

        end_frame(&mut engine)?;
    }

    exit_cleanup(&mut engine)?;
    Ok(())
}
