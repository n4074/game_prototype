use bevy::prelude::*;
use bevy_mod_picking::*;

use crate::physics;

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
    commands.spawn_bundle(ShipBundle {
        rigid_body: physics::RigidBodyBundle {
            position: [0.0, 0.5, 0.0].into(),
            ..physics::RigidBodyBundle::default()
        },
        collider: physics::ColliderBundle {
            shape: physics::ColliderShape::cuboid(0.5, 0.5, 0.5),
            ..physics::ColliderBundle::default()
        },
        ..Default::default()
    });
    //let tube = asset_server.load("models/tube.gltf#Meshx0/Primitive1");

    //commands.spawn_bundle(PbrBundle {
    //        mesh: tube,
    //        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
    //        transform: Transform::from_xyz(0.0, 0.5, 0.0),
    //        ..Default::default()
    //    })
    //    .insert_bundle(PickableBundle::default())
    //    .insert_bundle(physics::RigidBodyBundle {
    //        position: [0.0, 0.5, 0.0].into(),
    //        ..physics::RigidBodyBundle::default()
    //    })
    //    .insert_bundle(physics::ColliderBundle {
    //        shape: physics::ColliderShape::cuboid(0.5, 0.5, 0.5),
    //        ..physics::ColliderBundle::default()
    //    })
    //    .insert(physics::ColliderDebugRender::with_id(1usize))
    //    .insert(physics::ColliderPositionSync::Discrete);
}
