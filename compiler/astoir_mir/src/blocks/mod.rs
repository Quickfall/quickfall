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

/// Represents a function block or a branch.
pub struct MIRBlock {
	instructions: Vec<MIRInstruction>,

	/// Hints for the index of the SSA value for the given variable. Will be the pointer value if the variable is not SSA.
	pub variable_hints: Vec<usize>, 
	pub variable_kinds: Vec<MIRBlockVariableType>,

	pub hints: HintStorage,
	pub ctx: MIRBlockContext
}

impl MIRBlock {
	pub fn new() -> Self {
		MIRBlock { instructions: vec![], variable_hints: vec![], variable_kinds: vec![], hints: HintStorage::new(), ctx: MIRBlockContext::new() }
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