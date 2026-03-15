use std::collections::HashMap;

use crate::{blocks::{hints::MIRValueHint, refer::MIRBlockReference}, ctx::MIRContext, insts::{MIRInstruction, val::InstructionValue}, vals::base::BaseMIRValue};

pub mod refer;
pub mod hints;

/// The type of variable inside of a MIR block.
#[derive(Clone)]
pub enum MIRBlockVariableType {
	/// SSAs, allow for direct register usage.
	/// Requires:
	/// - Variable's address not being obtained (value never referenced)
	SSA,

	/// Pointer value, uses the stack. 
	/// Should be used incase SSA fails.
	Pointer
}

#[derive(Clone)]
pub struct MIRBlockVariableSSAHint {
	pub kind: MIRBlockVariableType,
	pub hint: Option<BaseMIRValue>
}

/// Represents a function block or a branch.
pub struct MIRBlock {
	instructions: Vec<MIRInstruction>,

	/// The block references that will merge into this one
	pub merge_blocks: Vec<MIRBlockReference>,

	/// Hints for the index of the SSA value for the given variable. Will be the pointer value if the variable is not SSA.
	pub variables: HashMap<usize, MIRBlockVariableSSAHint>
}

impl MIRBlock {
	pub fn new() -> Self {
		MIRBlock { instructions: vec![], variables: HashMap::new(), merge_blocks: vec![] }
	}

	pub fn new_merge(base: &mut MIRBlock, ctx: &mut MIRContext) -> MIRBlockReference {
		let ind = ctx.create_block();

		let block = &mut ctx.blocks[ind];

		for (ind, hint) in base.variables.iter() {
			block.variables.insert(*ind, hint.clone());
		}

		return ind;
	}

	pub fn append(&mut self, ctx: &mut MIRContext, instruction: MIRInstruction) -> InstructionValue {
		self.instructions.push(instruction.clone());

		if instruction.has_return(ctx) {
			let ret = instruction.get_return_type(ctx);

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