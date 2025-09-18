//! Module implementing a sparse bi-adjacency list.

use std::borrow::Borrow;
use std::collections::{HashSet, hash_set};

use crate::{Digraph, InGraph, InsertGraph, OutGraph};

use super::sparse;

#[allow(missing_docs)]
pub type Vert = super::key::SparseVert;
#[allow(missing_docs)]
pub type Edge = super::key::SparseEdge;
#[allow(missing_docs)]
pub type Verts<'a> = sparse::DomainKeys<'a, Vert, (HashSet<Edge>, HashSet<Edge>)>;
#[allow(missing_docs)]
pub type Edges<'a> = sparse::DomainKeys<'a, Edge, (Vert, Vert)>;
#[allow(missing_docs)]
pub type VertMap<T> = sparse::Map<Vert, T>;
#[allow(missing_docs)]
pub type EdgeMap<T> = sparse::Map<Edge, T>;
#[allow(missing_docs)]
pub type EphemeralVertMap<'a, T> = sparse::EphemeralMap<Vert, T>;
#[allow(missing_docs)]
pub type EphemeralEdgeMap<'a, T> = sparse::EphemeralMap<Edge, T>;
#[allow(missing_docs)]
pub type OutEdges<'a> = std::iter::Cloned<hash_set::Iter<'a, Edge>>;
#[allow(missing_docs)]
pub type InEdges<'a> = std::iter::Cloned<hash_set::Iter<'a, Edge>>;

/// Sparse bi-adjacency list directed graph representation.
#[derive(Default)]
pub struct SparseBiAdjacencyList {
	verts: sparse::Domain<Vert, (HashSet<Edge>, HashSet<Edge>)>,
	edges: sparse::Domain<Edge, (Vert, Vert)>,
}

impl Digraph for SparseBiAdjacencyList {
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

impl OutGraph for SparseBiAdjacencyList {
	type OutEdges<'a> = OutEdges<'a>;
	fn out_edges(&self, v: impl Borrow<Self::Vert>) -> Self::OutEdges<'_> {
		self.verts[*v.borrow()].0.iter().cloned()
	}
}

impl InGraph for SparseBiAdjacencyList {
	type InEdges<'a> = InEdges<'a>;
	fn in_edges(&self, v: impl Borrow<Self::Vert>) -> Self::InEdges<'_> {
		self.verts[*v.borrow()].1.iter().cloned()
	}
}

impl InsertGraph for SparseBiAdjacencyList {
	fn insert_vert(&mut self) -> Self::Vert {
		self.verts.insert_default()
	}

	fn insert_edge(&mut self, tail: Self::Vert, head: Self::Vert) -> Self::Edge {
		let e = self.edges.insert((tail, head));
		let out_inserted = self.verts[tail].0.insert(e);
		let in_inserted = self.verts[head].1.insert(e);
		debug_assert!(out_inserted);
		debug_assert!(in_inserted);
		e
	}
}

impl SparseBiAdjacencyList {
	/// Removes an edge.
	pub fn remove_edge(&mut self, e: Edge) {
		let (tail, head) = self.edges.remove(e);
		let out_removed = self.verts[tail].0.remove(&e);
		let in_removed = self.verts[head].1.remove(&e);
		debug_assert!(out_removed);
		debug_assert!(in_removed);
	}

	/// Removes a vertex and all adjacent edges.
	pub fn remove_vert(&mut self, v: Vert) {
		let (out_edges, in_edges) = self.verts.remove(v);
		for e in out_edges {
			let head = self.head(e);
			// Self loops will be handled by the in_edges loop so the tail lookup remains
			// valid.
			if head != v {
				self.edges.remove(e);
				self.verts[head].1.remove(&e);
			}
		}
		for e in in_edges {
			let tail = self.tail(e);
			self.edges.remove(e);
			// Self loops do not require adjacency updates.
			if tail != v {
				self.verts[tail].0.remove(&e);
			}
		}
	}
}

impl<G: Digraph> From<&G> for SparseBiAdjacencyList {
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
			let (g_prime, homomorphism) = SparseBiAdjacencyList::isomorphic_from(&g);
			assert!(g.is_isomorphic_with_maps(&g_prime, homomorphism.vert_map(), homomorphism.edge_map()));
		}

		#[test]
		fn invariants(g: TestGraph) {
			let g_prime = SparseBiAdjacencyList::from(&g);
			assert_all_bi_graph_invariants(&g_prime);
		}

		#[test]
		fn vert_map(g: TestGraph) {
			let g_prime = SparseBiAdjacencyList::from(&g);
			assert_vert_map_works(g_prime);
		}

		#[test]
		fn edge_map(g: TestGraph) {
			let g_prime = SparseBiAdjacencyList::from(&g);
			assert_edge_map_works(g_prime);
		}

		#[test]
		fn remove_vert(g: TestGraph) {
			let mut g_prime = SparseBiAdjacencyList::from(&g);
			let mut removed = HashSet::new();
			while let Some(v) = g_prime.verts().next() {
				g_prime.remove_vert(v);
				assert!(removed.insert(v));
				assert_all_bi_graph_invariants(&g_prime);
			}
		}

		#[test]
		fn remove_edge(g: TestGraph) {
			let mut g_prime = SparseBiAdjacencyList::from(&g);
			let mut removed = HashSet::new();
			while let Some(e) = g_prime.edges().next() {
				g_prime.remove_edge(e);
				assert!(removed.insert(e));
				assert_all_bi_graph_invariants(&g_prime);
			}
		}
	}
}
