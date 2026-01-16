use std::f32::consts::PI;

use rand::{Rng, rngs::ThreadRng};

use crate::{
    color::Color,
    draw::internal::{self},
    engine::Engine,
    frame::DrawCall,
};

pub struct ParticleState {
    pos: (f32, f32),
    velocity: (f32, f32),
    color: Color,
    spawn_timestamp: f32,
    death_timestamp: f32,
}

pub struct ParticleSpec {
    // TODO: Make this also support a weighted set of colors and possibly gradients
    color: Color,
    speed: f32,
    lifetime_sec: f32,
    gravity_scale: f32,
}

impl ParticleSpec {
    pub fn new(color: Color) -> Self {
        Self {
            color,
            speed: 30.0,
            lifetime_sec: 3.0,
            gravity_scale: 1.0,
        }
    }

    pub fn with_speed(mut self, value: f32) -> Self {
        self.speed = value;
        self
    }

    pub fn with_lifetime_sec(mut self, value: f32) -> Self {
        self.lifetime_sec = value;
        self
    }
}

pub struct ParticleEmitter {
    count: usize,
}

impl ParticleEmitter {
    pub fn new() -> Self {
        Self { count: 1 }
    }

    pub fn with_count(mut self, value: usize) -> Self {
        self.count = value;
        self
    }
}

pub(crate) fn update_and_draw_particles(
    particle_state: &mut Vec<ParticleState>,
    draw_calls: &mut Vec<DrawCall>,
    delta_time: f32,
    game_time: f32,
) {
    let gravity: f32 = 200.0;
    let drag: f32 = 3.0;

    // y:x aspect ratio to account for terminal cells not being perfect squares
    // and not making the end result look stretched out vertically
    let aspect_ratio: f32 = 1.0 / 2.0;

    let mut i: usize = 0;
    while i < particle_state.len() {
        let state: &mut ParticleState = &mut particle_state[i];

        if game_time >= state.death_timestamp {
            particle_state.swap_remove(i);
            continue;
        }

        state.velocity.1 += gravity * delta_time;

        let drag_decay: f32 = 1.0 / (1.0 + drag * delta_time);
        state.velocity.0 *= drag_decay;
        state.velocity.1 *= drag_decay;

        state.pos.0 += state.velocity.0 * delta_time;
        state.pos.1 += state.velocity.1 * delta_time * aspect_ratio;

        internal::draw_braille_dot(draw_calls, state.pos.0, state.pos.1, state.color);

        i += 1;
    }
}

pub fn spawn_particles(
    engine: &mut Engine,
    x: f32,
    y: f32,
    spec: &ParticleSpec,
    emitter: &ParticleEmitter,
) {
    let mut rng: ThreadRng = rand::rng();

    for _ in 0..emitter.count {
        let angle: f32 = rng.random_range(0.0..2.0 * PI);
        // let angle: f32 = (n as f32).to_radians();
        let velocity_x: f32 = spec.speed * angle.cos();
        let velocity_y: f32 = spec.speed * angle.sin();

        engine.particle_state.push(ParticleState {
            pos: (x, y),
            velocity: (velocity_x, velocity_y),
            color: spec.color,
            spawn_timestamp: engine.game_time,
            death_timestamp: engine.game_time + spec.lifetime_sec,
        })
    }
}
