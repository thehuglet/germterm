use germterm::{
    color::{Color, ColorGradient, GradientStop, sample_gradient},
    crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind},
    draw::{draw_twoxel, fill_screen},
    engine::{Engine, end_frame, exit_cleanup, init, start_frame},
    input::poll_input,
    particle::{ParticleColor, ParticleEmitter, ParticleSpec, spawn_particles},
};
use rand::{Rng, rngs::ThreadRng};

use std::io;

const TERM_COLS: u16 = 40;
const TERM_ROWS: u16 = 20;

const UP: (i16, i16) = (0, -1);
const LEFT: (i16, i16) = (-1, 0);
const DOWN: (i16, i16) = (0, 1);
const RIGHT: (i16, i16) = (1, 0);

fn main() -> io::Result<()> {
    let mut engine: Engine = Engine::new(TERM_COLS, TERM_ROWS)
        .title("twoxel-snake")
        .limit_fps(240);

    let movement_speed: f32 = 15.0;
    let mut segments: Vec<(i16, i16)> = vec![(20, 22), (20, 21), (20, 20), (20, 19)];
    let mut apple_pos: (i16, i16) = random_pos();
    let mut last_direction: (i16, i16) = DOWN;
    let mut direction: (i16, i16) = DOWN;
    let mut move_timer: f32 = 0.0;
    let snake_color_gradient: ColorGradient = ColorGradient::new(vec![
        GradientStop::new(0.0, Color::LIME),
        GradientStop::new(1.0, Color::DARK_GREEN),
    ]);

    init(&mut engine)?;
    'game_loop: loop {
        for event in poll_input() {
            match event {
                Event::Key(KeyEvent {
                    code: KeyCode::Char('q'),
                    ..
                }) => break 'game_loop,
                Event::Key(KeyEvent {
                    code: KeyCode::Char('w'),
                    kind: KeyEventKind::Press,
                    ..
                }) => {
                    if last_direction != DOWN {
                        direction = UP;
                    }
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Char('a'),
                    kind: KeyEventKind::Press,
                    ..
                }) => {
                    if last_direction != RIGHT {
                        direction = LEFT
                    }
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Char('s'),
                    kind: KeyEventKind::Press,
                    ..
                }) => {
                    if last_direction != UP {
                        direction = DOWN
                    }
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Char('d'),
                    kind: KeyEventKind::Press,
                    ..
                }) => {
                    if last_direction != LEFT {
                        direction = RIGHT
                    }
                }
                _ => (),
            }
        }

        start_frame(&mut engine);
        fill_screen(&mut engine, Color::BLACK);

        move_timer += engine.delta_time;
        let step_time: f32 = 1.0 / movement_speed;

        if move_timer >= step_time {
            move_timer -= step_time;
            last_direction = direction;

            let head: (i16, i16) = segments[0];
            let new_head = (
                (head.0 + direction.0).rem_euclid(TERM_COLS as i16),
                (head.1 + direction.1).rem_euclid((TERM_ROWS * 2) as i16),
            );

            if segments.contains(&new_head) {
                break 'game_loop;
            }
            segments.insert(0, new_head);

            if new_head == apple_pos {
                spawn_explosion(&mut engine, apple_pos.0 as f32, apple_pos.1 as f32 * 0.5);
                apple_pos = random_pos();
            } else {
                segments.pop();
            }
        }

        // --- Draw apple ---
        draw_twoxel(
            &mut engine,
            apple_pos.0 as f32,
            apple_pos.1 as f32 * 0.5,
            Color::RED,
        );

        // --- Draw snake ---
        for (i, segment) in segments.iter().enumerate() {
            let t: f32 = i as f32 / segments.len() as f32;
            // Multiplying the y axis by 0.5 here, as terminal cells usually have a 1:2 width to height ratio
            draw_twoxel(
                &mut engine,
                segment.0 as f32,
                segment.1 as f32 * 0.5,
                sample_gradient(&snake_color_gradient, t),
            );
        }

        end_frame(&mut engine)?;
    }

    exit_cleanup(&mut engine)?;
    Ok(())
}

fn random_pos() -> (i16, i16) {
    let mut rng: ThreadRng = rand::rng();
    (
        rng.random_range(0..TERM_COLS as i16),
        rng.random_range(0..TERM_ROWS as i16),
    )
}

fn spawn_explosion(engine: &mut Engine, x: f32, y: f32) {
    spawn_particles(
        engine,
        x,
        y,
        &ParticleSpec {
            gravity_scale: 0.01,
            speed: 30.0..=50.0,
            lifetime_sec: 0.6,
            color: ParticleColor::Gradient(ColorGradient::new(vec![
                GradientStop::new(0.0, Color::WHITE),
                GradientStop::new(0.2, Color::RED),
                GradientStop::new(1.0, Color::ORANGE.with_alpha(0)),
            ])),
            ..Default::default()
        },
        &ParticleEmitter {
            count: 50,
            ..Default::default()
        },
    );
}
