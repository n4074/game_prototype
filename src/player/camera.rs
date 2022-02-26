use std::f32::consts::PI;

use bevy::{
    input::mouse::MouseButton, prelude::*, render::camera::Camera,
    render::camera::PerspectiveProjection,
};

use crate::SystemLabels;
//use log::debug;
pub struct CameraControlPlugin;

impl Plugin for CameraControlPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup.system())
            .add_system(
                camera_movement
                    .system()
                    .label(SystemLabels::Camera)
                    .after(SystemLabels::Input),
            )
            .add_system(mouseray_system.system().after(SystemLabels::Input))
            .register_type::<CameraController>();
    }
}
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct CameraController {
    _sensitivity: f32,
    pub radius: f32,
    pub focus: Vec3,
    upside_down: bool,
}

impl Default for CameraController {
    fn default() -> CameraController {
        CameraController {
            focus: Vec3::ZERO,
            radius: 40f32,
            _sensitivity: 10f32,
            upside_down: false,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, num_derive::ToPrimitive)]
pub enum Pan {
    Left,
    Right,
    Forward,
    Backward,
    Up,
    Down,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, num_derive::ToPrimitive)]
pub enum Controls {
    Orbit,
    Pan,
    Zoom,
}

// Tags an entity as capable of panning and orbiting.
fn get_primary_window_size(windows: &Res<Windows>) -> Vec2 {
    let window = windows.get_primary().unwrap();
    let window = Vec2::new(window.width() as f32, window.height() as f32);
    window
}

fn setup(mut commands: Commands, mut inputmap: ResMut<crate::input::MappedInput>) {
    // spawn player camera
    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_xyz(0.0, 2.5, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        })
        .insert(CameraController::default())
        .insert(MouseRayComponent::default())
        .insert(ControlCursor::default());

    inputmap.bind([KeyCode::A], Pan::Left);
    inputmap.bind([KeyCode::D], Pan::Right);
    inputmap.bind([KeyCode::W], Pan::Forward);
    inputmap.bind([KeyCode::S], Pan::Backward);
    inputmap.bind([KeyCode::LShift], Pan::Up);
    inputmap.bind([KeyCode::LControl], Pan::Down);

    inputmap.bind(
        [
            crate::input::Switch::from(MouseButton::Right),
            crate::input::Switch::MouseMotion,
        ],
        Controls::Orbit,
    );
    inputmap.bind(
        [
            crate::input::Switch::from(MouseButton::Middle),
            crate::input::Switch::MouseMotion,
        ],
        Controls::Pan,
    );

    inputmap.bind([crate::input::Switch::MouseScroll], Controls::Zoom);
}

fn camera_movement(
    time: Res<Time>,
    windows: Res<Windows>,
    input: Res<crate::input::MappedInput>,
    mut q: Query<(
        &mut CameraController,
        &mut Transform,
        &PerspectiveProjection,
    )>,
) {
    // change input mapping for orbit and panning here
    let mut pan = Vec2::ZERO;
    let mut rotation_move = Vec2::ZERO;
    let mut orbit_button_changed = false;

    if let Some(motion) = input.motion(Controls::Orbit) {
        rotation_move += motion;
    } else if let Some(motion) = input.motion(Controls::Pan) {
        pan += motion;
    }

    if let Some(motion) = input.motion(Controls::Orbit) {
        rotation_move += motion;
    }

    if let Some(motion) = input.motion(Controls::Pan) {
        pan += motion;
    }

    let mut pan = input.motion(Controls::Pan).unwrap_or(Vec2::ZERO);
    let scroll = input.scroll(Controls::Zoom).unwrap_or(0.0);

    if input.just_deactivated(Controls::Orbit) || input.just_activated(Controls::Orbit) {
        orbit_button_changed = true;
    }

    let _rot_rate = PI * time.delta().as_secs_f32();

    let mut translation = Vec3::ZERO;

    if input.active(Pan::Left) {
        translation -= Vec3::X;
    }

    if input.active(Pan::Right) {
        translation += Vec3::X;
    }

    if input.active(Pan::Forward) {
        translation -= Vec3::Z;
    }

    if input.active(Pan::Backward) {
        translation += Vec3::Z;
    }

    if input.active(Pan::Up) {
        translation += Vec3::Y;
    }

    if input.active(Pan::Down) {
        translation -= Vec3::Y;
    }

    for (mut pan_orbit, mut transform, projection) in q.iter_mut() {
        if orbit_button_changed {
            // only check for upside down when orbiting started or ended this frame
            // if the camera is "upside" down, panning horizontally would be inverted, so invert the input to make it correct
            let up = transform.rotation * Vec3::Y;
            pan_orbit.upside_down = up.y <= 0.0;
        }

        let mut any = false;
        if rotation_move.length_squared() > 0.0 {
            any = true;
            let window = get_primary_window_size(&windows);
            let delta_x = {
                let delta = rotation_move.x / window.x * std::f32::consts::PI * 2.0;
                if pan_orbit.upside_down {
                    -delta
                } else {
                    delta
                }
            };
            let delta_y = rotation_move.y / window.y * std::f32::consts::PI;
            let yaw = Quat::from_rotation_y(-delta_x);
            let pitch = Quat::from_rotation_x(-delta_y);
            transform.rotation = yaw * transform.rotation; // rotate around global y axis
            transform.rotation = transform.rotation * pitch; // rotate around local x axis
        } else if pan.length_squared() > 0.0 {
            any = true;
            // make panning distance independent of resolution and FOV,
            let window = get_primary_window_size(&windows);
            pan *= Vec2::new(projection.fov * projection.aspect_ratio, projection.fov) / window;
            // translate by local axes
            let right = transform.rotation * Vec3::X * -pan.x;
            let up = transform.rotation * Vec3::Y * pan.y;
            // make panning proportional to distance away from focus point
            let translation = (right + up) * pan_orbit.radius;
            pan_orbit.focus += translation;
        } else if scroll.abs() > 0.0 {
            any = true;
            pan_orbit.radius -= scroll * pan_orbit.radius * 0.02;
            // dont allow zoom to reach zero or you get stuck
            pan_orbit.radius = f32::clamp(pan_orbit.radius, 10.00, 500.00);
        } else if translation.length_squared() > 0.0 {
            any = true;

            let local_z = transform.local_z();
            let theta = f32::atan2(local_z.x, local_z.z);
            let world_z = Quat::from_rotation_y(theta);

            //let right = transform.rotation * Vec3::X * translation.x;
            //let forward = transform.rotation * Vec3::Z * translation.z;
            pan_orbit.focus += world_z * translation; //(right + forward) + pan_orbit.focus;
        }

        if any {
            // emulating parent/child to make the yaw/y-axis rotation behave like a turntable
            // parent = x and y rotation
            // child = z-offset
            let rot_matrix = Mat3::from_quat(transform.rotation);
            transform.translation =
                pan_orbit.focus + rot_matrix.mul_vec3(Vec3::new(0.0, 0.0, pan_orbit.radius));
        }
    }
}

/// A structure which represents the players mouse position
/// within the game world, both as a ray from the near to
/// far fields, and as a point representing the intersection of that
/// ray with the control plane
#[derive(Component, Default)]
pub struct ControlCursor {
    pub pos: Option<Vec3>,
}

#[derive(Debug, Default, Copy, Clone)]
pub struct MouseRay {
    pub near: Vec3,
    pub far: Vec3,
    pub direction: Vec3,
}
#[derive(Component, Debug, Default, Copy, Clone)]
pub struct MouseRayComponent(pub Option<MouseRay>);

fn mouseray_system(
    mut commands: Commands,
    windows: Res<Windows>,
    mut query: Query<(
        Entity,
        &Camera,
        &GlobalTransform,
        &CameraController,
        &mut MouseRayComponent,
        &mut ControlCursor,
    )>,
    mut lines: ResMut<bevy_prototype_debug_lines::DebugLines>,
) {
    for (entity, camera, camera_transform, controller, mut mouseray, mut cursor) in query.iter_mut()
    {
        let window = windows.get(camera.window);
        let cursor_position = window.and_then(|w| w.cursor_position());

        if let (Some(window), Some(cursor_position)) = (window, cursor_position) {
            let camera_position = camera_transform.compute_matrix();

            let screen_size = Vec2::from([window.width() as f32, window.height() as f32]);
            let projection_matrix = camera.projection_matrix;

            // Normalized device coordinate cursor position from (-1, -1, -1) to (1, 1, 1)
            let cursor_ndc = (cursor_position / screen_size) * 2.0 - Vec2::from([1.0, 1.0]);
            let cursor_pos_ndc_near: Vec3 = cursor_ndc.extend(-1.0);
            let cursor_pos_ndc_far: Vec3 = cursor_ndc.extend(1.0);

            // Use near and far ndc points to generate a ray in world space
            // This method is more robust than using the location of the camera as the start of
            // the ray, because ortho cameras have a focal point at infinity!
            let ndc_to_world: Mat4 = camera_position * projection_matrix.inverse();
            let near: Vec3 = ndc_to_world.project_point3(cursor_pos_ndc_near);
            let far: Vec3 = ndc_to_world.project_point3(cursor_pos_ndc_far);
            let direction = far - near;

            let ray = MouseRay {
                near,
                far,
                direction,
            };

            //let _ = mouseray.insert(ray);
            mouseray.0 = Some(ray);

            let d = (-ray.direction).dot(Vec3::Y);

            if d == 0f32 {
                continue;
            }

            let t = Vec3::Y.dot(ray.near - controller.focus) / d;
            if t < 0f32 || t > 1f32 {
                continue;
            }

            let pos = ray.near + ray.direction * t;

            cursor.pos = Some(pos);

            lines.line(pos - Vec3::X - Vec3::Z, pos + Vec3::X + Vec3::Z, 1.0);
            lines.line(pos - Vec3::Z + Vec3::X, pos + Vec3::Z - Vec3::X, 1.0);
        }
    }
}
