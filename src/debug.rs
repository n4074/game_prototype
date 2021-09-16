use bevy::prelude::*;
use bevy::render::wireframe::{WireframeConfig, WireframePlugin};
use bevy::wgpu::{WgpuFeature, WgpuFeatures, WgpuOptions};
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_prototype_debug_lines::{DebugLines, DebugLinesPlugin};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            // wireframe doesn't seem to work despite WgpuFeatures option set below
            .add_plugin(WireframePlugin)
            .insert_resource(WgpuOptions {
                features: WgpuFeatures {
                    // The Wireframe requires NonFillPolygonMode feature
                    features: vec![WgpuFeature::NonFillPolygonMode],
                },
                ..Default::default()
            })
            .add_plugin(DebugLinesPlugin)
            // enable depth testing for debug lines
            .insert_resource(DebugLines {
                depth_test: true,
                ..Default::default()
            })
            .add_plugin(WorldInspectorPlugin::new())
            .add_startup_system(setup_wireframe.system());
    }
}

pub fn setup_wireframe(mut wireframe_config: ResMut<WireframeConfig>) {
    wireframe_config.global = false;
}
