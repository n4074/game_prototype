use bevy::{
    math::*,
    prelude::*,
    reflect::TypeUuid,
    render::{
        mesh::shape,
        pipeline::{PipelineDescriptor},
        render_graph::{base, AssetRenderResourcesNode, RenderGraph},
        renderer::RenderResources,
        shader::{ShaderStages, ShaderDefs},
    },
};

pub struct HealthBarPlugin;

impl Plugin for HealthBarPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_asset::<Overlay>()
            .add_startup_system(setup.system());
    }
}

#[derive(RenderResources, Default, TypeUuid, ShaderDefs)]
#[uuid = "3bf9e364-f29d-4d6c-92cf-93298466c627"]
pub struct Overlay {
    pub healthbar_transform: GlobalTransform,
    pub healthbar_fill: f32,
    pub icon_colour: Color,
    pub icon_transform: GlobalTransform,
    #[shader_def]
    pub icon_texture: Option<Handle<Texture>>,
}

fn setup(
    asset_server: Res<AssetServer>,
    mut pipelines: ResMut<Assets<PipelineDescriptor>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut render_graph: ResMut<RenderGraph>,
) {
    add_overlay_graph(&mut render_graph, &asset_server, &mut pipelines);

    meshes.set_untracked(SIMPLE_QUAD_MESH_HANDLE, Mesh::from(shape::Quad {
        size: bevy::math::vec2(1.0, 1.0),
        flip: false,
    }));
}

pub fn attach_ship_overlay(
    ship: Entity, 
    commands: &mut Commands, 
    asset_server: &AssetServer,
    overlay_materials: &mut Assets<Overlay>,
) {
    let texture_handle = asset_server.load("textures/unit_overlays/test.png");

    let overlay_material = overlay_materials.add(Overlay {
        healthbar_transform: GlobalTransform { 
            scale: vec3(2.0, 0.1, 0.0), 
            translation: vec3(0.0, 1.0, 0.0),
            ..GlobalTransform::default() 
        },
        healthbar_fill: 0.5, 
        icon_colour: Color::rgb(0.0, 1.0, 0.0),
        icon_texture: Some(texture_handle.clone()),
        ..Overlay::default()
    });

    let overlay = commands
        .spawn_bundle(MeshBundle {
            mesh: SIMPLE_QUAD_MESH_HANDLE.typed(),
            render_pipelines: RenderPipelines::from_handles(
                //&[ICON_PIPELINE_HANDLE.typed(), HEALTHBAR_PIPELINE_HANDLE.typed()]
                &[]
            ),
            visible: Visible {
                is_transparent: true,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(overlay_material)
        .id();

    commands.entity(ship).push_children(&[overlay]);
} 

pub const OVERLAY: &str = "Overlay";

pub const HEALTHBAR_PIPELINE_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(PipelineDescriptor::TYPE_UUID, 12976993416349439669);

pub const ICON_PIPELINE_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(PipelineDescriptor::TYPE_UUID, 1017551961862589956);

pub const SIMPLE_QUAD_MESH_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(Mesh::TYPE_UUID,12515628229712380851);

pub fn add_overlay_graph(
    render_graph: &mut RenderGraph,
    asset_server: &Res<AssetServer>,
    pipelines: &mut Assets<PipelineDescriptor>,
) {

    render_graph.add_system_node(
        OVERLAY,
        AssetRenderResourcesNode::<Overlay>::new(true),
    );
 
    render_graph    
        .add_node_edge(OVERLAY, base::node::MAIN_PASS)
        .unwrap();

    pipelines.set_untracked(HEALTHBAR_PIPELINE_HANDLE, 
        PipelineDescriptor::default_config(ShaderStages {
            vertex: asset_server.load::<Shader,_>("shaders/healthbar.vert"),
            fragment: Some(asset_server.load::<Shader,_>("shaders/healthbar.frag")),
        })
    );

    pipelines.set_untracked(ICON_PIPELINE_HANDLE, 
        PipelineDescriptor::default_config(ShaderStages {
            vertex: asset_server.load::<Shader,_>("shaders/icon.vert"),
            fragment: Some(asset_server.load::<Shader,_>("shaders/icon.frag")),
        })
    );
}