//! Common utilties to bypass / make lifecycles easier

use std::{mem::transmute, ops::{Deref, DerefMut}, rc::Rc};

/// A structure that expires only whenever the given K instance expires. Allows to use the 'static lifecycle safely on the T instance.
pub struct OwnedBy<T: Clone, K> {
	pub inner: T,
	pub owned: Rc<K>
}

impl<T: Clone, K> OwnedBy<T, K> {
	pub fn new(owned: &Rc<K>, inner: T) -> Self {
		return OwnedBy { inner: unsafe { transmute(inner) }, owned: owned.clone() }
	}

	pub fn new_ref(ctx: &Rc<K>, inner: &T) -> Self {
		return OwnedBy { inner: unsafe { transmute(T::clone(inner)) }, owned: ctx.clone() }
	}

	pub fn new_ownership(ctx: Rc<K>, inner: T) -> Self {
		return OwnedBy { inner: unsafe {transmute(inner) }, owned: ctx }
	}

}

impl<T: Clone, K> Deref for OwnedBy<T, K> {
	type Target = T;

	fn deref(&self) -> &Self::Target {
		return &self.inner;
	}
}

#[deprecated]
pub struct LateInit<K> {
	inner: Option<K>
}

#[derive(Eq, PartialEq)]
pub struct SelfHash {
	pub hash: u64
}


impl<K> Deref for LateInit<K> {
	type Target = K;

	fn deref(&self) -> &Self::Target {
		return &self.inner.as_ref().unwrap();
	}
}

impl<K> DerefMut for LateInit<K> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		let m = self.inner.as_mut();
		return m.unwrap();
	}
}

impl<K> LateInit<K> {
	pub fn new() -> Self {
		return LateInit { inner: None };
	}

	pub fn fill(&mut self, inner: K) {
		self.inner = Some(inner);
	}
} 