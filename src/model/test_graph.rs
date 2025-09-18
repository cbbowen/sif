use super::sparse;
use crate::{
	Digraph, ExactInDegreeDigraph, ExactOutDegreeDigraph, Homomorphism, InGraph, InsertGraph,
	OutGraph,
	map::{Map, MapMut},
};

use std::{borrow::Borrow, collections::HashSet};

use proptest::{
	arbitrary::Arbitrary,
	strategy::{BoxedStrategy, Strategy},
};

type Vert = usize;
type Edge = usize;

/// A simple test graph implementation for use in property tests.
#[derive(Default, Debug, Clone)]
pub struct TestGraph {
	order: Vert,
	edges: Vec<(Vert, Vert)>,
}

impl Digraph for TestGraph {
	type Vert = Vert;
	type Edge = Edge;

	fn endpoints(&self, e: impl std::borrow::Borrow<Self::Edge>) -> (Self::Vert, Self::Vert) {
		self.edges[*e.borrow()]
	}

	type Verts<'a> = std::ops::Range<Vert>;
	fn verts(&self) -> Self::Verts<'_> {
		0..self.order
	}

	type Edges<'a> = std::ops::Range<Edge>;
	fn edges(&self) -> Self::Edges<'_> {
		0..self.edges.len()
	}

	type VertMap<T: Clone> = sparse::Map<Vert, T>;
	fn vert_map<T: Clone>(&self, default: T) -> Self::VertMap<T> {
		sparse::Map::new(default)
	}

	type EdgeMap<T: Clone> = sparse::Map<Edge, T>;
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

impl Arbitrary for TestGraph {
	type Parameters = ();
	type Strategy = BoxedStrategy<TestGraph>;
	fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
		let order_range = 0..=100usize;
		order_range
			.prop_flat_map(|order| {
				let size_range = if order > 0 { 0..=100usize } else { 0..=0 };
				(
					proptest::strategy::Just(order),
					proptest::collection::vec(((0..order), (0..order)), size_range),
				)
			})
			.prop_map(|(order, edges)| TestGraph { order, edges })
			.boxed()
	}
}

impl InsertGraph for TestGraph {
	fn insert_vert(&mut self) -> Self::Vert {
		let v = self.order;
		self.order += 1;
		v
	}

	fn insert_edge(&mut self, tail: Self::Vert, head: Self::Vert) -> Self::Edge {
		let e = self.edges.len();
		self.edges.push((tail, head));
		e
	}
}

fn assert_endpoints_works(g: &impl Digraph) {
	let verts: HashSet<_> = g.verts().collect();
	for e in g.edges() {
		let (tail, head) = g.endpoints(e);
		assert!(verts.contains(&tail), "tail is a valid vertex");
		assert!(verts.contains(&head), "head is a valid vertex");
		assert_eq!(g.tail(e), tail, "tail is the first endpoint");
		assert_eq!(g.head(e), head, "head is the second endpoint");
	}
}

fn assert_verts_works(g: &impl Digraph) {
	let mut set = HashSet::new();
	for v in g.verts() {
		assert!(set.insert(v), "vertices are distinct");
	}
}

fn assert_edges_works(g: &impl Digraph) {
	let mut set = HashSet::new();
	for e in g.edges() {
		assert!(set.insert(e), "edges are distinct");
	}
}

fn assert_ephemeral_vert_map_works(g: &impl Digraph) {
	// Build an identity mapping.
	let mut map = g.ephemeral_vert_map(None);
	for v in g.verts() {
		assert_eq!(*map.get(v).borrow(), None);
		*map.get_mut(v) = Some(v);
	}
	// Verify the set values are retained.
	for v in g.verts() {
		assert_eq!(*map.get(v).borrow(), Some(v));
	}
}

fn assert_ephemeral_edge_map_works(g: &impl Digraph) {
	// Build an identity mapping.
	let mut map = g.ephemeral_edge_map(None);
	for e in g.edges() {
		assert_eq!(*map.get(e).borrow(), None);
		*map.get_mut(e) = Some(e);
	}
	// Verify the set values are retained.
	for e in g.edges() {
		assert_eq!(*map.get(e).borrow(), Some(e));
	}
}

fn assert_out_edges_works(g: &impl OutGraph) {
	let mut set = std::collections::HashSet::new();
	for v in g.verts() {
		for e in g.out_edges(v) {
			assert_eq!(g.tail(e), v, "out-edges of vertex matching tails");
			assert!(set.insert(e), "out-edges are distinct");
		}
	}
	for e in g.edges() {
		assert!(set.contains(&e));
	}
}

fn assert_in_edges_works(g: &impl InGraph) {
	let mut set = std::collections::HashSet::new();
	for v in g.verts() {
		for e in g.in_edges(v) {
			assert_eq!(g.head(e), v);
			assert!(set.insert(e));
		}
	}
	for e in g.edges() {
		assert!(set.contains(&e));
	}
}

fn assert_out_graph_invariants(g: &impl OutGraph) {
	assert_out_edges_works(g);
}

fn assert_in_graph_invariants(g: &impl InGraph) {
	assert_in_edges_works(g);
}

/// Asserts all invariants of a directed graph.
pub fn assert_all_digraph_invariants(g: &impl Digraph) {
	assert_endpoints_works(g);
	assert_verts_works(g);
	assert_edges_works(g);
	assert_ephemeral_vert_map_works(g);
	assert_ephemeral_edge_map_works(g);
}

/// Asserts all invariants of an out-graph.
pub fn assert_all_out_graph_invariants(g: &impl OutGraph) {
	assert_all_digraph_invariants(g);
	assert_out_graph_invariants(g);
}

/// Asserts all invariants of an in-graph.
pub fn assert_all_in_graph_invariants(g: &impl InGraph) {
	assert_all_digraph_invariants(g);
	assert_in_graph_invariants(g);
}

/// Asserts all invariants of a bi-graph.
pub fn assert_all_bi_graph_invariants(g: &(impl OutGraph + InGraph)) {
	assert_all_digraph_invariants(g);
	assert_out_graph_invariants(g);
	assert_in_graph_invariants(g);
}

/// Asserts that a vert map works correctly.
pub fn assert_vert_map_works(mut g: impl InsertGraph) {
	// Build an identity mapping.
	let mut map = g.vert_map(None);
	for v in g.verts() {
		assert_eq!(*map.get(v).borrow(), None);
		*map.get_mut(v) = Some(v);
	}
	// Modify the graph.
	let v_prime = g.insert_vert();
	assert_eq!(*map.get(v_prime).borrow(), None);
	*map.get_mut(v_prime) = Some(v_prime);
	// Verify the set values are retained.
	for v in g.verts() {
		assert_eq!(*map.get(v).borrow(), Some(v));
	}
}

/// Asserts that an edge map works correctly.
pub fn assert_edge_map_works(mut g: impl InsertGraph) {
	// Build an identity mapping.
	let mut map = g.edge_map(None);
	for e in g.edges() {
		assert_eq!(*map.get(e).borrow(), None);
		*map.get_mut(e) = Some(e);
	}
	// Modify the graph.
	let v_prime = g.insert_vert();
	let e_prime = g.insert_edge(v_prime, v_prime);
	assert_eq!(*map.get(e_prime).borrow(), None);
	*map.get_mut(e_prime) = Some(e_prime);
	// Verify the set values are retained.
	for e in g.edges() {
		assert_eq!(*map.get(e).borrow(), Some(e));
	}
}

/// Asserts that the out-degree function works correctly.
pub fn assert_out_degree_works(g: &impl ExactOutDegreeDigraph) {
	for v in g.verts() {
		assert_eq!(g.out_degree(v), g.out_edges(v).count());
	}
}

/// Asserts that the in-degree function works correctly.
pub fn assert_in_degree_works(g: &impl ExactInDegreeDigraph) {
	for v in g.verts() {
		assert_eq!(g.in_degree(v), g.in_edges(v).count());
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use proptest::proptest;

	// Basic tests that the test graph itself works.
	proptest! {
		#[test]
		fn ismorphic_from(g: TestGraph) {
			let (g_prime, homomorphism) = TestGraph::isomorphic_from(&g);
			assert!(g.is_isomorphic_with_maps(&g_prime, homomorphism.vert_map(), homomorphism.edge_map()));
		}

		#[test]
		fn vert_map(g: TestGraph) {
			assert_vert_map_works(g);
		}

		#[test]
		fn ephemeral_vert_map(g: TestGraph) {
			assert_ephemeral_vert_map_works(&g);
		}

		#[test]
		fn edge_map(g: TestGraph) {
			assert_edge_map_works(g);
		}

		#[test]
		fn ephemeral_edge_map(g: TestGraph) {
			assert_ephemeral_edge_map_works(&g);
		}
	}
}
