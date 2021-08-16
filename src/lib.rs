//! Library of data structures and algorithms for working with directed graphs.
//!
//! # Data structures
//!
//! | Type                        | `InsertGraph` | `OutGraph` | `InGraph` | `remove_vert` | `remove_edge` |
//! |----------------------------:|:-------------:|:----------:|:---------:|:-------------:|:-------------:|
//! | `DenseEdgeList`             | **Yes**       | No         | No        | No            | No            |
//! | `DenseInAdjacencyList`      | **Yes**       | No         | **Yes**   | No            | No            |
//! | `DenseOutAdjacencyList`     | **Yes**       | **Yes**    | No        | No            | No            |
//! | `DenseBiAdjacencyList`      | **Yes**       | **Yes**    | **Yes**   | No            | No            |
//! | `ImmutableInAdjacencyList`  | No            | No         | **Yes**   | No            | No            |
//! | `ImmutableOutAdjacencyList` | No            | **Yes**    | No        | No            | No            |
//! | `SparseEdgeList`            | **Yes**       | No         | No        | No            | **Yes**       |
//! | `SparseInAdjacencyList`     | **Yes**       | No         | **Yes**   | No            | **Yes**       |
//! | `SparseOutAdjacencyList`    | **Yes**       | **Yes**    | No        | No            | **Yes**       |
//! | `SparseBiAdjacencyList`     | **Yes**       | **Yes**    | **Yes**   | **Yes**       | **Yes**       |

#![warn(missing_docs)]
#![feature(associated_type_defaults)]
#![feature(generic_associated_types)]
#![cfg_attr(sif_index_niche, feature(rustc_attrs))]

pub mod adjacencies;
mod depth_first;
mod digraph;
mod homomorphism;
mod in_graph;
mod insert_graph;
pub mod map;
mod model;
mod out_graph;

pub use adjacencies::*;
pub use depth_first::*;
pub use digraph::{Digraph, ExactOrderDigraph, ExactSizeDigraph};
pub use homomorphism::*;
pub use in_graph::InGraph;
pub use insert_graph::InsertGraph;
pub use map::{Map, MapMut};
pub use model::*;
pub use out_graph::OutGraph;
