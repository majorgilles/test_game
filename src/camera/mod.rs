use crate::player::Player;
use bevy::app::{App, Plugin, Startup, Update};
use bevy::camera::{Camera, Camera2d, OrthographicProjection, Projection, ScalingMode};
use bevy::math::Vec2;
use bevy::prelude::{Added, Commands, Query, Transform, With, Without};

const INTERNAL_HEIGHT: f32 = 216.0;

/// Spawns the 2D camera and snaps it to the player on spawn. Camera-follow
/// proper is deferred to stage 7.
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(Update, snap_camera_to_player);
    }
}

fn spawn_camera(mut commands: Commands) {
    let projection = Projection::from(OrthographicProjection {
        scaling_mode: ScalingMode::FixedVertical {
            viewport_height: INTERNAL_HEIGHT,
        },
        ..OrthographicProjection::default_2d()
    });
    let transform = Transform::from_translation(Vec2::new(180.0, -100.0).extend(1000.0));
    commands.spawn((Camera2d, projection, transform));
}

fn snap_camera_to_player(
    player: Query<&Transform, (Added<Player>, Without<Camera>)>,
    mut camera: Query<&mut Transform, With<Camera>>,
) {
    let Ok(player_transform) = player.single() else {
        return;
    };
    let mut camera_transform = camera
        .single_mut()
        .expect("CameraPlugin must spawn exactly one Camera2d before snap_camera_to_player runs");
    camera_transform.translation.x = player_transform.translation.x;
    camera_transform.translation.y = player_transform.translation.y;
}
