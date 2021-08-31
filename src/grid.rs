use std::f32::consts::PI;

use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::{
        mesh::shape,
        pipeline::{PipelineDescriptor, RenderPipeline},
        render_graph::{base, AssetRenderResourcesNode, RenderGraph},
        renderer::RenderResources,
        shader::ShaderStages,
    },
};

use bevy_prototype_lyon::prelude::*;

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_asset::<OverlayShading>()
            .add_plugin(ShapePlugin)
            .add_startup_system(setup.system());
    }
}

#[derive(RenderResources, Default, TypeUuid)]
#[uuid = "3bf9e364-f29d-4d6c-92cf-93298466c6ff"]
struct OverlayShading;

fn setup(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut pipelines: ResMut<Assets<PipelineDescriptor>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut render_graph: ResMut<RenderGraph>,
) {
    // Watch for changes
    asset_server.watch_for_changes().unwrap();

    let pipeline_handle = pipelines.add(PipelineDescriptor::default_config(ShaderStages {
        vertex: asset_server.load::<Shader, _>("shaders/grid.vert"),
        fragment: Some(asset_server.load::<Shader, _>("shaders/grid.frag")),
    }));

    // Add an AssetRenderResourcesNode to our Render Graph. This will bind MyMaterial resources to
    // our shader
    render_graph.add_system_node(
        "overlayshader",
        AssetRenderResourcesNode::<OverlayShading>::new(true),
    );

    // Add a Render Graph edge connecting our new "my_material" node to the main pass node. This
    // ensures "my_material" runs before the main pass
    render_graph
        .add_node_edge("overlayshader", base::node::MAIN_PASS)
        .unwrap();

    // Create a new material
    let material = materials.add(Color::RED.into());

    let shape = shapes::Circle {
        radius: 10.0,
        ..shapes::Circle::default()
    };

    let mut stroke = StrokeOptions::default();
    stroke.line_width = 0.5;


    let shape_bundle = GeometryBuilder::build_as(
        &shape,
        ShapeColors::outlined(Color::GREEN, Color::BLACK),
        DrawMode::Stroke(stroke),
        Transform::from_xyz(0.0, 2.0, 0.0)
    );

    commands
        .spawn_bundle(MeshBundle {
            //mesh: shape_bundle.mesh,
            mesh: meshes.add(Mesh::from(shape::Quad {
                size: bevy::math::vec2(2.0, 2.0),
                flip: false,
            })),
            render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
                pipeline_handle,
            )]),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        })
        .insert(material);

}
