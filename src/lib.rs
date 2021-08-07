#![doc = include_str!("../README.md")]

use std::collections::HashMap;

use uuid::Uuid;

/// A graph generic over node and edge data.
///
/// Nodes and edges are public because this graph makes 
/// no guarantees as to its structure beyond being a graph.
///
/// The graph does provide convenience functions for 
/// some simple operations over its data.
///
/// Along with that, it provides some algorithms for them as well.
pub struct Graph<Node, Edge> {
    pub nodes: HashMap<NodeIndex, Node>,
    pub edges: HashMap<EdgeIndex, ((NodeIndex, NodeIndex), Edge)>,
}

/// An index pointing to a node in the graph.
#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct NodeIndex(Uuid);

/// An index pointing to an edge in the graph.

#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct EdgeIndex(Uuid);

impl NodeIndex {
    /// Create a new node index.
    pub fn new() -> NodeIndex {
        NodeIndex(Uuid::new_v4())
    }
}

impl EdgeIndex {
    /// Create a new edge index.
    pub fn new() -> EdgeIndex {
        EdgeIndex(Uuid::new_v4())
    }
}

impl<N, E> Graph<N, E> {
    /// Create a new graph with empty nodes and edges.
    /// 
    /// To initialize with capacity or other pre-defined
    /// settings, create it using public fields.
    pub fn new() -> Self { 
        Self { 
            nodes: HashMap::new(), 
            edges: HashMap::new() 
        }
    }

    /// An iterator over this graph's nodes, in no
    /// particular order.
    pub fn nodes<'a>(&'a self) -> impl Iterator + 'a {
        self.nodes.iter()
    }

    /// An iterator over this graph's edges, in no
    /// particular order.
    pub fn edges<'a>(&'a self) -> impl Iterator + 'a {
        self.edges.iter()
    }
}

pub mod prelude {
    pub use crate::{
        Graph,
        NodeIndex,
        EdgeIndex,
    };
}
