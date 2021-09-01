use std::borrow::Borrow;

use super::{
	map::{self, Map, MapMut},
	Digraph,
};

use crate::Homomorphism;

/// Represents a directed graph into which new vertices and edge can be
/// inserted.
pub trait InsertGraph: Default + Digraph {
	/// Constructs an empty graph.
	fn new() -> Self {
		Default::default()
	}

	/// Inserts a new vertex in the graph.
	fn insert_vert(&mut self) -> Self::Vert;

	/// Inserts a new edge in the graph with a given tail and head.
	fn insert_edge(&mut self, tail: Self::Vert, head: Self::Vert) -> Self::Edge;

	/// Constructs a graph isomorphic to the given graph and returns it along with
	/// mappings from the given graph's vertices and edges to those in the new
	/// graph.
	fn isomorphic_from<G: Digraph>(from: &G) -> (Self, Homomorphism<'_, G, Self>) {
		let mut to = Self::default();
		let mut vmap = from.ephemeral_vert_map(None);
		for v in from.verts() {
			*vmap.get_mut(v) = Some(to.insert_vert());
		}
		let mut emap = from.ephemeral_edge_map(None);
		for e in from.edges() {
			let (tail, head) = from.endpoints(e);
			*emap.get_mut(e) = Some(to.insert_edge(
				vmap.get(tail).borrow().expect("tail in verts"),
				vmap.get(head).borrow().expect("head in verts"),
			));
		}
		(
			to,
			Homomorphism::new(map::Unwrap::new(vmap), map::Unwrap::new(emap)),
		)
	}
}
