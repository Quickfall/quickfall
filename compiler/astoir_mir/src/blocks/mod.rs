use std::collections::HashMap;

use crate::{blocks::hints::{HintStorage, MIRValueHint}, ctx::{MIRBlockContext, MIRContext}, insts::{MIRInstruction, val::InstructionValue}, vals::base::BaseMIRValue};

pub mod refer;
pub mod hints;

/// The type of variable inside of a MIR block.
pub enum MIRBlockVariableType {
	/// SSAs, allow for direct register usage.
	/// Requires:
	/// - Variable's address not being obtained (value never referenced)
	SSA,

	/// Pointer value, uses the stack. 
	/// Should be used incase SSA fails.
	Pointer
}

pub struct MIRBlockVariableSSAHint {
	pub kind: MIRBlockVariableType,
	pub hint: Option<BaseMIRValue>
}

/// Represents a function block or a branch.
pub struct MIRBlock {
	instructions: Vec<MIRInstruction>,

	/// Hints for the index of the SSA value for the given variable. Will be the pointer value if the variable is not SSA.
	pub variables: HashMap<usize, MIRBlockVariableSSAHint>,

	#[deprecated(note = "Replaced by MIRContext::ssa_hints")]
	pub hints: HintStorage,
	pub ctx: MIRBlockContext
}

impl MIRBlock {
	pub fn new() -> Self {
		MIRBlock { instructions: vec![], variables: HashMap::new(), hints: HintStorage::new(), ctx: MIRBlockContext::new() }
	}

	pub fn append(&mut self, ctx: &mut MIRContext, instruction: MIRInstruction) -> InstructionValue {
		self.instructions.push(instruction.clone());

		if instruction.has_return(ctx) {
			let ret = instruction.get_return_type(ctx, self);

			// Hacky way of skipping hinting
			if !instruction.should_hint() {
				let hint_ind = ctx.ssa_hints.vec.len();

				return InstructionValue::new(Some(BaseMIRValue::new(hint_ind, ret)))
			}

			let hint_ind = ctx.ssa_hints.append_hint(MIRValueHint::Value(ret.clone()));

			return InstructionValue::new(Some(BaseMIRValue::new(hint_ind, ret)));
		}

		return InstructionValue::new(None);
	}
}