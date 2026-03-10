use crate::{insts::{MIRInstruction, val::InstructionValue}, vals::base::BaseMIRValue};

pub mod refer;

/// Represents a function block or a branch.
pub struct MIRBlock {
	instructions: Vec<MIRInstruction>
}

impl MIRBlock {
	pub fn append(&mut self, instruction: MIRInstruction) -> InstructionValue {
		self.instructions.push(instruction.clone());

		let ind = self.instructions.len() - 1;

		if instruction.has_return() {
			return InstructionValue::new(Some(BaseMIRValue::new(ind, instruction.get_return_type())));
		}

		return InstructionValue::new(None);
	}
}