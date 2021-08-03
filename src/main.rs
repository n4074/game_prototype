#![deny(unused_must_use)]
use bevy::{app::PluginGroupBuilder, prelude::*, render::camera::PerspectiveProjection};

mod camera;

use camera::CameraControlPlugin;

fn main() {
     App::build()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        //.add_plugin(GamePlugins)
        .add_plugin(StartupPlugin)
        .add_plugin(CameraControlPlugin)
        .run();
}

    /// A group of plugins that produce the "hello world" behavior
pub struct GamePlugins;

impl PluginGroup for GamePlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group
            .add(StartupPlugin)
            .add(PrintWorldPlugin);
    }
}

pub struct StartupPlugin;

impl Plugin for StartupPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system());
    }
}

fn setup(
    mut commands: Commands,
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
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..Default::default()
    });
    // light
    commands.spawn_bundle(LightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });
    // camera
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    }).insert(camera::CameraController::default());
}

pub struct PrintWorldPlugin;

impl Plugin for PrintWorldPlugin {
    fn build(&self, app: &mut AppBuilder) {
        //app.add_system(print_world_system.system());
    }
}

fn print_world_system() {
    println!("world");
}