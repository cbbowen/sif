use super::map::{Map, MapMut};
use crate::BinaryHeap;
use crate::{adjacencies::OutAdjacencies, DepthFirst, Digraph};
use std::borrow::Borrow;
use std::ops::Add;

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

	/// Returns an iterator that performs a depth-first traverals.
	fn depth_first_out(&self) -> DepthFirst<'_, Self, OutAdjacencies> {
		DepthFirst::new(self)
	}

	/// Returns a map from target vertices to the total cost of the shortest path from the given source and the last edge in that path. Assumes `d + costs.get(e) >= d` for every edge `e` in the graph and `d: D`.
	fn dijkstra<C: Clone, D: Clone + Ord>(
		&self,
		costs: &impl Map<Self::Edge, C>,
		source: Self::Vert,
		zero: D,
	) -> Self::EphemeralVertMap<'_, Option<D>>
	where
		D: Add<C, Output = D>,
	{
		let mut queue = BinaryHeap::new(self.ephemeral_vert_map(None));
		let mut distances = self.ephemeral_vert_map(None);
		queue.try_decrease(source, zero);
		while let Some((v, d)) = queue.pop() {
			*distances.get_mut(v) = Some(d.clone());
			for e in self.out_edges(v) {
				let u = self.head(e);
				if distances.get(u).is_some() { continue; }
				queue.try_decrease(u, d.clone() + costs.get(e).clone());
			}
		}
		distances
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
	use crate::{model::test_graph::*, DenseOutAdjacencyList};
	use proptest::proptest;

	#[derive(Debug, Clone, Copy)]
	struct TestCost<C, E>(C, E);

	#[derive(Debug, Clone, Copy)]
	struct TestDistance<C, E>(C, Option<E>);

	impl<C: PartialEq, E> PartialEq for TestDistance<C, E> {
		fn eq(&self, other: &Self) -> bool {
			self.0.eq(&other.0)
		}
	}

	impl<C: Eq, E> Eq for TestDistance<C, E> {}

	impl<C: PartialOrd, E> PartialOrd for TestDistance<C, E> {
		fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
			self.0.partial_cmp(&other.0)
		}
	}

	impl<C: Ord, E> Ord for TestDistance<C, E> {
		fn cmp(&self, other: &Self) -> std::cmp::Ordering {
			self.0.cmp(&other.0)
		}
	}

	impl<C: Add<Output=C>, E> Add<TestCost<C, E>> for TestDistance<C, E> {
		type Output = Self;
		fn add(self, rhs: TestCost<C, E>) -> Self::Output {
			TestDistance(self.0 + rhs.0, Some(rhs.1))
		}
	}

	proptest! {
		#[test]
		fn dijkstra(g: TestGraph) {
			let g = DenseOutAdjacencyList::from(&g);
			let mut costs = g.ephemeral_edge_map(None);
			let mut c = 0;
			for e in g.edges() {
				c = (c + 43) % 101;
				*costs.get_mut(e) = Some(TestCost(c, e));
			}
			let costs = crate::map::Unwrap::new(costs);
			// Start from each possible vertex in the graph.
			for source in g.verts() {
				let result = g.dijkstra(&costs, source, TestDistance(0, None));
				assert_eq!(result.get(source).unwrap().0, 0);
				assert!(result.get(source).unwrap().1.is_none());
				// No edge should admit a relaxation.
				for e in g.edges() {
					let tail_result = result.get(g.tail(e));
					let head_result = result.get(g.head(e));
					if let Some(TestDistance(d, _)) = tail_result {
						assert!(head_result.unwrap().0 <= d + costs.get(e).0);
					}
				}
				// Every vertex's distance should be correct.
				for v in g.verts() {
					if let Some(TestDistance(d, Some(e))) = *result.get(v) {
						let tail_result = result.get(g.tail(e)).unwrap();
						assert!(d == tail_result.0 + costs.get(e).0);
					}
				}
			}
		}
	}
}
