use compiler_errors::errs::BaseResult;

use crate::{insts::{MIRInstruction, val::InstructionValue}, vals::base::BaseMIRValue};

pub mod refer;

/// Represents a function block or a branch.
pub struct MIRBlock {
	instructions: Vec<MIRInstruction>
}

impl MIRBlock {
	pub fn append(&mut self, instruction: MIRInstruction) -> InstructionValue {
		self.instructions.push(instruction);

		if instruction.has_return() {
			return InstructionValue { val: Some(BaseMIRValue::) }
		}

	}
}