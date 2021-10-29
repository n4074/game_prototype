//! This module provides an graph based input mapping data structure.
//!
//! Mapped inputs are represented as follows:
//! A switch is a single kind of user input, such as a keypress, mouse click, or more abstractly
//! some motion of the mouse.
//!
//! Every binding maps some combination of switches to an action. For example, below we bind the switch
//! KeyCode::A to the action MyActions::Left.
//!
//! ```
//! let inputs = MappedInput::default();
//! inputs.bind([KeyCode::A], MyActions::Left);
//! ```
//!
//! This is represented in the graph by adding the KeyCode::A node if it doesn't already exist, and then adding
//! an edge from this node to the Root (empty) node.
//!
//! Likewise, below we add a second binding which binds the key combination LAlt + A to the DodgeLeft action.
//!
//! ```
//!  inputs.bind( [KeyCode::LAlt, KeyCode::A], MyActions::DodgeLeft);
//! ```
//!
//! This is represented in the graph by creating a node for KeyCode::LAlt.
//! There already exists a node for KeyCode::A so a new node is not created.
//! We add an edge from KeyCode::A to KeyCode::LAlt with the DodgeLeft action.
//!
//! Finally, we add a binding for the combination LAlt + LCtrl + A to DodgeLefter
//!
//! ```
//!  inputs.bind( [KeyCode::LAlt, KeyCode::LCtrl, KeyCode::A], MyActions::DodgeLefter);
//! ```
//!
//! This adds two new nodes to the graph: the LCtrl node and a compound LCtrl + LAlt node.
//! Layer edges are added from the LAlt and LCtrl nodes to the LCtrl + LAlt node. An Action edge is created
//! from the A node to the LCtrl + LAlt node, with the DodgeLefter action.
//!
//! # TODO
//! Finish documentation
use super::Switch;
use bevy::prelude::Vec2;
use num_traits::ToPrimitive;
use petgraph::{
    graph::{DiGraph, EdgeIndex, NodeIndex},
    Direction,
};
use std::{
    any::TypeId,
    collections::{HashMap, HashSet},
    fmt::Debug,
};

use log::debug;

#[derive(Copy, Clone, Debug)]
enum Edge {
    Action(ActionId),
    Layer,
}

#[derive(Debug, Clone)]
struct Node {
    //label: String,
    active: u8,
    threshold: u8,
}

type ActionId = (TypeId, u16);

pub trait Action: 'static + ToPrimitive + Send + Sync + std::fmt::Debug {
    fn to_id(&self) -> ActionId {
        (TypeId::of::<Self>(), self.to_u16().unwrap())
    }
}

impl<T: 'static + ToPrimitive + Send + Sync + std::fmt::Debug> Action for T {}

#[derive(Debug, Default)]
pub struct MappedInput {
    //boxed_types: HashMap<ActionId, Box<dyn Action>>,
    edge_labels: HashMap<EdgeIndex, String>,
    node_labels: HashMap<NodeIndex, String>,
    bindings: DiGraph<Node, Edge>,
    nodes: HashMap<Vec<Switch>, NodeIndex>,
    active: HashSet<ActionId>,
    just_activated: HashSet<ActionId>,
    just_deactivated: HashSet<ActionId>,
    mouse_motion: Vec2,
    mouse_scroll: f32,
}

impl MappedInput {
    /// Bind an action to a key binding

    pub fn bind<I, S>(&mut self, keys: I, action: impl Action + Copy)
    where
        S: Into<Switch>,
        I: IntoIterator<Item = S>,
    {
        let keys = keys.into_iter().map(|i| i.into()).collect::<Vec<Switch>>();

        let (&terminator, layer) = keys.split_last().expect("Received empty binding");

        let terminator_node = self.get_or_create_node(&[terminator.into()]);

        let layer_node = self.get_or_create_node(&layer);
        self.add_edge(terminator_node, layer_node, Some(&action));

        if layer.len() > 1 {
            for &switch in layer {
                let switch_node = self.get_or_create_node(&[switch]);
                self.add_edge(switch_node, layer_node, None);
            }
        }
    }

    /// Add an edge _edge_ from a to b, and update edge labels
    fn add_edge(&mut self, a: NodeIndex, b: NodeIndex, action: Option<&dyn Action>) {
        let edge = match &action {
            Some(action) => Edge::Action(action.to_id()),
            None => Edge::Layer,
        };
        let edge_idx = self.bindings.update_edge(a, b, edge);

        let label = match &action {
            Some(action) => format!("{:?}", action),
            None => "Layer".to_string(),
        };

        self.edge_labels.insert(edge_idx, label);
    }

    fn get_or_create_node(&mut self, keys: &[Switch]) -> NodeIndex {
        if let Some(&index) = self.nodes.get(keys) {
            return index;
        }

        let index = self.bindings.add_node(Node {
            active: 0,
            threshold: keys.len() as u8,
        });

        self.nodes.insert(keys.to_owned(), index);

        self.node_labels.insert(index, Self::node_label(keys));

        index
    }

    fn node_label(keys: &[Switch]) -> String {
        if keys == &[] {
            return "Root".to_string();
        }
        let labels: Vec<String> = keys.into_iter().map(|s| format!("{}", s)).collect();
        labels.join(" + ")
    }

    pub(crate) fn update(&mut self) {
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
                        debug!("Decrementing node {:?}:", self.node_labels[&node]);
                    }
                    (Edge::Action(action), _) => {
                        if self.active.remove(&action) {
                            self.just_deactivated.insert(action);
                            debug!("Deactivating {}", self.edge_labels[&edge]);
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

            debug!("Pressing: {} {:?}", self.node_labels[&index], node);

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
                            "Incrementing node {}:",
                            self.node_labels[&node] //to_debug_node(&self.bindings[node], self)
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
                        // in order of threshold, so that the highest threshold
                        // comes first, and then we can just break on the first result
                        //if self.active.insert(action) {
                        //    self.just_activated.insert(action);
                        //    break;
                        //}

                        if (*threshold as i32) > highest_threshold {
                            target_action = Some((action, edge));
                            highest_threshold = *threshold as i32;
                        }
                    }
                    _ => {}
                }
            }
            if let Some((action, edge)) = target_action {
                debug!(
                    "Activating: {}",
                    self.edge_labels[&edge] //to_debug_edge(&Edge::Action(action), &self)
                );
                if self.active.insert(action) {
                    self.just_activated.insert(action);
                }
            }
        }
    }

    pub(crate) fn press(&mut self, key: Switch) {
        self.activate(key);
    }

    pub fn just_activated(&self, key: impl Action) -> bool {
        self.just_activated.get(&key.to_id()).is_some()
    }

    pub fn just_deactivated(&self, key: impl Action) -> bool {
        self.just_deactivated.get(&key.to_id()).is_some()
    }

    pub fn active(&self, key: impl Action) -> bool {
        self.active.get(&key.to_id()).is_some()
    }

    pub fn motion(&self, key: impl Action) -> Option<Vec2> {
        //let key: AnyKey = key.into();
        if self.active(key) {
            Some(self.mouse_motion)
        } else {
            None
        }
    }

    pub fn scroll(&self, key: impl Action) -> Option<f32> {
        if self.active(key) {
            Some(self.mouse_scroll)
        } else {
            None
        }
    }

    pub(crate) fn move_mouse(&mut self, motion: Vec2) {
        // todo: Think about the performance here
        self.activate(Switch::MouseMotion);
        self.mouse_motion += motion;
    }

    pub(crate) fn scroll_mouse(&mut self, scroll: f32) {
        self.activate(Switch::MouseScroll);
        self.mouse_scroll += scroll;
    }
    pub(crate) fn release(&mut self, key: Switch) {
        self.deactivate(key);
    }

    pub(crate) fn bindings_graphviz(&self) -> String {
        let debug_graph = self
            .bindings
            .map(|n, _| &self.node_labels[&n], |e, _| &self.edge_labels[&e]);

        format!("{}", petgraph::dot::Dot::new(&debug_graph))
    }
}

pub(crate) fn debug_binding_graph(input: &MappedInput) {
    let mut file = std::fs::File::create("bindings.dot").expect("Failed to create file");
    std::io::Write::write_all(&mut file, input.bindings_graphviz().as_bytes())
        .expect("Failed to write dot");
}
