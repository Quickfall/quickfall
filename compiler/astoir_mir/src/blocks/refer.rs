use std::mem::transmute;

use crate::funcs::MIRFunction;

pub struct MIRBlockReference {
	block_ind: usize,
	func_ref: &'static MIRFunction
}

impl MIRBlockReference {
	/// Creates a new block reference.
	#[deprecated(note = "Warning: MIRBlockReference assumes the block and function will live longer than the reference.")]
	pub fn new(block_id: usize, func_ref: &MIRFunction) -> Self {
		return MIRBlockReference { block_ind: block_id, func_ref: unsafe { transmute::<&MIRFunction, &'static MIRFunction>(func_ref) } }
	}
}