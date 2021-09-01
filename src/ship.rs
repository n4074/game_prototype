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
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut colour_materials: ResMut<Assets<ColorMaterial>>,
    mut healthbar_materials: ResMut<Assets<overlay::HealthBar>>,
    mut billboard_materials: ResMut<Assets<overlay::Billboard>>,
) {
    let ship = commands
        .spawn_bundle(ShipBundle {
            rigid_body: physics::RigidBodyBundle {
                position: [0.0, 0.5, 0.0].into(),
                ..physics::RigidBodyBundle::default()
            },
            collider: physics::ColliderBundle {
                shape: physics::ColliderShape::cuboid(1.0, 1.0, 1.0),
                ..physics::ColliderBundle::default()
            },
            ..Default::default()
        })
        .insert(Selected)
        .id();

    overlay::attach_ship_overlay(ship, commands, asset_server, colour_materials, billboard_materials, healthbar_materials);
}
