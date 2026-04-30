use bevy::ecs::component::Component;
use bevy::math::Vec2;

/// Simulation-owned world-space position; visual `Transform` is lerped from this.
#[derive(Component, Default)]
pub struct Position(pub Vec2);

/// Simulation-owned velocity in pixels/second.
#[derive(Component, Default)]
pub struct Velocity(pub Vec2);
