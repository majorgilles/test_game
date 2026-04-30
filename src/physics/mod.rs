use avian2d::prelude::{Gravity, PhysicsPlugins};
use bevy::app::{App, FixedUpdate, Plugin};
use bevy::math::Vec2;

pub mod gravity;
pub mod interpolation;
pub mod kinematics;

/// Owns physics ECS wiring: gravity config, avian registration, and (later)
/// the gravity integration system.
pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PhysicsPlugins::default().with_length_unit(1.0))
            .insert_resource(Gravity(Vec2::ZERO))
            .init_resource::<gravity::GravityConfig>()
            .add_systems(FixedUpdate, gravity::apply_gravity);
    }
}
