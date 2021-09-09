use std::f32::consts::PI;

use bevy::{
    input::keyboard::KeyboardInput,
    input::mouse::{MouseButton, MouseMotion, MouseWheel},
    prelude::*,
    render::camera::PerspectiveProjection,
};

use log::debug;
pub struct CameraControlPlugin;

#[derive(Clone, PartialEq, Eq, Hash, Debug, SystemLabel)]
pub enum CameraControlSystem {
    CameraMovement,
}

pub struct CameraController {
    _sensitivity: f32,
    radius: f32,
    focus: Vec3,
    upside_down: bool,
    _keys: Keys,
}

pub struct Keys {
    pan_left: KeyCode,
    pan_right: KeyCode,
    pan_forward: KeyCode,
    pan_backward: KeyCode,
    pan_up: KeyCode,
    pan_down: KeyCode,
    rot_left: KeyCode,
    rot_right: KeyCode,
}

impl Default for Keys {
    fn default() -> Keys {
        Keys {
            pan_left: KeyCode::A,
            pan_right: KeyCode::D,
            pan_forward: KeyCode::W,
            pan_backward: KeyCode::S,
            pan_up: KeyCode::LShift,
            pan_down: KeyCode::LControl,
            rot_left: KeyCode::Q,
            rot_right: KeyCode::E,
        }
    }
}

impl Default for CameraController {
    fn default() -> CameraController {
        CameraController {
            focus: Vec3::ZERO,
            radius: 10f32,
            _sensitivity: 10f32,
            upside_down: false,
            _keys: Default::default(),
        }
    }
}

impl Plugin for CameraControlPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(
            camera_movement
                .system()
                .label(CameraControlSystem::CameraMovement),
        )
        .insert_resource(Keys::default());
    }
}

// Tags an entity as capable of panning and orbiting.
fn get_primary_window_size(windows: &Res<Windows>) -> Vec2 {
    let window = windows.get_primary().unwrap();
    let window = Vec2::new(window.width() as f32, window.height() as f32);
    window
}

fn camera_movement(
    time: Res<Time>,
    keyboard: Res<Input<KeyCode>>,
    windows: Res<Windows>,
    mut ev_motion: EventReader<MouseMotion>,
    mut ev_scroll: EventReader<MouseWheel>,
    input_mouse: Res<Input<MouseButton>>,
    keys: Res<Keys>,
    mut q: Query<(
        &mut CameraController,
        &mut Transform,
        &PerspectiveProjection,
    )>,
) {
    // change input mapping for orbit and panning here
    let orbit_button = MouseButton::Right;
    let pan_button = MouseButton::Other(5);

    let mut pan = Vec2::ZERO;
    let mut rotation_move = Vec2::ZERO;
    let mut scroll = 0.0;
    let mut orbit_button_changed = false;

    if input_mouse.pressed(orbit_button) {
        for ev in ev_motion.iter() {
            rotation_move += ev.delta;
        }
    } else if input_mouse.pressed(pan_button) {
        // Pan only if we're not rotating at the moment
        for ev in ev_motion.iter() {
            pan += ev.delta;
        }
    }
    for ev in ev_scroll.iter() {
        scroll += ev.y;
    }
    if input_mouse.just_released(orbit_button) || input_mouse.just_pressed(orbit_button) {
        orbit_button_changed = true;
    }

    let rot_rate = PI * time.delta().as_secs_f32();

    let mut rotation = Quat::IDENTITY;
    let mut translation = Vec3::ZERO;

    if keyboard.pressed(keys.pan_left) {
        //transform.translation -= forward * Vec3::X * rate;
        translation -= Vec3::X;
    }

    if keyboard.pressed(keys.pan_right) {
        //transform.translation += forward * Vec3::X * rate;
        translation += Vec3::X;
    }

    if keyboard.pressed(keys.pan_forward) {
        //transform.translation -= forward * Vec3::Z * rate;
        translation -= Vec3::Z;
    }

    if keyboard.pressed(keys.pan_backward) {
        //transform.translation += forward * Vec3::Z * rate;
        translation += Vec3::Z;
    }

    if keyboard.pressed(keys.pan_up) {
        // transform.translation += Vec3::Y * rate;
        translation += Vec3::Y;
    }

    if keyboard.pressed(keys.pan_down) {
        // transform.translation -= Vec3::Y * rate;
        translation -= Vec3::Y;
    }

    if keyboard.pressed(keys.rot_left) {
        //transform.rotation = Quat::from_rotation_y(rot_rate) * transform.rotation;
        rotation *= Quat::from_rotation_y(rot_rate);
    }

    if keyboard.pressed(keys.rot_right) {
        //transform.rotation = Quat::from_rotation_y(-rot_rate) * transform.rotation;
        rotation *= Quat::from_rotation_y(-rot_rate);
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
            pan_orbit.radius -= scroll * pan_orbit.radius * 0.2;
            // dont allow zoom to reach zero or you get stuck
            pan_orbit.radius = f32::max(pan_orbit.radius, 0.05);
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
