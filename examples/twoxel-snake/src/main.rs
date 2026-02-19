use germterm::{
    color::{Color, ColorGradient, GradientStop, sample_gradient},
    coord_space::{Position, native::NativePosition, octad::OctadPosition, twoxel::TwoxelPosition},
    crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind},
    draw::{draw_octad, draw_text, draw_twoxel},
    engine::{Engine, end_frame, exit_cleanup, init, start_frame},
    fps_counter::get_fps,
    input::poll_input,
    layer::{LayerIndex, create_layer},
    particle::{ParticleColor, ParticleEmitter, ParticleSpec, spawn_particles},
    rich_text::{Attributes, RichText},
};
use rand::{Rng, rngs::ThreadRng};
use std::io;

const TERM_COLS: u16 = 40;
const TERM_ROWS: u16 = 20;

const UP: TwoxelPosition = TwoxelPosition::new(0, -1);
const LEFT: TwoxelPosition = TwoxelPosition::new(-1, 0);
const DOWN: TwoxelPosition = TwoxelPosition::new(0, 1);
const RIGHT: TwoxelPosition = TwoxelPosition::new(1, 0);

enum GameState {
    Playing,
    GameOver,
}

fn main() -> io::Result<()> {
    let mut engine: Engine = Engine::new(TERM_COLS, TERM_ROWS)
        .title("twoxel-snake")
        .limit_fps(0);

    let layer_0 = create_layer(&mut engine, 0);
    let layer_1 = create_layer(&mut engine, 1);
    let layer_2 = create_layer(&mut engine, 2);

    let bg_decoration_color: Color = Color(0x45475aff);
    let movement_speed: f32 = 20.0;
    let mut segments = vec![
        TwoxelPosition::new(20, 22),
        TwoxelPosition::new(20, 21),
        TwoxelPosition::new(20, 20),
        TwoxelPosition::new(20, 19),
    ];

    let term_size_twoxel = NativePosition::new(TERM_COLS as i16, TERM_ROWS as i16).to_twoxel();
    let arena_area_a: TwoxelPosition = TwoxelPosition::new(2, 2);
    let arena_area_b: TwoxelPosition = term_size_twoxel.offset_xy(-2, -2);

    let mut apple_pos = random_pos_in_area(arena_area_a, arena_area_b);
    let mut last_direction: TwoxelPosition = DOWN;
    let mut direction: TwoxelPosition = DOWN;
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
        if matches!(game_state, GameState::Playing) {
            move_timer += engine.delta_time;
            let step_time: f32 = 1.0 / movement_speed;

            if move_timer >= step_time {
                move_timer -= step_time;
                last_direction = direction;

                let head: TwoxelPosition = segments[0];

                let new_head_pos = TwoxelPosition::new(
                    (head.x + direction.x - arena_area_a.x)
                        .rem_euclid(arena_area_b.x - arena_area_a.x)
                        + arena_area_a.x,
                    (head.y + direction.y - arena_area_a.y)
                        .rem_euclid(arena_area_b.y - arena_area_a.y)
                        + arena_area_a.y,
                );

                if segments.contains(&new_head_pos) {
                    game_state = GameState::GameOver;
                    spawn_death_explosion(&mut engine, layer_1, new_head_pos.to_octad());
                }
                segments.insert(0, new_head_pos);

                if new_head_pos == apple_pos {
                    spawn_explosion(&mut engine, layer_0, apple_pos.to_octad());
                    apple_pos = random_pos_in_area(arena_area_a, arena_area_b);
                    spawn_apple_create_particles(&mut engine, layer_0, apple_pos.to_octad());
                } else {
                    segments.pop();
                }
            }
        }

        // --- Draw border ---
        {
            let a = arena_area_a.offset_xy(-1, -1).to_octad();
            let b = arena_area_b.offset_xy(1, 1).to_octad();

            // top + bottom
            for x in a.x..b.x {
                let y_top = a.y;
                let y_bottom = b.y - 1;
                let p_top = (x + y_top) % 2;
                let p_bottom = (x + y_bottom) % 2;

                draw_octad(
                    &mut engine,
                    layer_2,
                    (x, y_top + p_top),
                    bg_decoration_color,
                );
                draw_octad(
                    &mut engine,
                    layer_2,
                    (x, y_bottom - p_bottom),
                    bg_decoration_color,
                );
            }

            // left + right
            for y in a.y..b.y {
                let x_left = a.x;
                let x_right = b.x - 1;
                let p_left = (x_left + y) % 2;
                let p_right = (x_right + y) % 2;

                draw_octad(
                    &mut engine,
                    layer_2,
                    (x_left + p_left, y),
                    bg_decoration_color,
                );
                draw_octad(
                    &mut engine,
                    layer_2,
                    (x_right - p_right, y),
                    bg_decoration_color,
                );
            }
        }

        // --- Draw apple ---
        draw_twoxel(&mut engine, layer_2, apple_pos, Color::RED);

        // --- Draw snake ---
        for (i, segment) in segments.iter().enumerate() {
            let t: f32 = i as f32 / segments.len() as f32;
            draw_twoxel(
                &mut engine,
                layer_2,
                (segment.x, segment.y),
                sample_gradient(&snake_color_gradient, t),
            );
        }

        // --- FPS Counter ---
        let fps_text: String = format!("UNCAPPED FPS: {:2.0}", get_fps(&engine));
        draw_text(
            &mut engine,
            layer_1,
            (10, 1),
            RichText::new(fps_text)
                .with_fg(Color(0x45475aff))
                .with_attributes(Attributes::BOLD),
        );

        if matches!(game_state, GameState::GameOver) {
            draw_text(
                &mut engine,
                layer_2,
                ((TERM_COLS / 2 - 6) as i16, (TERM_ROWS / 2 - 1) as i16),
                RichText::new("GAME OVER!")
                    .with_fg(Color::RED)
                    .with_attributes(Attributes::BOLD),
            );
        }

        end_frame(&mut engine)?;
    }

    exit_cleanup(&mut engine)?;
    Ok(())
}

fn random_pos_in_area<T: Position>(a: T, b: T) -> T {
    let mut rng: ThreadRng = rand::rng();
    T::new(
        rng.random_range(a.x()..b.x()),
        rng.random_range(a.y()..b.y()),
    )
}

fn spawn_explosion(engine: &mut Engine, layer: LayerIndex, position: OctadPosition) {
    spawn_particles(
        engine,
        layer,
        position,
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

fn spawn_apple_create_particles(engine: &mut Engine, layer: LayerIndex, position: OctadPosition) {
    spawn_particles(
        engine,
        layer,
        position,
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

fn spawn_death_explosion(engine: &mut Engine, layer: LayerIndex, position: OctadPosition) {
    spawn_particles(
        engine,
        layer,
        position,
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
