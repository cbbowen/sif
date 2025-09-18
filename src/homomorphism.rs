use std::borrow::Borrow;

use super::map::{self, Map};
use crate::Digraph;

/// Represents a homomorphism between two graphs. A homomorphism is a mapping from vertices of one graph to vertices of the other and a mapping from edges to edges such that these mappings commute. That is, the head and tail a mapped edge are the mapped head and tail of the original edge.
pub struct Homomorphism<'a, From: Digraph + 'a, To: Digraph + 'a> {
	vert_map: map::Unwrap<From::EphemeralVertMap<'a, Option<To::Vert>>>,
	edge_map: map::Unwrap<From::EphemeralEdgeMap<'a, Option<To::Edge>>>,
}

impl<'a, From: Digraph, To: Digraph> Homomorphism<'a, From, To> {
	pub(crate) fn new(
		vert_map: map::Unwrap<From::EphemeralVertMap<'a, Option<To::Vert>>>,
		edge_map: map::Unwrap<From::EphemeralEdgeMap<'a, Option<To::Edge>>>,
	) -> Self {
		Homomorphism { vert_map, edge_map }
	}

	/// A mapping from vertices of one graph to vertices of another.
	pub fn vert_map(&self) -> &map::Unwrap<From::EphemeralVertMap<'a, Option<To::Vert>>> {
		&self.vert_map
	}

	/// Maps a vertex from one graph to another.
	pub fn map_vert(&self, v: From::Vert) -> To::Vert {
		*self.vert_map.get(v).borrow()
	}

	/// A mapping from edges of one graph to edges of another.
	pub fn edge_map(&self) -> &map::Unwrap<From::EphemeralEdgeMap<'a, Option<To::Edge>>> {
		&self.edge_map
	}

	/// Maps an edge from one graph to another.
	pub fn map_edge(&self, e: From::Edge) -> To::Edge {
		*self.edge_map.get(e).borrow()
	}
}
