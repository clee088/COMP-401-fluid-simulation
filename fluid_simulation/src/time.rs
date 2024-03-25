use bevy::{input::common_conditions::input_just_pressed, prelude::*};

pub struct TimePlugin;

impl Plugin for TimePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (toggle_pause.run_if(input_just_pressed(KeyCode::Space)),),
        );
    }
}

fn toggle_pause(mut time: ResMut<Time<Virtual>>) {
    if time.is_paused() {
        time.unpause();
    } else {
        time.pause();
    }
}
