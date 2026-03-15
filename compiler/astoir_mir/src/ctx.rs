use crate::{blocks::{MIRBlock, hints::HintStorage, refer::MIRBlockReference}, funcs::MIRFunction, vals::ptr::MIRPointerValue};


pub struct MIRContext {
	pub functions: Vec<MIRFunction>,
	pub blocks: Vec<MIRBlock>,

	pub ssa_hints: HintStorage,
}

impl MIRContext {
	pub fn new() -> Self {
		MIRContext { functions: vec![], ssa_hints: HintStorage::new(), blocks: vec![] }
	}

	pub fn create_block(&mut self) -> MIRBlockReference {
		let ind = self.blocks.len();

		self.blocks.push(MIRBlock::new());

		return ind;
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