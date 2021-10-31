use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;

pub mod camera;
pub mod selection;

pub struct PlayerPluginGroup;

impl PluginGroup for PlayerPluginGroup {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group
            .add(camera::CameraControlPlugin)
            .add(selection::SelectionPlugin);
    }
}
//struct PlayerCamera {
//    #[bundle]
//    cam: PerspectiveCameraBundle,
//    controller: CameraController,
//    mouseray: Option<MouseRay>,
//    cursor: Option<WorldCursor>
//}