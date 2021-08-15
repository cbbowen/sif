use std::borrow::Borrow;
use std::fmt::Debug;
use std::hash::Hash;

use super::map::{Map, MapMut};

/// Represents a [directed graph](https://en.wikipedia.org/wiki/Directed_graph).
pub trait Digraph {
	/// The type of a vertex.
	type Vert: Copy + Debug + Eq + Hash + Ord;

	/// The type of an edge.
	type Edge: Copy + Debug + Eq + Hash + Ord;

	/// Returns the vertices at the tail and head of an edge.
	///
	/// # Examples
	/// ```
	/// # use sif::*;
	/// # let mut g = DenseEdgeList::new();
	/// # let tail = g.insert_vert();
	/// # let head = g.insert_vert();
	/// let e = g.insert_edge(tail, head);
	/// assert_eq!(g.endpoints(e), (tail, head));
	/// ```
	fn endpoints(&self, e: impl Borrow<Self::Edge>) -> (Self::Vert, Self::Vert);

	/// Returns the vertex at the tail of an edge.
	///
	/// # Examples
	/// ```
	/// # use sif::*;
	/// # let mut g = DenseEdgeList::new();
	/// # let v = g.insert_vert();
	/// # let e = g.insert_edge(v, v);
	/// assert_eq!(g.tail(e), g.endpoints(e).0);
	/// ```
	fn tail(&self, e: impl Borrow<Self::Edge>) -> Self::Vert {
		self.endpoints(e).0
	}

	/// Returns the vertex at the head of an edge.
	///
	/// # Examples
	/// ```
	/// # use sif::*;
	/// # let mut g = DenseEdgeList::new();
	/// # let v = g.insert_vert();
	/// # let e = g.insert_edge(v, v);
	/// assert_eq!(g.tail(e), g.endpoints(e).1);
	/// ```
	fn head(&self, e: impl Borrow<Self::Edge>) -> Self::Vert {
		self.endpoints(e).1
	}

	/// An iterator over all vertices.
	type Verts<'a>: Clone + Iterator<Item = Self::Vert>;

	/// Returns an iterator over all vertices.
	///
	/// # Examples
	/// ```
	/// # use sif::*;
	/// # let mut g = DenseEdgeList::new();
	/// let v = g.insert_vert();
	/// assert!(g.verts().any(|u| u == v));
	/// ```
	fn verts(&self) -> Self::Verts<'_>;

	/// An iterator over all edges.
	type Edges<'a>: Clone + Iterator<Item = Self::Edge>;

	/// Returns an iterator over all edges.
	///
	/// # Examples
	/// ```
	/// # use sif::*;
	/// # let mut g = DenseEdgeList::new();
	/// # let v = g.insert_vert();
	/// # let u = g.insert_vert();
	/// let e = g.insert_edge(v, u);
	/// assert!(g.edges().any(|d| d == e));
	/// ```
	fn edges(&self) -> Self::Edges<'_>;

	/// A mutable map from vertices to values.
	type VertMap<T: Clone>: MapMut<Self::Vert, T>;

	/// Constructs a new mutable mapping from vertices to values with all vertices
	/// initially mapped to the given default.
	///
	/// # Examples
	/// ```
	/// # use sif::*;
	/// # let mut g = DenseEdgeList::new();
	/// let mut m = g.vert_map("foo");
	/// let v = g.insert_vert();
	/// assert_eq!(*m.get(v), "foo");
	/// *m.get_mut(v) = "bar";
	/// assert_eq!(*m.get(v), "bar");
	/// ```
	fn vert_map<T: Clone>(&self, default: T) -> Self::VertMap<T>;

	/// Constructs a new mutable mapping from vertices to values with all vertices
	/// initially mapped to the default.
	///
	/// # Examples
	/// ```
	/// # use sif::*;
	/// # let mut g = DenseEdgeList::new();
	/// let mut m = g.default_vert_map();
	/// let v = g.insert_vert();
	/// assert_eq!(*m.get(v), 0);
	/// *m.get_mut(v) = 42;
	/// assert_eq!(*m.get(v), 42);
	/// ```
	fn default_vert_map<T: Clone + Default>(&self) -> Self::VertMap<T> {
		self.vert_map(Default::default())
	}

	/// A mutable map from edges to values.
	type EdgeMap<T: Clone>: MapMut<Self::Edge, T>;

	/// Constructs a new mutable mapping from edges to values with all edges
	/// initially mapped to the given default.
	///
	/// # Examples
	/// ```
	/// # use sif::*;
	/// # let mut g = DenseEdgeList::new();
	/// # let v = g.insert_vert();
	/// # let u = g.insert_vert();
	/// let mut m = g.edge_map("foo");
	/// let e = g.insert_edge(v, u);
	/// assert_eq!(*m.get(e), "foo");
	/// *m.get_mut(e) = "bar";
	/// assert_eq!(*m.get(e), "bar");
	/// ```
	fn edge_map<T: Clone>(&self, default: T) -> Self::EdgeMap<T>;

	/// Constructs a new mutable mapping from edges to values with all edges
	/// initially mapped to the default.
	///
	/// # Examples
	/// ```
	/// # use sif::*;
	/// # let mut g = DenseEdgeList::new();
	/// # let v = g.insert_vert();
	/// # let u = g.insert_vert();
	/// let mut m = g.default_edge_map();
	/// let e = g.insert_edge(v, u);
	/// assert_eq!(*m.get(e), 0);
	/// *m.get_mut(e) = 42;
	/// assert_eq!(*m.get(e), 42);
	/// ```
	fn default_edge_map<T: Clone + Default>(&self) -> Self::EdgeMap<T> {
		self.edge_map(Default::default())
	}

	/// A mutable map from vertices to values that requires the graph remain
	/// immutable for its lifetime.
	type EphemeralVertMap<'a, T: Clone>: MapMut<Self::Vert, T> = Self::VertMap<T>;

	/// Constructs a new mutable mapping from vertices to values with all vertices
	/// initially mapped to the given default. The mapping may not outlive the
	/// immutable reference from which it was constructed.
	///
	/// # Examples
	/// ```
	/// # use sif::*;
	/// # let mut g = DenseEdgeList::new();
	/// # let v = g.insert_vert();
	/// let mut m = g.ephemeral_vert_map("foo");
	/// assert_eq!(*m.get(v), "foo");
	/// *m.get_mut(v) = "bar";
	/// assert_eq!(*m.get(v), "bar");
	/// ```
	fn ephemeral_vert_map<T: Clone>(&self, default: T) -> Self::EphemeralVertMap<'_, T>;

	/// Constructs a new mutable mapping from vertices to values with all vertices
	/// initially mapped to the default. The mapping may not outlive the immutable
	/// reference from which it was constructed.
	///
	/// # Examples
	/// ```
	/// # use sif::*;
	/// # let mut g = DenseEdgeList::new();
	/// # let v = g.insert_vert();
	/// let mut m = g.default_ephemeral_vert_map();
	/// assert_eq!(*m.get(v), 0);
	/// *m.get_mut(v) = 42;
	/// assert_eq!(*m.get(v), 42);
	/// ```
	fn default_ephemeral_vert_map<T: Clone + Default>(&self) -> Self::EphemeralVertMap<'_, T> {
		self.ephemeral_vert_map(Default::default())
	}

	/// A mutable map from edges to values that requires the graph remain
	/// immutable for its lifetime.
	type EphemeralEdgeMap<'a, T: Clone>: MapMut<Self::Edge, T> = Self::EdgeMap<T>;

	/// Constructs a new mutable mapping from edges to values with all edges
	/// initially mapped to the given default. The mapping may not outlive the
	/// immutable reference from which it was constructed.
	///
	/// # Examples
	/// ```
	/// # use sif::*;
	/// # let mut g = DenseEdgeList::new();
	/// # let v = g.insert_vert();
	/// # let u = g.insert_vert();
	/// # let e = g.insert_edge(v, u);
	/// let mut m = g.ephemeral_edge_map("foo");
	/// assert_eq!(*m.get(e), "foo");
	/// *m.get_mut(e) = "bar";
	/// assert_eq!(*m.get(e), "bar");
	/// ```
	fn ephemeral_edge_map<T: Clone>(&self, default: T) -> Self::EphemeralEdgeMap<'_, T>;

	/// Constructs a new mutable mapping from edges to values with all edges
	/// initially mapped to the default. The mapping may not outlive the immutable
	/// reference from which it was constructed.
	///
	/// # Examples
	/// ```
	/// # use sif::*;
	/// # let mut g = DenseEdgeList::new();
	/// # let v = g.insert_vert();
	/// # let u = g.insert_vert();
	/// # let e = g.insert_edge(v, u);
	/// let mut m = g.default_ephemeral_edge_map();
	/// assert_eq!(*m.get(e), 0);
	/// *m.get_mut(e) = 42;
	/// assert_eq!(*m.get(e), 42);
	/// ```
	fn default_ephemeral_edge_map<T: Clone + Default>(&self) -> Self::EphemeralEdgeMap<'_, T> {
		self.ephemeral_edge_map(Default::default())
	}

	/// Returns whether a given graph is isomorphic to this graph with given vertex and edge mappings.
	fn is_isomorphic_with_maps<G: Digraph>(
		&self,
		g: &G,
		vert_map: &impl Map<Self::Vert, G::Vert>,
		edge_map: &impl Map<Self::Edge, G::Edge>,
	) -> bool {
		// `vert_map` is a function.
		let mut gverts = std::collections::HashSet::new();
		for v in self.verts() {
			// `vert_map` is surjective.
			let inserted = gverts.insert(*vert_map.get(v));
			debug_assert!(inserted);
		}
		// `vert_map` is injective.
		for v in g.verts() {
			if !gverts.contains(&v) {
				return false;
			}
		}

		// `edge_map` is a function.
		let mut gedges = std::collections::HashSet::new();
		for e in self.edges() {
			// `edge_map` is surjective.
			let inserted = gedges.insert(*edge_map.get(e));
			debug_assert!(inserted);
		}
		// `edge_map` is injective.
		for e in g.edges() {
			if !gedges.contains(&e) {
				return false;
			}
		}

		// `edge_map` preserve endpoints.
		for e in self.edges() {
			let (s, t) = self.endpoints(e);
			let (gs, gt) = g.endpoints(*edge_map.get(e));
			if *vert_map.get(s) != gs || *vert_map.get(t) != gt {
				return false;
			}
		}
		true
	}
}

impl<'g, G: Digraph> Digraph for &'g G {
	type Vert = G::Vert;
	type Edge = G::Edge;

	fn endpoints(&self, e: impl Borrow<Self::Edge>) -> (Self::Vert, Self::Vert) {
		(**self).endpoints(e)
	}
	fn tail(&self, e: impl Borrow<Self::Edge>) -> Self::Vert {
		(**self).tail(e)
	}
	fn head(&self, e: impl Borrow<Self::Edge>) -> Self::Vert {
		(**self).head(e)
	}

	type Verts<'a> = G::Verts<'a>;
	fn verts(&self) -> Self::Verts<'_> {
		(**self).verts()
	}

	type Edges<'a> = G::Edges<'a>;
	fn edges(&self) -> Self::Edges<'_> {
		(**self).edges()
	}

	type VertMap<T: Clone> = G::EphemeralVertMap<'g, T>;
	fn vert_map<T: Clone>(&self, default: T) -> Self::VertMap<T> {
		(**self).ephemeral_vert_map(default)
	}

	type EdgeMap<T: Clone> = G::EphemeralEdgeMap<'g, T>;
	fn edge_map<T: Clone>(&self, default: T) -> Self::EdgeMap<T> {
		(**self).ephemeral_edge_map(default)
	}

	fn ephemeral_vert_map<T: Clone>(&self, default: T) -> Self::EphemeralVertMap<'_, T> {
		self.vert_map(default)
	}

	fn ephemeral_edge_map<T: Clone>(&self, default: T) -> Self::EphemeralEdgeMap<'_, T> {
		self.edge_map(default)
	}
}

/// Represents a directed graph with a known order.
pub trait ExactOrderDigraph: Digraph {
	/// Returns the order of the graph, that is, the number of vertices.
	fn order(&self) -> usize;
}
impl<G: Digraph> ExactOrderDigraph for G
where
	for<'a> G::Verts<'a>: ExactSizeIterator,
{
	fn order(&self) -> usize {
		self.verts().len()
	}
}

/// Represents a directed graph with a known size.
pub trait ExactSizeDigraph: Digraph {
	/// Returns the size of the graph, that is, the number of edges.
	fn size(&self) -> usize;
}
impl<G: Digraph> ExactSizeDigraph for G
where
	for<'a> G::Edges<'a>: ExactSizeIterator,
{
	fn size(&self) -> usize {
		self.edges().len()
	}
}
