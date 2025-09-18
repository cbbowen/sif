use std::borrow::Borrow;

use crate::{
	Digraph, Homomorphism,
	map::{Map, Unwrap},
};

pub struct IsomorphicFrom<'a, From: Digraph + 'a, To: Digraph + 'a> {
	vert_map: Unwrap<From::EphemeralVertMap<'a, Option<To::Vert>>>,
	edge_map: Unwrap<From::EphemeralEdgeMap<'a, Option<To::Edge>>>,
}

impl<'a, From: Digraph, To: Digraph> IsomorphicFrom<'a, From, To> {
	pub(crate) fn new(
		vert_map: From::EphemeralVertMap<'a, Option<To::Vert>>,
		edge_map: From::EphemeralEdgeMap<'a, Option<To::Edge>>,
	) -> Self {
		let vert_map = Unwrap::new(vert_map);
		let edge_map = Unwrap::new(edge_map);
		IsomorphicFrom { vert_map, edge_map }
	}
}

impl<'a, From: Digraph, To: Digraph> Homomorphism<From, To> for IsomorphicFrom<'a, From, To> {
	/// A mapping from vertices of one graph to vertices of another.
	fn vert_map(&self) -> &impl Map<From::Vert, Value = To::Vert> {
		&self.vert_map
	}

	/// Maps a vertex from one graph to another.
	fn map_vert(&self, v: From::Vert) -> To::Vert {
		*self.vert_map.get(v).borrow()
	}

	/// A mapping from edges of one graph to edges of another.
	fn edge_map(&self) -> &impl Map<From::Edge, Value = To::Edge> {
		&self.edge_map
	}

	/// Maps an edge from one graph to another.
	fn map_edge(&self, e: From::Edge) -> To::Edge {
		*self.edge_map.get(e).borrow()
	}
}
