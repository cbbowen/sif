//! Module enabling abstraction over in- and out- adjacencies.

use crate::{
	BinaryHeap, Digraph, InGraph, OutGraph,
	map::{Map, MapMut},
};
use std::borrow::Borrow;
use std::ops::Add;

/// Adjacencies of a graph.
pub trait Adjacencies<G: Digraph + ?Sized> {
	/// Iterator over the adjacencies of a vertex.
	type Of<'a>: Clone + Iterator<Item = G::Edge>
	where
		G: 'a;

	/// Returns edges adjacent to a vertes.
	fn of(g: &G, v: G::Vert) -> Self::Of<'_>;

	/// Returns the vertex from which an edge is an adjacency, that is
	/// `e ∈ of(from(e))` should hold.
	fn from(g: &G, e: G::Edge) -> G::Vert;

	/// The endpoint of the edge which it is not from per this definition of
	/// adjacency.
	fn to(g: &G, e: G::Edge) -> G::Vert;

	/// Returns a map from source/target vertices to the total cost of the shortest path from the given source/target. Assumes `d + costs.get(e) >= d` for every edge `e` in the graph and `d: D`.
	fn dijkstra<'g, C: Clone, D: Clone + Ord + Add<C, Output = D>>(
		g: &'g G,
		costs: &impl Map<G::Edge, Value = C>,
		v0: G::Vert,
		zero: D,
	) -> G::EphemeralVertMap<'g, Option<D>> {
		let mut queue = BinaryHeap::new(g.ephemeral_vert_map(None));
		let mut distances = g.ephemeral_vert_map(None);
		queue.try_decrease(v0, zero);
		while let Some((v, d)) = queue.pop() {
			*distances.get_mut(v) = Some(d.clone());
			for e in Self::of(g, v) {
				let u = Self::to(g, e);
				if let Some(u_distance) = distances.get(u).borrow() {
					debug_assert!(
						u_distance <= &(d.clone() + costs.get(e).borrow().clone()),
						"negative cost edge"
					);
				} else {
					queue.try_decrease(u, d.clone() + costs.get(e).borrow().clone());
				}
			}
		}
		distances
	}
}

/// Out-adjacencies.
pub struct OutAdjacencies;

impl<G: OutGraph + ?Sized> Adjacencies<G> for OutAdjacencies {
	type Of<'a>
		= G::OutEdges<'a>
	where
		G: 'a;

	fn of(g: &G, v: G::Vert) -> Self::Of<'_> {
		g.out_edges(v)
	}

	fn from(g: &G, e: G::Edge) -> G::Vert {
		g.tail(e)
	}

	fn to(g: &G, e: G::Edge) -> G::Vert {
		g.head(e)
	}
}

/// Out-adjacencies.
pub struct InAdjacencies;

impl<G: InGraph + ?Sized> Adjacencies<G> for InAdjacencies {
	type Of<'a>
		= G::InEdges<'a>
	where
		G: 'a;

	fn of(g: &G, v: G::Vert) -> Self::Of<'_> {
		g.in_edges(v)
	}

	fn from(g: &G, e: G::Edge) -> G::Vert {
		g.head(e)
	}

	fn to(g: &G, e: G::Edge) -> G::Vert {
		g.tail(e)
	}
}
