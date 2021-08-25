use bevy::prelude::*;
use bevy_mod_picking::*;

use crate::physics;
use crate::overlay;

#[derive(Bundle, Default)]
struct ShipBundle {
    collider_render: physics::ColliderDebugRender,
    collider_position_sync: physics::ColliderPositionSync,
    #[bundle]
    collider: physics::ColliderBundle,
    #[bundle]
    rigid_body: physics::RigidBodyBundle,
    #[bundle]
    pickable: PickableBundle,
}

pub fn spawn_ship(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let ship = commands
        .spawn_bundle(ShipBundle {
            rigid_body: physics::RigidBodyBundle {
                position: [0.0, 0.5, 0.0].into(),
                ..physics::RigidBodyBundle::default()
            },
            collider: physics::ColliderBundle {
                shape: physics::ColliderShape::cuboid(0.5, 0.5, 0.5),
                ..physics::ColliderBundle::default()
            },
            ..Default::default()
        }).id();

    //overlay::attach_ship_overlay(ship, commands, , commands, symbols, meshes, colour_materials, billboard_materials, healthbar_materials, overlay_materials)
}
