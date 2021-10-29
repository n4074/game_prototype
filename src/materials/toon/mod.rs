use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::{
        camera::{ActiveCameras, Camera},
        pass::{
            LoadOp, Operations, PassDescriptor, RenderPassColorAttachmentDescriptor,
            RenderPassDepthStencilAttachmentDescriptor, TextureAttachment,
        },
        pipeline::PipelineDescriptor,
        render_graph::{
            base::{self, MainPass},
            AssetRenderResourcesNode, CameraNode, PassNode, RenderGraph, WindowSwapChainNode,
            WindowTextureNode,
        },
        renderer::RenderResources,
        shader::ShaderStages,
        texture::{Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsage},
    },
    window::{CreateWindow, WindowId},
};

pub struct ToonPlugin;

impl Plugin for ToonPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_asset::<Toon>().add_startup_system(setup.system());
    }
}

#[derive(RenderResources, Default, TypeUuid)]
#[uuid = "3bf9e364-f29d-4d6c-92cf-932983663333"]
pub struct Toon;

pub const TOON_PIPELINE_HANDLE: HandleUntyped = HandleUntyped::weak_from_u64(
    PipelineDescriptor::TYPE_UUID,
    const_random::const_random!(u64),
);

const TOON_VERTEX_SHADER_PATH: &str = "shaders/toon/toon.vert.spv";
const TOON_FRAGMENT_SHADER_PATH: &str = "shaders/toon/toon.frag.spv";

pub fn setup(
    mut render_graph: ResMut<RenderGraph>,
    asset_server: Res<AssetServer>,
    mut pipelines: ResMut<Assets<PipelineDescriptor>>,
) {
    asset_server.watch_for_changes().unwrap();
    const TOON_NODE: &'static str = "Toon";

    let vert_shader = asset_server.load::<Shader, _>(TOON_VERTEX_SHADER_PATH);
    let frag_shader = asset_server.load::<Shader, _>(TOON_FRAGMENT_SHADER_PATH);

    // Silly hack due to https://github.com/bevyengine/bevy/issues/1359
    // which results in panics randomly when a pipeline is created before the shader assets are loaded
    std::thread::sleep(std::time::Duration::from_millis(200));

    render_graph.add_system_node(TOON_NODE, AssetRenderResourcesNode::<Toon>::new(true));

    render_graph
        .add_node_edge(TOON_NODE, base::node::MAIN_PASS)
        .unwrap();

    let descriptor = PipelineDescriptor::default_config(ShaderStages {
        vertex: vert_shader,
        fragment: Some(frag_shader),
    });

    pipelines.set_untracked(TOON_PIPELINE_HANDLE, descriptor);
}
