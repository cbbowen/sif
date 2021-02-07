use itertools::{Itertools, MapInto};
use std::marker::PhantomData;
use std::ops::Range;
use std::ops::{Index, IndexMut};

pub trait Key: From<usize> {
	fn index(&self) -> usize;
}

#[derive(Clone, Debug)]
pub struct Domain<K, T = ()> {
	values: Vec<T>,
	_phantom: PhantomData<K>,
}

impl<K, T> Default for Domain<K, T> {
	fn default() -> Self {
		Domain {
			values: Default::default(),
			_phantom: PhantomData,
		}
	}
}

pub type DomainKeys<'a, K> = MapInto<Range<usize>, K>;

impl<K: Key, T> Domain<K, T> {
	pub fn keys(&self) -> DomainKeys<'_, K> {
		(0..self.len()).map_into::<K>()
	}

	pub fn values(&self) -> &[T] {
		&self.values
	}

	pub fn len(&self) -> usize {
		self.values.len()
	}

	pub fn insert(&mut self, value: T) -> K {
		let key = self.len().into();
		self.values.push(value);
		key
	}
}

impl<K: Key, T: Default> Domain<K, T> {
	pub fn insert_default(&mut self) -> K {
		self.insert(Default::default())
	}
}

impl<K: Key, T> Index<K> for Domain<K, T> {
	type Output = T;
	fn index(&self, k: K) -> &Self::Output {
		&self.values[k.index()]
	}
}

impl<K: Key, T> IndexMut<K> for Domain<K, T> {
	fn index_mut(&mut self, k: K) -> &mut Self::Output {
		&mut self.values[k.index()]
	}
}

#[derive(Clone, Debug)]
pub struct Map<K, T> {
	values: Vec<T>,
	default: T,
	_phantom: PhantomData<*const K>,
}

impl<K: Key, T: Clone> Map<K, T> {
	pub fn with_capacity(default: T, capacity: usize) -> Map<K, T> {
		Map {
			values: Vec::with_capacity(capacity),
			default,
			_phantom: PhantomData,
		}
	}
}

impl<K: Key, T: Clone> crate::Map<K, T> for Map<K, T> {
	type Ref<'a>
	where
		T: 'a,
	= &'a T;
	fn get<'a>(&'a self, k: K) -> Self::Ref<'a>
	where
		T: 'a,
	{
		let index = k.index();
		self.values.get(index).unwrap_or(&self.default)
	}
}

impl<K: Key, T: Clone> crate::MapMut<K, T> for Map<K, T> {
	type RefMut<'a>
	where
		T: 'a,
	= &'a mut T;
	fn get_mut(&mut self, k: K) -> Self::RefMut<'_> {
		let index = k.index();
		if index >= self.values.len() {
			self.values.resize(index + 1, self.default.clone());
		}
		&mut self.values[index]
	}
}

// TODO: We can implement this more efficiently because it can assume valid
// indices (or panic).
pub type EphemeralMap<K, T> = Map<K, T>;

#[cfg(test)]
mod tests {
	use std::collections::HashSet;

	type Key = usize;
	impl super::Key for Key {
		fn index(&self) -> usize {
			*self
		}
	}
	type Value = Key;

	fn assert_domain_invariants(domain: &super::Domain<Key, Value>) {
		assert_eq!(domain.keys().len(), domain.len());
		let mut keys = HashSet::new();
		for key in domain.keys() {
			assert!(keys.insert(key));
		}
	}

	#[test]
	fn domain() {
		let mut domain = super::Domain::default();
		assert_eq!(domain.len(), 0);
		assert_domain_invariants(&domain);

		assert_eq!(domain.insert_default(), 0);
		assert_eq!(domain.len(), 1);
		assert_domain_invariants(&domain);

		assert_eq!(domain.insert(1), 1);
		assert_eq!(domain.len(), 2);
		assert_domain_invariants(&domain);
		for key in domain.keys() {
			assert_eq!(key, domain[key]);
		}
	}
}
