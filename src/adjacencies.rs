//! Module enabling abstraction over in- and out- adjacencies.

use crate::{Digraph, InGraph, OutGraph};

/// Adjacencies of a graph.
pub trait Adjacencies<G: Digraph + ?Sized> {
	/// Iterator over the adjacencies of a vertex.
	type Of<'a>: Clone + Iterator<Item = G::Edge>;

	/// Returns edges adjacent to a vertes.
	fn of<'a>(g: &G, v: G::Vert) -> Self::Of<'_>;

	/// Returns the vertex from which an edge is an adjacency, that is
	/// `e ∈ of(from(e))` should hold.
	fn from(g: &G, e: G::Edge) -> G::Vert;

	/// The endpoint of the edge which it is not from per this definition of
	/// adjacency.
	fn to(g: &G, e: G::Edge) -> G::Vert;
}

/// Out-adjacencies.
pub struct OutAdjacencies;

impl<G: OutGraph + ?Sized> Adjacencies<G> for OutAdjacencies {
	type Of<'a> = G::OutEdges<'a>;

	fn of<'a>(g: &G, v: G::Vert) -> Self::Of<'_> {
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
	type Of<'a> = G::InEdges<'a>;

	fn of<'a>(g: &'a G, v: G::Vert) -> Self::Of<'a> {
		g.in_edges(v)
	}

	fn from(g: &G, e: G::Edge) -> G::Vert {
		g.head(e)
	}

	fn to(g: &G, e: G::Edge) -> G::Vert {
		g.tail(e)
	}
}
