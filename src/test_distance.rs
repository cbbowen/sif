use std::ops::Add;

#[derive(Debug, Clone, Copy)]
pub struct TestCost<C, E>(pub C, pub E);

#[derive(Debug, Clone, Copy)]
pub struct TestDistance<C, E> {
	pub cost: C,
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
