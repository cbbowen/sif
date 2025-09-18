use crate::MapMut;
use crate::model::index::Index;
use std::borrow::Borrow;
use std::marker::PhantomData;

pub struct BinaryHeap<K, T, M> {
	heap: Vec<Option<(K, T)>>,
	map: M,
	_phantom_data: PhantomData<T>,
}

impl<K: Clone, T: Ord, M: MapMut<K, Value = Option<Index>>> BinaryHeap<K, T, M> {
	/// Constructs a new binary heap.
	pub fn new(map: M) -> Self {
		BinaryHeap {
			heap: Vec::new(),
			map,
			_phantom_data: PhantomData,
		}
	}

	fn set_item(&mut self, index: usize, item: (K, T)) {
		*self.map.get_mut(item.0.clone()) = Some(index.into());
		self.heap[index].replace(item);
		// The value we're replacing will be None except when decreasing the value associated with an existing key.
	}

	fn bubble_up(&mut self, mut index: usize, item: (K, T)) {
		while index > 0 {
			let parent_index = (index - 1) >> 1;
			let parent = &mut self.heap[parent_index];
			if parent.as_ref().unwrap().1 <= item.1 {
				break;
			}

			let parent_item = parent.take().unwrap();
			self.set_item(index, parent_item);
			index = parent_index;
		}

		self.set_item(index, item);
	}

	/// Sets `map[key]` to `Some((value, index))` and restores the heap property assuming the value was increased.
	fn sink_down(&mut self, mut index: usize, item: (K, T)) {
		loop {
			let left_index = (index << 1) + 1;
			let right_index = left_index + 1;

			if right_index >= self.heap.len() {
				if left_index < self.heap.len() {
					let child_index = left_index;
					let child = &mut self.heap[child_index];

					if child.as_ref().unwrap().1 < item.1 {
						let child_item = child.take().unwrap();
						self.set_item(index, child_item);
						index = child_index;
					}
				}
				break;
			}

			let child_index =
				if self.heap[left_index].as_ref().unwrap().1 < self.heap[right_index].as_ref().unwrap().1 {
					left_index
				} else {
					right_index
				};
			let child = &mut self.heap[child_index];
			if item.1 <= child.as_ref().unwrap().1 {
				break;
			}

			let child_item = child.take().unwrap();
			self.set_item(index, child_item);
			index = child_index;
		}

		self.set_item(index, item);
	}

	/// If an item already exists and has a value not greater than `value`, return false. Otherwise, decreases the value or adds a new item.
	pub fn try_decrease(&mut self, key: K, value: T) -> bool {
		let index = if let Some(index) = self.map.get(key.clone()).borrow() {
			let index = index.index();
			if self.heap[index].as_ref().unwrap().1 <= value {
				return false;
			}
			index
		} else {
			let index = self.heap.len();
			self.heap.push(None);
			index
		};
		self.bubble_up(index, (key, value));
		true
	}

	/// Removes and returns an item with the least value.
	pub fn pop(&mut self) -> Option<(K, T)> {
		let last = self.heap.pop()?;
		debug_assert!(last.is_some());
		let result = if let Some(first) = self.heap.first_mut() {
			let result = first.take().unwrap();
			self.sink_down(0, last.unwrap());
			result
		} else {
			last.unwrap()
		};
		*self.map.get_mut(result.0.clone()) = None;
		Some(result)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use proptest::proptest;
	use std::collections::{BTreeMap, HashMap, HashSet};
	use std::hash::Hash;

	struct TestMap<K, T> {
		map: HashMap<K, T>,
		default: T,
	}

	impl<K, T: Default> Default for TestMap<K, T> {
		fn default() -> Self {
			TestMap {
				map: HashMap::default(),
				default: T::default(),
			}
		}
	}

	impl<K: Eq + Hash, T: Clone> crate::Map<K> for TestMap<K, T> {
		type Value = T;
		type Ref<'a>
			= &'a T
		where
			Self: 'a;
		fn get<'a>(&'a self, k: K) -> Self::Ref<'a>
		where
			T: 'a,
		{
			&self.map.get(&k).unwrap_or(&self.default)
		}
	}

	impl<K: Eq + Hash, T: Clone> crate::MapMut<K> for TestMap<K, T> {
		type RefMut<'a>
			= &'a mut T
		where
			Self: 'a;
		fn get_mut<'a>(&'a mut self, k: K) -> Self::RefMut<'a> {
			let default = &self.default;
			self.map.entry(k).or_insert_with(|| default.clone())
		}
	}

	proptest! {
		#[test]
		fn try_decrease_and_pop(items: Vec<(u8, u32)>) {
			// Determine the expected order for popped items.
			let mut minimums = HashMap::new();
			for (k, v) in items.iter() {
				minimums.entry(*k).and_modify(|m: &mut u32| *m = (*m).min(*v)).or_insert(*v);
			}
			let mut sorted = BTreeMap::<u32, HashSet<u8>>::new();
			for (k, v) in minimums {
				sorted.entry(v).or_insert(HashSet::new()).insert(k);
			}

			// Add all the items to a heap.
			let mut heap = BinaryHeap::<u8, u32, TestMap<_, _>>::new(TestMap::default());
			for (k, v) in items {
				heap.try_decrease(k, v);
			}

			// Pop them off, asserting they arrive in the right order.
			while let Some((key, value)) = heap.pop() {
				while let Some(e) = sorted.first_entry() {
					if !e.get().is_empty() { break; }
					e.remove_entry();
				}
				let mut e = sorted.first_entry().unwrap();
				assert_eq!(value, *e.key());
				assert!(e.get_mut().remove(&key));
			}
		}
	}
}
