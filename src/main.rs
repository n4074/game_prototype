#![deny(unused_must_use)]
mod camera;
mod grid;
mod healthbar;
mod physics;
mod ship;

use bevy::prelude::*;

use bevy_mod_picking::*;
use camera::CameraControlPlugin;
use grid::GridPlugin;
use healthbar::HealthBarPlugin;
use physics::PhysicsPlugin;
use ship::spawn_ship;

fn main() {
    env_logger::init();

    log::debug!("Launching...");

    App::build()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        //.add_plugin(GamePlugins)
        .add_plugin(StartupPlugin)
        .add_plugin(CameraControlPlugin)
        .add_plugin(PickingPlugin)
        .add_plugin(InteractablePickingPlugin)
        .add_plugin(HighlightablePickingPlugin)
        .add_plugin(PhysicsPlugin)
        .add_plugin(GridPlugin)
        .add_plugin(HealthBarPlugin)
        .run();
}

pub struct StartupPlugin;

impl Plugin for StartupPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system());
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..Default::default()
    });

    // cube
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, -1.5, 0.0),
        ..Default::default()
    });
    //.insert_bundle(PickableBundle::default())
    //.insert_bundle(physics::RigidBodyBundle {
    //    position: [0.0, 0.5, 0.0].into(),
    //    ..physics::RigidBodyBundle::default()
    //})
    //.insert_bundle(physics::ColliderBundle {
    //    shape: physics::ColliderShape::cuboid(0.5, 0.5, 0.5),
    //    ..physics::ColliderBundle::default()
    //})
    //.insert(physics::ColliderDebugRender::with_id(1usize))
    //.insert(physics::ColliderPositionSync::Discrete);

    // light
    commands.spawn_bundle(LightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });

    // camera
    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_xyz(0.0, 2.5, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        })
        .insert(camera::CameraController::default())
        .insert_bundle(PickingCameraBundle::default())
        .insert_bundle(bevy_rapier3d::prelude::RigidBodyBundle::default());

    spawn_ship(commands, asset_server, meshes, materials);
    //spawn_ship(commands, meshes, materials);
}
