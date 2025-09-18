pub(crate) mod index;
pub(crate) mod isomorphic_from;
mod key;

mod dense;
pub mod dense_bi_adjacency_list;
pub mod dense_edge_list;
pub mod dense_in_adjacency_list;
pub mod dense_out_adjacency_list;
pub mod immutable_in_adjacency_list;
pub mod immutable_out_adjacency_list;

mod sparse;
pub mod sparse_bi_adjacency_list;
pub mod sparse_edge_list;
pub mod sparse_in_adjacency_list;
pub mod sparse_out_adjacency_list;
pub mod tensor_product;

/// Module containing utilities for testing graph data structures and algorithms.
#[cfg(test)]
pub mod test_graph;

pub use dense_bi_adjacency_list::DenseBiAdjacencyList;
pub use dense_edge_list::DenseEdgeList;
pub use dense_in_adjacency_list::DenseInAdjacencyList;
pub use dense_out_adjacency_list::DenseOutAdjacencyList;
pub use immutable_in_adjacency_list::ImmutableInAdjacencyList;
pub use immutable_out_adjacency_list::ImmutableOutAdjacencyList;
pub use sparse_bi_adjacency_list::SparseBiAdjacencyList;
pub use sparse_edge_list::SparseEdgeList;
pub use sparse_in_adjacency_list::SparseInAdjacencyList;
pub use sparse_out_adjacency_list::SparseOutAdjacencyList;
