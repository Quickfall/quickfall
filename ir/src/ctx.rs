//! IR context related code

use commons::{err::{PositionlessError, PositionlessResult}, utils::map::HashedMap};
use inkwell::{AddressSpace, builder::Builder, context::Context, module::Module, types::{PointerType, VoidType}};

use crate::{irstruct::{funcs::IRFunction, ptr::IRPointer, staticvars::IRStaticVariable}, types::storage::IRTypeStorage};

/// The global IR context.
/// Basically holds anything related to the current IR compilation (eg: functions, types, global vars)
pub struct IRContext<'a> {
	pub inkwell_ctx: &'a Context,
	pub builder: Builder<'a>,
	pub ptr_type: PointerType<'a>,
	pub void_type: VoidType<'a>,

	pub module: Module<'a>,

	pub type_storage: IRTypeStorage<'a>,

	pub functions: HashedMap<IRFunction<'a>>,
	pub static_vars: HashedMap<IRStaticVariable<'a>>
}

impl<'a> IRContext<'a> {
	pub fn new(builder: Builder<'a>, ctx: &'a Context) -> Self {
		return IRContext { inkwell_ctx: ctx, builder, ptr_type: ctx.ptr_type(AddressSpace::from(0)), functions: HashedMap::new(0), static_vars: HashedMap::new(0), type_storage: IRTypeStorage::new(&ctx), void_type: ctx.void_type(), module: ctx.create_module("quickfall_module") }
	}

	pub fn add_variable(&'a mut self, hash: u64, var: IRStaticVariable<'a>) -> PositionlessResult<bool> {
		if self.is_key_taken(hash) {
			return Err(PositionlessError::new("There already is an element named like this!"));
		}

		self.static_vars.put(hash, var);
		return Ok(true);
	}

	pub fn get_variable(&'a self, hash: u64) -> PositionlessResult<&'a IRStaticVariable<'a>> {
		return match self.static_vars.get(hash) {
			Some(v) => Ok(v),
			None => return Err(PositionlessError::new("Invalid variable name"))
		};
	}

	pub fn is_key_taken(&self, hash: u64) -> bool {
		return self.functions.get(hash).is_some() || self.static_vars.get(hash).is_some() || self.type_storage.get(hash).is_some();
	}

	pub fn get_funtion(&'a self, hash: u64) -> PositionlessResult<&'a IRFunction<'a>> {
		return match self.functions.get(hash) {
			Some(v) => Ok(v),
			None => Err(PositionlessError::new("Invalid function name!"))
		}
	}

	pub fn add_function(&'a mut self, hash: u64, func: IRFunction<'a>) -> PositionlessResult<bool> {
		if self.is_key_taken(hash) {
			return Err(PositionlessError::new("There already is an element named like this!"));
		}

		self.functions.put(hash, func);
		return Ok(true);
	}

}

pub struct LocalIRVariable<'a> {
	pub ptr: IRPointer<'a>,
	pub depth: usize // Depth is depth in body.
}

/// The local IR context.
/// Holds anything held and created in the given body (eg: vars).
pub struct IRLocalContext<'a> {
	pub vars: HashedMap<LocalIRVariable<'a>>,
	pub current_depth: usize, // Starts at 0 where 0 is function body
}

impl<'a> IRLocalContext<'a> {
	pub fn new() -> Self {
		return IRLocalContext { vars: HashedMap::new(0), current_depth: 0 }
	}	

	/// Attempts to add a variable in the current local context. Will return an error if the operation is impossible
	pub fn add_variable(&mut self, hash: u64, var: IRPointer<'a>) -> PositionlessResult<bool> {
		if self.vars.get(hash).is_some() {
			return Err(PositionlessError::new(&format!("Variable named {} is already registered in the current context.", hash)));
		}

		self.vars.put(hash, LocalIRVariable { ptr: var, depth: self.current_depth });
		return Ok(true);
	}

	pub fn get_variable(&'a self, hash: u64) -> PositionlessResult<&'a IRPointer<'a>> {
		return match self.vars.get(hash) {
			Some(v) => Ok(&v.ptr),
			None => return Err(PositionlessError::new("Invalid variable name"))
		};
	}

	pub fn increment_body_depth(&mut self) {
		self.current_depth += 1;
	}

	/// Ends the current nested body. Is responsible for removing non-valid variable indices
	/// Example:
	/// ```
	/// func test() {
	/// 	// body of depth 0 starts
	/// 	if(true) {
	/// 		// body of depth 1 starts
	/// 		// body of depth 1 ends
	/// 	}
	/// 
	/// 	// body of depth 0 ends
	/// }
	/// ```
	pub fn end_nested_body_depth(&mut self) {
		self.current_depth -= 1;

		let mut hashToRemove: Vec<u64> = vec![];

		for entry in self.vars.entries() {
			if entry.1.depth > self.current_depth {
				hashToRemove.push(entry.0);
			}
		}

		for hash in hashToRemove {
			self.vars.erase(hash);
		}
	}

}