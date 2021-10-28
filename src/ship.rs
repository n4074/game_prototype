use bevy::prelude::*;
use bevy_mod_picking::*;

use crate::materials::overlay;
use crate::physics;

pub struct Selected;

#[derive(Bundle, Default)]
struct ShipBundle {
    //collider_render: physics::ColliderDebugRender,
    #[bundle]
    pbr_bundle: PbrBundle,
    collider_position_sync: physics::ColliderPositionSync,
    #[bundle]
    collider: physics::ColliderBundle,
    #[bundle]
    rigid_body: physics::RigidBodyBundle,
    #[bundle]
    pickable: PickableBundle,
}

pub fn spawn_station(
    transform: Transform,
    commands: &mut Commands,
    asset_server: &mut AssetServer,
    materials: &mut Assets<StandardMaterial>,
    color: &mut Assets<ColorMaterial>,
) {
    let station_handle = asset_server.load("models/ships/iss/ISS_stationary.gltf#Mesh0/Primitive0");

    let material_handle = materials.add(StandardMaterial {
        base_color: Color::rgb(1.0, 1.0, 1.0),
        ..Default::default()
    });

    //commands.spawn_bundle(PbrBundle {
    //    mesh: station_handle,
    //    material: material_handle,
    //    transform,
    //    ..Default::default()
    //});

    //commands.spawn_bundle(MeshBundle {});
    commands
        .spawn_bundle(MeshBundle {
            mesh: station_handle,
            render_pipelines: RenderPipelines::from_handles(&[
                crate::materials::toon::TOON_PIPELINE_HANDLE.typed(),
            ]),
            ..Default::default()
        })
        .insert(color.add(ColorMaterial {
            color: Color::RED,
            ..Default::default()
        }));
}

pub fn spawn_ship(
    transform: Transform,
    commands: &mut Commands,
    asset_server: &mut AssetServer,
    overlay_materials: &mut Assets<overlay::Overlay>,
    materials: &mut Assets<StandardMaterial>,
) {
    let cube_handle = asset_server.load("models/houdini/torchship.gltf#Mesh0/Primitive0");

    let material_handle = materials.add(StandardMaterial {
        base_color: Color::rgb(0.8, 0.7, 0.6),
        ..Default::default()
    });

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
            pbr_bundle: PbrBundle {
                mesh: cube_handle,
                material: material_handle,
                ..Default::default()
            },
            ..Default::default()
        })
        .id();

    overlay::attach_ship_overlay(ship, commands, asset_server, overlay_materials);
}
