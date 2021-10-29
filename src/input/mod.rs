use crate::SystemLabels;
use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy::prelude::*;
use bevy::render::camera::Camera;
use bevy_rapier3d::rapier::parry::query::PersistentQueryDispatcher;
use std::fmt;
//use keymap::AnyKey;
use num_derive::{FromPrimitive, ToPrimitive};
//use num_traits::ToPrimitive;
//use std::any::Any;
use std::fmt::Debug;
//use std::marker::PhantomData;

mod inputmap;

pub(crate) use inputmap::{debug_binding_graph, MappedInput};

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            //.add_startup_system(input_setup.system())
            .add_system(input_handling.system().label(SystemLabels::Input))
            .add_system(mouseray_system.system())
            .add_system(debug_input.system().after(SystemLabels::Input))
            .add_startup_system(setup_debug_input.system())
            .insert_resource(MappedInput::default());
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
pub enum Switch {
    Key(KeyCode),
    Mouse(MouseButton),
    MouseMotion,
    MouseScroll,
}

impl fmt::Display for Switch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Switch::Key(k) => write!(f, "{:?}", k),
            Switch::Mouse(k) => write!(f, "Mouse{:?}", k),
            s => write!(f, "{:?}", s), //Switch::Mouse(b) => write!(f, "{:?}", b),
        }
    }
}

impl From<MouseButton> for Switch {
    fn from(button: MouseButton) -> Self {
        Switch::Mouse(button)
    }
}

impl From<KeyCode> for Switch {
    fn from(button: KeyCode) -> Self {
        Switch::Key(button)
    }
}

#[derive(Debug, Default)]
pub struct MouseRay {
    ray: Option<bevy_mod_raycast::Ray3d>,
}

fn mouseray_system(
    windows: Res<Windows>,
    mut query: Query<(&Camera, &GlobalTransform, &mut MouseRay)>,
) {
    for (camera, camera_transform, mut mouseray) in query.iter_mut() {
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
            let cursor_pos_near: Vec3 = ndc_to_world.project_point3(cursor_pos_ndc_near);
            let cursor_pos_far: Vec3 = ndc_to_world.project_point3(cursor_pos_ndc_far);
            let ray_direction = cursor_pos_far - cursor_pos_near;
            mouseray.ray = Some(bevy_mod_raycast::Ray3d::new(cursor_pos_near, ray_direction));
            debug!("{:?}", mouseray);
        }
    }
}

fn input_handling(
    mut inputs: ResMut<MappedInput>,
    mut keyboard_input: EventReader<bevy::input::keyboard::KeyboardInput>,
    mut mouse_button: EventReader<bevy::input::mouse::MouseButtonInput>,
    mut mouse_motion: EventReader<MouseMotion>,
    mut mouse_scroll: EventReader<MouseWheel>,
) {
    inputs.update();

    for event in keyboard_input.iter() {
        if let &bevy::input::keyboard::KeyboardInput {
            key_code: Some(key_code),
            state,
            ..
        } = event
        {
            match state {
                bevy::input::ElementState::Pressed => inputs.press(key_code.into()),
                bevy::input::ElementState::Released => inputs.release(key_code.into()),
            }
        }
    }

    for event in mouse_button.iter() {
        match event.state {
            bevy::input::ElementState::Pressed => inputs.press(event.button.into()),
            bevy::input::ElementState::Released => inputs.release(event.button.into()),
        }
    }

    for event in mouse_motion.iter() {
        inputs.move_mouse(event.delta);
    }

    for scroll in mouse_scroll.iter() {
        inputs.scroll_mouse(scroll.y);
    }

    //for window in windows.iter() {
    //    inputs.set_cursor_position(window.id(), window.cursor_position())
    //}
}

#[derive(Hash, PartialEq, Eq, Debug, ToPrimitive, FromPrimitive, Clone, Copy)]
enum SomeKeyBindings {
    SomeAction,
    SomeOtherAction,
    SomeModifiedAction,
    SomeOtherModifiedAction,
    SomeDoubleModifiedAction,
}

#[derive(Hash, PartialEq, Eq, Debug, ToPrimitive, FromPrimitive, Clone, Copy)]
enum SomeOtherKeyBindings {
    SomeAction,
    SomeOtherAction,
}

fn setup_debug_input(mut inputs: ResMut<MappedInput>) {
    inputs.bind(
        [KeyCode::LAlt, KeyCode::A],
        SomeKeyBindings::SomeModifiedAction,
    );
    inputs.bind(
        [KeyCode::LAlt.into(), Switch::from(MouseButton::Left)],
        SomeKeyBindings::SomeModifiedAction,
    );

    inputs.bind(
        [KeyCode::RAlt, KeyCode::RControl, KeyCode::A],
        SomeKeyBindings::SomeDoubleModifiedAction,
    );

    inputs.bind(
        [KeyCode::RAlt, KeyCode::A],
        SomeKeyBindings::SomeModifiedAction,
    );

    inputs.bind(
        [KeyCode::RControl, KeyCode::A],
        SomeKeyBindings::SomeOtherModifiedAction,
    );

    inputs.bind([MouseButton::Left], SomeKeyBindings::SomeAction);
}

fn debug_input(inputs: ResMut<MappedInput>) {
    debug_binding_graph(&inputs);
}
