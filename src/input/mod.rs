use bevy::prelude::*;
use bevy::input::mouse::{MouseMotion, MouseWheel};
use std::collections::{HashMap, HashSet};
use crate::SystemLabels;

mod keymap;
use keymap::Key;

use log::debug;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_startup_system(input_setup.system())
            .add_system(input_handling.system().label(SystemLabels::Input))
            .add_system(debug_input.system().after(SystemLabels::Input))
            .insert_resource(Inputs::default())
            .insert_resource(InputMap::default())
            .insert_resource::<Inputs2>(HashSet::new())
            .insert_resource(Input::<Action>::default())
        ;
    }
}

#[derive(PartialEq, Eq, Hash)]
enum Binding {
    Key(KeyCode),
    KeyWithModifier(KeyCode, KeyCode),
    MouseButton(MouseButton),
    MouseButtonWithModifier(MouseButton, KeyCode),
}

type KeyBindings = HashMap<KeyCode, dyn Key>;

#[derive(Default, Debug)]
struct Inputs {
    just_pressed: HashSet<Box<dyn Key>>,
    just_released: HashSet<Box<dyn Key>>,
    pressed: HashSet<Box<dyn Key>>,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum Controls {
    Select,
    Target,
}

type Inputs2 = HashSet<Box<dyn Key>>;

type Action = Box<dyn Key>;

#[derive(Default)]
struct InputMap(HashMap<Binding,Box<dyn Key>>);

fn input_setup(
    mut mapping: ResMut<InputMap>,
) {

    mapping.0.insert(Binding::Key(KeyCode::A), Box::new(Controls::Select));
    mapping.0.insert(Binding::Key(KeyCode::S), Box::new(Controls::Target));
}

trait TestTrait: std::any::Any + Send + Sync + 'static {}


fn input_handling(
    mapping: Res<InputMap>,
    mut inputs: ResMut<Inputs>,
    keys: Res<Input<KeyCode>>,
    mouse_button: Res<Input<MouseButton>>,
    mouse_motion: EventReader<MouseMotion>,
    mouse_scroll: EventReader<MouseWheel>,
    //mut something: ResMut<Input<Box<dyn Key>>>,
    mut inputs2: ResMut<Inputs2>,
    mut actions: ResMut<Input<Action>>,
) {
    *inputs = Inputs::default(); 

    let bindings = &mapping.0;

    for &key in keys.get_just_pressed() {
        if let Some(&action) = bindings.get(&Binding::Key(key)) {
            inputs.just_pressed.insert(action.as_any());
            //actions.press(Box::new(action));
        }
    }

    for &key in keys.get_pressed() {
        if let Some(action) = bindings.get(&Binding::Key(key)) {
            //inputs.pressed.insert(action);
        }
    }

    for &key in keys.get_just_released() {
        if let Some(action) = bindings.get(&Binding::Key(key)) {
            //inputs.just_released.insert(action);
        }
    }

    //something.press(Box::new(Controls::Select));
    inputs2.insert(Box::new(Controls::Select));
    inputs2.insert(Box::new(Controls::Target));
}

fn debug_input(
    inputs: Res<Inputs>,
    mut something: ResMut<Inputs2>,
) {

    debug!("{:?}", *inputs);
    debug!("{:?}", *something);
}