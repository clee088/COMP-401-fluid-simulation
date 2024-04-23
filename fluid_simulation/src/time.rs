use bevy::{input::common_conditions::input_just_pressed, prelude::*};

pub struct TimePlugin;

impl Plugin for TimePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                toggle_pause.run_if(input_just_pressed(KeyCode::Space)),
                change_time_speed::<1>.run_if(input_just_pressed(KeyCode::Up)),
                change_time_speed::<-1>.run_if(input_just_pressed(KeyCode::Down)),
            ),
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

/// Update the speed of `Time<Virtual>.` by `DELTA`
fn change_time_speed<const DELTA: i8>(mut time: ResMut<Time<Virtual>>) {
    let time_speed = (time.relative_speed() + DELTA as f32)
        .round()
        .clamp(0.25, 5.);

    // set the speed of the virtual time to speed it up or slow it down
    time.set_relative_speed(time_speed);
}
