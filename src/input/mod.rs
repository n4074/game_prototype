use crate::SystemLabels;
use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy::prelude::*;
use keymap::AnyKey;
use num_derive::{FromPrimitive, ToPrimitive};
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;

use petgraph::graphmap::DiGraphMap;

mod keymap;

use log::debug;

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
pub enum Binding {
    Simple(Switch),
    Modified(Switch, Switch),
    DoubleModified(Switch, Switch, Switch),
}

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

impl From<Switch> for Binding {
    fn from(switch: Switch) -> Binding {
        Binding::Simple(switch)
    }
}

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

impl From<MouseButton> for Binding {
    fn from(button: MouseButton) -> Binding {
        Binding::Simple(button.into())
    }
}

impl From<KeyCode> for Binding {
    fn from(keycode: KeyCode) -> Binding {
        Binding::Simple(keycode.into())
    }
}

impl<T, Q> From<(T, Q)> for Binding
where
    T: Into<Switch>,
    Q: Into<Switch>,
{
    fn from(keys: (T, Q)) -> Binding {
        Binding::Modified(keys.0.into(), keys.1.into())
    }
}

impl<T, Q, R> From<(T, Q, R)> for Binding
where
    T: Into<Switch>,
    Q: Into<Switch>,
    R: Into<Switch>,
{
    fn from(keys: (T, Q, R)) -> Binding {
        Binding::DoubleModified(keys.0.into(), keys.1.into(), keys.2.into())
    }
}

pub trait Action: Send + Sync + std::fmt::Debug {}

impl Action for SomeKeyBindings {}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Copy, Clone)]
enum Node {
    Root,
    Switch(Switch),
    Layer(u8),
}

impl Default for Node {
    fn default() -> Self {
        Self::Root
    }
}

#[derive(Debug, Copy, Clone)]
enum Edge {
    Action(AnyKey),
    Layer(Node),
}

#[derive(Debug, Default)]
pub struct MappedInput {
    boxed_types: HashMap<AnyKey, Box<dyn Action>>,
    bindings: DiGraphMap<Node, Edge>,
    layer: Node,
    active: HashSet<AnyKey>,
    just_activated: HashSet<AnyKey>,
    just_deactivated: HashSet<AnyKey>,
    pressed: HashSet<Switch>,
    mouse_motion: Vec2,
    mouse_scroll: f32,
    layer_count: u8,
}

impl MappedInput {
    /// Bind an action to a key binding

    pub fn bind<T>(&mut self, key: impl Into<Binding>, action: T)
    where
        T: Into<AnyKey> + Action + 'static + Copy + Clone,
    {
        let binding = key.into();

        match binding {
            Binding::DoubleModified(first, second, switch) => {
                let combined = self.bindings.add_node(Node::Layer(self.layer_count));
                self.layer_count += 1; // TODO: This will crash on overflow.

                self.bindings.extend(&[
                    (
                        Node::Switch(first),
                        Node::Switch(second),
                        Edge::Layer(combined),
                    ),
                    (
                        Node::Switch(second),
                        Node::Switch(first),
                        Edge::Layer(combined),
                    ),
                    (combined, Node::Switch(switch), Edge::Action(action.into())),
                ]);
            }
            Binding::Modified(modifier, switch) => {
                self.bindings.extend(&[
                    (
                        Node::Root,
                        Node::Switch(modifier),
                        Edge::Layer(Node::Switch(modifier)),
                    ),
                    (
                        Node::Switch(modifier),
                        Node::Switch(switch),
                        Edge::Action(action.into()),
                    ),
                ]);
            }

            Binding::Simple(switch) => {
                self.bindings.add_edge(
                    Node::Root,
                    Node::Switch(switch),
                    Edge::Action(action.into()),
                );
            }
        }

        self.boxed_types.insert(action.into(), Box::new(action));
    }

    fn update(&mut self) {
        self.just_activated.clear();
        self.just_deactivated.clear();
        self.mouse_motion = Vec2::ZERO;
        self.mouse_scroll = 0f32;
    }

    /// Deactivate a bound action
    fn deactivate(&mut self, node: Node) {
        for (_, _, e) in self.bindings.edges(node) {
            // Outgoing edges only exist on layers.

            match e {
                Edge::Action(action) => {
                    // Deactivate action depending on this key
                    if self.active.remove(action) {
                        self.just_deactivated.insert(*action);
                    }
                }
                Edge::Layer(_) => {} // Outgoing layer edges aren't useful here
            }
        }

        let mut new_layer = None;

        for neighbour in self
            .bindings
            .neighbors_directed(node, petgraph::EdgeDirection::Incoming)
        {
            match self.bindings.edge_weight(neighbour, node) {
                Some(Edge::Action(action)) => {
                    if self.active.remove(action) {
                        self.just_deactivated.insert(*action);
                    }
                }
                Some(Edge::Layer(layer)) => {
                    if *layer == self.layer {
                        // the current layer depends on this node
                        // TODO: only assign if len of neighbour layer is longer than current
                        new_layer = Some(neighbour);
                    }
                }
                None => {}
            }
        }

        if let Some(layer) = new_layer {
            if self.layer != node {
                self.deactivate(self.layer);
            }
            self.layer = layer;
        }
    }

    /// Activate a bound action
    fn activate(&mut self, node: Node) {
        let edge = self
            .bindings
            .edge_weight(self.layer, node)
            .or(self.bindings.edge_weight(Node::Root, node));

        match edge {
            Some(Edge::Action(action)) => {
                if self.active.insert(*action) {
                    self.just_activated.insert(*action);
                }
            }
            Some(Edge::Layer(layer)) => {
                // Switch to a new layer
                self.layer = *layer;
            }
            None => {}
        }
    }

    fn press(&mut self, key: Switch) {
        if !self.pressed.contains(&key) {
            self.activate(Node::Switch(key));
            self.pressed.insert(key);
        }
    }

    pub fn just_activated<T>(&self, key: T) -> bool
    where
        T: Into<AnyKey>,
    {
        self.just_activated.get(&key.into()).is_some()
    }

    pub fn just_deactivated<T>(&self, key: T) -> bool
    where
        T: Into<AnyKey>,
    {
        self.just_deactivated.get(&key.into()).is_some()
    }

    pub fn active<T>(&self, key: T) -> bool
    where
        T: Into<AnyKey>,
    {
        self.active.get(&key.into()).is_some()
    }

    pub fn move_mouse(&mut self, motion: Vec2) {
        // todo: Think about the performance here
        self.activate(Node::Switch(Switch::MouseMotion));
        self.mouse_motion += motion;
    }

    pub fn scroll_mouse(&mut self, scroll: f32) {
        self.activate(Node::Switch(Switch::MouseScroll));
        self.mouse_scroll += scroll;
    }

    pub fn motion<T>(&self, key: T) -> Option<Vec2>
    where
        T: Into<AnyKey>,
    {
        if self.active(key.into()) {
            Some(self.mouse_motion)
        } else {
            None
        }
    }

    pub fn scroll<T>(&self, key: T) -> Option<f32>
    where
        T: Into<AnyKey>,
    {
        if self.active(key.into()) {
            Some(self.mouse_scroll)
        } else {
            None
        }
    }

    fn release(&mut self, key: Switch) {
        if self.pressed.remove(&key) {
            self.deactivate(Node::Switch(key));
        }
    }

    fn get_active(&self) -> Vec<&Box<dyn Action>> {
        self.active
            .iter()
            .filter_map(|key| self.boxed_types.get(key))
            .collect()
    }

    fn get_just_activated(&self) -> Vec<&Box<dyn Action>> {
        self.just_activated
            .iter()
            .filter_map(|key| self.boxed_types.get(key))
            .collect()
    }

    fn get_just_deactivated(&self) -> Vec<&Box<dyn Action>> {
        self.just_deactivated
            .iter()
            .filter_map(|key| self.boxed_types.get(key))
            .collect()
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

fn to_debug_edge(edge: &Edge, input: &MappedInput) -> String {
    match *edge {
        Edge::Action(a) => format!("{:?}", input.boxed_types.get(&a).unwrap()),
        a => format!("{:?}", a),
    }
}

fn to_debug_node(node: &Node, _input: &MappedInput) -> String {
    match *node {
        a => format!("{:?}", a),
    }
}

fn debug_binding_graph(input: &MappedInput) {
    let graph = input.bindings.clone().into_graph::<u32>();
    let debug_graph = graph.map(
        |_, n| to_debug_node(n, input),
        |_, e| to_debug_edge(e, input),
    );
    let dot = petgraph::dot::Dot::new(&debug_graph);

    let mut file = std::fs::File::create("bindings.dot").expect("Failed to create file");
    std::io::Write::write_all(&mut file, format!("{:?}", dot).as_bytes())
        .expect("Failed to write dot");
    //file.write_all(dot);
}

fn setup_debug_input(mut inputs: ResMut<MappedInput>) {
    inputs.bind(
        (KeyCode::LAlt, KeyCode::A),
        SomeKeyBindings::SomeModifiedAction,
    );
    inputs.bind(
        (KeyCode::LAlt, MouseButton::Left),
        SomeKeyBindings::SomeModifiedAction,
    );

    inputs.bind(
        (KeyCode::RAlt, KeyCode::RControl, KeyCode::A),
        SomeKeyBindings::SomeDoubleModifiedAction,
    );

    inputs.bind(
        (KeyCode::RAlt, KeyCode::A),
        SomeKeyBindings::SomeModifiedAction,
    );

    inputs.bind(
        (KeyCode::RControl, KeyCode::A),
        SomeKeyBindings::SomeOtherModifiedAction,
    );

    inputs.bind(MouseButton::Left, SomeKeyBindings::SomeAction);
}

fn debug_input(inputs: ResMut<MappedInput>) {
    debug!("Pressed: {:?}", inputs.get_active());
    debug!("Just Pressed: {:?}", inputs.get_just_activated());
    debug!("Just Released: {:?}", inputs.get_just_deactivated());
    debug!("Layer: {:?}", inputs.layer);

    debug_binding_graph(&inputs);
}
