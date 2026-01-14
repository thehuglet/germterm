use crate::{Engine, color::Color};

pub struct ParticleState {
    pos: (f32, f32),
    velocity: (f32, f32),
    color: Color,
    // spawn_timestamp: f32,
    // death_timestamp: f32,
}

pub struct ParticleRecipe {
    // TODO: Make this also support a weighted set of colors
    color: Color,
    speed: f32,
    // lifetime_sec: f32,
}

impl ParticleRecipe {
    pub fn new(color: Color) -> Self {
        Self { color, speed: 30.0 }
    }

    pub fn with_speed(mut self, value: f32) -> Self {
        self.speed = value;
        self
    }
}

pub fn update_particles(particles_state: &mut Vec<ParticleRecipe>) {
    for particle in particles_state.iter_mut() {
        todo!();
    }
}

// TODO: separate recipe from the emitter
pub fn spawn_particle(engine: &mut Engine, x: f32, y: f32, particle_recipe: &ParticleRecipe) {
    spawn_particles(engine, x, y, particle_recipe, 1);
}

pub fn spawn_particles(engine: &mut Engine, x: f32, y: f32, recipe: &ParticleRecipe, count: usize) {
    for _ in 0..count {
        // Velocity (0, 0) for now
        let velocity_x: f32 = 0.0;
        let velocity_y: f32 = 0.0;

        engine.particle_state.push(ParticleState {
            pos: (x, y),
            velocity: (velocity_x, velocity_y),
            color: recipe.color,
        })
    }
}
