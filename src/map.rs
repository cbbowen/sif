//! Module for representing maps from vertices or edges to values.

use std::borrow::Borrow;
use std::ops::DerefMut;

/// Represents a mapping from keys to values.
pub trait Map<K> {
	/// The type to which keys are mapped.
	type Value;

	/// A type which borrows a value.
	type Ref<'a>: Borrow<Self::Value>
	where
		Self::Value: 'a;

	/// Borrows the value associated with a key.
	fn get<'a>(&'a self, k: K) -> Self::Ref<'a>
	where
		Self::Value: 'a;
}

impl<K, T, F: Fn(K) -> T> Map<K> for F {
	type Value = T;
	type Ref<'a>
	where
		Self::Value: 'a,
	= Self::Value;

	fn get<'a>(&'a self, k: K) -> Self::Ref<'a>
	where
		Self::Value: 'a,
	{
		self(k)
	}
}

/// Represents a mutable mapping from keys to values.
pub trait MapMut<K>: Map<K> {
	/// A type which can be dereferenced to a mutable value.
	type RefMut<'a>: DerefMut<Target = Self::Value>
	where
		Self::Value: 'a;

	/// Returns a mutable reference to the value associated with a key.
	fn get_mut(&mut self, k: K) -> Self::RefMut<'_>;
}

/// Adaptor which dereferences by unwrapping an `Option`.
pub struct UnwrapRef<R, T>(R, std::marker::PhantomData<T>);

impl<T, R: Borrow<Option<T>>> Borrow<T> for UnwrapRef<R, T> {
	fn borrow(&self) -> &T {
		self.0.borrow().as_ref().unwrap()
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

impl<K, T: Copy, M: Map<K, Value = Option<T>>> Map<K> for Unwrap<M> {
	type Value = T;

	type Ref<'a>
	where
		Self::Value: 'a,
	= UnwrapRef<M::Ref<'a>, Self::Value>;

	fn get<'a>(&'a self, k: K) -> Self::Ref<'a>
	where
		Self::Value: 'a,
	{
		UnwrapRef(self.0.get(k), std::marker::PhantomData)
	}
}
