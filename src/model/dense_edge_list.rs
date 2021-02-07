//! Module implementing a dense edge list.

use std::borrow::Borrow;

use super::dense;

use crate::{Digraph, InsertGraph};

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

/// Dense edge list directed graph representation.
#[derive(Default)]
pub struct DenseEdgeList {
	verts: dense::Domain<Vert>,
	edges: dense::Domain<Edge, (Vert, Vert)>,
}

impl Digraph for DenseEdgeList {
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

impl InsertGraph for DenseEdgeList {
	fn insert_vert(&mut self) -> Self::Vert {
		self.verts.insert_default()
	}

	fn insert_edge(&mut self, tail: Self::Vert, head: Self::Vert) -> Self::Edge {
		self.edges.insert((tail, head))
	}
}

impl<G: Digraph> From<&G> for DenseEdgeList {
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
			let (g_prime, vmap, emap) = DenseEdgeList::isomorphic_from(&g);
			assert!(g.is_isomorphic_with_maps(&g_prime, &vmap, &emap));
		}

		#[test]
		fn invariants(g: TestGraph) {
			let g_prime = DenseEdgeList::from(&g);
			assert_all_digraph_invariants(&g_prime);
		}

		#[test]
		fn vert_map(g: TestGraph) {
			let g_prime = DenseEdgeList::from(&g);
			assert_vert_map_works(g_prime);
		}

		#[test]
		fn edge_map(g: TestGraph) {
			let g_prime = DenseEdgeList::from(&g);
			assert_edge_map_works(g_prime);
		}
	}
}
