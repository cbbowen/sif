use crate::Digraph;
use std::borrow::Borrow;

/// Represents a directed graph in which the out-adjacencies of vertices can be
/// iterated.
pub trait OutGraph: Digraph {
	/// An iterator over out-adjacencies.
	type OutEdges<'a>: Clone + Iterator<Item = Self::Edge>;

	/// Returns an iterator over the out-adjacencies of a vertex, that is, the
	/// edges with a given tail.
	///
	/// # Examples
	/// ```
	/// # use sif::*;
	/// # let mut g = DenseOutAdjacencyList::new();
	/// # let tail = g.insert_vert();
	/// # let head = g.insert_vert();
	/// let e = g.insert_edge(tail, head);
	/// assert!(g.out_edges(tail).any(|d| d == e));
	/// ```
	fn out_edges(&self, v: impl Borrow<Self::Vert>) -> Self::OutEdges<'_>;
}

/// Represents a directed graph in which the out-degree of vertices is known.
pub trait ExactOutDegreeDigraph: OutGraph {
	/// Returns the out-degree of a vertex, that is, the number of
	/// out-adjacencies.
	fn out_degree(&self, v: impl Borrow<Self::Vert>) -> usize;
}
impl<G: OutGraph> ExactOutDegreeDigraph for G
where
	for<'a> G::OutEdges<'a>: ExactSizeIterator,
{
	fn out_degree(&self, v: impl Borrow<Self::Vert>) -> usize {
		self.out_edges(v).len()
	}
}
