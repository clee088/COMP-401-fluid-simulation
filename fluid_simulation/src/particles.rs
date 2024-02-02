use crate::movement::Velocity;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

const NUM_PARTICLES: i32 = 25;
const PARTICLE_SIZE: f32 = 0.5;
const PARTICLE_SPACING: f32 = 1.;

#[derive(Bundle)]
struct ParticleBundle {
    velocity: Velocity,
    material: ColorMesh2dBundle,
}

pub struct ParticlesPlugin;

impl Plugin for ParticlesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_particles);
    }
}

fn spawn_particles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let particles_per_row: i32 = (NUM_PARTICLES as f32).sqrt() as i32;
    let particles_per_col: i32 = (NUM_PARTICLES - 1) / particles_per_row + 1;
    let space_between = PARTICLE_SIZE * 2. + PARTICLE_SPACING;

    let center_x_offset: f32 = if particles_per_row % 2 == 0 { 1. } else { 0. };
    let center_y_offset: f32 = if particles_per_col % 2 == 0 { 1. } else { 0. };

    for i in 0..NUM_PARTICLES {
        let x = ((i % particles_per_row - particles_per_row / 2) as f32) * space_between
            + center_x_offset;
        let y = ((i / particles_per_row - particles_per_col / 2) as f32) * space_between
            + center_y_offset;

        commands.spawn(ParticleBundle {
            velocity: Velocity::new(0., 0., 0.),
            material: MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(PARTICLE_SIZE).into()).into(),
                material: materials.add(ColorMaterial::from(Color::BLUE)),
                transform: Transform::from_xyz(x, y, 0.),
                ..default()
            },
        });
    }
}
