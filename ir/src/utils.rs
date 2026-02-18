//! Codebase wide utilities. Are mostly used to escape the Inkwell lifecycle hell

use std::{mem::transmute, rc::Rc};

use inkwell::context::Context;

/// A type containing a counted reference to the Inkwell context. 
/// This allows for the type to have the 'static lifecycle safely, allowing to safely use without annoying lifecycles.
pub struct OwnedType<T> {
	pub inner: T,
	pub owned: Rc<Context>
}

impl<K> OwnedType<K> {
	pub fn new(ctx: &Rc<Context>, inner: K) -> Self {
		return OwnedType { inner: unsafe { transmute(inner)} , owned: ctx.clone() }
	}
}