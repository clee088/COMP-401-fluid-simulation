use std::f32::consts::PI;

use bevy::prelude::*;

use crate::BOUNDS;

// const DEFAULT_GRAVITY: f32 = -9.81;
const DEFAULT_GRAVITY: f32 = 0.0;
const DAMPENING_FACTOR: f32 = 0.55;
pub const SMOOTHING_RADIUS: f32 = 4.;
const MASS: f32 = 1.0;

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

#[derive(Component, Debug)]
pub struct Density {
    pub value: f32,
}

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                update_densities,
                compute_external_force,
                compute_pressure_force,
                simulate_particles,
            )
                .chain(),
        );
    }
}

fn smoothing_kernel(radius: f32, distance: f32) -> f32 {
    if distance > radius || distance < 0. {
        return 0.;
    }
    let scale = 15.0 / (PI * radius.powi(6));
    let value = (radius - distance).powi(3);
    scale * value
}

fn smoothing_kernel_gradient(radius: f32, distance: f32) -> f32 {
    if distance > radius || distance < 0. {
        return 0.;
    }
    -((45. * (radius - distance).powi(2)) / (PI * radius.powi(6)))
}

fn update_densities(mut query: Query<(&mut Density, &Transform)>) {
    query.par_iter_mut().for_each(|(mut density, _)| {
        density.value = 0.;
    });
    // for (mut density, _) in query.iter_mut() {
    //     density.value = 0.;
    // }

    let mut combinations = query.iter_combinations_mut();
    while let Some([(mut density, transform1), (_, transform2)]) = combinations.fetch_next() {
        let distance = (transform2.translation - transform1.translation).length();
        let influence = smoothing_kernel(SMOOTHING_RADIUS, distance);
        density.value += MASS * influence;
    }
}

fn compute_external_force(time: Res<Time>, mut query: Query<&mut Velocity>) {
    for mut velocity in query.iter_mut() {
        velocity.value += Vec3::Y * DEFAULT_GRAVITY * time.delta_seconds();
    }
}

const REST_DENSITY: f32 = 1.0;

const GAS_CONSTANT: f32 = 461.52;

fn get_target_pressure(density: f32) -> f32 {
    GAS_CONSTANT * (density - REST_DENSITY)
}

fn get_shared_pressure(d1: f32, d2: f32) -> f32 {
    let a = get_target_pressure(d1);
    let b = get_target_pressure(d2);
    (a + b) / 2.
}

fn compute_pressure_force(
    mut particles_q: Query<(&Density, &Transform, &mut Velocity)>,
    time: Res<Time>,
) {
    let mut combinations = particles_q.iter_combinations_mut();

    while let Some([(density, t1, mut velocity), (d2, t2, mut v2)]) = combinations.fetch_next() {
        if density.value == 0. {
            continue;
        }
        let offset = t2.translation - t1.translation;
        let distance = offset.length();
        let mut pressure_force = match offset.try_normalize() {
            Some(x) => x,
            None => Vec3 {
                x: rand::random(),
                y: rand::random(),
                z: 0.0,
            },
        };
        let gradient = smoothing_kernel_gradient(SMOOTHING_RADIUS, distance);
        let shared_pressure = get_shared_pressure(density.value, d2.value);
        pressure_force *= -(shared_pressure * gradient * MASS);
        // F = MA => A = F / M
        let acceleration = pressure_force / MASS;
        velocity.value += acceleration * time.delta_seconds();
    }
}

fn simulate_particles(time: Res<Time>, mut query: Query<(&mut Velocity, &mut Transform)>) {
    for (mut velocity, mut transform) in query.iter_mut() {
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
