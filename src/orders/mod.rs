use bevy::prelude::*;
use crate::SystemLabels;
pub struct OrdersPlugin;

impl Plugin for OrdersPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system())
            .add_system(movement_system.system().after(SystemLabels::Input))
            .insert_resource(NavPlane::default());
    }
}

#[derive(num_derive::ToPrimitive, Debug)]
pub enum Controls {
    Raise,
    Lower,
}

#[derive(Debug, Copy, Clone)]
struct NavPlane {
    /// Any point on the plane
    p: Vec3,
    /// Normal of plane
    norm: Vec3,
}

impl Default for NavPlane {
    fn default() -> Self {
        NavPlane {
            p: Vec3::ZERO,
            norm: Vec3::Y
        }
    }
}

fn setup(mut input_map: ResMut<crate::input::MappedInput>) {
    input_map.bind([KeyCode::Comma], Controls::Raise);
    input_map.bind([KeyCode::Period], Controls::Lower);
}

fn movement_system(
    _commands: Commands,
    mut nav_plane: Local<NavPlane>,
    mut lines: ResMut<bevy_prototype_debug_lines::DebugLines>,
    input: Res<crate::input::MappedInput>,
    query: Query<(&Option<crate::input::MouseRay>)>,
) {
    if input.active(Controls::Raise) {
        nav_plane.p = nav_plane.p + nav_plane.norm;
        log::debug!("nav_plane: {:?}", nav_plane.p);
    }

    if input.active(Controls::Lower) {
        nav_plane.p = nav_plane.p - nav_plane.norm;
    }

    for ray in query.iter() {
        if let Some(ray) = ray {
            let d = (-ray.direction).dot(nav_plane.norm);
            if d == 0f32 {
                continue;
            }

            let t = Vec3::Y.dot(ray.near - nav_plane.p) / d;
            if t < 0f32 || t > 1f32 {
                continue;
            }

            let int = ray.near + ray.direction * t;
            lines.line(int - Vec3::X - Vec3::Z, int + Vec3::X + Vec3::Z, 1.0);
            lines.line(int - Vec3::Z + Vec3::X, int + Vec3::Z - Vec3::X, 1.0);
        }
    }
}
