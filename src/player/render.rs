use bevy::ecs::query::With;
use bevy::ecs::system::Query;
use bevy::transform::components::Transform;

use crate::player::movement::{Player, Position};

/// Stop-gap visual sync: copies simulation `Position` straight to `Transform`.
/// Replaced by render-time interpolation when `physics::interpolation` ships.
pub fn sync_player_transform(mut query: Query<(&Position, &mut Transform), With<Player>>) {
    for (position, mut transform) in &mut query {
        transform.translation.x = position.0.x;
        transform.translation.y = position.0.y;
    }
}
