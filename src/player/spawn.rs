use crate::input::{PlayerAction, default_input_map};
use crate::level::PlayerSpawn;
use crate::physics::ground::Grounded;
use crate::physics::kinematics::{Position, Velocity};
use crate::player::Player;
use avian2d::prelude::Collider;
use bevy::color::Color;
use bevy::math::Vec2;
use bevy::prelude::{Added, Commands, Entity, Query, Transform};
use bevy::sprite::Sprite;
use bevy::utils::default;
use leafwing_input_manager::prelude::ActionState;

const PLAYER_SIZE: f32 = 16.0;

/// Spwans the player at the position of the `PlayerSpawn` LDtk entity marker.
///
/// Runs every `Update` until a marker appears (LDtk loads async, so the marker isn't present at
/// startup). Once consumed, the marker is despawned so this system becomes a no-op for the rest of
/// the run.
pub fn spawn_player_at_marker(
    mut commands: Commands,
    spawn_markers: Query<(Entity, &Transform), Added<PlayerSpawn>>,
) {
    for (marker_entity, marker_transform) in &spawn_markers {
        let position = marker_transform.translation.truncate();

        commands.spawn((
            Player,
            Position(position),
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
            Transform::from_translation(position.extend(0.0)),
        ));
    }
}
