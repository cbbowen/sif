//! Module implementing the tensor product of graphs.

#![allow(type_alias_bounds)]

use std::borrow::Borrow;

use super::sparse;

use itertools::{Itertools, Product};

use crate::{Digraph, InGraph, OutGraph};

#[allow(missing_docs)]
pub type Vert<G0: Digraph, G1: Digraph> = (G0::Vert, G1::Vert);
#[allow(missing_docs)]
pub type Edge<G0: Digraph, G1: Digraph> = (G0::Edge, G1::Edge);
#[allow(missing_docs)]
pub type Verts<'a, G0: Digraph, G1: Digraph> = Product<G0::Verts<'a>, G1::Verts<'a>>;
#[allow(missing_docs)]
pub type Edges<'a, G0: Digraph, G1: Digraph> = Product<G0::Edges<'a>, G1::Edges<'a>>;

// TODO: Ideally, we would like to leverage density when both factor graphs have
// dense mappings.
#[allow(missing_docs)]
pub type VertMap<G0: Digraph, G1: Digraph, T> = sparse::Map<Vert<G0, G1>, T>;
#[allow(missing_docs)]
pub type EdgeMap<G0: Digraph, G1: Digraph, T> = sparse::Map<Edge<G0, G1>, T>;

#[allow(missing_docs)]
pub type OutEdges<'a, G0: OutGraph, G1: OutGraph> = Product<G0::OutEdges<'a>, G1::OutEdges<'a>>;
#[allow(missing_docs)]
pub type InEdges<'a, G0: InGraph, G1: InGraph> = Product<G0::InEdges<'a>, G1::InEdges<'a>>;

impl<G0: Digraph, G1: Digraph> Digraph for (G0, G1) {
	type Vert = Vert<G0, G1>;
	type Edge = Edge<G0, G1>;

	fn endpoints(&self, e: impl Borrow<Self::Edge>) -> (Self::Vert, Self::Vert) {
		let (e0, e1) = e.borrow();
		let (t0, h0) = self.0.endpoints(e0);
		let (t1, h1) = self.1.endpoints(e1);
		((t0, t1), (h0, h1))
	}

	fn tail(&self, e: impl Borrow<Self::Edge>) -> Self::Vert {
		let (e0, e1) = e.borrow();
		(self.0.tail(e0), self.1.tail(e1))
	}

	fn head(&self, e: impl Borrow<Self::Edge>) -> Self::Vert {
		let (e0, e1) = e.borrow();
		(self.0.head(e0), self.1.head(e1))
	}

	type Verts<'a> = Verts<'a, G0, G1>;
	fn verts(&self) -> Self::Verts<'_> {
		self.0.verts().cartesian_product(self.1.verts())
	}

	type Edges<'a> = Edges<'a, G0, G1>;
	fn edges(&self) -> Self::Edges<'_> {
		self.0.edges().cartesian_product(self.1.edges())
	}

	type VertMap<T: Clone> = VertMap<G0, G1, T>;
	fn vert_map<T: Clone>(&self, default: T) -> Self::VertMap<T> {
		sparse::Map::new(default)
	}

	type EdgeMap<T: Clone> = EdgeMap<G0, G1, T>;
	fn edge_map<T: Clone>(&self, default: T) -> Self::EdgeMap<T> {
		sparse::Map::new(default)
	}

	fn ephemeral_vert_map<T: Clone>(&self, default: T) -> Self::EphemeralVertMap<'_, T> {
		self.vert_map(default)
	}

	fn ephemeral_edge_map<T: Clone>(&self, default: T) -> Self::EphemeralEdgeMap<'_, T> {
		self.edge_map(default)
	}
}

impl<G0: OutGraph, G1: OutGraph> OutGraph for (G0, G1) {
	type OutEdges<'a> = OutEdges<'a, G0, G1>;

	fn out_edges(&self, v: impl Borrow<Self::Vert>) -> Self::OutEdges<'_> {
		let v = v.borrow();
		self
			.0
			.out_edges(v.0)
			.cartesian_product(self.1.out_edges(v.1))
	}
}

impl<G0: InGraph, G1: InGraph> InGraph for (G0, G1) {
	type InEdges<'a> = InEdges<'a, G0, G1>;

	fn in_edges(&self, v: impl Borrow<Self::Vert>) -> Self::InEdges<'_> {
		let v = v.borrow();
		self.0.in_edges(v.0).cartesian_product(self.1.in_edges(v.1))
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::model::test_graph::*;
	use proptest::proptest;
	proptest! {
		#[test]
		fn order(g0: TestGraph, g1: TestGraph) {
			let g = (g0, g1);
			let mut order = 0usize;
			for _v in g.verts() {
				order += 1;
			}
			assert_eq!(order, g.0.verts().len() * g.1.verts().len());
		}

		#[test]
		fn size(g0: TestGraph, g1: TestGraph) {
			let g = (g0, g1);
			let mut size = 0usize;
			for _e in g.edges() {
				size += 1;
			}
			assert_eq!(size, g.0.edges().len() * g.1.edges().len());
		}

		#[test]
		fn invariants(g0: TestGraph, g1: TestGraph) {
			assert_all_digraph_invariants(&(g0, g1));
		}

		#[test]
		fn bi_invariants(g0: TestGraph, g1: TestGraph) {
			let g0_prime = crate::DenseBiAdjacencyList::from(&g0);
			let g1_prime = crate::DenseBiAdjacencyList::from(&g1);
			assert_all_bi_graph_invariants(&(g0_prime, g1_prime));
		}
	}
}
