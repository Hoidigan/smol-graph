# smol-graph

A bare bones graph implementaion for when you just want
a graph with no hassle.

It uses [`uuid::Uuid`](uuid::Uuid) for its indices (inside newtypes).
This means it is possible to generate an id that already exists,
but the chance is essentially 0.

This crate also provides a [`prelude`] that exports all of the
types used in this crate: [`Graph`], [`NodeIndex`], [`EdgeIndex`].
