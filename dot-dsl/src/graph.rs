pub mod graph_items;
use graph_items::edge::Edge;
use graph_items::node::Node;
use std::collections::HashMap;

#[derive(Default)]
pub struct Graph {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
    pub attrs: HashMap<String, String>,
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            nodes: Vec::new(),
            edges: Vec::new(),
            attrs: HashMap::new(),
        }
    }

    pub fn with_nodes(mut self, nodes: &[Node]) -> Graph {
        self.nodes = nodes.to_vec();
        self
    }

    pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Graph {
        attrs.iter().for_each(|(a, b)| {
            self.attrs.insert((*a).to_string(), (*b).to_string());
        });
        self
    }

    pub fn with_edges(mut self, edges: &[Edge]) -> Graph {
        self.edges = edges.to_vec();
        self
    }

    pub fn get_node(self, node_id: &str) -> Option<Node> {
        let node = self.nodes.iter().find(|node| node.id == node_id);
        if let Some(val) = node {
            Some(val.clone())
        } else {
            None
        }
    }
}
