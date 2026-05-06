use avian2d::prelude::{Collider, RigidBody};
use bevy::app::{App, Plugin, Startup};
use bevy::asset::AssetServer;
use bevy::ecs::system::{Commands, Res};
use bevy::prelude::default;
use bevy_ecs_ldtk::{
    IntGridCell, LdtkEntity, LdtkIntCell, LdtkPlugin, LdtkWorldBundle, LevelSelection,
    app::LdtkEntityAppExt, app::LdtkIntCellAppExt,
};

/// Loads the LDtk world and spawns colliders for IntGrid walls.
///
/// Owns the `LdtkPlugin` registration so `main.rs` only composes plugins.
pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(LdtkPlugin)
            .insert_resource(LevelSelection::index(0))
            .register_ldtk_int_cell::<WallBundle>(1) // 1 represents the "solid" identifier in the ldtk file
            .register_ldtk_entity::<PlayerSpawnBundle>("PlayerSpawn") // name given to the player spawn entity in the ldtk file
            .add_systems(Startup, spawn_world);
    }
}

// Creating a bundle of fields we'll attach to entities later on
#[derive(Default, bevy::ecs::bundle::Bundle, LdtkIntCell)]
struct WallBundle {
    collider: Collider,
    rigid_body: RigidBody,
}

impl From<IntGridCell> for WallBundle {
    fn from(_: IntGridCell) -> Self {
        WallBundle {
            collider: Collider::rectangle(18.0, 18.0),
            rigid_body: RigidBody::Static,
        }
    }
}

/// Marker component for the position where the player should spawn.
///
/// Placed in LDtk as a `PlayerSpawn` entity; the player-spawn system reads
/// its `Transform` to position the player at startup.
#[derive(Default, bevy::ecs::component::Component)]
pub struct PlayerSpawn;

#[derive(Default, bevy::ecs::bundle::Bundle, LdtkEntity)]
struct PlayerSpawnBundle {
    spawn: PlayerSpawn,
}

fn spawn_world(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: assets.load("levels/world.ldtk").into(),
        ..default()
    });
}
