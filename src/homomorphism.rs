use std::borrow::Borrow;

use super::map::Map;
use crate::Digraph;

/// Represents a homomorphism between two graphs. A homomorphism is a mapping from vertices of one graph to vertices of the other and a mapping from edges to edges such that these mappings commute. That is, the head and tail a mapped edge are the mapped head and tail of the original edge.
pub trait Homomorphism<From: Digraph, To: Digraph> {
	/// A mapping from vertices of one graph to vertices of another.
	fn vert_map(&self) -> &impl Map<From::Vert, Value = To::Vert>;

	/// Maps a vertex from one graph to another.
	fn map_vert(&self, v: From::Vert) -> To::Vert {
		*self.vert_map().get(v).borrow()
	}

	/// A mapping from edges of one graph to edges of another.
	fn edge_map(&self) -> &impl Map<From::Edge, Value = To::Edge>;

	/// Maps an edge from one graph to another.
	fn map_edge(&self, e: From::Edge) -> To::Edge {
		*self.edge_map().get(e).borrow()
	}
}
