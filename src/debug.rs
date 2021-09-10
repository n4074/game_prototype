
use bevy::prelude::*;
use bevy::render::render_graph::RenderGraph;
use bevy_prototype_debug_lines::{DebugLines, DebugLinesPlugin};
use bevy_mod_debugdump::schedule_graph::schedule_graph_dot;
use bevy::render::wireframe::{WireframePlugin, WireframeConfig};
use bevy::wgpu::{WgpuOptions, WgpuFeatures, WgpuFeature};
use bevy_inspector_egui::WorldInspectorPlugin;

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
            .insert_resource(DebugLines { depth_test: true, ..Default::default() })
            .add_plugin(WorldInspectorPlugin::new())
            .add_startup_system(setup_wireframe.system())
            //.add_startup_system(print_render_schedule.system())
            ;
    }
}

pub fn setup_wireframe(
    mut wireframe_config: ResMut<WireframeConfig>,
) {
    wireframe_config.global = false;
}

pub fn print_render_schedule(mut render_graph: ResMut<RenderGraph>) {
    let schedule = render_graph.take_schedule().unwrap();
    let dot = schedule_graph_dot(&schedule);
    render_graph.set_schedule(schedule);
    println!("{}", dot);
}