mod input;
mod level;
mod physics;
mod player;

use bevy::DefaultPlugins;
use bevy::app::{App, PluginGroup, Startup};
use bevy::camera::{Camera2d, OrthographicProjection, Projection, ScalingMode};
use bevy::ecs::system::Commands;
use bevy::image::ImagePlugin;
use bevy::math::Vec2;
use bevy::time::{Fixed, Time};
use bevy::transform::components::Transform;

use crate::level::LevelPlugin;
use crate::physics::PhysicsPlugin;
use crate::player::PlayerPlugin;

const INTERNAL_HEIGHT: f32 = 216.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(PlayerPlugin)
        .add_plugins(PhysicsPlugin)
        .add_plugins(LevelPlugin)
        .insert_resource(Time::<Fixed>::from_hz(60.0))
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
        Transform::from_translation(Vec2::new(180.0, -100.0).extend(1000.0)),
    ));
}
