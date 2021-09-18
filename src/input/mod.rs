use crate::SystemLabels;
use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy::prelude::*;
use keymap::AnyKey;
use num_derive::{FromPrimitive, ToPrimitive};
use std::fmt::Debug;

mod graph;
//mod graphmap;
mod keymap;

pub(crate) use graph::{debug_binding_graph, MappedInput};

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

//#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
//pub enum Binding {
//    Simple(Switch),
//    Modified(Switch, Switch),
//    DoubleModified(Switch, Switch, Switch),
//}

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone, Ord, PartialOrd)]
pub enum Switch {
    Key(KeyCode),
    Mouse(MouseButton_),
    MouseMotion,
    MouseScroll,
}

// bevy::input::mouse::MouseButton annoying doesn't implement Ord/PartialOrd which is required to use our graphmap
#[derive(Eq, Ord, PartialEq, PartialOrd, Copy, Clone, Hash, Debug)]
pub enum MouseButton_ {
    Left,
    Right,
    Middle,
    Other(u16),
}

impl From<MouseButton> for MouseButton_ {
    fn from(button: MouseButton) -> Self {
        match button {
            MouseButton::Left => Self::Left,
            MouseButton::Right => Self::Right,
            MouseButton::Middle => Self::Middle,
            MouseButton::Other(u16) => Self::Other(u16),
        }
    }
}

//impl From<Switch> for Binding {
//    fn from(switch: Switch) -> Binding {
//        Binding::Simple(switch)
//    }
//}

impl From<MouseButton> for Switch {
    fn from(button: MouseButton) -> Self {
        Switch::Mouse(button.into())
    }
}

impl From<KeyCode> for Switch {
    fn from(button: KeyCode) -> Self {
        Switch::Key(button)
    }
}

//impl From<MouseButton> for Binding {
//    fn from(button: MouseButton) -> Binding {
//        Binding::Simple(button.into())
//    }
//}
//
//impl From<KeyCode> for Binding {
//    fn from(keycode: KeyCode) -> Binding {
//        Binding::Simple(keycode.into())
//    }
//}
//
//impl<T, Q> From<(T, Q)> for Binding
//where
//    T: Into<Switch>,
//    Q: Into<Switch>,
//{
//    fn from(keys: (T, Q)) -> Binding {
//        Binding::Modified(keys.0.into(), keys.1.into())
//    }
//}
//
//impl<T, Q, R> From<(T, Q, R)> for Binding
//where
//    T: Into<Switch>,
//    Q: Into<Switch>,
//    R: Into<Switch>,
//{
//    fn from(keys: (T, Q, R)) -> Binding {
//        Binding::DoubleModified(keys.0.into(), keys.1.into(), keys.2.into())
//    }
//}

pub trait Action: Send + Sync + std::fmt::Debug {}

impl Action for SomeKeyBindings {}

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
