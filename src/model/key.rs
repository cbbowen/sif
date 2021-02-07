use super::{dense, index::Index, sparse};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct DenseVert(Index);

impl From<usize> for DenseVert {
	fn from(index: usize) -> Self {
		DenseVert(index.into())
	}
}

impl dense::Key for DenseVert {
	fn index(&self) -> usize {
		self.0.index()
	}
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct DenseEdge(Index);

impl From<usize> for DenseEdge {
	fn from(index: usize) -> Self {
		DenseEdge(index.into())
	}
}

impl dense::Key for DenseEdge {
	fn index(&self) -> usize {
		self.0.index()
	}
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct SparseVert(Index);

impl From<usize> for SparseVert {
	fn from(index: usize) -> Self {
		SparseVert(index.into())
	}
}

impl sparse::Key for SparseVert {}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct SparseEdge(Index);

impl From<usize> for SparseEdge {
	fn from(index: usize) -> Self {
		SparseEdge(index.into())
	}
}

impl sparse::Key for SparseEdge {}
