use std::borrow::Borrow;

use crate::{Adjacencies, Digraph, Map, MapMut};

/// Step of a depth-first graph traversal.
#[non_exhaustive]
pub enum DepthFirstEvent<G: Digraph + ?Sized> {
	/// Start of a new tree.
	StartTree(G::Vert),
	/// End of the current tree.
	EndTree,
	/// Opened a new edge, exporing from its head.
	OpenEdge(G::Edge),
	/// Found an edge to a cousin.
	CrossEdge(G::Edge),
	/// Found an edge to an ancestor.
	BackEdge(G::Edge),
	/// Closed an edge, returning to its tail.
	CloseEdge(G::Edge),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum DepthFirstVisited {
	No,
	Open,
	Closed,
}

impl Default for DepthFirstVisited {
	fn default() -> Self {
		DepthFirstVisited::No
	}
}

/// Iterator that performs a depth-first graph traversal.
pub struct DepthFirst<'a, G: Digraph + ?Sized, Adj: Adjacencies<G>> {
	graph: &'a G,
	visited: G::EphemeralVertMap<'a, DepthFirstVisited>,
	stack: Vec<(Option<G::Edge>, Adj::Of<'a>)>,
	vert_iter: G::Verts<'a>,
}

impl<'a, G: Digraph + ?Sized, Adj: Adjacencies<G>> DepthFirst<'a, G, Adj> {
	/// Constructs a new depth-first search over a graph.
	pub fn new(g: &'a G) -> Self {
		let (size_hint, _) = g.edges().size_hint();
		DepthFirst {
			graph: g,
			visited: g.default_ephemeral_vert_map(),
			stack: Vec::with_capacity(size_hint),
			vert_iter: g.verts(),
		}
	}
}

impl<'a, G: Digraph + ?Sized, Adj: Adjacencies<G>> Iterator for DepthFirst<'a, G, Adj> {
	type Item = DepthFirstEvent<G>;

	fn next(&mut self) -> Option<Self::Item> {
		use DepthFirstEvent::*;
		use DepthFirstVisited::*;
		let visited = &mut self.visited;
		if let Some(frame) = self.stack.last_mut() {
			if let Some(e) = frame.1.next() {
				let v = Adj::from(self.graph, e);
				let v_visited = *visited.get(v).borrow();
				match v_visited {
					No => {
						*visited.get_mut(v) = Open;
						self.stack.push((Some(e), Adj::of(self.graph, v)));
						Some(OpenEdge(e))
					}
					Open => Some(BackEdge(e)),
					Closed => Some(CrossEdge(e)),
				}
			} else if let (Some(e), _) = self.stack.pop().unwrap() {
				let v = Adj::from(self.graph, e);
				*visited.get_mut(v) = Closed;
				Some(CloseEdge(e))
			} else {
				Some(EndTree)
			}
		} else {
			let v = self.vert_iter.find(|v| *visited.get(*v).borrow() == No)?;
			*visited.get_mut(v) = Open;
			self.stack.push((None, Adj::of(self.graph, v)));
			Some(StartTree(v))
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::model::test_graph::*;
	use proptest::proptest;
	use std::collections::HashSet;

	proptest! {
		#[test]
		fn depth_first_out(g_test: TestGraph) {
			use crate::OutGraph;
			let g = crate::DenseOutAdjacencyList::from(&g_test);
			use DepthFirstEvent::*;
			let mut vs = HashSet::new();
			let mut es = HashSet::new();
			let mut stack = Vec::new();
			for event in g.depth_first_out() {
				match event {
					StartTree(v) => {
						assert!(vs.insert(v));
						stack.push(v);
					},
					EndTree => {
						stack.pop().unwrap();
					}
					OpenEdge(e) => {
						assert!(es.insert(e));
						let v = g.head(e);
						assert!(vs.insert(v));
						stack.push(v);
					},
					CrossEdge(e) => {
						assert_eq!(g.tail(e), *stack.last().unwrap());
						assert!(es.insert(e));
					},
					BackEdge(e) => {
						assert_eq!(g.tail(e), *stack.last().unwrap());
						assert!(es.insert(e));
					},
					CloseEdge(e) => {
						assert_eq!(g.head(e), stack.pop().unwrap());
						assert_eq!(g.tail(e), *stack.last().unwrap());
					}
				}
			}
			// Every vertex and edge should have been visited.
			assert_eq!(g.verts().collect::<HashSet<_>>(), vs);
			assert_eq!(g.edges().collect::<HashSet<_>>(), es);
			assert_eq!(stack.len(), 0);
		}

		#[test]
		fn depth_first_in(g_test: TestGraph) {
			use crate::InGraph;
			let g = crate::DenseInAdjacencyList::from(&g_test);
			use DepthFirstEvent::*;
			let mut vs = HashSet::new();
			let mut es = HashSet::new();
			let mut stack = Vec::new();
			for event in g.depth_first_in() {
				match event {
					StartTree(v) => {
						assert!(vs.insert(v));
						stack.push(v);
					},
					EndTree => {
						stack.pop().unwrap();
					}
					OpenEdge(e) => {
						assert!(es.insert(e));
						let v = g.tail(e);
						assert!(vs.insert(v));
						stack.push(v);
					},
					CrossEdge(e) => {
						assert_eq!(g.head(e), *stack.last().unwrap());
						assert!(es.insert(e));
					},
					BackEdge(e) => {
						assert_eq!(g.head(e), *stack.last().unwrap());
						assert!(es.insert(e));
					},
					CloseEdge(e) => {
						assert_eq!(g.tail(e), stack.pop().unwrap());
						assert_eq!(g.head(e), *stack.last().unwrap());
					}
				}
			}
			// Every vertex and edge should have been visited.
			assert_eq!(g.verts().collect::<HashSet<_>>(), vs);
			assert_eq!(g.edges().collect::<HashSet<_>>(), es);
			assert_eq!(stack.len(), 0);
		}
	}
}
