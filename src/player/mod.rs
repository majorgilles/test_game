use bevy::app::{App, FixedUpdate, Plugin, Update};
use leafwing_input_manager::plugin::InputManagerPlugin;

use crate::input::PlayerAction;

pub mod movement;
pub mod render;

/// Registers leafwing for `PlayerAction`, the `MovementConfig` resource, the
/// FixedUpdate movement system, and the Update transform-sync.
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<PlayerAction>::default())
            .init_resource::<movement::MovementConfig>()
            .add_systems(FixedUpdate, movement::apply_horizontal_movement)
            .add_systems(Update, render::sync_player_transform);
    }
}
