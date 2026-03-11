use crate::{blocks::hints::HintStorage, insts::{MIRInstruction, val::InstructionValue}, vals::base::BaseMIRValue};

pub mod refer;
pub mod hints;

/// Represents a function block or a branch.
pub struct MIRBlock {
	instructions: Vec<MIRInstruction>,
	pub hints: HintStorage
}

impl MIRBlock {
	pub fn new() -> Self {
		MIRBlock { instructions: vec![], hints: HintStorage::new() }
	}

	pub fn append(&mut self, instruction: MIRInstruction) -> InstructionValue {
		self.instructions.push(instruction.clone());

		let ind = self.instructions.len() - 1;

		if instruction.has_return() {
			return InstructionValue::new(Some(BaseMIRValue::new(ind, instruction.get_return_type())));
		}

		return InstructionValue::new(None);
	}
}