use avian2d::prelude::{Gravity, PhysicsPlugins};
use bevy::app::{App, FixedUpdate, Plugin};
use bevy::ecs::schedule::IntoScheduleConfigs;
use bevy::math::Vec2;

pub mod gravity;
pub mod ground;
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
            .add_systems(
                FixedUpdate,
                (
                    ground::update_grounded,
                    gravity::apply_gravity.after(ground::update_grounded),
                    ground::resolve_ground_collision.after(gravity::apply_gravity),
                ),
            );
    }
}
