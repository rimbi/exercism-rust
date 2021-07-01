use std::collections::HashMap;

use maplit::hashmap;

#[derive(Clone, Debug, PartialEq)]
pub struct Edge {
    node1: String,
    node2: String,
}
impl Edge {
    pub fn new(node1: &str, node2: &str) -> Self {
        Self {
            node1: node1.into(),
            node2: node2.into(),
        }
    }
    pub fn with_attrs(self, _attrs: &[(&str, &str)]) -> Self {
        self
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Node {
    pub name: String,
    attrs: HashMap<String, String>,
}
impl Node {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            attrs: hashmap! {},
        }
    }
    pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
        self.attrs = attrs
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();
        self
    }
    pub fn get_attr(&self, attr: &str) -> Option<&str> {
        self.attrs.get(attr).map(|s| s.as_str())
    }
}

#[derive(Default)]
pub struct Graph {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
    pub attrs: HashMap<String, String>,
}

impl Graph {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_nodes(mut self, nodes: &[Node]) -> Self {
        self.nodes = nodes.to_vec();
        self
    }
    pub fn with_edges(mut self, edges: &[Edge]) -> Self {
        self.edges = edges.to_vec();
        self
    }
    pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
        self.attrs = attrs
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();
        self
    }
    pub fn get_node(self, node: &str) -> Result<Node, ()> {
        self.nodes
            .iter()
            .find(|n| n.name == node)
            .cloned()
            .ok_or(())
    }
}

pub mod graph {
    pub use super::Graph;

    pub mod graph_items {
        pub mod edge {
            pub use super::super::super::Edge;
        }
        pub mod node {
            pub use super::super::super::Node;
        }
    }
}
