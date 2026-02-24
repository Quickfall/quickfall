//! IR context related code

use std::{collections::HashMap, mem::transmute, rc::Rc};

use commons::{utils::map::HashedMap};
use errors::{IR_ALREADY_EXISTING_ELEM, IR_FIND_FUNCTION, IR_FIND_VARIABLE, errs::{BaseResult, base::BaseError}};
use inkwell::{AddressSpace, builder::Builder, context::Context, module::Module, types::{PointerType, VoidType}};

use crate::{irstruct::{funcs::IRFunction, ptr::IRPointer, staticvars::IRStaticVariable}, types::storage::IRTypeStorage, utils::{LateInit, SelfHash}, values::IRValue};

/// The global IR context.
/// Basically holds anything related to the current IR compilation (eg: functions, types, global vars)
pub struct IRContext {
	pub inkwell_ctx: Rc<Context>,
	pub builder: Builder<'static>,
	pub ptr_type: PointerType<'static>,
	pub void_type: VoidType<'static>,

	pub module: Module<'static>,

	pub type_storage: LateInit<IRTypeStorage>,

	pub functions: HashMap<SelfHash, Rc<IRFunction>>,
	pub static_vars: HashMap<SelfHash, Rc<IRStaticVariable>>
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
			functions: HashMap::new(), 
			static_vars: HashMap::new(),
			type_storage: LateInit::new(),
			void_type,
			module
		};

		ir.type_storage.fill(IRTypeStorage::new(&ir));

		return ir;
	}

	pub fn add_variable(&mut self, hash: u64, var: IRStaticVariable) -> BaseResult<bool> {
		if self.is_key_taken(hash) {
			return Err(BaseError::err(IR_ALREADY_EXISTING_ELEM!().to_string()));
		}

		self.static_vars.insert(SelfHash { hash }, Rc::new(var));
		return Ok(true);
	}

	pub fn get_variable(&self, hash: u64) -> BaseResult<Rc<IRStaticVariable>> {
		return match self.static_vars.get(&SelfHash { hash }) {
			Some(v) => Ok(v.clone()),
			None => return Err(BaseError::err(IR_FIND_VARIABLE!().to_string()))
		};
	}

	pub fn is_key_taken(&self, hash: u64) -> bool {
		return self.functions.get(&SelfHash { hash }).is_some() || self.static_vars.get(&SelfHash {hash}).is_some() || self.type_storage.get(hash).is_some();
	}

	pub fn get_function(&self, hash: u64) -> BaseResult<Rc<IRFunction>> {
		return match self.functions.get(&SelfHash { hash }) {
			Some(v) => Ok(v.clone()),
			None => return Err(BaseError::err(IR_FIND_FUNCTION!().to_string()))
		}
	}

	pub fn add_function(&mut self, hash: u64, func: IRFunction) -> BaseResult<bool> {
		if self.is_key_taken(hash) {
			return Err(BaseError::err(IR_ALREADY_EXISTING_ELEM!().to_string()));
		}

		self.functions.insert(SelfHash { hash }, Rc::new(func));
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
	pub arguments: HashedMap<IRValue>,
	pub current_depth: i64, // Starts at 0 where 0 is function body
}

impl IRLocalContext {
	pub fn new() -> Self {
		return IRLocalContext { vars: HashedMap::new(0), arguments: HashedMap::new(0), current_depth: 0 }
	}	

	/// Attempts to add a variable in the current local context. Will return an error if the operation is impossible
	pub fn add_variable(&mut self, hash: u64, var: IRPointer) -> BaseResult<bool> {
		if self.vars.get(hash).is_some() {
			return Err(BaseError::err(IR_ALREADY_EXISTING_ELEM!().to_string()));
		}

		self.vars.put(hash, LocalIRVariable { ptr: var, depth: self.current_depth });
		return Ok(true);
	}

	pub fn add_argument(&mut self, hash: u64, val: IRValue) -> BaseResult<bool> {
		if self.arguments.get(hash).is_some() {
			return Err(BaseError::err(IR_ALREADY_EXISTING_ELEM!().to_string()));
		}

		self.arguments.put(hash, val);
		return Ok(true);
	}

	pub fn get_variable(&self, hash: u64) -> BaseResult<&IRPointer> {
		return match self.vars.get(hash) {
			Some(v) => Ok(&v.ptr),
			None => return Err(BaseError::err(IR_FIND_VARIABLE!().to_string()))
		};
	}

	pub fn get_argument(&self, hash: u64) -> BaseResult<&IRValue> {
		return match self.arguments.get(hash) {
			Some(v) => Ok(v),
			None => return Err(BaseError::err(IR_FIND_VARIABLE!().to_string()))
		}
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

		let mut hash_to_remove: Vec<u64> = vec![];

		for entry in self.vars.entries() {
			if entry.1.depth > self.current_depth {
				hash_to_remove.push(entry.0);
			}
		}

		for hash in hash_to_remove {
			self.vars.erase(hash);
		}
	}

}