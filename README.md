# smol-graph

A bare bones graph implementaion for when you just want
a graph with no hassle.

It uses [`Uuid`][Uuid] for its indices (inside newtypes).
This means it is possible to generate an id that already exists,
but the chance is essentially 0.

This crate also provides a [`prelude`][prelude] that exports all of the
types used in this crate: [`Graph`][Graph], [`NodeIndex`][NodeIndex], 
and[`EdgeIndex`][EdgeIndex].

[Uuid]: https://docs.rs/uuid/*/uuid/
[prelude]: https://docs.rs/smol-graph/*/smol_graph/prelude/index.html
[Graph]: https://docs.rs/smol-graph/*/smol_graph/struct.Graph.html
[NodeIndex]: https://docs.rs/smol-graph/*/smol_graph/struct.NodeIndex.html
[EdgeIndex]: https://docs.rs/smol-graph/*/smol_graph/struct.EdgeIndex.html