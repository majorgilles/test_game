use bevy::app::{App, Plugin, Startup};
use bevy::asset::AssetServer;
use bevy::ecs::system::{Commands, Res};
use bevy::prelude::default;
use bevy_ecs_ldtk::{LdtkIntCell, LdtkPlugin, LdtkWorldBundle, app::LdtkIntCellAppExt};

/// Loads the LDtk world and spawns colliders for IntGrid walls.
///
/// Owns the `LdtkPlugin` registration so `main.rs` only composes plugins.
pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(LdtkPlugin)
            .add_systems(Startup, spawn_world);
    }
}

fn spawn_world(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: assets.load("levels/world.ldtk").into(),
        ..default()
    });
}
