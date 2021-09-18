use super::{Action, AnyKey, Switch};
use bevy::prelude::Vec2;
use petgraph::{
    graph::{DiGraph, NodeIndex},
    Direction,
};
use std::collections::{HashMap, HashSet};

use log::debug;

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
enum Edge {
    Action(AnyKey),
    Layer,
}

//#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, Default)]
//enum Node {
//    #[default]
//    Root,
//    Layer {
//        active: u8,
//        required: u8,
//    },
//}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct Node {
    label: String,
    active: u8,
    threshold: u8,
}

#[derive(Debug, Default)]
pub struct MappedInput {
    boxed_types: HashMap<AnyKey, Box<dyn Action>>,
    bindings: DiGraph<Node, Edge>,
    //layer: Option<Node>,
    layer_depth: u8,
    active: HashSet<AnyKey>,
    just_activated: HashSet<AnyKey>,
    just_deactivated: HashSet<AnyKey>,
    pressed: HashSet<Switch>,
    mouse_motion: Vec2,
    mouse_scroll: f32,
    layer_count: u8,
    nodes: HashMap<Vec<Switch>, NodeIndex>,
}

impl MappedInput {
    /// Bind an action to a key binding

    fn get_or_create_node(&mut self, keys: &[Switch]) -> NodeIndex {
        if let Some(&index) = self.nodes.get(keys) {
            return index;
        }

        let index = self.bindings.add_node(Node {
            label: format!("{:?}", keys),
            active: 0,
            threshold: keys.len() as u8,
        });

        self.nodes.insert(keys.to_owned(), index);

        index
    }

    pub fn bind<I, T, S>(&mut self, keys: I, action: T)
    where
        S: Into<Switch>,
        I: IntoIterator<Item = S>,
        T: Into<AnyKey> + Action + 'static + Copy + Clone,
    {
        let keys = keys.into_iter().map(|i| i.into()).collect::<Vec<Switch>>();
        self.boxed_types.insert(action.into(), Box::new(action));

        let (&terminator, layer) = keys[..].split_last().expect("Received empty binding");

        let terminator_node = self.get_or_create_node(&[terminator]);

        let layer_node = self.get_or_create_node(layer);

        // edge from switch to layer, with action
        self.bindings
            .add_edge(terminator_node, layer_node, Edge::Action(action.into()));

        if layer.len() > 1 {
            for &switch in layer {
                let switch_node = self.get_or_create_node(&[switch]);

                self.bindings.add_edge(switch_node, layer_node, Edge::Layer);
            }
        }
    }

    pub fn update(&mut self) {
        self.just_activated.clear();
        self.just_deactivated.clear();
        self.mouse_motion = Vec2::ZERO;
        self.mouse_scroll = 0f32;
        self.deactivate(Switch::MouseMotion);
        self.deactivate(Switch::MouseScroll);
    }

    /// Deactivate a bound action
    fn deactivate(&mut self, switch: Switch) {
        if let Some(&index) = self.nodes.get(&[switch].to_vec()) {
            let node = &mut self.bindings[index];

            if node.active == 0 {
                // already inactive
                return;
            }
            node.active -= 1;

            let mut neighbours = self
                .bindings
                .neighbors_directed(index, Direction::Outgoing)
                .detach();

            while let Some((edge, node)) = neighbours.next(&self.bindings) {
                match (self.bindings[edge], &mut self.bindings[node]) {
                    (Edge::Layer, Node { active, .. }) => {
                        *active -= 1;
                        debug!(
                            "Decrementing node {:?}:",
                            to_debug_node(&self.bindings[node], self)
                        );
                    }
                    (Edge::Action(action), _) => {
                        if self.active.remove(&action) {
                            self.just_deactivated.insert(action);
                            debug!(
                                "Deactivating {}",
                                to_debug_edge(&Edge::Action(action), self)
                            );
                        }
                    }
                }
            }
        }
    }

    /// Activate a bound action
    fn activate(&mut self, switch: Switch) {
        // TODO: Get rid of this vec here. Maybe a bitmap?
        if let Some(&index) = self.nodes.get(&[switch].to_vec()) {
            let node = &mut self.bindings[index];

            if node.active != 0 {
                return;
            }

            node.active += 1;

            debug!("Pressing: {:?}", node);

            let mut neighbours = self
                .bindings
                .neighbors_directed(index, Direction::Outgoing)
                .detach();

            let mut highest_threshold = -1i32;
            let mut target_action = None;

            while let Some((edge, node)) = neighbours.next(&self.bindings) {
                match (self.bindings[edge], &mut self.bindings[node]) {
                    (Edge::Layer, Node { active, .. }) => {
                        *active += 1;
                        debug!(
                            "Incrementing node {:?}:",
                            to_debug_node(&self.bindings[node], self)
                        );
                    }
                    (
                        Edge::Action(action),
                        Node {
                            ref active,
                            ref threshold,
                            ..
                        },
                    ) if active >= threshold => {
                        debug!(
                            "Threshold: {:?} Active: {:?}, Max: {:?}",
                            threshold, active, highest_threshold
                        );
                        // # TODO: Eventually we want our edges to be presorted
                        // so we can break here;
                        //if self.active.insert(action) {
                        //    self.just_activated.insert(action);
                        //    break;
                        //}

                        if (*threshold as i32) > highest_threshold {
                            target_action = Some(action);
                            highest_threshold = *threshold as i32;
                        }
                    }
                    _ => {}
                }
            }
            if let Some(action) = target_action {
                debug!(
                    "Activating: {:?}",
                    to_debug_edge(&Edge::Action(action), &self)
                );
                if self.active.insert(action) {
                    self.just_activated.insert(action);
                }
            }
        }
    }

    pub fn press(&mut self, key: Switch) {
        self.activate(key);
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
        self.activate(Switch::MouseMotion);
        self.mouse_motion += motion;
    }

    pub fn scroll_mouse(&mut self, scroll: f32) {
        self.activate(Switch::MouseScroll);
        self.mouse_scroll += scroll;
    }

    pub fn motion<T>(&self, key: T) -> Option<Vec2>
    where
        T: Into<AnyKey>,
    {
        let key: AnyKey = key.into();
        if self.active(key) {
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

    pub fn release(&mut self, key: Switch) {
        self.deactivate(key);
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

fn to_debug_edge(edge: &Edge, input: &MappedInput) -> String {
    match *edge {
        Edge::Action(a) => format!("{:?}", input.boxed_types.get(&a).unwrap()),
        a => format!("{:?}", a),
    }
}

fn to_debug_node(node: &Node, _input: &MappedInput) -> String {
    match node {
        a => format!("{:?}", a),
    }
}

pub(crate) fn debug_binding_graph(input: &MappedInput) {
    //debug!("Pressed: {:?}", input.get_active());
    //debug!("Just Pressed: {:?}", input.get_just_activated());
    //debug!("Just Released: {:?}", input.get_just_deactivated());
    //debug!("Layer: {:?}", input.layer_depth);
    //debug!("Bindings: {:?}", input.nodes);

    let graph = input.bindings.clone();
    let debug_graph = graph.map(
        |_, n| to_debug_node(n, input),
        |_, e| to_debug_edge(e, input),
    );
    let dot = petgraph::dot::Dot::new(&debug_graph);

    let mut file = std::fs::File::create("bindings.dot").expect("Failed to create file");
    std::io::Write::write_all(&mut file, format!("{:?}", dot).as_bytes())
        .expect("Failed to write dot");
}
