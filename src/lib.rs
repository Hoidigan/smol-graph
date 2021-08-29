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
#[derive(Debug)]
pub struct Graph<Node, EdgeData> {
    pub nodes: HashMap<NodeIndex, Node>,
    pub edges: HashMap<EdgeIndex, Edge<EdgeData>>,
}

/// A struct representing an edge between two nodes.
#[derive(Debug, Clone)]
pub struct Edge<Data> {
    pub nodes: (NodeIndex, NodeIndex),
    pub data: Data,
}

impl<Data> Edge<Data> {
    /// Create a new edge with nodes and data.
    pub fn new(nodes: (NodeIndex, NodeIndex), data: Data) -> Edge<Data> {
        Edge {
            nodes,
            data,
        }
    }
}

/// An index pointing to a node in the graph.
#[derive(Debug, Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct NodeIndex(Uuid);

/// An index pointing to an edge in the graph.
#[derive(Debug, Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
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

impl<Node, EdgeData> Graph<Node, EdgeData> {
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

    /// A convenience function to insert an edge at a new index.
    pub fn edge(&mut self, edge: Edge<EdgeData>) -> EdgeIndex {
        let idx = EdgeIndex::new();
        self.edges.insert(idx, edge);
        idx
    }

    /// Attempt to remove an edge from this graph, returning it if it 
    /// existed.
    pub fn r_edge(&mut self, idx: EdgeIndex) -> Option<Edge<EdgeData>> {
        self.edges.remove(&idx)
    }

    /// A convenience function to insert a node at a new index.
    pub fn node(&mut self, node: Node) -> NodeIndex {
        let idx = NodeIndex::new();
        self.nodes.insert(idx, node);
        idx
    }

    /// Attempt to remove a node from this graph, returning it if it 
    /// existed.
    ///
    /// Note that this will *not* check if any edges still have the node
    /// referenced.
    pub fn r_node(&mut self, idx: NodeIndex) -> Option<Node> {
        self.nodes.remove(&idx)
    }
}

pub mod prelude {
    pub use crate::{
        Graph,
        NodeIndex,
        EdgeIndex,
        Edge,
    };
}
