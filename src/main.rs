#![cfg_attr(not(target_os = "macos"), feature(derive_default_enum))]
//#![feature(const_generics)]
#![deny(unused_must_use)]
#![warn(unused_imports)]
//mod camera;
mod debug;
mod input;
mod materials;
//mod movement;
mod physics;
//mod selection;
mod player;
//mod skysphere;
//mod units;

//mod orders;

use anyhow::Result;
use bevy::{pbr::wireframe, prelude::*};
use bevy_mod_picking::*;
use input::InputPlugin;
//use materials::overlay::{Overlay, OverlayPlugin};
//use movement::PlayerControllerPlugin;
use physics::PhysicsPlugin;
//use skysphere::SkySpherePlugin;
//use units::ship::{spawn_ship, spawn_station};

#[derive(SystemLabel, Clone, Debug, PartialEq, Eq, Hash)]
enum SystemLabels {
    Input,
    Camera,
}

const SAMPLES: u32 = if cfg!(target_os = "macos") { 4 } else { 4 };

fn main() -> Result<()> {
    let mut app = App::new();

    app.insert_resource(Msaa { samples: SAMPLES })
        .add_plugins(DefaultPlugins)
        .add_plugin(StartupPlugin)
        .add_plugin(PickingPlugin)
        .add_plugin(InteractablePickingPlugin)
        .add_plugin(HighlightablePickingPlugin)
        .add_plugin(PhysicsPlugin)
        //.add_plugin(OverlayPlugin)
        .add_plugin(InputPlugin)
        //.add_plugin(SkySpherePlugin)
        .add_plugin(debug::DebugPlugin)
        .add_plugin(materials::MaterialsPlugin)
        //.add_plugin(ToonPlugin)
        .add_plugins(player::PlayerPluginGroup)
        //.add_plugin(units::UnitsPlugin)
        ;

    bevy::log::debug!("Got here");

    debug::dump_render_graph(&mut app)?;

    app.run();
    Ok(())
}

pub struct StartupPlugin;

impl Plugin for StartupPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup.system());
    }
}

fn setup(
    mut commands: Commands,
    mut asset_server: ResMut<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut color: ResMut<Assets<ColorMaterial>>,
    //mut overlay_materials: ResMut<Assets<Overlay>>,
    mut ambient_light: ResMut<bevy::pbr::AmbientLight>,
) {
    //debug!("Ambient_light: {:?}", ambient_light.color);
    //ambient_light.brightness = 0f32;

    //for i in -3..3 {
    //    for j in -3..3 {
    //        for k in -3..3 {
    //            if i == 0 && j == 0 && k == 0 {
    //                continue;
    //            }

    //            let transform =
    //                Transform::from_xyz(-i as f32 * 1.001, j as f32 * 1.001, k as f32 * 1.001);

    //            spawn_ship(
    //                transform,
    //                &mut commands,
    //                &mut asset_server,
    //                &mut overlay_materials,
    //                &mut materials,
    //            );
    //        }
    //    }
    //}

    //let _transform = Transform::from_xyz(5.0, -0.5, -0.5);

    //spawn_ship(
    //    Transform::from_xyz(5.0, -0.5, -0.5),
    //    &mut commands,
    //    &mut asset_server,
    //    &mut overlay_materials,
    //    &mut materials,
    //);

    //spawn_station(
    //    Transform::from_xyz(-100.0, 0.0, 0.0),
    //    &mut commands,
    //    &mut asset_server,
    //    &mut materials,
    //    &mut color,
    //);

    // light
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        point_light: PointLight {
            intensity: 1000f32,
            ..Default::default()
        },
        ..Default::default()
    });

    // camera

    let cube_handle = asset_server.load("models/houdini/cube.gltf#Mesh0/Primitive0");

    //commands.spawn_bundle(PbrBundle {
    //    mesh: cube_handle,
    //    //material: material_handle.clone(),
    //    transform: Transform::from_xyz(10.0, 0.0, 0.0),
    //    ..Default::default()
    //});

    commands
        .spawn_bundle(PbrBundle {
            mesh: cube_handle,
            //    material: material_handle.clone(),
            material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..Default::default()
        })
        .insert(wireframe::Wireframe)
        .insert(materials::overlay::OverlayComponent {});

    // plane
    //commands.spawn_bundle(PbrBundle {
    //    mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
    //    material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
    //    ..Default::default()
    //});
    //// cube
    //commands.spawn_bundle(PbrBundle {
    //    mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
    //    material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
    //    transform: Transform::from_xyz(0.0, 0.5, 0.0),
    //    ..Default::default()
    //});
    //// light
    //commands.spawn_bundle(PointLightBundle {
    //    point_light: PointLight {
    //        intensity: 1500.0,
    //        shadows_enabled: true,
    //        ..Default::default()
    //    },
    //    transform: Transform::from_xyz(4.0, 8.0, 4.0),
    //    ..Default::default()
    //});
    // camera
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}
