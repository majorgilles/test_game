mod input;
mod physics;
mod player;

use bevy::DefaultPlugins;
use bevy::app::{App, PluginGroup, Startup};
use bevy::camera::{Camera2d, OrthographicProjection, Projection, ScalingMode};
use bevy::color::Color;
use bevy::ecs::system::Commands;
use bevy::image::ImagePlugin;
use bevy::math::Vec2;
use bevy::sprite::Sprite;
use bevy::time::{Fixed, Time};
use bevy::transform::components::Transform;
use bevy::utils::default;
use leafwing_input_manager::action_state::ActionState;

use crate::input::{PlayerAction, default_input_map};
use crate::physics::PhysicsPlugin;
use crate::physics::ground::Grounded;
use crate::physics::kinematics::{Position, Velocity};
use crate::player::Player;
use crate::player::PlayerPlugin;
use avian2d::prelude::{Collider, RigidBody};

const INTERNAL_HEIGHT: f32 = 216.0;
const PLAYER_SIZE: f32 = 16.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(PlayerPlugin)
        .add_plugins(PhysicsPlugin)
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
    ));

    commands.spawn((
        Player,
        Position(Vec2::ZERO),
        Velocity(Vec2::ZERO),
        ActionState::<PlayerAction>::default(),
        default_input_map(),
        Collider::rectangle(PLAYER_SIZE, PLAYER_SIZE),
        Grounded::default(),
        Sprite {
            color: Color::WHITE,
            custom_size: Some(Vec2::splat(PLAYER_SIZE)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    commands.spawn((
        RigidBody::Static,
        Collider::rectangle(384.0, 16.0),
        Sprite {
            color: Color::srgb(0.3, 0.3, 0.3),
            custom_size: Some(Vec2::new(384.0, 16.0)),
            ..default()
        },
        Transform::from_xyz(0.0, -80.0, 0.0),
    ));
}
