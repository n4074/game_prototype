use std::f32::consts::PI;

use bevy::{math::*, prelude::*, reflect::TypeUuid, render::{camera::CameraProjection, mesh::shape, pipeline::{PipelineDescriptor, RenderPipeline}, render_graph::{base, AssetRenderResourcesNode, RenderGraph}, renderer::RenderResources, shader::ShaderStages}};

pub struct OverlayPlugin;

impl Plugin for OverlayPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
        .add_asset::<HealthBar>()
        .add_startup_system(setup.system());
    }
}

#[derive(RenderResources, Default, TypeUuid)]
#[uuid = "3bf9e364-f29d-4d6c-92cf-93298466c623"]
struct HealthBar {
    pub colour: Color,
    pub offset: Vec3,
}

fn setup(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut pipelines: ResMut<Assets<PipelineDescriptor>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<HealthBar>>,
    mut render_graph: ResMut<RenderGraph>,
) {
    // Watch for changes
    asset_server.watch_for_changes().unwrap();

    // Create a new shader pipeline with shaders loaded from the asset directory
    let pipeline_handle = pipelines.add(PipelineDescriptor::default_config(ShaderStages {
        vertex: asset_server.load::<Shader, _>("shaders/billboard.vert"),
        fragment: Some(asset_server.load::<Shader, _>("shaders/healthbar.frag")),
    }));

    // Add an AssetRenderResourcesNode to our Render Graph. This will bind MyMaterial resources to
    // our shader
    render_graph.add_system_node(
        "healthbar",
        AssetRenderResourcesNode::<HealthBar>::new(true),
    );

    // Add a Render Graph edge connecting our new "my_material" node to the main pass node. This
    // ensures "my_material" runs before the main pass
    render_graph
        .add_node_edge("healthbar", base::node::MAIN_PASS)
        .unwrap();

    // Create a new material
    let material = materials.add(HealthBar {
        colour: Color::rgba(0.0, 0.0, 1.0, 0.0),
        offset: vec3(0.0, 2.0, 0.0),
    });


    // cube
    commands
        .spawn_bundle(MeshBundle {
            mesh: meshes.add(Mesh::from(shape::Quad { size: bevy::math::vec2(2.0, 2.0), flip: false })),
            render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
                pipeline_handle,
            )]),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        })
        .insert(material);
}