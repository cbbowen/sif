use crate::Digraph;
use std::borrow::Borrow;

/// Represents a directed graph in which the in-adjacencies of vertices can be
/// iterated.
pub trait InGraph: Digraph {
	/// An iterator over in-adjacencies.
	type InEdges<'a>: Clone + Iterator<Item = Self::Edge>;

	/// Returns an iterator over the in-adjacencies of a vertex, that is, the
	/// edges with a given head.
	///
	/// # Examples
	/// ```
	/// # use sif::*;
	/// # let mut g = DenseInAdjacencyList::new();
	/// # let tail = g.insert_vert();
	/// # let head = g.insert_vert();
	/// let e = g.insert_edge(tail, head);
	/// assert!(g.in_edges(head).any(|d| d == e));
	/// ```
	fn in_edges(&self, v: impl Borrow<Self::Vert>) -> Self::InEdges<'_>;
}

/// Represents a directed graph in which the in-degree of vertices is known.
pub trait ExactInDegreeDigraph: InGraph {
	/// Returns the in-degree of a vertex, that is, the number of in-adjacencies.
	fn in_degree(&self, v: impl Borrow<Self::Vert>) -> usize;
}
impl<G: InGraph> ExactInDegreeDigraph for G
where
	for<'a> G::InEdges<'a>: ExactSizeIterator,
{
	fn in_degree(&self, v: impl Borrow<Self::Vert>) -> usize {
		self.in_edges(v).len()
	}
}
