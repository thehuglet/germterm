use std::io;

use germterm::draw::Layer;
use germterm::{
    color::Color,
    crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind},
    draw::fill_screen,
    engine::{Engine, end_frame, exit_cleanup, init, start_frame},
    fps_counter::draw_fps_counter,
    input::poll_input,
    particle::{ParticleColor, ParticleEmitter, ParticleSpec, spawn_particles},
};

pub const TERM_COLS: u16 = 40;
pub const TERM_ROWS: u16 = 20;
pub const PARTICLE_COUNT: usize = 100_000;

fn main() -> io::Result<()> {
    let mut engine: Engine = Engine::new(TERM_COLS, TERM_ROWS)
        .title("particle-benchmark")
        .limit_fps(240);

    let mut layer = Layer::new(&mut engine, 0);

    init(&mut engine)?;

    'update_loop: loop {
        start_frame(&mut engine);
        fill_screen(&mut layer, Color::BLACK);

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
                        &mut layer,
                        TERM_COLS as f32 * 0.5,
                        TERM_ROWS as f32 * 0.5,
                        &ParticleSpec {
                            color: ParticleColor::Solid(Color::VIOLET),
                            lifetime_sec: 4.0,
                            speed: 0.5..=35.0,
                            gravity_scale: 0.01,
                        },
                        &ParticleEmitter {
                            count: PARTICLE_COUNT,
                            ..Default::default()
                        },
                    );
                }
                _ => (),
            }
        }

        draw_fps_counter(&mut layer, 0, 1);

        end_frame(&mut engine)?;
    }

    exit_cleanup(&mut engine)?;
    Ok(())
}
