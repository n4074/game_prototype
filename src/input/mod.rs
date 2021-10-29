use crate::SystemLabels;
use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy::prelude::*;
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

//
fn mouseray() {}

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
