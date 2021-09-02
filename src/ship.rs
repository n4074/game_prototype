use bevy::prelude::*;
use bevy_mod_picking::*;

use crate::physics;
use crate::overlay;

pub struct Selected;

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
    transform: Transform,
    commands: &mut Commands,
    asset_server: &AssetServer,
    overlay_materials: &mut Assets<overlay::Overlay>,
) {
    let ship = commands
        .spawn_bundle(ShipBundle {
            rigid_body: physics::RigidBodyBundle {
                //position: [0.0, 0.5, 0.0].into(),
                position: (transform.translation, transform.rotation).into(),
                ..physics::RigidBodyBundle::default()
            },
            collider: physics::ColliderBundle {
                shape: physics::ColliderShape::cuboid(0.5, 0.5, 0.5),
                ..physics::ColliderBundle::default()
            },
            ..Default::default()
        })
        .id();

    overlay::attach_ship_overlay(ship, commands, asset_server, overlay_materials);
}
