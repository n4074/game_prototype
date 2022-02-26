use bevy::{
    core_pipeline::Transparent3d,
    ecs::system::{lifetimeless::SRes, SystemParamItem},
    pbr::MaterialPipeline,
    prelude::*,
    reflect::TypeUuid,
    render::{
        render_asset::{PrepareAssetError, RenderAsset},
        render_phase::{
            AddRenderCommand, DrawFunctions, EntityRenderCommand, RenderCommandResult, RenderPhase,
            SetItemPipeline, TrackedRenderPass,
        },
        render_resource::*,
        render_resource::{
            std140::{AsStd140, Std140},
            *,
        },
        renderer::{RenderDevice, RenderQueue},
        view::{ComputedVisibility, ExtractedView, Msaa, Visibility},
        RenderApp, RenderStage,
    },
};

pub struct OverlayPlugin;

impl Plugin for OverlayPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(MaterialPlugin::<OverlayMaterial>::default())
            .get_sub_app_mut(RenderApp)
            .unwrap()
            .add_system_to_stage(RenderStage::Extract, extract_custom);
    }
}

fn extract_custom(
    mut commands: Commands,
    mut previous_len: Local<usize>,
    mut query: Query<Entity, With<OverlayComponent>>,
) {
    log::debug!("Got here");

    let mut values = Vec::with_capacity(*previous_len);
    for entity in query.iter_mut() {
        values.push((entity, (OverlayComponent,)));
    }
    *previous_len = values.len();
    commands.insert_or_spawn_batch(values);
}

fn queue_custom(mut views: Query<(&ExtractedView, &mut RenderPhase<Transparent3d>)>) {
    log::debug!("Got here")
}

//fn main() {
//    App::new()
//        .add_plugins(DefaultPlugins)
//        .add_plugin(MaterialPlugin::<OverlayMaterial>::default())
//        .add_startup_system(setup)
//        .run();
//}
//
///// set up a simple 3D scene
//fn setup(
//    mut commands: Commands,
//    mut meshes: ResMut<Assets<Mesh>>,
//    mut materials: ResMut<Assets<OverlayMaterial>>,
//) {
//    // cube
//    commands.spawn().insert_bundle(MaterialMeshBundle {
//        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
//        transform: Transform::from_xyz(0.0, 0.5, 0.0),
//        material: materials.add(OverlayMaterial {
//            color: Color::GREEN,
//        }),
//        ..Default::default()
//    });
//
//    // camera
//    commands.spawn_bundle(PerspectiveCameraBundle {
//        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
//        ..Default::default()
//    });
//}

#[derive(Component)]
pub struct OverlayComponent;

#[derive(Debug, Clone, TypeUuid)]
#[uuid = "4ee9c363-1124-4113-890e-199d81b00281"]
pub struct OverlayMaterial {
    color: Color,
}

#[derive(Clone)]
pub struct GpuOverlayMaterial {
    _buffer: Buffer,
    bind_group: BindGroup,
}

impl RenderAsset for OverlayMaterial {
    type ExtractedAsset = OverlayMaterial;
    type PreparedAsset = GpuOverlayMaterial;
    type Param = (SRes<RenderDevice>, SRes<MaterialPipeline<Self>>);

    fn extract_asset(&self) -> Self::ExtractedAsset {
        self.clone()
    }

    fn prepare_asset(
        extracted_asset: Self::ExtractedAsset,
        (render_device, material_pipeline): &mut SystemParamItem<Self::Param>,
    ) -> Result<Self::PreparedAsset, PrepareAssetError<Self::ExtractedAsset>> {
        let color = Vec4::from_slice(&extracted_asset.color.as_linear_rgba_f32());
        let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
            contents: color.as_std140().as_bytes(),
            label: None,
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });
        let bind_group = render_device.create_bind_group(&BindGroupDescriptor {
            entries: &[BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
            label: None,
            layout: &material_pipeline.material_layout,
        });

        Ok(GpuOverlayMaterial {
            _buffer: buffer,
            bind_group,
        })
    }
}

impl Material for OverlayMaterial {
    fn fragment_shader(asset_server: &AssetServer) -> Option<Handle<Shader>> {
        Some(asset_server.load::<Shader, _>("shaders/overlay/healthbar.frag.spv"))
    }

    fn vertex_shader(asset_server: &AssetServer) -> Option<Handle<Shader>> {
        Some(asset_server.load::<Shader, _>("shaders/overlay/healthbar.vert.spv"))
    }

    fn bind_group(render_asset: &<Self as RenderAsset>::PreparedAsset) -> &BindGroup {
        &render_asset.bind_group
    }

    fn bind_group_layout(render_device: &RenderDevice) -> BindGroupLayout {
        render_device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            entries: &[BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::FRAGMENT,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: BufferSize::new(Vec4::std140_size_static() as u64),
                },
                count: None,
            }],
            label: None,
        })
    }
}
