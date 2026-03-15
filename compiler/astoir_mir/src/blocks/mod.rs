use std::{collections::HashMap, f32::consts::E};

use compiler_errors::errs::BaseResult;

use crate::{blocks::{hints::MIRValueHint, refer::MIRBlockReference}, builder::build_phi, ctx::MIRContext, insts::{MIRInstruction, val::InstructionValue}, vals::base::BaseMIRValue};

pub mod refer;
pub mod hints;

/// The type of variable inside of a MIR block.
#[derive(Clone, PartialEq)]
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

impl PartialEq for MIRBlockVariableSSAHint {
	fn eq(&self, other: &Self) -> bool {
		return self.kind == other.kind && self.hint == other.hint;
	}
}

/// Represents a function block or a branch.
pub struct MIRBlock {
	instructions: Vec<MIRInstruction>,

	/// The block references that will merge into this one
	pub merge_blocks: Vec<MIRBlockReference>,

	/// Hints for the index of the SSA value for the given variable. Will be the pointer value if the variable is not SSA.
	/// The indexes used are the variable indexes and not the SSA indexes.
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

	/// Resolves changes in SSA handled variables from the different merge blocks.
	/// 
	/// # Behavior
	/// First checks inside of every merge blocks for changes of SSA values for variables in the hinting table.
	/// Then uses a `phi` instruction to obtain the SSA values in this block. Also automatically updates the variable hints inside of this block.
	///
	pub fn resolve_ssa_changes(&mut self, ctx: &mut MIRContext) -> BaseResult<bool> {
		let mut vals = vec![];

		for (ind, hint) in self.variables.iter() {
			let mut choices: Vec<(MIRBlockReference, BaseMIRValue)> = vec![];

			for block_ref in &self.merge_blocks {
				let block = &ctx.blocks[*block_ref];
				let block_hint = &block.variables[ind];

				if hint != block_hint && block_hint.hint.is_some() {
					choices.push((*block_ref, block_hint.hint.clone().unwrap()));
				}
			}

			vals.push((*ind, choices));
		}

		for val in vals {
			let res = build_phi(ctx, self, val.1)?;

			let mut hint = self.variables[&val.0].clone();
			hint.hint = Some(res);

			self.variables.insert(val.0, hint);
		}

		return Ok(true);
	} 

}