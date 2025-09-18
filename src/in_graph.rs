use crate::{Adjacencies, DepthFirst, Digraph, InAdjacencies, map::Map};
use std::borrow::Borrow;
use std::ops::Add;

/// Represents a directed graph in which the in-adjacencies of vertices can be
/// iterated.
pub trait InGraph: Digraph {
	/// An iterator over in-adjacencies.
	type InEdges<'a>: Clone + Iterator<Item = Self::Edge>
	where
		Self: 'a;

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

	/// Returns an iterator that performs a depth-first traverals.
	fn depth_first_in(&self) -> DepthFirst<'_, Self, InAdjacencies> {
		DepthFirst::new(self)
	}

	/// Returns a map from source vertices to the total cost of the shortest path from the given target. Assumes `d + costs.get(e) >= d` for every edge `e` in the graph and `d: D`.
	fn dijkstra_to<C: Clone, D: Clone + Ord + Add<C, Output = D>>(
		&self,
		costs: &impl Map<Self::Edge, Value = C>,
		source: Self::Vert,
		zero: D,
	) -> Self::EphemeralVertMap<'_, Option<D>> {
		InAdjacencies::dijkstra(self, costs, source, zero)
	}
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

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{DenseInAdjacencyList, map::MapMut, model::test_graph::*, test_distance::*};
	use proptest::proptest;

	proptest! {
		#[test]
		fn dijkstra(g: TestGraph) {
			let g = DenseInAdjacencyList::from(&g);
			let mut costs = g.ephemeral_edge_map(None);
			let mut c = 0;
			for e in g.edges() {
				c = (c + 43) % 101;
				*costs.get_mut(e) = Some(c);
			}
			let costs = crate::map::Unwrap::new(costs);
			// Start from each possible vertex in the graph.
			for source in g.verts() {
				let results = g.dijkstra_to(&|e| TestCost(*costs.get(e).borrow(), e), source, TestDistance::new(0));
				assert_eq!(results.get(source).unwrap().cost, 0);
				assert!(results.get(source).unwrap().pred.is_none());
				// No edge should admit a relaxation.
				for e in g.edges() {
					if let Some(head_result) = results.get(g.head(e)) {
						let tail_result = results.get(g.tail(e)).unwrap();
						assert!(tail_result.cost <= head_result.cost + costs.get(e).borrow());
					}
				}
				// Every vertex's distance should be correct.
				for v in g.verts() {
					if let Some(tail_result) = results.get(v) {
						if let Some(e) = tail_result.pred {
							let head_result = results.get(g.head(e)).unwrap();
							assert!(tail_result.cost == head_result.cost + costs.get(e).borrow());
						}
					}
				}
			}
		}
	}
}
