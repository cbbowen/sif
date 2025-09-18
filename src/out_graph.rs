use super::map::{Map, MapMut};
use crate::BinaryHeap;
use crate::{adjacencies::OutAdjacencies, DepthFirst, Digraph};
use std::borrow::Borrow;
use std::ops::Add;

/// Represents a directed graph in which the out-adjacencies of vertices can be
/// iterated.
pub trait OutGraph: Digraph {
	/// An iterator over out-adjacencies.
	type OutEdges<'a>: Clone + Iterator<Item = Self::Edge> where Self: 'a;

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
	fn dijkstra<C: Clone, D: Clone + Ord + Add<C, Output = D>>(
		&self,
		costs: &impl Map<Self::Edge, Value = C>,
		source: Self::Vert,
		zero: D,
	) -> Self::EphemeralVertMap<'_, Option<D>>
	{
		let mut queue = BinaryHeap::new(self.ephemeral_vert_map(None));
		let mut distances = self.ephemeral_vert_map(None);
		queue.try_decrease(source, zero);
		while let Some((v, d)) = queue.pop() {
			*distances.get_mut(v) = Some(d.clone());
			for e in self.out_edges(v) {
				let u = self.head(e);
				if distances.get(u).borrow().is_none() {
					queue.try_decrease(u, d.clone() + costs.get(e).borrow().clone());
				}
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
	struct TestDistance<C, E> {
		cost: C,
		pred: Option<E>,
	}

	impl<C: PartialEq, E> PartialEq for TestDistance<C, E> {
		fn eq(&self, other: &Self) -> bool {
			self.cost.eq(&other.cost)
		}
	}

	impl<C: Eq, E> Eq for TestDistance<C, E> {}

	impl<C, E> TestDistance<C, E> {
		fn new(cost: C) -> Self {
			TestDistance { cost, pred: None }
		}
	}

	impl<C: PartialOrd, E> PartialOrd for TestDistance<C, E> {
		fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
			self.cost.partial_cmp(&other.cost)
		}
	}

	impl<C: Ord, E> Ord for TestDistance<C, E> {
		fn cmp(&self, other: &Self) -> std::cmp::Ordering {
			self.cost.cmp(&other.cost)
		}
	}

	impl<C: Add<Output = C>, E> Add<TestCost<C, E>> for TestDistance<C, E> {
		type Output = Self;
		fn add(self, rhs: TestCost<C, E>) -> Self::Output {
			TestDistance {
				cost: self.cost + rhs.0,
				pred: Some(rhs.1),
			}
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
				*costs.get_mut(e) = Some(c);
			}
			let costs = crate::map::Unwrap::new(costs);
			// Start from each possible vertex in the graph.
			for source in g.verts() {
				let results = g.dijkstra(&|e| TestCost(*costs.get(e).borrow(), e), source, TestDistance::new(0));
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
