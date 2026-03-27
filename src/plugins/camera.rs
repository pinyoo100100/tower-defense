// Tower Defense Game - Camera Plugin
// Sets up an isometric orthographic camera for the 2.5D view.

use bevy::prelude::*;

use crate::resources::AppState;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Playing), setup_isometric_camera);
    }
}

/// Spawns an orthographic camera tilted for an isometric perspective.
fn setup_isometric_camera(mut commands: Commands) {
    // Classic isometric angles: rotate 45° around Y, then ~35.264° around X
    let camera_distance = 15.0;
    let camera_pos = Vec3::new(camera_distance, camera_distance * 0.8, camera_distance);

    commands.spawn((
        Camera3d::default(),
        Projection::from(OrthographicProjection {
            scale: 6.0,
            near: -100.0,
            ..OrthographicProjection::default_3d()
        }),
        Transform::from_translation(camera_pos).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}
