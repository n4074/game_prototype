use bevy::{
    prelude::{shape::Cube, *},
    reflect::TypeUuid,
    render::{
        pipeline::{self, DepthStencilState, PipelineDescriptor},
        render_graph::{base, AssetRenderResourcesNode, RenderGraph},
        renderer::RenderResources,
        shader::ShaderStages,
    },
};

pub struct ToonPlugin;

impl Plugin for ToonPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_asset::<Toon>().add_startup_system(setup.system());
    }
}

#[derive(RenderResources, Default, TypeUuid)]
#[uuid = "3bf9e364-f29d-4d6c-92cf-932983663333"]
pub struct Toon {
    pub something: f32,
}

pub const TOON_PIPELINE_HANDLE: HandleUntyped = HandleUntyped::weak_from_u64(
    PipelineDescriptor::TYPE_UUID,
    const_random::const_random!(u64),
);

const VERTEX_SHADER: &str = r#"
#version 450
layout(location = 0) in vec3 Vertex_Position;

layout(set = 0, binding = 0) uniform CameraViewProj {
    mat4 ViewProj;
};
layout(set = 1, binding = 0) uniform Transform {
    mat4 Model;
};
void main() {
    gl_Position = ViewProj * Model * vec4(Vertex_Position, 1.0);
}
"#;

const FRAGMENT_SHADER: &str = r#"
#version 450
layout(location = 0) out vec4 o_Target;
void main() {
    o_Target = vec4(1.0, 0.0, 0.0, 1.0);
}
"#;

const TOON_VERTEX_SHADER_PATH: &str = "shaders/toon/shaders/toon/toon.vert";
const TOON_FRAGMENT_SHADER_PATH: &str = "shaders/toon/shaders/toon/toon.frag";

pub fn setup(
    mut commands: Commands,
    mut render_graph: ResMut<RenderGraph>,
    asset_server: Res<AssetServer>,
    mut pipelines: ResMut<Assets<PipelineDescriptor>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut colormaterial: ResMut<Assets<StandardMaterial>>,
) {
    const TOON_NODE: &'static str = "Toon";

    let mut descriptor = PipelineDescriptor::default_config(ShaderStages {
        vertex: asset_server.load::<Shader, _>("shaders/toon/shaders/toon/toon.vert"),
        fragment: Some(asset_server.load::<Shader, _>("shaders/toon/shaders/toon.frag")),
    });

    if let Some(stencil) = &mut descriptor.depth_stencil {
        stencil.depth_write_enabled = false;
        stencil.depth_compare = pipeline::CompareFunction::LessEqual;
    }

    render_graph.add_system_node(TOON_NODE, AssetRenderResourcesNode::<Toon>::new(true));

    render_graph
        .add_node_edge(TOON_NODE, base::node::MAIN_PASS)
        .unwrap();

    pipelines.set_untracked(PIPELINE_HANDLE, descriptor);

    commands.spawn_bundle(MeshBundle {
        mesh: meshes.add(shape::Quad::new(Vec2::new(2.0, 2.0)).into()),
        render_pipelines: RenderPipelines::from_handles(&[PIPELINE_HANDLE.typed()]),
        ..Default::default()
    });
}
