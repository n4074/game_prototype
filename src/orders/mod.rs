use bevy::prelude::*;
use crate::SystemLabels;
pub struct OrdersPlugin;

impl Plugin for OrdersPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_system(movement_system.system().after(SystemLabels::Input))
        ;
    }
}

fn movement_system(
    _commands: Commands,
    mut lines: ResMut<bevy_prototype_debug_lines::DebugLines>,
    input: Res<crate::input::MappedInput>,
    query: Query<(&crate::camera::CameraController, &Option<crate::input::MouseRay>)>,
) {
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

            let int = ray.near + ray.direction * t;
            lines.line(int - Vec3::X - Vec3::Z, int + Vec3::X + Vec3::Z, 1.0);
            lines.line(int - Vec3::Z + Vec3::X, int + Vec3::Z - Vec3::X, 1.0);
        }
    }
}
