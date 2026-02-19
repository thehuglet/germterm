use std::io;

use germterm::{
    color::Color,
    coord_space::{Position, native::NativePosition, octad::OctadPosition},
    crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind},
    draw::draw_fps_counter,
    engine::{Engine, end_frame, exit_cleanup, init, start_frame},
    input::poll_input,
    layer::create_layer,
    particle::{ParticleColor, ParticleEmitter, ParticleSpec, spawn_particles},
};

pub const TERM_COLS: u16 = 40;
pub const TERM_ROWS: u16 = 20;
pub const PARTICLE_COUNT: usize = 100_000;

fn main() -> io::Result<()> {
    let mut engine: Engine = Engine::new(TERM_COLS, TERM_ROWS)
        .title("particle-benchmark")
        .limit_fps(240);

    let layer = create_layer(&mut engine, 0);

    init(&mut engine)?;

    'update_loop: loop {
        start_frame(&mut engine);

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
                    let screen_center: OctadPosition =
                        NativePosition::new((TERM_COLS / 2) as i16, (TERM_ROWS / 2) as i16)
                            .to_octad();

                    spawn_particles(
                        &mut engine,
                        layer,
                        screen_center,
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

        draw_fps_counter(&mut engine, layer, (0, 1));

        end_frame(&mut engine)?;
    }

    exit_cleanup(&mut engine)?;
    Ok(())
}
