use crate::SystemLabels;
use bevy::prelude::*;
pub struct OrdersPlugin;

impl Plugin for OrdersPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(movement_system.system().after(SystemLabels::Input))
            .insert_resource(Option::<WorldCursor>::default());
    }
}

#[derive(Default)]
struct WorldCursor {
    pos: Vec3,
}

fn movement_system(
    mut cursor: ResMut<Option<WorldCursor>>,
    mut lines: ResMut<bevy_prototype_debug_lines::DebugLines>,
    query: Query<(
        &crate::player::camera::CameraController,
        &Option<crate::input::MouseRay>,
    )>,
) {
    *cursor = None;
    for (cam, ray) in query.iter() {
        if let Some(ray) = ray {
            let d = (-ray.direction).dot(Vec3::Y);
            if d == 0f32 {
                continue;
            }

            let t = Vec3::Y.dot(ray.near - cam.focus) / d;
            if t < 0f32 || t > 1f32 {
                continue;
            }

            let pos = ray.near + ray.direction * t;

            let _ = cursor.insert(WorldCursor { pos });

            lines.line(pos - Vec3::X - Vec3::Z, pos + Vec3::X + Vec3::Z, 1.0);
            lines.line(pos - Vec3::Z + Vec3::X, pos + Vec3::Z - Vec3::X, 1.0);
        }
    }
}
