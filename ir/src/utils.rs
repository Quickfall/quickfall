//! Codebase wide utilities. Are mostly used to escape the Inkwell lifecycle hell

use std::{hash::Hash, mem::transmute, ops::{Deref, DerefMut}, rc::Rc};

use inkwell::context::Context;

/// A type containing a counted reference to the Inkwell context. 
/// This allows for the type to have the 'static lifecycle safely, allowing to safely use without annoying lifecycles.
#[derive(Clone, Debug)]
pub struct OwnedType<T: Clone> {
	pub inner: T,
	pub owned: Rc<Context>
}

impl<K: Clone> OwnedType<K> {
	pub fn new(ctx: &Rc<Context>, inner: K) -> Self {
		return OwnedType { inner: unsafe { transmute(inner)} , owned: ctx.clone() }
	}

	pub fn new_ref(ctx: &Rc<Context>, inner: &K) -> Self {
		return OwnedType { inner: unsafe { transmute(K::clone(inner)) }, owned: ctx.clone() }
	}

	pub fn new_ownership(ctx: Rc<Context>, inner: K) -> Self {
		return OwnedType { inner: unsafe {transmute(inner) }, owned: ctx }
	}
}

impl<T: Clone> Deref for OwnedType<T> {
	type Target = T;

	fn deref(&self) -> &Self::Target {
		return &self.inner;
	}
}

/// Warning: unsafe temporary struct
pub struct LateInit<K> {
	inner: Option<K>
}

#[derive(Eq, PartialEq)]
pub struct SelfHash {
	pub hash: u64
}

impl Hash for SelfHash {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		state.write_u64(self.hash);
	}
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