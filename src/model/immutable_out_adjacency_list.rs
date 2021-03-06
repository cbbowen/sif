//! Module implementing an immutable out-adjacency list.

use std::borrow::Borrow;

use itertools::{Itertools, MapInto};
use std::ops::Range;

use crate::{
	map::{self, Map, MapMut},
	Digraph, OutGraph,
};

use super::dense::{self, Key};

#[allow(missing_docs)]
pub type Vert = super::key::DenseVert;
#[allow(missing_docs)]
pub type Edge = super::key::DenseEdge;
#[allow(missing_docs)]
pub type Verts<'a> = dense::DomainKeys<'a, Vert>;
#[allow(missing_docs)]
pub type Edges<'a> = dense::DomainKeys<'a, Edge>;
#[allow(missing_docs)]
pub type VertMap<T> = dense::EphemeralMap<Vert, T>;
#[allow(missing_docs)]
pub type EdgeMap<T> = dense::EphemeralMap<Edge, T>;
#[allow(missing_docs)]
pub type EphemeralVertMap<'a, T> = VertMap<T>;
#[allow(missing_docs)]
pub type EphemeralEdgeMap<'a, T> = EdgeMap<T>;
#[allow(missing_docs)]
pub type OutEdges<'a> = MapInto<Range<usize>, Edge>;

#[derive(Debug)]
/// Immutable out-adjacency list directed graph representation.
pub struct ImmutableOutAdjacencyList {
	// Mapping from vertices to the first edge with it as the tail. This also
	// has an extra element mapped to the size of the graph to facilitate
	// lookups.
	outs: dense::Domain<Vert, Edge>,
	// Mapping from edges to its out vertex.
	heads: dense::Domain<Edge, Vert>,
}

impl Digraph for ImmutableOutAdjacencyList {
	type Vert = Vert;
	type Edge = Edge;

	fn endpoints(&self, e: impl Borrow<Self::Edge>) -> (Self::Vert, Self::Vert) {
		let e = e.borrow();
		(self.tail(e), self.head(e))
	}

	fn tail(&self, e: impl Borrow<Self::Edge>) -> Self::Vert {
		let e = e.borrow();
		(self.outs.values().partition_point(|q| q <= e) - 1).into()
	}

	fn head(&self, e: impl Borrow<Self::Edge>) -> Self::Vert {
		self.heads[*e.borrow()]
	}

	type Verts<'a> = Verts<'a>;
	fn verts(&self) -> Self::Verts<'_> {
		(0..self.outs.len() - 1).map_into::<Vert>()
	}

	type Edges<'a> = Edges<'a>;
	fn edges(&self) -> Self::Edges<'_> {
		self.heads.keys()
	}

	type VertMap<T: Clone> = VertMap<T>;
	fn vert_map<T: Clone>(&self, default: T) -> Self::VertMap<T> {
		VertMap::with_capacity(default, self.outs.len() - 1)
	}

	type EdgeMap<T: Clone> = EdgeMap<T>;
	fn edge_map<T: Clone>(&self, default: T) -> Self::EdgeMap<T> {
		EdgeMap::with_capacity(default, self.heads.len())
	}

	type EphemeralVertMap<'a, T: Clone> = EphemeralVertMap<'a, T>;
	fn ephemeral_vert_map<T: Clone>(&self, default: T) -> Self::EphemeralVertMap<'_, T> {
		self.vert_map(default)
	}

	type EphemeralEdgeMap<'a, T: Clone> = EphemeralEdgeMap<'a, T>;
	fn ephemeral_edge_map<T: Clone>(&self, default: T) -> Self::EphemeralEdgeMap<'_, T> {
		self.edge_map(default)
	}
}

impl OutGraph for ImmutableOutAdjacencyList {
	type OutEdges<'a> = OutEdges<'a>;
	fn out_edges(&self, v: impl Borrow<Self::Vert>) -> Self::OutEdges<'_> {
		let v = v.borrow();
		let start = self.outs[*v].index();
		let end = self.outs[(v.index() + 1).into()].index();
		(start..end).map_into::<Edge>()
	}
}

impl ImmutableOutAdjacencyList {
	/// Constructs a graph isomorphic to the given graph and returns it along with
	/// mappings from the given graph's vertices and edges to those in the new
	/// graph.
	fn isomorphic_from<G: OutGraph>(
		from: &G,
	) -> (
		Self,
		map::Unwrap<G::EphemeralVertMap<'_, Option<Vert>>>,
		map::Unwrap<G::EphemeralEdgeMap<'_, Option<Edge>>>,
	) {
		let mut vmap = from.ephemeral_vert_map(None);
		for (order, v) in from.verts().enumerate() {
			*vmap.get_mut(v) = Some(order.into());
		}
		let mut emap = from.ephemeral_edge_map(None);
		let mut outs = dense::Domain::default();
		let mut heads = dense::Domain::default();
		for tail in from.verts() {
			outs.insert(heads.len().into());
			for e in from.out_edges(tail) {
				let head = from.head(e);
				let e_prime = heads.len().into();
				*emap.get_mut(e) = Some(e_prime);
				heads.insert(vmap.get(head).expect("head in verts"));
			}
		}
		outs.insert(heads.len().into());
		let g = ImmutableOutAdjacencyList { outs, heads };
		(g, map::Unwrap::new(vmap), map::Unwrap::new(emap))
	}
}

impl<G: OutGraph> From<&G> for ImmutableOutAdjacencyList {
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
		fn isomorphic_from(g: TestGraph) {
			let g_out = crate::DenseOutAdjacencyList::from(&g);
			let (g_prime, vmap, emap) = ImmutableOutAdjacencyList::isomorphic_from(&g_out);
			assert!(g_out.is_isomorphic_with_maps(&g_prime, &vmap, &emap));
		}

		#[test]
		fn invariants(g: TestGraph) {
			let g_out = crate::DenseOutAdjacencyList::from(&g);
			let g_prime = ImmutableOutAdjacencyList::from(&g_out);
			assert_all_out_graph_invariants(&g_prime);
		}
	}
}
