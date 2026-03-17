use std::{mem::transmute, ops::Deref, rc::Rc};

use inkwell::context::Context;

pub struct LLVMObject<T: Clone> {
	pub inner: T,
	pub ctx: Rc<Context>
}

impl<K: Clone> LLVMObject<K> {
	pub fn new(ctx: &Rc<Context>, inner: K) -> Self {
		return LLVMObject { inner: unsafe { transmute(inner)} , ctx: ctx.clone() }
	}

	pub fn new_ref(ctx: &Rc<Context>, inner: &K) -> Self {
		return LLVMObject { inner: unsafe { transmute(K::clone(inner)) }, ctx: ctx.clone() }
	}

	pub fn new_ownership(ctx: Rc<Context>, inner: K) -> Self {
		return LLVMObject { inner: unsafe {transmute(inner) }, ctx: ctx }
	}
}

impl<T: Clone> Deref for LLVMObject<T> {
	type Target = T;

	fn deref(&self) -> &Self::Target {
		return &self.inner;
	}
}
