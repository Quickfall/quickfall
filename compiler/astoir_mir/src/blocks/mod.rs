use crate::{blocks::hints::HintStorage, ctx::{MIRBlockContext, MIRContext}, insts::{MIRInstruction, val::InstructionValue}, vals::base::BaseMIRValue};

pub mod refer;
pub mod hints;

/// Represents a function block or a branch.
pub struct MIRBlock {
	instructions: Vec<MIRInstruction>,
	pub hints: HintStorage,
	pub ctx: MIRBlockContext
}

impl MIRBlock {
	pub fn new() -> Self {
		MIRBlock { instructions: vec![], hints: HintStorage::new(), ctx: MIRBlockContext::new() }
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