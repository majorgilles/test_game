mod camera;
mod input;
mod level;
mod physics;
mod player;

use crate::camera::CameraPlugin;
use crate::level::LevelPlugin;
use crate::physics::PhysicsPlugin;
use crate::player::PlayerPlugin;
use bevy::DefaultPlugins;
use bevy::app::{App, PluginGroup, Startup};
use bevy::camera::{Camera2d, OrthographicProjection, Projection, ScalingMode};
use bevy::ecs::system::Commands;
use bevy::image::ImagePlugin;
use bevy::math::Vec2;
use bevy::time::{Fixed, Time};
use bevy::transform::components::Transform;

const INTERNAL_HEIGHT: f32 = 216.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(PlayerPlugin)
        .add_plugins(PhysicsPlugin)
        .add_plugins(LevelPlugin)
        .add_plugins(CameraPlugin)
        .insert_resource(Time::<Fixed>::from_hz(60.0))
        .run();
}