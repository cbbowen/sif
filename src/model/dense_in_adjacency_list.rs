//! Module implementing a dense in-adjacency list.

use std::borrow::Borrow;

use crate::{Digraph, InGraph, InsertGraph};

use super::dense;

#[allow(missing_docs)]
pub type Vert = super::key::DenseVert;
#[allow(missing_docs)]
pub type Edge = super::key::DenseEdge;
#[allow(missing_docs)]
pub type Verts<'a> = dense::DomainKeys<'a, Vert>;
#[allow(missing_docs)]
pub type Edges<'a> = dense::DomainKeys<'a, Edge>;
#[allow(missing_docs)]
pub type VertMap<T> = dense::Map<Vert, T>;
#[allow(missing_docs)]
pub type EdgeMap<T> = dense::Map<Edge, T>;
#[allow(missing_docs)]
pub type EphemeralVertMap<'a, T> = dense::EphemeralMap<Vert, T>;
#[allow(missing_docs)]
pub type EphemeralEdgeMap<'a, T> = dense::EphemeralMap<Edge, T>;
#[allow(missing_docs)]
pub type InEdges<'a> = std::iter::Cloned<std::slice::Iter<'a, Edge>>;

/// Dense in-adjacency list directed graph representation.
#[derive(Default)]
pub struct DenseInAdjacencyList {
	verts: dense::Domain<Vert, Vec<Edge>>,
	edges: dense::Domain<Edge, (Vert, Vert)>,
}

impl Digraph for DenseInAdjacencyList {
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

impl InGraph for DenseInAdjacencyList {
	type InEdges<'a> = InEdges<'a>;
	fn in_edges(&self, v: impl Borrow<Self::Vert>) -> Self::InEdges<'_> {
		self.verts[*v.borrow()].iter().cloned()
	}
}

impl InsertGraph for DenseInAdjacencyList {
	fn insert_vert(&mut self) -> Self::Vert {
		self.verts.insert_default()
	}

	fn insert_edge(&mut self, tail: Self::Vert, head: Self::Vert) -> Self::Edge {
		let e = self.edges.insert((tail, head));
		self.verts[head].push(e);
		e
	}
}

impl<G: Digraph> From<&G> for DenseInAdjacencyList {
	fn from(from: &G) -> Self {
		Self::isomorphic_from(from).0
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::model::test_graph::*;
	use proptest::proptest;

	proptest! {
		#[test]
		fn ismorphic_from(g: TestGraph) {
			let (g_prime, homomorphism) = DenseInAdjacencyList::isomorphic_from(&g);
			assert!(g.is_isomorphic_with_maps(&g_prime, homomorphism.vert_map(), homomorphism.edge_map()));
		}

		#[test]
		fn invariants(g: TestGraph) {
			let g_prime = DenseInAdjacencyList::from(&g);
			assert_all_in_graph_invariants(&g_prime);
		}

		#[test]
		fn vert_map(g: TestGraph) {
			let g_prime = DenseInAdjacencyList::from(&g);
			assert_vert_map_works(g_prime);
		}

		#[test]
		fn edge_map(g: TestGraph) {
			let g_prime = DenseInAdjacencyList::from(&g);
			assert_edge_map_works(g_prime);
		}
	}
}
