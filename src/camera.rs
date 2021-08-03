
use bevy::prelude::*;
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
            sensitivity: 0.1,
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
        //println!("{:?}", transform);
        if keyboard.pressed(controller.keys.pan_left) {
            transform.translation -= current_transform.rotation * Vec3::X * controller.sensitivity
        }

        if keyboard.pressed(controller.keys.pan_right) {
            transform.translation += current_transform.rotation * Vec3::X * controller.sensitivity
        }

        if keyboard.pressed(controller.keys.pan_forward) {
            transform.translation += current_transform.rotation * Vec3::Z * controller.sensitivity
        }

        if keyboard.pressed(controller.keys.pan_backward) {
            transform.translation -= current_transform.rotation * Vec3::Z * controller.sensitivity
        }

        if keyboard.pressed(controller.keys.pan_up) {
            transform.translation += current_transform.rotation * Vec3::Y * controller.sensitivity
        }

        if keyboard.pressed(controller.keys.pan_down) {
            transform.translation -= current_transform.rotation * Vec3::Y * controller.sensitivity
        }

        if keyboard.just_pressed(KeyCode::D) {
            println!("{:?}:{:?}", "just_pressed", time.delta());
        }
    }



}
