mod boundary;
mod camera;
mod debug;
mod movement;
mod particles;

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use boundary::BoundaryPlugin;
use camera::CameraPlugin;
// use debug::DebugPlugin;
use movement::SimulationPlugin;
use particles::ParticlesPlugin;

const BOUNDS: Vec2 = Vec2::new(25.0, 20.0);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ShapePlugin)
        .add_plugins(BoundaryPlugin)
        .add_plugins(CameraPlugin)
        // .add_plugins(DebugPlugin)
        .add_plugins(ParticlesPlugin)
        .add_plugins(SimulationPlugin)
        .run();
}
