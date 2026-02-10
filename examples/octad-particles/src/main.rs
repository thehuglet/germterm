use germterm::{
    color::{Color, ColorGradient, GradientStop},
    crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind},
    draw::{draw_fps_counter, draw_text},
    engine::{Engine, end_frame, exit_cleanup, init, start_frame},
    input::poll_input,
    layer::create_layer,
    particle::{
        ParticleColor, ParticleEmitter, ParticleEmitterShape, ParticleSpec, spawn_particles,
    },
    rich_text::{Attributes, RichText},
};
use rand::{Rng, rngs::ThreadRng};
use std::io;

pub const TERM_COLS: u16 = 80;
pub const TERM_ROWS: u16 = 24;

fn main() -> io::Result<()> {
    let mut engine: Engine = Engine::new(TERM_COLS, TERM_ROWS)
        .title("octad-particles")
        .limit_fps(0);

    let main_layer = create_layer(&mut engine, 0);
    let text_top_layer = create_layer(&mut engine, 1);

    init(&mut engine)?;
    'game_loop: loop {
        start_frame(&mut engine);

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
                    color: ParticleColor::Gradient(ColorGradient::new(vec![
                        GradientStop::new(0.0, Color::WHITE),
                        GradientStop::new(0.13, random_bright_color(&mut rng).with_alpha(255)),
                        GradientStop::new(1.0, random_bright_color(&mut rng).with_alpha(0)),
                    ])),
                };
                let emitter: ParticleEmitter = ParticleEmitter {
                    shape: ParticleEmitterShape::Circle,
                    count: rng.random_range(25..200),
                };

                let x_a: f32 = TERM_COLS as f32 * 0.3;
                let y_a: f32 = TERM_ROWS as f32 * 0.3;
                let x_b: f32 = TERM_COLS as f32 * 0.7;
                let y_b: f32 = TERM_ROWS as f32 * 0.7;

                spawn_particles(
                    &mut engine,
                    main_layer,
                    rng.random_range(x_a..=x_b),
                    rng.random_range(y_a..=y_b),
                    &spec,
                    &emitter,
                );
            }
        }

        draw_text(
            &mut engine,
            text_top_layer,
            26,
            (TERM_ROWS / 2) as i16,
            RichText::new("Press W to spawn particles!")
                .with_fg(Color::WHITE.with_alpha(100))
                .with_attributes(Attributes::BOLD),
        );

        draw_fps_counter(&mut engine, text_top_layer, 0, 0);

        end_frame(&mut engine)?;
    }

    exit_cleanup(&mut engine)?;
    Ok(())
}

fn random_bright_color(rng: &mut impl rand::Rng) -> Color {
    let h = rng.random::<f32>() * std::f32::consts::TAU;

    let r = ((h + 0.0).sin() * 0.5 + 0.5) * 255.0;
    let g = ((h + 2.094).sin() * 0.5 + 0.5) * 255.0;
    let b = ((h + 4.188).sin() * 0.5 + 0.5) * 255.0;

    Color::new(r as u8, g as u8, b as u8, 255)
}
