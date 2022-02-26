use bevy::prelude::*;
pub mod overlay;
//pub mod toon;

pub struct MaterialsPlugin;

impl Plugin for MaterialsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(overlay::OverlayPlugin);
    }
}
