use crate::{blocks::hints::HintStorage, funcs::MIRFunction, vals::ptr::MIRPointerValue};


pub struct MIRContext {
	pub functions: Vec<MIRFunction>,
	pub ssa_hints: HintStorage,
}

impl MIRContext {
	pub fn new() -> Self {
		MIRContext { functions: vec![], ssa_hints: HintStorage::new() }
	}
}

pub struct MIRBlockContext {
	/// Converts the raw HIR variable indexes into actual pointer values
	pub pointer_vals: Vec<MIRPointerValue> 
}

impl MIRBlockContext {
	pub fn new() -> Self {
		MIRBlockContext { pointer_vals: vec![] }
	}
}