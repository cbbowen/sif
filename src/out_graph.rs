use crate::{Adjacencies, DepthFirst, Digraph, OutAdjacencies, map::Map};
use std::borrow::Borrow;
use std::ops::Add;

/// Represents a directed graph in which the out-adjacencies of vertices can be
/// iterated.
pub trait OutGraph: Digraph {
	/// An iterator over out-adjacencies.
	type OutEdges<'a>: Clone + Iterator<Item = Self::Edge>
	where
		Self: 'a;

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

	/// Returns an iterator that performs a depth-first traverals.
	fn depth_first_out(&self) -> DepthFirst<'_, Self, OutAdjacencies> {
		DepthFirst::new(self)
	}

	/// Returns a map from target vertices to the total cost of the shortest path from the given source. Assumes `d + costs.get(e) >= d` for every edge `e` in the graph and `d: D`.
	fn dijkstra_from<C: Clone, D: Clone + Ord + Add<C, Output = D>>(
		&self,
		costs: &impl Map<Self::Edge, Value = C>,
		source: Self::Vert,
		zero: D,
	) -> Self::EphemeralVertMap<'_, Option<D>> {
		OutAdjacencies::dijkstra(self, costs, source, zero)
	}
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

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{DenseOutAdjacencyList, map::MapMut, model::test_graph::*, test_distance::*};
	use proptest::proptest;

	proptest! {
		#[test]
		fn dijkstra(g: TestGraph) {
			let g = DenseOutAdjacencyList::from(&g);
			let mut costs = g.ephemeral_edge_map(None);
			let mut c = 0;
			for e in g.edges() {
				c = (c + 43) % 101;
				*costs.get_mut(e) = Some(c);
			}
			let costs = crate::map::Unwrap::new(costs);
			// Start from each possible vertex in the graph.
			for source in g.verts() {
				let results = g.dijkstra_from(&|e| TestCost(*costs.get(e).borrow(), e), source, TestDistance::new(0));
				assert_eq!(results.get(source).unwrap().cost, 0);
				assert!(results.get(source).unwrap().pred.is_none());
				// No edge should admit a relaxation.
				for e in g.edges() {
					if let Some(tail_result) = results.get(g.tail(e)) {
						let head_result = results.get(g.head(e)).unwrap();
						assert!(head_result.cost <= tail_result.cost + costs.get(e).borrow());
					}
				}
				// Every vertex's distance should be correct.
				for v in g.verts() {
					if let Some(head_result) = results.get(v) {
						if let Some(e) = head_result.pred {
							let tail_result = results.get(g.tail(e)).unwrap();
							assert!(head_result.cost == tail_result.cost + costs.get(e).borrow());
						}
					}
				}
			}
		}
	}
}
