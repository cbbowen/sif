//! Module implementing a sparse in-adjacency list.

use std::borrow::Borrow;
use std::collections::{hash_set, HashSet};

use crate::{Digraph, InGraph, InsertGraph};

use super::{dense, sparse};

#[allow(missing_docs)]
pub type Vert = super::key::DenseVert;
#[allow(missing_docs)]
pub type Edge = super::key::SparseEdge;
#[allow(missing_docs)]
pub type Verts<'a> = dense::DomainKeys<'a, Vert>;
#[allow(missing_docs)]
pub type Edges<'a> = sparse::DomainKeys<'a, Edge, (Vert, Vert)>;
#[allow(missing_docs)]
pub type VertMap<T> = dense::Map<Vert, T>;
#[allow(missing_docs)]
pub type EdgeMap<T> = sparse::Map<Edge, T>;
#[allow(missing_docs)]
pub type EphemeralVertMap<'a, T> = dense::EphemeralMap<Vert, T>;
#[allow(missing_docs)]
pub type EphemeralEdgeMap<'a, T> = sparse::EphemeralMap<Edge, T>;
#[allow(missing_docs)]
pub type InEdges<'a> = std::iter::Cloned<hash_set::Iter<'a, Edge>>;

/// Sparse in-adjacency list directed graph representation.
#[derive(Default)]
pub struct SparseInAdjacencyList {
	verts: dense::Domain<Vert, HashSet<Edge>>,
	edges: sparse::Domain<Edge, (Vert, Vert)>,
}

impl Digraph for SparseInAdjacencyList {
	type Vert = Vert;
	type Edge = Edge;

	fn endpoints(&self, e: impl Borrow<Self::Edge>) -> (Self::Vert, Self::Vert) {
		self.edges[*e.borrow()]
	}

	type Verts<'a> = Verts<'a>;
	fn verts(&self) -> Self::Verts<'_> {
		self.verts.keys()
	}

	type Edges<'a> = Edges<'a>;
	fn edges(&self) -> Self::Edges<'_> {
		self.edges.keys()
	}

	type VertMap<T: Clone> = VertMap<T>;
	fn vert_map<T: Clone>(&self, default: T) -> Self::VertMap<T> {
		VertMap::with_capacity(default, self.verts.len())
	}

	type EdgeMap<T: Clone> = EdgeMap<T>;
	fn edge_map<T: Clone>(&self, default: T) -> Self::EdgeMap<T> {
		EdgeMap::with_capacity(default, self.edges.len())
	}

	type EphemeralVertMap<'a, T: Clone> = EphemeralVertMap<'a, T>;
	fn ephemeral_vert_map<T: Clone>(&self, default: T) -> Self::EphemeralVertMap<'_, T> {
		EphemeralVertMap::with_capacity(default, self.verts.len())
	}

	type EphemeralEdgeMap<'a, T: Clone> = EphemeralEdgeMap<'a, T>;
	fn ephemeral_edge_map<T: Clone>(&self, default: T) -> Self::EphemeralEdgeMap<'_, T> {
		EphemeralEdgeMap::with_capacity(default, self.edges.len())
	}
}

impl InGraph for SparseInAdjacencyList {
	type InEdges<'a> = InEdges<'a>;
	fn in_edges(&self, v: impl Borrow<Self::Vert>) -> Self::InEdges<'_> {
		self.verts[*v.borrow()].iter().cloned()
	}
}

impl InsertGraph for SparseInAdjacencyList {
	fn insert_vert(&mut self) -> Self::Vert {
		self.verts.insert_default()
	}

	fn insert_edge(&mut self, tail: Self::Vert, head: Self::Vert) -> Self::Edge {
		let e = self.edges.insert((tail, head));
		let inserted = self.verts[head].insert(e);
		debug_assert!(inserted);
		e
	}
}

impl SparseInAdjacencyList {
	/// Removes an edge.
	pub fn remove_edge(&mut self, e: Edge) {
		let (_, head) = self.edges.remove(e);
		let removed = self.verts[head].remove(&e);
		debug_assert!(removed);
	}
}

impl<G: Digraph> From<&G> for SparseInAdjacencyList {
	fn from(from: &G) -> Self {
		Self::isomorphic_from(from).0
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{Homomorphism, model::test_graph::*};
	use proptest::proptest;

	proptest! {
		#[test]
		fn ismorphic_from(g: TestGraph) {
			let (g_prime, homomorphism) = SparseInAdjacencyList::isomorphic_from(&g);
			assert!(g.is_isomorphic_with_maps(&g_prime, homomorphism.vert_map(), homomorphism.edge_map()));
		}

		#[test]
		fn invariants(g: TestGraph) {
			let g_prime = SparseInAdjacencyList::from(&g);
			assert_all_in_graph_invariants(&g_prime);
		}

		#[test]
		fn vert_map(g: TestGraph) {
			let g_prime = SparseInAdjacencyList::from(&g);
			assert_vert_map_works(g_prime);
		}

		#[test]
		fn edge_map(g: TestGraph) {
			let g_prime = SparseInAdjacencyList::from(&g);
			assert_edge_map_works(g_prime);
		}

		#[test]
		fn remove_edge(g: TestGraph) {
			let mut g_prime = SparseInAdjacencyList::from(&g);
			let mut removed = HashSet::new();
			while let Some(e) = g_prime.edges().next() {
				g_prime.remove_edge(e);
				assert!(removed.insert(e));
				assert_all_in_graph_invariants(&g_prime);
			}
		}
	}
}
