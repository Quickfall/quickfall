use std::{mem::transmute, ops::Deref, rc::Rc};

use inkwell::{basic_block::BasicBlock, context::Context};

pub type LLVMBlock = LLVMObject<BasicBlock<'static>>;

pub struct LLVMObject<T: Clone> {
	pub inner: T,
	pub ctx: Rc<Context>
}

/// The LLVMObject without a safety reference.
/// 
/// # Safety
/// Using this is only safe when the object it is contained in contains a Context ref
pub struct LLVMSiblingObject<T: Clone> {
	pub inner: T
}

impl<K: Clone> LLVMSiblingObject<K> {
	pub fn new(inner: K) -> Self {
		return LLVMSiblingObject { inner: unsafe { transmute(inner)}}
	}

	pub fn new_ref(inner: &K) -> Self {
		return LLVMSiblingObject { inner: unsafe { transmute(K::clone(inner)) }}
	}
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

impl<T: Clone> Deref for LLVMSiblingObject<T> {
	type Target = T;

	fn deref(&self) -> &Self::Target {
		return &self.inner;
	}
}