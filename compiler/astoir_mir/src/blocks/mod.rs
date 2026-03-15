use std::collections::HashMap;

use crate::{blocks::hints::HintStorage, ctx::{MIRBlockContext, MIRContext}, insts::{MIRInstruction, val::InstructionValue}, vals::base::BaseMIRValue};

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

	pub hints: HintStorage,
	pub ctx: MIRBlockContext
}

impl MIRBlock {
	pub fn new() -> Self {
		MIRBlock { instructions: vec![], variables: HashMap::new(), hints: HintStorage::new(), ctx: MIRBlockContext::new() }
	}

	pub fn append(&mut self, ctx: &MIRContext, instruction: MIRInstruction) -> InstructionValue {
		self.instructions.push(instruction.clone());

		let ind = self.instructions.len() - 1;

		if instruction.has_return(ctx) {
			return InstructionValue::new(Some(BaseMIRValue::new(ind, instruction.get_return_type(ctx, self))));
		}

		return InstructionValue::new(None);
	}
}