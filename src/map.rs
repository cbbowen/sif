//! Module for representing maps from vertices or edges to values.

use std::ops::{Deref, DerefMut};

/// Represents a mapping from keys to values.
pub trait Map<K, T> {
	/// A type which can be dereferenced to a value.
	type Ref<'a>: Deref<Target = T>
	where
		T: 'a;

	/// Returns a reference to the value associated with a key.
	fn get<'a>(&'a self, k: K) -> Self::Ref<'a>
	where
		T: 'a;
}

/// Represents a mutable mapping from keys to values.
pub trait MapMut<K, T>: Map<K, T> {
	/// A type which can be dereferenced to a mutable value.
	type RefMut<'a>: DerefMut<Target = T>
	where
		T: 'a;

	/// Returns a mutable reference to the value associated with a key.
	fn get_mut(&mut self, k: K) -> Self::RefMut<'_>;
}

/// Adaptor which dereferences by unwrapping an `Option`.
pub struct UnwrapRef<R>(R);

impl<T, R: Deref<Target = Option<T>>> Deref for UnwrapRef<R> {
	type Target = T;
	fn deref(&self) -> &Self::Target {
		self.0.deref().as_ref().unwrap()
	}
}

/// Map adaptor which unwraps `Option` values.
pub struct Unwrap<M>(M);

impl<M> Unwrap<M> {
	/// Constructs an adaptor which unwraps the `Option` values of the given map.
	pub fn new(m: M) -> Self {
		Unwrap(m)
	}
}

impl<K, T: Copy, M: Map<K, Option<T>>> Map<K, T> for Unwrap<M> {
	type Ref<'a>
	where
		T: 'a,
	= UnwrapRef<M::Ref<'a>>;
	fn get<'a>(&'a self, k: K) -> Self::Ref<'a>
	where
		T: 'a,
	{
		UnwrapRef(self.0.get(k))
	}
}
