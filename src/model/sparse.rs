use std::collections::{hash_map, HashMap};
use std::hash::Hash;
use std::ops::{Index, IndexMut};

pub trait Key: Clone + Copy + Eq + Hash + From<usize> {}

#[derive(Clone, Debug)]
pub struct Domain<K, T = ()> {
	values: HashMap<K, T>,
	free: Vec<K>,
	next: usize,
}

impl<K, T> Default for Domain<K, T> {
	fn default() -> Self {
		Domain {
			values: Default::default(),
			free: Default::default(),
			next: 0,
		}
	}
}

pub type DomainKeys<'a, K, T = ()> = std::iter::Cloned<hash_map::Keys<'a, K, T>>;

impl<K: Key, T> Domain<K, T> {
	pub fn keys(&self) -> DomainKeys<'_, K, T> {
		self.values.keys().cloned()
	}

	pub fn len(&self) -> usize {
		self.values.len()
	}

	pub fn insert(&mut self, value: T) -> K {
		let key = self.free.pop().unwrap_or_else(|| {
			let next = self.next + 1;
			std::mem::replace(&mut self.next, next).into()
		});
		let old_value = self.values.insert(key, value);
		debug_assert!(old_value.is_none(), "key not unique");
		key
	}

	pub fn remove(&mut self, key: K) -> T {
		let result = self.values.remove(&key).expect("key in domain");
		self.free.push(key);
		result
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
		&self.values[&k]
	}
}

impl<K: Key, T> IndexMut<K> for Domain<K, T> {
	fn index_mut(&mut self, k: K) -> &mut Self::Output {
		self.values.get_mut(&k).expect("key in domain")
	}
}

#[derive(Clone, Debug, Default)]
pub struct Map<K, T> {
	values: HashMap<K, T>,
	default: T,
}

impl<K, T> Map<K, T> {
	pub fn new(default: T) -> Map<K, T> {
		Map {
			values: HashMap::new(),
			default,
		}
	}

	pub fn with_capacity(default: T, capacity: usize) -> Map<K, T> {
		Map {
			values: HashMap::with_capacity(capacity),
			default,
		}
	}
}

impl<K: Eq + Hash, T> crate::Map<K> for Map<K, T> {
	type Value = T;
	type Ref<'a>
	where
		T: 'a,
	= &'a T;
	fn get<'a>(&'a self, k: K) -> Self::Ref<'_>
	where
		T: 'a,
	{
		self.values.get(&k).unwrap_or(&self.default)
	}
}

impl<K: Eq + Hash, T: Clone> crate::MapMut<K> for Map<K, T> {
	type RefMut<'a>
	where
		T: 'a,
	= &'a mut T;
	fn get_mut(&mut self, k: K) -> Self::RefMut<'_> {
		let default = &self.default;
		self.values.entry(k).or_insert_with(|| default.clone())
	}
}

pub type EphemeralMap<K, T> = Map<K, T>;

#[cfg(test)]
mod tests {
	use std::collections::HashSet;

	type Key = usize;
	impl super::Key for Key {}
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

		domain.remove(0);
		assert_eq!(domain.len(), 1);
		assert_domain_invariants(&domain);
		for key in domain.keys() {
			assert_eq!(key, domain[key]);
		}

		domain.remove(1);
		assert_eq!(domain.len(), 0);
		assert_domain_invariants(&domain);
	}
}
