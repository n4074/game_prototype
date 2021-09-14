
use bevy::prelude::*;
use bevy::input::mouse::{MouseMotion, MouseWheel};
use std::any::TypeId;
use std::borrow::Borrow;
use std::collections::{HashMap, HashSet};
use crate::SystemLabels;
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{ToPrimitive};
use keymap::{AnyKey, AsAnyKey};
use std::fmt::Debug;

use std::any::Any;

mod keymap;

use log::debug;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            //.add_startup_system(input_setup.system())
            .add_system(input_handling.system().label(SystemLabels::Input))
            .add_system(debug_input.system().after(SystemLabels::Input))
            .add_event::<KeyEvent<{ KeyCode::A }>>()
            .insert_resource(MappedInput::default())
        ;
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
pub enum Binding {
    Simple(Switch),
    Modified(Switch, Switch),
}

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
pub enum Switch {
    Key(KeyCode),
    Mouse(MouseButton),
    MouseMotion,
}

impl From<Switch> for Binding {
    fn from(switch: Switch) -> Binding {
        Binding::Simple(switch)
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

impl From<MouseButton> for Binding {
    fn from(keycode: MouseButton) -> Binding {
        Binding::Simple(Switch::Mouse(keycode))
    }
}

impl From<KeyCode> for Binding {
    fn from(keycode: KeyCode) -> Binding {
        Binding::Simple(keycode.into())
    }
}

impl<T: Into<Switch>, Q: Into<Switch>> From<(T, Q)> for Binding {
    fn from(keys: (T, Q)) -> Binding {
        Binding::Modified(keys.0.into(), keys.1.into())
    }
}



#[derive(Eq, PartialEq, Debug)]
enum TestEnum {
    A,
    B,
}

#[derive(Eq, PartialEq, Debug)]
struct KeyEvent<const T: KeyCode>(KeyCode);

pub trait Action: Send + Sync + std::fmt::Debug {}

impl Action for SomeKeyBindings {}

#[derive(Debug, Default)]
pub struct MappedInput {
    boxed_types: HashMap<AnyKey, Box<dyn Action>>,
    bindings: HashMap<Binding, AnyKey>,
    //bindings_: HashMap<AnyKey, Binding>,
    bindings_interested: HashMap<Switch, Binding>,
    modifiers: HashSet<Switch>,
    modifier: Option<Switch>,
    pressed: HashSet<AnyKey>,
    just_pressed: HashSet<AnyKey>,
    just_released: HashSet<AnyKey>,
    pressed_: HashSet<Switch>,
    just_pressed_: HashSet<Switch>,
    just_released_: HashSet<Switch>,
    moving: HashSet<AnyKey>,
    mouse_motion: Vec2,
}

impl MappedInput {

    fn update(&mut self) {
        self.just_pressed.clear();
        self.just_released.clear();
        self.moving.clear();
        self.just_pressed_.clear();
        self.just_released_.clear();
        self.mouse_motion = Vec2::ZERO;
    }

    fn is_active(&self, binding: &Binding) -> bool {
        match binding {
            Binding::Simple(switch) => { self.pressed_.contains(switch) }
            Binding::Modified(modifier, switch) =>  { self.pressed_.contains(switch) && self.pressed_.contains(modifier) }
        }
    }

    fn resolve(&mut self) {
        for switch in self.just_pressed_.iter().chain(self.just_released_.iter()) {
            if let Some(binding) = self.bindings_interested.get(switch) {
                if self.is_active(binding) {
                    let key = self.bindings.get(&binding);
                    // todo shouldn't need to clone here
                    self.pressed.insert(key.unwrap().clone());
                    self.just_pressed.insert(key.unwrap().clone());
                } else {
                    let key = self.bindings.get(&binding);
                    // todo shouldn't need to clone here
                    self.pressed.remove(key.unwrap());
                    self.just_released.insert(key.unwrap().clone());
                }
            }
        }
    }

    pub fn bind<T>(&mut self, key: impl Into<Binding>, action: T) 
        where T: Into<AnyKey> + Action + 'static + Copy + Clone {
        let binding = key.into();
        if let Binding::Modified(modifier, _) = binding {
            self.modifiers.insert(modifier);
        }

        self.boxed_types.insert(action.into(), Box::new(action));
        self.bindings.insert(binding, action.into());
    }

    fn binding(&mut self, key: Switch) -> Option<&AnyKey> {
        
        let binding = if let Some(modifier) = self.modifier {
            Binding::Modified(modifier, key)
        } else {
            Binding::Simple(key)
        };

        self.bindings.get(&binding)
    }

    fn press(&mut self, key: Switch) -> bool {

        self.pressed_.insert(key);
        self.just_pressed_.insert(key);


        if self.modifiers.contains(&key) {
            self.modifier = Some(key);
        }

        if let Some(&binding) = self.binding(key) {
            self.pressed.insert(binding);
            self.just_pressed.insert(binding);
            true
        } else {
            false
        }
    }

    pub fn pressed<T>(&self, key: T) -> bool 
        where T: Into<AnyKey>
    {
        self.pressed.get(&key.into()).is_some()
    }

    pub fn moving(&mut self, motion: Vec2) {
        if let Some(&binding) = self.binding(Switch::MouseMotion) {
            self.moving.insert(binding);
            self.mouse_motion += motion;

        }
    }

    pub fn motion<T>(&self, key: T) -> Option<Vec2> where T: Into<AnyKey> {
        if self.moving.contains(&key.into()) {
            Some(self.mouse_motion)
        } else {
            None
        }
    }

    pub fn just_pressed<T>(&self, key: T) -> bool 
        where T: Into<AnyKey>
    {
        self.just_pressed.get(&key.into()).is_some()
    }

    pub fn just_released<T>(&self, key: T) -> bool 
        where T: Into<AnyKey>
    {
        self.just_released.get(&key.into()).is_some()
    }

    fn release(&mut self, key: Switch) {

        self.pressed_.remove(&key);
        self.just_released_.insert(key);

        if self.modifier == Some(key) {
            self.modifier = None;
        }

        if let Some(&binding) = self.binding(key) {
            self.pressed.remove(&binding);
            self.just_released.insert(binding);
        }
    }

    fn get_pressed(&self) -> Vec<&Box<dyn Action>> {
        let mut res = vec!();
        for item in self.pressed.iter() {
            if let Some(obj) = self.boxed_types.get(item) {
                res.push(obj);
            };
        }
        res
    }

    fn get_just_pressed(&self) -> Vec<&Box<dyn Action>> {
        let mut res = vec!();
        for item in self.just_pressed.iter() {
            if let Some(obj) = self.boxed_types.get(item) {
                res.push(obj);
            };
        }
        res
    }

    fn get_just_released(&self) -> Vec<&Box<dyn Action>> {
        let mut res = vec!();
        for item in self.just_released.iter() {
            if let Some(obj) = self.boxed_types.get(item) {
                res.push(obj);
            };
        }
        res
    }
}


fn input_handling(
    mut inputs: ResMut<MappedInput>,
    keys: Res<Input<KeyCode>>,
    mouse: Res<Input<MouseButton>>,
    mut mouse_motion: EventReader<MouseMotion>,
    mouse_scroll: EventReader<MouseWheel>,
) {
    inputs.update();

    let pressed  = 
        keys.get_just_pressed().map(|&k| k.into())
        .chain(mouse.get_just_pressed().map(|&b| b.into()));

    let released  = 
        keys.get_just_released().map(|&k| k.into())
        .chain(mouse.get_just_released().map(|&b| b.into()));


    let mut pressed_count = 0;
    let mut released_count = 0;

    for switch in pressed {
        let added = inputs.press(switch);
        debug!("{:?} {:?}", added, switch);
        pressed_count += 1;
    }

    for switch in released {
        inputs.release(switch);
        released_count += 1;
    }

    //assert_eq!(inputs.get_just_pressed().len(), keys.get_just_pressed().len() + mouse.get_just_pressed().len());
    //debug!("{:?} {:?} {:?}", inputs.get_just_pressed().len(), keys.get_just_pressed().len() + mouse.get_just_pressed().len(), count);
    debug!("MyReleased: {:?}, ActualReleased: {:?}, Release Count: {:?}", inputs.get_just_released().len(), keys.get_just_released().len() + mouse.get_just_released().len(), released_count);

    //for event in mouse_motion.iter() {
    //    inputs.moving(event.delta);
    //}
}

#[derive(Hash, PartialEq, Eq, Debug, ToPrimitive, FromPrimitive, Clone, Copy)]
enum SomeKeyBindings {
    SomeAction,
    SomeOtherAction,
    SomeModifiedAction,
}

#[derive(Hash, PartialEq, Eq, Debug, ToPrimitive, FromPrimitive, Clone, Copy)]
enum SomeOtherKeyBindings {
    SomeAction,
    SomeOtherAction
}

fn input<T, const K: KeyCode>(
    mut write_key: EventWriter<KeyEvent<K>>,
    mut inputs: ResMut<MappedInput>,
) {
    write_key.send(KeyEvent::<{ K }>(K));
}

fn debug_input(
    mut inputs: ResMut<MappedInput>,
) {

    //debug!("{:?}", *inputs);
    //debug!("{:?}", inputs.modifier);

    //inputs.bind(KeyCode::A, SomeKeyBindings::SomeAction);
    //inputs.bind(KeyCode::S, SomeKeyBindings::SomeAction);
    //inputs.bind(KeyCode::D, SomeKeyBindings::SomeOtherAction);

    inputs.bind((KeyCode::LAlt, KeyCode::A), SomeKeyBindings::SomeModifiedAction);
    inputs.bind((KeyCode::LAlt, MouseButton::Left), SomeKeyBindings::SomeModifiedAction);
    inputs.bind(MouseButton::Left, SomeKeyBindings::SomeAction);

    let key: AnyKey = SomeKeyBindings::SomeAction.into();
    let anotherkey: AnyKey = SomeKeyBindings::SomeOtherAction.into();

    debug!("Pressed: {:?}", inputs.get_pressed());
    debug!("Just Pressed: {:?}", inputs.get_just_pressed());
    debug!("Just Released: {:?}", inputs.get_just_released());


    //debug!("{:?}", Wat::<{ KeyCode::A }, { TestEnum::A }> {});
    //debug!("{:?}", Wat::<{ KeyCode::B }, { TestEnum::B }> {});
    debug!("{:?}", KeyEvent::<{ KeyCode::A }>(KeyCode::B));
    //debug!("{:?}", foo(KeyEvent::<{ KeyCode::B }>(KeyCode::A)));
    //debug!("{:?}", foo(KeyEvent::<{ KeyCode::B }>(KeyCode::B)));
    //debug!("{:?}", foo(KeyEvent::<{ KeyCode::A }>(KeyCode::A)));
    //debug!("{:?}", foo(KeyEvent::<{ KeyCode::A }>(KeyCode::B)));

    //debug!("{:?} {:?}", key, from_key::<SomeKeyBindings>(&key));
}