use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;

pub mod camera;
pub mod commands;
pub mod selection;

pub struct PlayerPluginGroup;

impl PluginGroup for PlayerPluginGroup {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group
            .add(camera::CameraControlPlugin)
            .add(selection::SelectionPlugin)
            .add(commands::CommandPlugin);
    }
}
//struct PlayerCamera {
//    #[bundle]
//    cam: PerspectiveCameraBundle,
//    controller: CameraController,
//    mouseray: Option<MouseRay>,
//    cursor: Option<WorldCursor>
//}
