use std::ops::Add;

/// A test cost associated with an edge.
#[derive(Debug, Clone, Copy)]
pub struct TestCost<C, E>(pub C, pub E);

/// A test distance that includes the edge used to reach the current vertex.
#[derive(Debug, Clone, Copy)]
pub struct TestDistance<C, E> {
	/// The total cost to reach the current vertex.
	pub cost: C,
	/// The edge used to reach the current vertex or `None` if this is the initial distance.
	pub pred: Option<E>,
}

impl<C: Default, E> Default for TestDistance<C, E> {
	fn default() -> Self {
		TestDistance::new(C::default())
	}
}

impl<C: PartialEq, E> PartialEq for TestDistance<C, E> {
	fn eq(&self, other: &Self) -> bool {
		self.cost.eq(&other.cost)
	}
}

impl<C: Eq, E> Eq for TestDistance<C, E> {}

impl<C, E> TestDistance<C, E> {
	/// Constructs a new `TestDistance` with the given cost and no predecessor edge.
	pub fn new(cost: C) -> Self {
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
