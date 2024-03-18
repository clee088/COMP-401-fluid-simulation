use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::BOUNDS;

pub struct BoundaryPlugin;

impl Plugin for BoundaryPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_boundary);
    }
}

fn spawn_boundary(mut commands: Commands) {
    let rect = shapes::Rectangle {
        extents: BOUNDS + (0.5 * 2.) + 0.1, // adding particle size (1) * 2 + stroke size (0.1)
        ..default()
    };

    commands.spawn((
        ShapeBundle {
            path: GeometryBuilder::build_as(&rect),
            ..default()
        },
        Stroke::new(Color::LIME_GREEN, 0.1),
    ));
}
