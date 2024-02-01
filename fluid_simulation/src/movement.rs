use bevy::prelude::*;

use crate::BOUNDS;

const DEFAULT_GRAVITY: f32 = -9.8;
const DAMPENING_FACTOR: f32 = 0.8;

#[derive(Component, Debug)]
pub struct Velocity {
    pub value: Vec3,
}

impl Velocity {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            value: Vec3::new(x, y, z),
        }
    }
}

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, simulate_particles);
    }
}

fn simulate_particles(time: Res<Time>, mut query: Query<(&mut Velocity, &mut Transform)>) {
    for (mut velocity, mut transform) in query.iter_mut() {
        velocity.value += Vec3::Y * DEFAULT_GRAVITY * time.delta_seconds();
        transform.translation += velocity.value * time.delta_seconds();

        let half_bounds_size = BOUNDS / 2.0;

        if transform.translation.x.abs() > half_bounds_size.x {
            transform.translation.x = half_bounds_size.x * transform.translation.x.signum();
            velocity.value.x *= -1. * DAMPENING_FACTOR;
        }
        if transform.translation.y.abs() > half_bounds_size.y {
            transform.translation.y = half_bounds_size.y * transform.translation.y.signum();
            velocity.value.y *= -1. * DAMPENING_FACTOR;
        }
    }
}
