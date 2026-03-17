use std::{collections::HashMap, fmt::Display};

use compiler_errors::errs::{BaseResult, base::BaseError};

use crate::{blocks::refer::MIRBlockReference, builder::build_phi, ctx::MIRContext, inst_writer::BlockPosition, insts::MIRInstruction, vals::{base::BaseMIRValue, refer::MIRVariableReference}};

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

#[derive(Clone)]
pub enum MIRBlockHeldInstruction {
	Valueless(MIRInstruction),
	Valued(MIRInstruction, usize)
}

/// Represents a function block or a branch.
pub struct MIRBlock {
	instructions: Vec<MIRBlockHeldInstruction>,

	/// The block references that will merge into this one
	pub merge_blocks: Vec<MIRBlockReference>,

	pub self_ref: MIRBlockReference,

	/// Hints for the index of the SSA value for the given variable. Will be the pointer value if the variable is not SSA.
	/// The indexes used are the variable indexes and not the SSA indexes.
	pub variables: HashMap<usize, MIRBlockVariableSSAHint>
}

impl MIRBlock {
	pub fn new(self_ref: MIRBlockReference) -> Self {
		MIRBlock { instructions: vec![], variables: HashMap::new(), merge_blocks: vec![], self_ref }
	}

	pub fn new_merge(base: MIRBlockReference, ctx: &mut MIRContext, append_to_merge_blocks: bool) -> MIRBlockReference {
		let ind = ctx.create_block();

		let variables = ctx.blocks[base].variables.clone();

		let block = &mut ctx.blocks[ind];

		for (ind, hint) in variables {
			block.variables.insert(ind, hint);
		}

		if append_to_merge_blocks {
			ctx.blocks[base].merge_blocks.push(ind)
		}

		return ind;
	}

	pub fn get_variable_ref(&self, var_ind: usize) -> BaseResult<MIRVariableReference> {
		let var = &self.variables[&var_ind];

		if var.kind == MIRBlockVariableType::SSA {
			return Ok(MIRVariableReference::from(var_ind));
		}

		let unpacked = match &var.hint {
			Some(v) => v.clone(),
			None => return Err(BaseError::err("Missing BaseMIRValue in pointer hint".to_string()))
		};

		return Ok(MIRVariableReference::from(unpacked.as_ptr()?));
	}

	pub fn append(&mut self, instruction: MIRBlockHeldInstruction, pos: &BlockPosition) {
		match pos {
			BlockPosition::END => {
				self.instructions.push(instruction.clone());
			},

			BlockPosition::START => {
				if self.instructions.is_empty() {
					self.instructions.push(instruction.clone());
				} else {
					self.instructions.insert(0, instruction.clone());
				}
			}
		}
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

		ctx.writer.move_end(self.self_ref);
		
		for val in vals {
			let res = build_phi(ctx, val.1)?;

			let mut hint = self.variables[&val.0].clone();
			hint.hint = Some(res);

			self.variables.insert(val.0, hint);
		}

		return Ok(true);
	} 

	pub fn is_empty(&self) -> bool {
		return self.instructions.is_empty();
	}
}

impl Display for MIRBlock {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		writeln!(f, "%block_{}", self.self_ref)?;

		if !self.merge_blocks.is_empty() {
			writeln!(f, "merge_blocks")?;

			for block in &self.merge_blocks {
				writeln!(f, "- {}", block)?;
			}
		}

		for inst in &self.instructions {
			write!(f, "	{}", inst)?;
		}

		Ok(())
	}
}

impl Display for MIRBlockHeldInstruction {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Valued(a, b) => write!(f, "#{} = {}", *b, a),
			Self::Valueless(a) => write!(f, "{}", a)
		}
	}
}