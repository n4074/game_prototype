use std::process::{ExitStatus, Stdio};

//use bevy::render::wireframe::{WireframeConfig, WireframePlugin};
//use bevy::wgpu::{WgpuFeature, WgpuFeatures, WgpuOptions};
use anyhow::Result;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_prototype_debug_lines::{DebugLines, DebugLinesPlugin};

use bevy::prelude::*;
use bevy::{
    pbr::wireframe::{Wireframe, WireframeConfig, WireframePlugin},
    render::{options::WgpuOptions, render_resource::WgpuFeatures},
};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WgpuOptions {
            features: WgpuFeatures::POLYGON_MODE_LINE,
            ..Default::default()
        })
        .add_plugin(WireframePlugin)
        .add_plugin(DebugLinesPlugin::with_depth_test(true))
        .add_plugin(WorldInspectorPlugin::new())
        .add_startup_system(setup);
    }
}

pub fn setup(mut wireframe_config: ResMut<WireframeConfig>) {
    wireframe_config.global = false;
}

fn render_dotviz<P: AsRef<std::path::Path>>(graph: String, path: P) -> Result<ExitStatus> {
    use std::io::Write;
    use std::process::Command;

    let mut dot = Command::new("dot")
        .arg("-Tsvg")
        .arg("-o")
        .arg(path.as_ref().as_os_str())
        .stdin(Stdio::piped())
        .spawn()?;

    write!(dot.stdin.as_ref().unwrap(), "{}", graph)?;
    let ecode = dot.wait()?;
    Ok(ecode)
}

pub fn dump_render_graph(app: &mut App) -> Result<()> {
    use bevy::render::{render_graph::RenderGraph, RenderApp, RenderStage};

    app.update();

    let render_app = app.get_sub_app(RenderApp).expect("no render app");
    let render_graph = render_app.world.get_resource::<RenderGraph>().unwrap();

    let render_graph = bevy_mod_debugdump::render_graph::render_graph_dot(&*render_graph);
    render_dotviz(render_graph, "docs/render_graph.svg")?;

    let render_schedule = bevy_mod_debugdump::schedule_graph::schedule_graph_dot(app);
    render_dotviz(render_schedule, "docs/render_schedule.svg")?;

    let system_schedule = bevy_mod_debugdump::schedule_graph::schedule_graph_dot_sub_app_styled(
        app,
        RenderApp,
        &[&RenderStage::Extract],
        &bevy_mod_debugdump::schedule_graph::ScheduleGraphStyle::default(),
    );

    render_dotviz(system_schedule, "docs/system_schedule.svg")?;

    Ok(())
}
