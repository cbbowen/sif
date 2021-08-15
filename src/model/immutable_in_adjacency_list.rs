//! Module implementing an immutable in-adjacency list.

use std::borrow::Borrow;

use itertools::{Itertools, MapInto};
use std::ops::Range;

use crate::{
	map::{self, Map, MapMut},
	Digraph, Homomorphism, InGraph,
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
pub type InEdges<'a> = MapInto<Range<usize>, Edge>;

#[derive(Debug)]
/// Immutable in-adjacency list directed graph representation.
pub struct ImmutableInAdjacencyList {
	// Mapping from vertices to the first edge with it as the tail. This also
	// has an extra element mapped to the size of the graph to facilitate
	// lookups.
	ins: dense::Domain<Vert, Edge>,
	// Mapping from edges to their tails.
	tails: dense::Domain<Edge, Vert>,
}

impl Digraph for ImmutableInAdjacencyList {
	type Vert = Vert;
	type Edge = Edge;

	fn endpoints(&self, e: impl Borrow<Self::Edge>) -> (Self::Vert, Self::Vert) {
		let e = e.borrow();
		(self.tail(e), self.head(e))
	}

	fn tail(&self, e: impl Borrow<Self::Edge>) -> Self::Vert {
		self.tails[*e.borrow()]
	}

	fn head(&self, e: impl Borrow<Self::Edge>) -> Self::Vert {
		let e = e.borrow();
		(self.ins.values().partition_point(|q| q <= e) - 1).into()
	}

	type Verts<'a> = Verts<'a>;
	fn verts(&self) -> Self::Verts<'_> {
		(0..self.ins.len() - 1).map_into::<Vert>()
	}

	type Edges<'a> = Edges<'a>;
	fn edges(&self) -> Self::Edges<'_> {
		self.tails.keys()
	}

	type VertMap<T: Clone> = VertMap<T>;
	fn vert_map<T: Clone>(&self, default: T) -> Self::VertMap<T> {
		VertMap::with_capacity(default, self.ins.len() - 1)
	}

	type EdgeMap<T: Clone> = EdgeMap<T>;
	fn edge_map<T: Clone>(&self, default: T) -> Self::EdgeMap<T> {
		EdgeMap::with_capacity(default, self.tails.len())
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

impl InGraph for ImmutableInAdjacencyList {
	type InEdges<'a> = InEdges<'a>;
	fn in_edges(&self, v: impl Borrow<Self::Vert>) -> Self::InEdges<'_> {
		let v = v.borrow();
		let start = self.ins[*v].index();
		let end = self.ins[(v.index() + 1).into()].index();
		(start..end).map_into::<Edge>()
	}
}

impl ImmutableInAdjacencyList {
	/// Constructs a graph isomorphic to the given graph and returns it along with
	/// mappings from the given graph's vertices and edges to those in the new
	/// graph.
	fn isomorphic_from<G: InGraph>(from: &G) -> (Self, Homomorphism<'_, G, Self>) {
		let mut vmap = from.ephemeral_vert_map(None);
		for (order, v) in from.verts().enumerate() {
			*vmap.get_mut(v) = Some(order.into());
		}
		let mut emap = from.ephemeral_edge_map(None);
		let mut ins = dense::Domain::default();
		let mut tails = dense::Domain::default();
		for head in from.verts() {
			ins.insert(tails.len().into());
			for e in from.in_edges(head) {
				let tail = from.tail(e);
				let e_prime = tails.len().into();
				*emap.get_mut(e) = Some(e_prime);
				tails.insert(vmap.get(tail).expect("tail in verts"));
			}
		}
		ins.insert(tails.len().into());
		let g = ImmutableInAdjacencyList { ins, tails };
		(
			g,
			Homomorphism::new(map::Unwrap::new(vmap), map::Unwrap::new(emap)),
		)
	}
}

impl<G: InGraph> From<&G> for ImmutableInAdjacencyList {
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
			let g_in = crate::DenseInAdjacencyList::from(&g);
			let (g_prime, homomorphism) = ImmutableInAdjacencyList::isomorphic_from(&g_in);
			assert!(g_in.is_isomorphic_with_maps(&g_prime, homomorphism.vert_map(), homomorphism.edge_map()));
		}

		#[test]
		fn invariants(g: TestGraph) {
			let g_in = crate::DenseInAdjacencyList::from(&g);
			let g_prime = ImmutableInAdjacencyList::from(&g_in);
			assert_all_in_graph_invariants(&g_prime);
		}
	}
}
