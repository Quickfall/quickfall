//! IR context related code

use std::{mem::transmute, ops::{Add, Deref, DerefMut}, rc::Rc};

use commons::{err::{PositionlessError, PositionlessResult}, utils::map::HashedMap};
use inkwell::{AddressSpace, builder::Builder, context::Context, module::Module, types::{PointerType, VoidType}};

use crate::{irstruct::{funcs::IRFunction, ptr::IRPointer, staticvars::IRStaticVariable}, types::storage::IRTypeStorage, utils::LateInit};

/// The global IR context.
/// Basically holds anything related to the current IR compilation (eg: functions, types, global vars)
pub struct IRContext {
	pub inkwell_ctx: Rc<Context>,
	pub builder: Builder<'static>,
	pub ptr_type: PointerType<'static>,
	pub void_type: VoidType<'static>,

	pub module: Module<'static>,

	pub type_storage: LateInit<IRTypeStorage>,

	pub functions: HashedMap<Rc<IRFunction>>,
	pub static_vars: HashedMap<Rc<IRStaticVariable>>
}

impl IRContext {
	pub fn new(ctx: Rc<Context>) -> Self {

		let ptr_type = unsafe { transmute::<PointerType, PointerType<'static>>(ctx.ptr_type(AddressSpace::from(0))) };
		let void_type = unsafe { transmute::<VoidType, VoidType<'static>>(ctx.void_type()) };
		let module = unsafe { transmute::<Module, Module<'static>>(ctx.create_module("quickfall_module")) };

		let mut ir =  IRContext { 
			inkwell_ctx: ctx.clone(), 
			builder: unsafe { transmute::<Builder, Builder<'static>>(ctx.create_builder()) }, 
			ptr_type,
			functions: HashedMap::new(0), 
			static_vars: HashedMap::new(0),
			type_storage: LateInit::new(),
			void_type,
			module
		};

		ir.type_storage.fill(IRTypeStorage::new(&ir));

		return ir;
	}

	pub fn add_variable(&mut self, hash: u64, var: IRStaticVariable) -> PositionlessResult<bool> {
		if self.is_key_taken(hash) {
			return Err(PositionlessError::new("There already is an element named like this!"));
		}

		self.static_vars.put(hash, Rc::new(var));
		return Ok(true);
	}

	pub fn get_variable(&self, hash: u64) -> PositionlessResult<Rc<IRStaticVariable>> {
		return match self.static_vars.get(hash) {
			Some(v) => Ok(v.clone()),
			None => return Err(PositionlessError::new("Invalid variable name"))
		};
	}

	pub fn is_key_taken(&self, hash: u64) -> bool {
		return self.functions.get(hash).is_some() || self.static_vars.get(hash).is_some() || self.type_storage.get(hash).is_some();
	}

	pub fn get_funtion(&self, hash: u64) -> PositionlessResult<Rc<IRFunction>> {
		return match self.functions.get(hash) {
			Some(v) => Ok(v.clone()),
			None => Err(PositionlessError::new("Invalid function name!"))
		}
	}

	pub fn add_function(&mut self, hash: u64, func: IRFunction) -> PositionlessResult<bool> {
		if self.is_key_taken(hash) {
			return Err(PositionlessError::new("There already is an element named like this!"));
		}

		self.functions.put(hash, Rc::new(func));
		return Ok(true);
	}

}

pub struct LocalIRVariable {
	pub ptr: IRPointer,
	pub depth: i64 // Depth is depth in body.
}

/// The local IR context.
/// Holds anything held and created in the given body (eg: vars).
pub struct IRLocalContext {
	pub vars: HashedMap<LocalIRVariable>,
	pub current_depth: i64, // Starts at 0 where 0 is function body
}

impl IRLocalContext {
	pub fn new() -> Self {
		return IRLocalContext { vars: HashedMap::new(0), current_depth: 0 }
	}	

	/// Attempts to add a variable in the current local context. Will return an error if the operation is impossible
	pub fn add_variable(&mut self, hash: u64, var: IRPointer) -> PositionlessResult<bool> {
		if self.vars.get(hash).is_some() {
			return Err(PositionlessError::new(&format!("Variable named {} is already registered in the current context.", hash)));
		}

		self.vars.put(hash, LocalIRVariable { ptr: var, depth: self.current_depth });
		return Ok(true);
	}

	pub fn get_variable(&self, hash: u64) -> PositionlessResult<&IRPointer> {
		return match self.vars.get(hash) {
			Some(v) => Ok(&v.ptr),
			None => return Err(PositionlessError::new(&format!("Invalid variable hash {}", hash)))
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
				println!("Dropping variable in lctx with hash {}", entry.0);
				hashToRemove.push(entry.0);
			}
		}

		for hash in hashToRemove {
			self.vars.erase(hash);
		}
	}

}