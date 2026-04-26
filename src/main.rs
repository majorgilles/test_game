mod player;

use bevy::DefaultPlugins;
use bevy::app::{App, PluginGroup, Startup};
use bevy::camera::{Camera2d, OrthographicProjection, Projection, ScalingMode};
use bevy::color::Color;
use bevy::ecs::system::Commands;
use bevy::image::ImagePlugin;
use bevy::math::Vec2;
use bevy::sprite::Sprite;
use bevy::transform::components::Transform;
use bevy::utils::default;

const INTERNAL_HEIGHT: f32 = 216.0;
const PLAYER_SIZE: f32 = 16.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Projection::from(OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: INTERNAL_HEIGHT,
            },
            ..OrthographicProjection::default_2d()
        }),
    ));

    commands.spawn((
        Sprite {
            color: Color::WHITE,
            custom_size: Some(Vec2::splat(PLAYER_SIZE)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}
