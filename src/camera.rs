
use std::f32::consts::PI;

use bevy::prelude::*;

use log::{debug};
pub struct CameraControlPlugin;

#[derive(Clone, PartialEq, Eq, Hash, Debug, SystemLabel)]
pub enum CameraControlSystem {
    CameraMovement
}

pub struct CameraController {
    sensitivity: f32,
    keys: Keys,
}

pub struct Keys {
    pan_left: KeyCode,
    pan_right: KeyCode,
    pan_forward: KeyCode,
    pan_backward: KeyCode,
    pan_up: KeyCode,
    pan_down: KeyCode,
    rot_left: KeyCode,
    rot_right: KeyCode
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
            sensitivity: 10f32,
            keys: Default::default()
        }
    }
}

impl Plugin for CameraControlPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(
            camera_movement
                .system()
                .label(CameraControlSystem::CameraMovement),
        );
    }
}

fn camera_movement(
    time: Res<Time>,
    keyboard: Res<Input<KeyCode>>,
    mut q: Query<(&CameraController, &mut Transform)>
) {
    for (controller, mut transform) in q.iter_mut() {
        let current_transform = transform.clone();
        let rate = controller.sensitivity * time.delta().as_secs_f32();
        let rot_rate = PI * time.delta().as_secs_f32();

        let mut forward = current_transform.local_z();
        forward.y = 0f32;
        forward.normalize();

        let mut move_direction =- bevy::math::Quat::IDENTITY;

        if keyboard.pressed(controller.keys.pan_left) {
            transform.translation -= Vec3::X * rate;
        }

        if keyboard.pressed(controller.keys.pan_right) {
            transform.translation += Vec3::X * forward * rate;
            debug!("{:?}", forward * rate);
        }

        if keyboard.pressed(controller.keys.pan_forward) {
            transform.translation -= forward * rate;
        }

        if keyboard.pressed(controller.keys.pan_backward) {
            transform.translation += forward * rate;
        }

        if keyboard.pressed(controller.keys.pan_up) {
            transform.translation += Vec3::Y * rate;
        }

        if keyboard.pressed(controller.keys.pan_down) {
            transform.translation -= Vec3::Y * rate;
        }

        if keyboard.pressed(controller.keys.rot_left) {
            let mut cur = transform.clone();
            transform.rotate(Quat::from_rotation_y(rot_rate))
        }

        if keyboard.pressed(controller.keys.rot_right) {
            let mut cur = transform.clone();
            transform.rotate(Quat::from_rotation_y(-rot_rate));
        }

        if keyboard.just_pressed(KeyCode::X) {
            let mut rot_vec = -current_transform.local_z();
            rot_vec.y = 0f32;
            rot_vec.normalize();

            debug!("{:?}", f32::atan2(forward.x, forward.z));
            debug!("{:?}", forward.angle_between(Vec3::Z));

            debug!("loc: {:?}", current_transform.translation);
            debug!("{:?}", rot_vec);
        }


        
    }



}
