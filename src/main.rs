#![deny(unused_must_use)]
mod camera;
mod movement;
mod overlay;
mod physics;
mod ship;
mod selection;

use bevy::prelude::*;

use bevy_mod_picking::*;
use camera::CameraControlPlugin;
use movement::PlayerControllerPlugin;
use overlay::{HealthBarPlugin};
use physics::PhysicsPlugin;
use selection::SelectionPlugin;
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
        .add_plugin(PlayerControllerPlugin)
        .add_plugin(HealthBarPlugin)
        .add_plugin(SelectionPlugin)
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
    mut asset_server: ResMut<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut overlay_materials: ResMut<Assets<overlay::Overlay>>,
) {
    for i in -3..3 {
        for j in -3..3 {
            for k in -3..3 {

                if i == 0 && j == 0 && k == 0 {
                    continue;
                }

                let transform = Transform::from_xyz(-i as f32 * 1.001, j as f32 * 1.001, k as f32 * 1.001);

                spawn_ship(transform, &mut commands, &mut asset_server, &mut overlay_materials, &mut materials);
            }
        }
    }

    let transform = Transform::from_xyz(5.0, -0.5, -0.5);

    spawn_ship(transform, &mut commands, &mut asset_server, &mut overlay_materials, &mut materials);

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

    let cube_handle = asset_server.load("models/houdini_cube/cube.gltf#Mesh0/Primitive0");

     // You can also add assets directly to their Assets<T> storage:
    let material_handle = materials.add(StandardMaterial {
        base_color: Color::rgb(0.8, 0.7, 0.6),
        ..Default::default()
    });

    commands.spawn_bundle(PbrBundle {
        mesh: cube_handle,
        //material: material_handle.clone(),
        transform: Transform::from_xyz(10.0, 0.0, 0.0),
        ..Default::default()
    });
}
