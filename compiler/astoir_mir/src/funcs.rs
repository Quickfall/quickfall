use std::fmt::Display;

use astoir_typing::compacted::CompactedType;
use compiler_errors::errs::{BaseResult, base::BaseError};
use compiler_utils::hash::HashedString;

use crate::{blocks::{refer::{MIRBlockReference}}, ctx::MIRContext};

/// Represents a function in the MIR. Owns one or more blocks
pub struct MIRFunction {
	/// The block storage. index 0 is entry block
	pub blocks: Vec<MIRBlockReference>,
	pub name: HashedString,

	/// This will prevent the function from being usable by normal function calls if true
	pub is_from_struct: bool, 

	pub arguments: Vec<CompactedType>,
	pub return_type: Option<CompactedType>
}

impl MIRFunction {
	pub fn new(name: String, arguments: Vec<CompactedType>, return_type: Option<CompactedType>, is_from_struct: bool) -> Self {
		return MIRFunction { blocks: vec![], name: HashedString::new(name), arguments, return_type, is_from_struct }
	}

	pub fn append_entry_block(&mut self, ctx: &mut MIRContext) -> BaseResult<MIRBlockReference> {
		if !self.blocks.is_empty() {
			return Err(BaseError::err("Tried using append_entry_block on non-empty function blocks!".to_string()))
		}

		let reference = ctx.create_block();

		self.blocks.push(reference);

		return Ok(reference);
	}

	pub fn append_block(&mut self, ctx: &mut MIRContext) -> BaseResult<MIRBlockReference> {
		if self.blocks.is_empty() {
			return Err(BaseError::err("Tried using append_block on empty function blocks!".to_string()))
		}

		let reference = ctx.create_block();

		self.blocks.push(reference);

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