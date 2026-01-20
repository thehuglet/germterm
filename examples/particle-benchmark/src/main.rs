use std::io::Write;
use std::{fs::File, io};

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

    init(&mut engine)?;

    let mut frametimes: Vec<f32> = Vec::with_capacity(200_000);

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

        frametimes.push(engine.delta_time);
        draw_fps_counter(&mut engine, 0, 1);

        end_frame(&mut engine)?;
    }

    let mut file = File::create("examples/particle-benchmark/frametimes.csv")?;
    writeln!(file, "frametime_ms")?;
    for ft in frametimes {
        writeln!(file, "{:.4}", ft * 1000.0)?;
    }

    exit_cleanup(&mut engine)?;
    Ok(())
}
