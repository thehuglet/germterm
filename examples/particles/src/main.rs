use germterm::{
    color::Color,
    crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind},
    draw::{Pos, draw_braille_dot, draw_text, fill_screen},
    engine::Engine,
    engine::{end_frame, exit_cleanup, init, start_frame},
    fps_counter::draw_fps_counter,
    input::poll_input,
    particle::{ParticleEmitter, ParticleSpec, spawn_particles},
};

use rand::Rng;

use std::{f32::consts::PI, io};

pub const TERM_COLS: u16 = 80;
pub const TERM_ROWS: u16 = 24;

// struct ParticleState {
//     pos: (f32, f32),
//     velocity: (f32, f32),
//     color: Color,
// }

fn main() -> io::Result<()> {
    let mut engine: Engine = Engine::new(TERM_COLS, TERM_ROWS)
        .title("particles")
        .limit_fps(0);

    init(&mut engine)?;

    // let mut particles_state: Vec<ParticleState> = Vec::with_capacity(400);

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
                let spec: ParticleSpec = ParticleSpec::new(Color::LIME)
                    .with_lifetime_sec(10.0)
                    .with_speed(60.0);
                let emitter: ParticleEmitter = ParticleEmitter::new().with_count(1000);
                spawn_particles(
                    &mut engine,
                    TERM_COLS as f32 / 2.0,
                    TERM_ROWS as f32 / 2.0,
                    &spec,
                    &emitter,
                );
            }
        }

        //     for particle in particles_state.iter_mut() {
        //         // Friction (your current squared style)
        //         let friction = 20.0 * engine.delta_time;
        //         particle.velocity.0 -= particle.velocity.0 * friction.powi(2);
        //         particle.velocity.1 -= particle.velocity.1 * friction.powi(2);

        //         // Gravity: constant downward acceleration
        //         let gravity = 13.8; // tweak this for stronger/weaker effect
        //         particle.velocity.1 += gravity * engine.delta_time;

        //         // Update position
        //         let (x, y) = particle.pos;

        //         let (dx, dy) = particle.velocity;
        //         let new_x = x + dx * engine.delta_time * 10.0;
        //         let new_y = y + dy * engine.delta_time * 10.0;

        //         particle.pos = (new_x, new_y);

        //         draw_braille_dot(&mut engine, new_x, new_y, particle.color);
        //     }

        //     // Remove OOB bottom particles
        //     particles_state.retain(|p| p.pos.1 < TERM_ROWS as f32);

        draw_fps_counter(&mut engine, Pos::new(0, 0));

        end_frame(&mut engine)?;
    }

    exit_cleanup(&mut engine)?;
    Ok(())
}

// fn spawn_particles(particles_state: &mut Vec<ParticleState>) {
//     let mut rng = rand::rng();

//     let x = TERM_COLS as f32 / 2.0;
//     let y = TERM_ROWS as f32 / 3.5;

//     for _ in 0..1000 {
//         let angle = rng.random_range(0.0..2.0 * PI);
//         let speed = rng.random_range(0.0..8.0);

//         let vx = speed * angle.cos();
//         let vy = (speed * angle.sin() - 8.0) * 0.5;

//         let color = if rng.random_bool(0.5) {
//             Color::CYAN.with_alpha(70)
//         } else {
//             Color::VIOLET.with_alpha(70)
//         };

//         // let color = Color::new(
//         //     rng.random_range(0..=255),
//         //     rng.random_range(0..=255),
//         //     rng.random_range(0..=255),
//         //     255,
//         // );

//         particles_state.push(ParticleState {
//             pos: (x, y),
//             velocity: (vx, vy),
//             color,
//         });
//     }
// }
