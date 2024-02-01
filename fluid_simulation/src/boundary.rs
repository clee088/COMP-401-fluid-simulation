use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::BOUNDS;

pub struct BoundaryPlugin;

impl Plugin for BoundaryPlugin {
    fn build(&self, app: &mut App) {
        // app.add_systems(Startup, spawn_boundary);
    }
}

fn spawn_boundary(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // commands.spawn(NodeBundle {
    //     style: Style {
    //         width: Val::Px(BOUNDS.x),
    //         height: Val::Px(BOUNDS.y),
    //         border: UiRect::all(Val::Px(1.)),
    //         ..default()
    //     },
    //     border_color: BorderColor::from(Color::LIME_GREEN),
    //     transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
    //     ..default()
    // });
    // commands.spawn(SpriteBundle {
    //     sprite: Sprite {
    //         color: Color::rgb(0.25, 0.25, 0.75),
    //         custom_size: Some(Vec2::new(50.0, 100.0)),
    //         ..default()
    //     },
    //     ..default()
    // });
    // commands.spawn(MaterialMesh2dBundle {
    //     mesh: meshes
    //         .add(shape::Box::new(BOUNDS.x, BOUNDS.y, 0.).into())
    //         .into(),
    //     material: materials.add(ColorMaterial::from(Color::LIME_GREEN)),
    //     ..default()
    // });
}
