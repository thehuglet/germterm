use germterm::{
    color::{Color, ColorGradient, GradientStop, sample_gradient},
    crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind},
    draw::{Layer, draw_octad, draw_text, draw_twoxel},
    engine::{Engine, end_frame, exit_cleanup, init, start_frame},
    input::poll_input,
    particle::{ParticleColor, ParticleEmitter, ParticleSpec, spawn_particles},
    rich_text::{Attributes, RichText},
};
use rand::{Rng, rngs::ThreadRng};
use std::io;

const TERM_COLS: u16 = 40;
const TERM_ROWS: u16 = 20;

const UP: (i16, i16) = (0, -1);
const LEFT: (i16, i16) = (-1, 0);
const DOWN: (i16, i16) = (0, 1);
const RIGHT: (i16, i16) = (1, 0);

enum GameState {
    Playing,
    GameOver,
}

fn main() -> io::Result<()> {
    let mut engine: Engine = Engine::new(TERM_COLS, TERM_ROWS)
        .title("twoxel-snake")
        .limit_fps(0);

    let mut layer_0 = Layer::new(&mut engine, 0);
    let mut layer_1 = Layer::new(&mut engine, 1);
    let mut layer_2 = Layer::new(&mut engine, 2);

    let bg_decoration_color: Color = Color(0x45475aff);
    let movement_speed: f32 = 20.0;
    let mut segments: Vec<(i16, i16)> = vec![(20, 22), (20, 21), (20, 20), (20, 19)];
    let mut apple_pos: (i16, i16) = random_pos();
    let mut last_direction: (i16, i16) = DOWN;
    let mut direction: (i16, i16) = DOWN;
    let mut move_timer: f32 = 0.0;
    let snake_color_gradient: ColorGradient = ColorGradient::new(vec![
        GradientStop::new(0.0, Color::CYAN),
        GradientStop::new(1.0, Color::VIOLET),
    ]);
    let mut game_state: GameState = GameState::Playing;

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
        // fill_screen(&mut layer_1, Color(0x1E_1E_2E_FF));

        if matches!(game_state, GameState::Playing) {
            move_timer += engine.delta_time;
            let step_time: f32 = 1.0 / movement_speed;

            if move_timer >= step_time {
                move_timer -= step_time;
                last_direction = direction;

                let head: (i16, i16) = segments[0];
                let new_head = (
                    2 + (head.0 + direction.0 - 2).rem_euclid((TERM_COLS - 4) as i16),
                    2 + (head.1 + direction.1 - 2).rem_euclid((TERM_ROWS - 2) as i16 * 2),
                );

                if segments.contains(&new_head) {
                    game_state = GameState::GameOver;
                    spawn_death_explosion(
                        &mut layer_1,
                        new_head.0 as f32 + 0.5,
                        (new_head.1 as f32 + 0.5) * 0.5,
                    );
                }
                segments.insert(0, new_head);

                if new_head == apple_pos {
                    spawn_explosion(
                        &mut layer_0,
                        apple_pos.0 as f32 + 0.5,
                        (apple_pos.1 as f32 + 0.5) * 0.5,
                    );
                    apple_pos = random_pos();
                    spawn_apple_create_particles(
                        &mut layer_0,
                        (apple_pos.0 as f32) + 0.5,
                        ((apple_pos.1 as f32) + 0.5) * 0.5,
                    );
                } else {
                    segments.pop();
                }
            }
        }

        let mut draw = |x: f32, y: f32| {
            draw_octad(&mut layer_2, x, y, bg_decoration_color);
        };

        // --- Horizontal borders ---
        for (dx, top, bottom, n) in [
            (1.5, 0.99, (TERM_ROWS - 1) as f32, TERM_COLS - 3),
            (1.0, 0.50, TERM_ROWS as f32 - 0.75, TERM_COLS - 2),
        ] {
            for x in 0..n {
                let xf = x as f32;
                draw(xf + dx, top);
                draw(xf + dx + 0.5, bottom);
            }
        }

        // --- Vertical borders ---
        for (xl, xr, offl, offr, n) in [
            (1.99, (TERM_COLS - 2) as f32, 0.99, 1.0, TERM_ROWS * 2 - 3),
            (1.0, TERM_COLS as f32 - 1.5, 0.5, 0.75, TERM_ROWS * 2 - 2),
        ] {
            for y in 0..n {
                let yf = y as f32 * 0.5;
                draw(xl, yf + offl);
                draw(xr, yf + offr);
            }
        }

        // --- Draw apple ---
        draw_twoxel(
            &mut layer_2,
            apple_pos.0 as f32,
            apple_pos.1 as f32 * 0.5,
            Color::RED,
        );

        // --- Draw snake ---
        for (i, segment) in segments.iter().enumerate() {
            let t: f32 = i as f32 / segments.len() as f32;
            // Multiplying the y axis by 0.5 here, as terminal cells usually have a 1:2 width to height ratio
            draw_twoxel(
                &mut layer_2,
                segment.0 as f32,
                segment.1 as f32 * 0.5,
                sample_gradient(&snake_color_gradient, t),
            );
        }

        // --- FPS Counter
        let fps_text: String = format!("UNCAPPED FPS: {:2.0}", engine.fps_counter.fps_ema);
        draw_text(
            &mut layer_1,
            10,
            1,
            RichText::new(fps_text)
                .fg(Color(0x45475aff))
                .attributes(Attributes::BOLD),
        );

        if matches!(game_state, GameState::GameOver) {
            draw_text(
                &mut layer_2,
                (TERM_COLS / 2 - 6) as i16,
                (TERM_ROWS / 2 - 1) as i16,
                RichText::new("GAME OVER!")
                    .fg(Color::RED)
                    .attributes(Attributes::BOLD),
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
        rng.random_range(2..(TERM_COLS - 2) as i16),
        rng.random_range(2..(TERM_ROWS * 2 - 2) as i16),
    )
}

fn spawn_explosion(layer: &mut Layer, x: f32, y: f32) {
    spawn_particles(
        layer,
        x,
        y,
        &ParticleSpec {
            gravity_scale: 0.1,
            speed: 20.0..=70.0,
            lifetime_sec: 2.0,
            color: ParticleColor::Gradient(ColorGradient::new(vec![
                GradientStop::new(0.0, Color::WHITE),
                GradientStop::new(0.05, Color::RED),
                GradientStop::new(1.0, Color::VIOLET.with_alpha(0)),
            ])),
        },
        &ParticleEmitter {
            count: 30,
            ..Default::default()
        },
    );
}

fn spawn_apple_create_particles(layer: &mut Layer, x: f32, y: f32) {
    spawn_particles(
        layer,
        x,
        y,
        &ParticleSpec {
            gravity_scale: 0.0,
            speed: 8.0..=10.0,
            lifetime_sec: 0.7,
            color: ParticleColor::Gradient(ColorGradient::new(vec![
                GradientStop::new(0.0, Color::RED.with_alpha(100)),
                GradientStop::new(1.0, Color::RED.with_alpha(0)),
            ])),
        },
        &ParticleEmitter {
            count: 70,
            ..Default::default()
        },
    );
}

fn spawn_death_explosion(layer: &mut Layer, x: f32, y: f32) {
    spawn_particles(
        layer,
        x,
        y,
        &ParticleSpec {
            gravity_scale: 0.5,
            speed: 10.0..=180.0,
            lifetime_sec: 2.5,
            color: ParticleColor::Gradient(ColorGradient::new(vec![
                GradientStop::new(0.0, Color::WHITE),
                GradientStop::new(0.05, Color::RED),
                GradientStop::new(1.0, Color::YELLOW.with_alpha(0)),
            ])),
        },
        &ParticleEmitter {
            count: 500,
            ..Default::default()
        },
    );
}
