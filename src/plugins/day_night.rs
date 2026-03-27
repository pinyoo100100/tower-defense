// Tower Defense Game - Day/Night Cycle Plugin
// Advances time-of-day and adjusts ambient lighting.

use bevy::prelude::*;

use crate::resources::{AppState, DayNightState};

pub struct DayNightPlugin;

impl Plugin for DayNightPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DayNightState>()
            .add_systems(OnEnter(AppState::Playing), setup_lighting)
            .add_systems(
                Update,
                day_night_cycle_system.run_if(in_state(AppState::Playing)),
            );
    }
}

/// Creates the sun directional light and ambient light.
fn setup_lighting(mut commands: Commands) {
    // Directional "sun" light
    commands.spawn((
        DirectionalLight {
            illuminance: 10000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(
            EulerRot::XYZ,
            -std::f32::consts::FRAC_PI_4,
            std::f32::consts::FRAC_PI_4,
            0.0,
        )),
    ));
}

/// Advances the day/night cycle, toggling `is_night` and modulating light intensity.
fn day_night_cycle_system(
    time: Res<Time>,
    mut day_night: ResMut<DayNightState>,
    mut lights: Query<&mut DirectionalLight>,
) {
    day_night.time_of_day = (day_night.time_of_day + day_night.cycle_speed * time.delta_secs()) % 1.0;

    // Night is roughly the second half of the cycle (0.5..1.0)
    day_night.is_night = day_night.time_of_day >= 0.5;

    // Compute light intensity: brightest at 0.25 (noon), darkest at 0.75 (midnight)
    let angle = day_night.time_of_day * std::f32::consts::TAU;
    let intensity_factor = (angle.cos() + 1.0) * 0.5; // 0..1
    let illuminance = 2000.0 + intensity_factor * 8000.0;

    for mut light in lights.iter_mut() {
        light.illuminance = illuminance;
    }
}
