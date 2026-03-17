use std::fmt::Display;

use astoir_typing::compacted::CompactedType;
use compiler_errors::errs::{BaseResult, base::BaseError};
use compiler_utils::hash::HashedString;

use crate::{blocks::{MIRBlockVariableSSAHint, MIRBlockVariableType, refer::{self, MIRBlockReference}}, ctx::MIRContext, vals::base::BaseMIRValue};

/// Represents a function in the MIR. Owns one or more blocks
pub struct MIRFunction {
	/// The block storage. index 0 is entry block
	pub blocks: Vec<MIRBlockReference>,
	pub name: HashedString,
	
	pub id: usize,

	/// This will prevent the function from being usable by normal function calls if true
	pub is_from_struct: bool, 

	pub arguments: Vec<CompactedType>,
	pub return_type: Option<CompactedType>
}

impl MIRFunction {
	pub fn new(name: String, arguments: Vec<CompactedType>, return_type: Option<CompactedType>, is_from_struct: bool) -> Self {
		return MIRFunction { blocks: vec![], name: HashedString::new(name), arguments, return_type, is_from_struct, id: 0 }
	}

	pub fn append_entry_block(&mut self, ctx: &mut MIRContext) -> BaseResult<MIRBlockReference> {
		if !self.blocks.is_empty() {
			return Err(BaseError::err("Tried using append_entry_block on non-empty function blocks!".to_string()))
		}

		let reference = ctx.create_block_handled(self.id);

		let block = &ctx.blocks[reference];
		
		let mut ind = 0;
		for arg in &self.arguments {
			block.variables.insert(ind, MIRBlockVariableSSAHint { kind: MIRBlockVariableType::SSA, hint: Some(BaseMIRValue::new(ind, arg)) });

			ind += 1;
		}

		self.blocks.push(reference);

		return Ok(reference);
	}

	pub fn append_block(&mut self, ctx: &mut MIRContext) -> BaseResult<MIRBlockReference> {
		if self.blocks.is_empty() {
			return Err(BaseError::err("Tried using append_block on empty function blocks!".to_string()))
		}

		let reference = ctx.create_block(self.id);

		return Ok(reference)
	}
}

impl Display for MIRFunction {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		writeln!(f, ".func_{}_struct{}", self.name.val, self.is_from_struct)?;

		for block in &self.blocks {
			writeln!(f, "- block_{}", block)?;
		}

		Ok(())
	}
}