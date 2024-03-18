use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::movement::SMOOTHING_RADIUS;

pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_cursor_circle);
    }
}

#[derive(Component)]
pub struct CursorIndicator;

fn spawn_cursor_circle(mut commands: Commands) {
    let circle = shapes::Circle {
        radius: SMOOTHING_RADIUS,
        ..default()
    };

    commands.spawn((
        ShapeBundle {
            path: GeometryBuilder::build_as(&circle),
            spatial: SpatialBundle {
                visibility: Visibility::Hidden,
                ..default()
            },
            ..default()
        },
        Fill {
            options: FillOptions::tolerance(0.01),
            color: Color::Rgba {
                red: 0.,
                green: 0.,
                blue: 0.,
                alpha: 0.,
            },
        },
        Stroke::new(Color::LIME_GREEN, 0.1),
        CursorIndicator,
    ));
}
