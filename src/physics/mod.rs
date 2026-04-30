use bevy::app::{App, Plugin};

pub mod gravity;
pub mod interpolation;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<gravity::GravityConfig>();
    }
}
