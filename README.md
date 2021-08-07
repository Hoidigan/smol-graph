# smol-graph

A simple and to-the-point Rust graph implementation.

It uses [`Uuid`][Uuid] for its indices (inside newtypes).
This means it is possible to generate an id that already exists,
but the chance is essentially 0.

This crate also provides a [`prelude`][prelude] that exports all of the
types used in this crate: [`Graph`][Graph], [`NodeIndex`][NodeIndex], 
and[`EdgeIndex`][EdgeIndex].

# Examples

Usage:
First add it to your Cargo.toml, then use it in your project.
```rust
// Import the few types this library exports.
use smol_graph::prelude::*;
```
```rust
// Or do it explicitely.
use smol_graph::{
    Graph, Edge, EdgeIndex, NodeIndex,
};
```

Creating a new graph:
```rust
use smol_graph::Graph;

// Convience function:
let graph: Graph<(), ()> = Graph::new();

// Or alternately, explicit field filling.
// This could be used for with capacity or prefilled maps, for example.
use std::collections::HashMap;
let graph: Graph<(), ()> = Graph {
    nodes: HashMap::new(),
    edges: HashMap::new(),
};
```

Inserting nodes and edges:
```rust
use smol_graph::{Graph, Edge};

let mut graph: Graph<i32, String> = Graph::new();

// Insert nodes and get their indices.
let one = graph.node(1);
let two = graph.node(2);
// or do it with field access
let three = {
    use smol_graph::NodeIndex;
    
    let idx = NodeIndex::new();
    graph.nodes.insert(idx, 3);
    idx
};

// And then for edges:
let a = graph.edge(Edge::new((one, two), String::from("a")));
// Or with field access:
let b = {
    use smol_graph::EdgeIndex;

    let idx = EdgeIndex::new();
    let edge = Edge {
        nodes: (two, three),
        data: String::from("b"),
    };
    graph.edges.insert(idx, edge);
    idx
};
```

Removing nodes or edges:
```rust
use smol_graph::{Graph, Edge};

let mut graph = Graph::new();
let a = graph.node(5);
let b = graph.node(6);
let edge = graph.edge(Edge::new((a, b), ()));

// Watch out, the edge still points to the node that doesn't exist.
graph.r_node(a).unwrap();
graph.r_node(b).unwrap();
// And then remove the edge.
graph.r_edge(edge).unwrap();
```

# Notes

* It uses Uuid for indices. This does mean that, though virtually impossible,
a duplicate index could be generated. For more information on this, see
<https://en.wikipedia.org/wiki/Universally_unique_identifier>.

* The graph is backed by the std hashmap for both nodes and edges.
While that may not be the best in all cases, it's certainly one of the easiest.

* It doesn't check for edges pointing to valid nodes.
While it would be possible to implement that, this library doesn't aim to 
do anything beyond have a small and easy type for a graph.
Checking for nodes requires design decisions, and that is left up to the user.

[Uuid]: https://docs.rs/uuid/
[prelude]: https://docs.rs/smol-graph/*/smol_graph/prelude/index.html
[Graph]: https://docs.rs/smol-graph/*/smol_graph/struct.Graph.html
[NodeIndex]: https://docs.rs/smol-graph/*/smol_graph/struct.NodeIndex.html
[EdgeIndex]: https://docs.rs/smol-graph/*/smol_graph/struct.EdgeIndex.html