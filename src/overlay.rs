use bevy::{
    math::*,
    prelude::*,
    reflect::TypeUuid,
    render::{
        camera::CameraProjection,
        mesh::shape,
        pipeline::{PipelineDescriptor, RenderPipeline},
        render_graph::{base, AssetRenderResourcesNode, RenderGraph},
        renderer::RenderResources,
        shader::{ShaderStages, ShaderStage},
    },
};

pub struct HealthBarPlugin;

impl Plugin for HealthBarPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_asset::<HealthBar>()
            .add_asset::<Billboard>()
            .add_startup_system(setup.system());
    }
}

#[derive(RenderResources, Default, TypeUuid)]
#[uuid = "3bf9e364-f29d-4d6c-92cf-93298466c624"]
pub struct HealthBar {
    pub colour: Color,
    pub fill: f32,
}

#[derive(RenderResources, Default, TypeUuid)]
#[uuid = "3bf9e364-f29d-4d6c-92cf-93298466c626"]
pub struct Billboard {
    pub offset: Vec3,
}

fn setup(
    mut commands: Commands,
    //mut shaders: ResMut<Assets<Shader>>,
    asset_server: Res<AssetServer>,
    mut pipelines: ResMut<Assets<PipelineDescriptor>>,
    //mut textures: ResMut<Assets<Texture>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut colour_materials: ResMut<Assets<ColorMaterial>>,
    mut billboard_materials: ResMut<Assets<Billboard>>,
    mut healthbar_materials: ResMut<Assets<HealthBar>>,
    mut render_graph: ResMut<RenderGraph>,
) {
    add_overlay_graph(&mut render_graph, &asset_server, &mut pipelines);

    //let texture_handle = asset_server.load("textures/unit_overlays/test.png");

    //let healthbar_material = healthbar_materials.add(HealthBar {
    //    colour: Color::rgba(0.0, 0.0, 1.0, 1.0),
    //    fill: 0.75,
    //});

    meshes.set_untracked(SIMPLE_QUAD_MESH_HANDLE, Mesh::from(shape::Quad {
        size: bevy::math::vec2(1.0, 1.0),
        flip: false,
    }));

    billboard_materials.set_untracked(BILLBOARD_DEFAULT_HANDLE, Billboard::default() );

    //let enemy = commands.spawn_bundle(PbrBundle {
    //    mesh: meshes.add(Mesh::from(shape::Cube { size: 20.0 })),
    //    material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
    //    transform: Transform::from_xyz(20.0, 0.0, 0.0),
    //    ..Default::default()
    //}).id();

 
    //let ship = commands.spawn_bundle(PbrBundle {
    //    mesh: meshes.add(Mesh::from(shape::Icosphere { radius: 1.0, subdivisions: 10 })),
    //    material: materials.add(Color::rgb(0.0, 0.0, 1.0).into()),
    //    transform: Transform::from_xyz(2.0, 0.0, 0.0),
    //    ..Default::default()
    //}).id();

    //let overlay = commands
    //    .spawn_bundle(MeshBundle {
    //        mesh: SIMPLE_QUAD_MESH_HANDLE.typed(),
    //        render_pipelines: RenderPipelines::from_handles(
    //            &[UNITOVERLAY_PIPELINE_HANDLE.typed()]
    //        ),
    //        visible: Visible {
    //            is_transparent: true,
    //            ..Default::default()
    //        },
    //        transform: Transform::from_scale(vec3(2.0, 2.0, 0.0)),
    //        ..Default::default()
    //    })
    //    //.insert(UNITOVERLAY_TEXTURE_HANDLE.typed::<UnitOverlay>())
    //    .insert(colour_materials.add(
    //        ColorMaterial {
    //            color: Color::rgb(0.0, 1.0, 0.0),
    //            texture: Some(texture_handle.clone())
    //        }
    //    ))
    //    .insert(billboard_materials.add(Billboard { offset: vec3(0.0, 0.0, 0.0) }))
    //    .id();

    //let healthbar = commands
    //    .spawn_bundle(MeshBundle {
    //        mesh: SIMPLE_QUAD_MESH_HANDLE.typed(),
    //        render_pipelines: RenderPipelines::from_handles(
    //            &[HEALTHBAR_PIPELINE_HANDLE.typed(), ]
    //        ),
    //        visible: Visible {
    //            is_transparent: true,
    //            ..Default::default()
    //        },
    //        transform: Transform::from_scale(vec3(2.0, 0.1, 0.0)),
    //        ..Default::default()
    //    })
    //    .insert(billboard_materials.add(Billboard { offset: vec3(0.0, 1.0, 0.0) }))
    //    .insert(healthbar_materials.add(HealthBar {
    //        colour: Color::rgba(0.0, 0.0, 1.0, 1.0),
    //        fill: 0.25,
    //    }))
    //    .id();

    //commands.entity(ship).push_children(&[healthbar, overlay]);
}

pub fn attach_ship_overlay(
    ship: Entity, 
    mut commands: Commands, 
    //mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
    mut colour_materials: ResMut<Assets<ColorMaterial>>,
    mut billboard_materials: ResMut<Assets<Billboard>>,
    mut healthbar_materials: ResMut<Assets<HealthBar>>,
) {
    let texture_handle = asset_server.load("textures/unit_overlays/test.png");

    let overlay = commands
        .spawn_bundle(MeshBundle {
            mesh: SIMPLE_QUAD_MESH_HANDLE.typed(),
            render_pipelines: RenderPipelines::from_handles(
                &[UNITOVERLAY_PIPELINE_HANDLE.typed()]
            ),
            visible: Visible {
                is_transparent: true,
                ..Default::default()
            },
            transform: Transform::from_scale(vec3(2.0, 2.0, 0.0)),
            ..Default::default()
        })
        //.insert(UNITOVERLAY_TEXTURE_HANDLE.typed::<UnitOverlay>())
        .insert(colour_materials.add(
            ColorMaterial {
                color: Color::rgb(0.0, 1.0, 0.0),
                texture: Some(texture_handle.clone())
            }
        ))
        .insert(billboard_materials.add(Billboard { offset: vec3(0.0, 0.0, 0.0) }))
        .id();

    let healthbar = commands
        .spawn_bundle(MeshBundle {
            mesh: SIMPLE_QUAD_MESH_HANDLE.typed(),
            render_pipelines: RenderPipelines::from_handles(
                &[HEALTHBAR_PIPELINE_HANDLE.typed(), ]
            ),
            visible: Visible {
                is_transparent: true,
                ..Default::default()
            },
            transform: Transform::from_scale(vec3(2.0, 0.1, 0.0)),
            ..Default::default()
        })
        .insert(billboard_materials.add(Billboard { offset: vec3(0.0, 1.0, 0.0) }))
        .insert(healthbar_materials.add(HealthBar {
            colour: Color::rgba(0.0, 0.0, 1.0, 1.0),
            fill: 0.25,
        }))
        .id();

    commands.entity(ship).push_children(&[healthbar, overlay]);
} 

pub const HEALTHBAR: &str = "HealthBar";
pub const UNITOVERLAY: &str = "UnitOverlay";
pub const BILLBOARD: &str = "Billboard";

pub const HEALTHBAR_PIPELINE_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(PipelineDescriptor::TYPE_UUID, 12976993416349439669);

pub const UNITOVERLAY_PIPELINE_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(PipelineDescriptor::TYPE_UUID, 9531356988440774023);

//pub const UNITOVERLAY_TEXTURE_HANDLE: HandleUntyped =
//    HandleUntyped::weak_from_u64(UnitOverlay::TYPE_UUID, 18019413496449622828);

pub const SIMPLE_QUAD_MESH_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(Mesh::TYPE_UUID,12515628229712380851);

pub const BILLBOARD_DEFAULT_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(Billboard::TYPE_UUID,15044190116403830730);


pub fn add_overlay_graph(
    render_graph: &mut RenderGraph,
    asset_server: &Res<AssetServer>,
    pipelines: &mut Assets<PipelineDescriptor>,
) {

    render_graph.add_system_node(
        HEALTHBAR,
        AssetRenderResourcesNode::<HealthBar>::new(true),
    );
 
    render_graph    
        .add_node_edge(HEALTHBAR, base::node::MAIN_PASS)
        .unwrap();

    render_graph.add_system_node(
        BILLBOARD,
        AssetRenderResourcesNode::<Billboard>::new(true),
    );

    render_graph    
        .add_node_edge(BILLBOARD, base::node::MAIN_PASS)
        .unwrap();

    //render_graph.add_system_node(
    //    UNITOVERLAY,
    //    AssetRenderResourcesNode::<UnitOverlay>::new(true),
    //);

    //render_graph
    //    .add_node_edge(UNITOVERLAY, base::node::MAIN_PASS)
    //    .unwrap();

    pipelines.set_untracked(HEALTHBAR_PIPELINE_HANDLE, 
        PipelineDescriptor::default_config(ShaderStages {
            vertex: asset_server.load::<Shader,_>("shaders/billboard.vert"),
            fragment: Some(asset_server.load::<Shader,_>("shaders/healthbar.frag")),
        })
    );

    pipelines.set_untracked(UNITOVERLAY_PIPELINE_HANDLE, 
        PipelineDescriptor::default_config(ShaderStages {
            vertex: asset_server.load::<Shader,_>("shaders/billboard.vert"),
            fragment: Some(asset_server.load::<Shader,_>("shaders/unitsymbol.frag")),
            //fragment: colormaterial_fragment_shader,
        })
    );
}