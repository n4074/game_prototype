use super::{Action, AnyKey, Binding, Edge, Node, Switch};
use bevy::prelude::Vec2;
use petgraph::graphmap::DiGraphMap;
use std::collections::{HashMap, HashSet};

use log::debug;

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

    pub fn update(&mut self) {
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
                        // TODO: Layer edges should be sorted by length
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

    pub fn press(&mut self, key: Switch) {
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

    pub fn release(&mut self, key: Switch) {
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

pub(crate) fn debug_binding_graph(input: &MappedInput) {
    debug!("Pressed: {:?}", input.get_active());
    debug!("Just Pressed: {:?}", input.get_just_activated());
    debug!("Just Released: {:?}", input.get_just_deactivated());
    debug!("Layer: {:?}", input.layer);

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
