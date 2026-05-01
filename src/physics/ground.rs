use avian2d::prelude::Collider;
use avian2d::prelude::{ShapeCastConfig, SpatialQuery, SpatialQueryFilter};
use bevy::ecs::component::Component;
use bevy::ecs::entity::Entity;
use bevy::ecs::query::With;
use bevy::ecs::system::Query;
use bevy::math::Dir2;

use crate::physics::kinematics::Position;
use crate::player::Player;

/// Set by the ground-detection system each FixedUpdate tick. Read by movement
/// and (later) jump systems to decide whether the player is on solid ground.
/// A component, not a resource, because each entity that cares about grounding
/// (player now, NPCs later) carries its own.
#[derive(Component, Default)]
pub struct Grounded(pub bool);

/// FixedUpdate system. Casts the player's collider one pixel downward and sets
/// `Grounded` based on whether anything was hit. Must run before any system
/// that reads `Grounded` (movement, jump) so all readers see a tick coherent
/// value.
pub fn update_grounded(
    spatial: SpatialQuery,
    mut query: Query<(Entity, &Position, &Collider, &mut Grounded), With<Player>>,
) {
    for (entity, position, collider, mut grounded) in &mut query {
        let hit = spatial.cast_shape(
            collider,
            position.0,
            0.0,
            Dir2::NEG_Y,
            &ShapeCastConfig::from_max_distance(1.0),
            &SpatialQueryFilter::default().with_excluded_entities([entity]),
        );
        grounded.0 = hit.is_some();
    }
}
