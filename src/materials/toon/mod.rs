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
        app.add_asset::<Toon>()
            .add_startup_system(setup.system())
            .add_state(AppState::CreateWindow)
            .add_system_set(
                SystemSet::on_update(AppState::CreateWindow).with_system(setup_window.system()),
            )
            .add_system_set(
                SystemSet::on_update(AppState::Setup).with_system(setup_pipeline.system()),
            );
    }
}

// NOTE: this "state based" approach to multiple windows is a short term workaround.
// Future Bevy releases shouldn't require such a strict order of operations.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum AppState {
    CreateWindow,
    Setup,
    Done,
}

#[derive(RenderResources, Default, TypeUuid)]
#[uuid = "3bf9e364-f29d-4d6c-92cf-932983663333"]
pub struct Toon;

pub const TOON_PIPELINE_HANDLE: HandleUntyped = HandleUntyped::weak_from_u64(
    PipelineDescriptor::TYPE_UUID,
    const_random::const_random!(u64),
);

const TOON_VERTEX_SHADER_PATH: &str = "shaders/toon/toon.vert";
const TOON_FRAGMENT_SHADER_PATH: &str = "shaders/toon/toon.frag";

const OUTLINE_PASS: &str = "OutlinePass";
const NORMALS_TEXTURE: &str = "NormalsTexture";
const NORMALS_SLOT: &str = "NormalsSlot";

pub struct NormPass;

pub fn setup(
    mut render_graph: ResMut<RenderGraph>,
    asset_server: Res<AssetServer>,
    mut pipelines: ResMut<Assets<PipelineDescriptor>>,
    windows: Res<Windows>,
    msaa: Res<Msaa>,
) {
    asset_server.watch_for_changes().unwrap();
    const TOON_NODE: &'static str = "Toon";

    let vert_shader = asset_server.load::<Shader, _>(TOON_VERTEX_SHADER_PATH);
    let frag_shader = asset_server.load::<Shader, _>(TOON_FRAGMENT_SHADER_PATH);

    render_graph.add_system_node(TOON_NODE, AssetRenderResourcesNode::<Toon>::new(true));

    render_graph
        .add_node_edge(TOON_NODE, base::node::MAIN_PASS)
        .unwrap();

    let descriptor = PipelineDescriptor::default_config(ShaderStages {
        vertex: vert_shader,
        fragment: Some(frag_shader),
    });

    pipelines.set_untracked(TOON_PIPELINE_HANDLE, descriptor);

    // second pass for outlines

    //render_graph.add_node(
    //    "second_window_depth_texture",
    //    WindowTextureNode::new(
    //        windows
    //            .get_primary()
    //            .expect("Failed to find primary window ID")
    //            .id(),
    //        TextureDescriptor {
    //            format: TextureFormat::Depth32Float,
    //            usage: TextureUsage::OUTPUT_ATTACHMENT,
    //            sample_count: msaa.samples,
    //            ..Default::default()
    //        },
    //    ),
    //);

    //let mut outline_pass = PassNode::<&base::MainPass>::new(PassDescriptor {
    //    color_attachments: vec![msaa.color_attachment_descriptor(
    //        TextureAttachment::Input("color_attachment".to_string()),
    //        TextureAttachment::Input("color_resolve_target".to_string()),
    //        Operations {
    //            load: LoadOp::Clear(Color::rgb(0.5, 0.5, 0.8)),
    //            store: true,
    //        },
    //    )],
    //    depth_stencil_attachment: Some(RenderPassDepthStencilAttachmentDescriptor {
    //        attachment: TextureAttachment::Input("depth".to_string()),
    //        depth_ops: Some(Operations {
    //            load: LoadOp::Clear(1.0),
    //            store: true,
    //        }),
    //        stencil_ops: None,
    //    }),
    //    sample_count: msaa.samples,
    //});

    //render_graph.add_node(OUTLINE_PASS, outline_pass);

    //render_graph
    //    .add_slot_edge(
    //        "second_window_swap_chain",
    //        WindowSwapChainNode::OUT_TEXTURE,
    //        "second_window_pass",
    //        if msaa.samples > 1 {
    //            "color_resolve_target"
    //        } else {
    //            "color_attachment"
    //        },
    //    )
    //    .unwrap();

    //render_graph
    //    .add_slot_edge(
    //        "second_window_depth_texture",
    //        WindowTextureNode::OUT_TEXTURE,
    //        OUTLINE_PASS,
    //        "depth",
    //    )
    //    .unwrap();

    //render_graph
    //    .add_node_edge("secondary_camera", OUTLINE_PASS)
    //    .unwrap();
}

fn setup_window(
    mut app_state: ResMut<State<AppState>>,
    mut create_window_events: EventWriter<CreateWindow>,
) {
    let window_id = WindowId::new();

    // sends out a "CreateWindow" event, which will be received by the windowing backend
    create_window_events.send(CreateWindow {
        id: window_id,
        descriptor: WindowDescriptor {
            width: 800.,
            height: 600.,
            vsync: false,
            title: "second window".to_string(),
            ..Default::default()
        },
    });

    app_state.set(AppState::Setup).unwrap();
}

fn setup_pipeline(
    mut commands: Commands,
    windows: Res<Windows>,
    mut active_cameras: ResMut<ActiveCameras>,
    mut render_graph: ResMut<RenderGraph>,
    asset_server: Res<AssetServer>,
    msaa: Res<Msaa>,
    mut app_state: ResMut<State<AppState>>,
) {
    // get the non-default window id
    let window_id = windows
        .iter()
        .find(|w| w.id() != WindowId::default())
        .map(|w| w.id());

    let window_id = match window_id {
        Some(window_id) => window_id,
        None => return,
    };

    // here we setup our render graph to draw our second camera to the new window's swap chain

    // add a swapchain node for our new window
    //render_graph.add_node(
    //    "second_window_swap_chain",
    //    WindowSwapChainNode::new(window_id),
    //);

    // add a new depth texture node for our new window
    //render_graph.add_node(
    //    "second_window_depth_texture",
    //    WindowTextureNode::new(
    //        window_id,
    //        TextureDescriptor {
    //            format: TextureFormat::Depth32Float,
    //            usage: TextureUsage::OUTPUT_ATTACHMENT,
    //            sample_count: msaa.samples,
    //            ..Default::default()
    //        },
    //    ),
    //);

    // add a new camera node for our new window
    //render_graph.add_system_node("secondary_camera", CameraNode::new("Secondary"));

    // add a new render pass for our new window / camera
    let mut second_window_pass = PassNode::<&MainPass>::new(PassDescriptor {
        color_attachments: vec![
            RenderPassColorAttachmentDescriptor {
                attachment: TextureAttachment::Name(NORMALS_TEXTURE.to_string()),
                resolve_target: None,
                ops: Operations {
                    load: LoadOp::Clear(Color::rgb(0.0, 0.0, 1.0)),
                    store: true,
                },
            },
            msaa.color_attachment_descriptor(
                TextureAttachment::Input("color_attachment".to_string()),
                TextureAttachment::Input("color_resolve_target".to_string()),
                Operations {
                    load: LoadOp::Clear(Color::rgb(0.5, 0.5, 0.8)),
                    store: true,
                },
            ),
        ],
        depth_stencil_attachment: Some(RenderPassDepthStencilAttachmentDescriptor {
            attachment: TextureAttachment::Input("depth2".to_string()),
            depth_ops: Some(Operations {
                load: LoadOp::Clear(1.0),
                store: true,
            }),
            stencil_ops: None,
        }),
        sample_count: msaa.samples,
    });

    //second_window_pass.add_camera("Secondary");
    //active_cameras.add("Secondary");

    render_graph.add_node(OUTLINE_PASS, second_window_pass);

    render_graph
        .add_slot_edge(
            "second_window_swap_chain",
            WindowSwapChainNode::OUT_TEXTURE,
            OUTLINE_PASS,
            if msaa.samples > 1 {
                "color_resolve_target"
            } else {
                "color_attachment"
            },
        )
        .unwrap();

    //render_graph
    //    .add_slot_edge(
    //        base::node::MAIN_PASS,
    //        WindowTextureNode::OUT_TEXTURE,
    //        OUTLINE_PASS,
    //        NORMALS_SLOT,
    //    )
    //    .expect("Failed to add slot edge");

    render_graph
        .add_slot_edge(
            "second_window_depth_texture",
            WindowTextureNode::OUT_TEXTURE,
            OUTLINE_PASS,
            "depth2",
        )
        .unwrap();

    render_graph
        .add_node_edge("secondary_camera", OUTLINE_PASS)
        .unwrap();

    if msaa.samples > 1 {
        render_graph.add_node(
            "second_multi_sampled_color_attachment",
            WindowTextureNode::new(
                window_id,
                TextureDescriptor {
                    size: Extent3d {
                        depth: 1,
                        width: 1,
                        height: 1,
                    },
                    mip_level_count: 1,
                    sample_count: msaa.samples,
                    dimension: TextureDimension::D2,
                    format: TextureFormat::default(),
                    usage: TextureUsage::OUTPUT_ATTACHMENT,
                },
            ),
        );

        render_graph
            .add_slot_edge(
                "second_multi_sampled_color_attachment",
                WindowSwapChainNode::OUT_TEXTURE,
                OUTLINE_PASS,
                "color_attachment",
            )
            .unwrap();
    }

    // second window camera
    commands.spawn_bundle(PerspectiveCameraBundle {
        camera: Camera {
            name: Some("Secondary".to_string()),
            window: window_id,
            ..Default::default()
        },
        transform: Transform::from_xyz(6.0, 0.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });

    app_state.set(AppState::Done).unwrap();
}
