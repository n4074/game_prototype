use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::{
        pipeline::{self, PipelineDescriptor},
        render_graph::{base, AssetRenderResourcesNode, RenderGraph},
        renderer::RenderResources,
        shader::ShaderStages,
    },
};

pub struct SkySpherePlugin;

impl Plugin for SkySpherePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup.system())
            .add_asset::<SkySphere>();
    }
}

#[derive(Bundle, Default)]
struct SkySphereBundle {
    #[bundle]
    mesh: MeshBundle,
}

#[derive(RenderResources, Default, TypeUuid)]
#[uuid = "3bf9e364-f29d-4d6c-92cf-93298466c333"]
pub struct SkySphere {
    texture: Handle<Texture>,
}

pub const PIPELINE_HANDLE: HandleUntyped = HandleUntyped::weak_from_u64(
    PipelineDescriptor::TYPE_UUID,
    const_random::const_random!(u64),
);

pub fn setup(
    mut commands: Commands,
    mut render_graph: ResMut<RenderGraph>,
    asset_server: Res<AssetServer>,
    mut pipelines: ResMut<Assets<PipelineDescriptor>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut skysphere: ResMut<Assets<SkySphere>>,
) {
    const SKYSPHERE: &'static str = "Skysphere";

    let texture = asset_server.load("textures/skysphere/skysphere1.mantra1.png");

    let shaders = ShaderStages {
        vertex: asset_server.load::<Shader, _>("shaders/skysphere/skysphere.vert.spv"),
        fragment: Some(asset_server.load::<Shader, _>("shaders/skysphere/skysphere.frag.spv")),
    };

    // Silly hack due to https://github.com/bevyengine/bevy/issues/1359
    // which results in panics randomly when a pipeline is created before the shader assets are loaded
    std::thread::sleep(std::time::Duration::from_millis(200));

    let mut descriptor = PipelineDescriptor::default_config(shaders);

    if let Some(stencil) = &mut descriptor.depth_stencil {
        stencil.depth_write_enabled = false;
        stencil.depth_compare = pipeline::CompareFunction::LessEqual;
    }

    render_graph.add_system_node(SKYSPHERE, AssetRenderResourcesNode::<SkySphere>::new(true));

    render_graph
        .add_node_edge(SKYSPHERE, base::node::MAIN_PASS)
        .unwrap();

    pipelines.set_untracked(PIPELINE_HANDLE, descriptor);

    commands.spawn_bundle(SkySphereBundle {
        ..Default::default()
    });

    commands
        .spawn_bundle(MeshBundle {
            mesh: meshes.add(shape::Quad::new(Vec2::new(2.0, 2.0)).into()),
            render_pipelines: RenderPipelines::from_handles(&[PIPELINE_HANDLE.typed()]),
            ..Default::default()
        })
        .insert(skysphere.add(SkySphere { texture }));
}
