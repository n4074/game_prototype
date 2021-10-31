#![cfg_attr(not(target_os = "macos"), feature(derive_default_enum))]
//#![feature(const_generics)]
#![deny(unused_must_use)]
#![warn(unused_imports)]
//mod camera;
mod debug;
mod input;
mod materials;
mod movement;
mod physics;
//mod selection;
mod player;
mod ship;
mod skysphere;

mod orders;

use bevy::prelude::*;

use bevy_mod_picking::*;
use input::InputPlugin;
use materials::{
    overlay::{Overlay, OverlayPlugin},
    toon::ToonPlugin,
};
use movement::PlayerControllerPlugin;
use physics::PhysicsPlugin;
use ship::{spawn_ship, spawn_station};
use skysphere::SkySpherePlugin;

#[derive(SystemLabel, Clone, Debug, PartialEq, Eq, Hash)]
enum SystemLabels {
    Input,
    Camera,
}

fn main() {
    env_logger::init();

    log::debug!("Launching...");

    App::build()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        //.add_plugin(GamePlugins)
        .add_plugin(StartupPlugin)
        //.add_plugin(CameraControlPlugin)
        .add_plugin(PickingPlugin)
        .add_plugin(InteractablePickingPlugin)
        .add_plugin(HighlightablePickingPlugin)
        .add_plugin(PhysicsPlugin)
        .add_plugin(PlayerControllerPlugin)
        .add_plugin(OverlayPlugin)
        .add_plugin(crate::orders::OrdersPlugin)
        //.add_plugin(SelectionPlugin)
        .add_plugin(InputPlugin)
        .add_plugin(SkySpherePlugin)
        .add_plugin(debug::DebugPlugin)
        .add_plugin(ToonPlugin)
        .add_plugins(player::PlayerPluginGroup)
        //.insert_resource(ClearColor(Color::BLACK))
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
    mut color: ResMut<Assets<ColorMaterial>>,
    mut overlay_materials: ResMut<Assets<Overlay>>,
    mut ambient_light: ResMut<bevy::pbr::AmbientLight>,
) {
    //debug!("Ambient_light: {:?}", ambient_light.color);
    //ambient_light.brightness = 0f32;

    for i in -3..3 {
        for j in -3..3 {
            for k in -3..3 {
                if i == 0 && j == 0 && k == 0 {
                    continue;
                }

                //let transform =
                //    Transform::from_xyz(-i as f32 * 1.001, j as f32 * 1.001, k as f32 * 1.001);

                //spawn_ship(
                //    transform,
                //    &mut commands,
                //    &mut asset_server,
                //    &mut overlay_materials,
                //    &mut materials,
                //);
            }
        }
    }

    let _transform = Transform::from_xyz(5.0, -0.5, -0.5);

    spawn_ship(
        Transform::from_xyz(5.0, -0.5, -0.5),
        &mut commands,
        &mut asset_server,
        &mut overlay_materials,
        &mut materials,
    );

    spawn_station(
        Transform::from_xyz(-100.0, 0.0, 0.0),
        &mut commands,
        &mut asset_server,
        &mut materials,
        &mut color,
    );

    // light
    commands.spawn_bundle(LightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        light: Light {
            intensity: 1000f32,
            ..Default::default()
        },
        ..Default::default()
    });

    // camera
    

    let cube_handle = asset_server.load("models/houdini/cube.gltf#Mesh0/Primitive0");

    commands.spawn_bundle(PbrBundle {
        mesh: cube_handle,
        //material: material_handle.clone(),
        transform: Transform::from_xyz(10.0, 0.0, 0.0),
        ..Default::default()
    });
}
