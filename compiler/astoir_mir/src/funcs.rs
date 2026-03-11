use compiler_errors::errs::{BaseResult, base::BaseError};
use compiler_utils::hash::HashedString;

use crate::{blocks::{MIRBlock, refer::MIRBlockReference}, vals::base::BaseValueType};

/// Represents a function in the MIR. Owns one or more blocks
pub struct MIRFunction {
	/// The block storage. index 0 is entry block
	pub blocks: Vec<MIRBlock>,
	pub name: HashedString,

	pub arguments: Vec<BaseValueType>,
	pub return_type: Option<BaseValueType>
}

impl MIRFunction {
	pub fn new(name: String, arguments: Vec<BaseValueType>, return_type: Option<BaseValueType>) -> Self {
		return MIRFunction { blocks: vec![], name: HashedString::new(name), arguments, return_type }
	}

	pub fn append_entry_block(&mut self) -> BaseResult<MIRBlockReference> {
		if !self.blocks.is_empty() {
			return Err(BaseError::err("Tried using append_entry_block on non-empty function blocks!".to_string()))
		}

		let ind = self.blocks.len();

		self.blocks.push(MIRBlock::new());

		return Ok(MIRBlockReference::new(ind, self));
	}

	pub fn append_block(&mut self) -> BaseResult<MIRBlockReference> {
		if self.blocks.is_empty() {
			return Err(BaseError::err("Tried using append_block on empty function blocks!".to_string()))
		}

		let ind = self.blocks.len();

		self.blocks.push(MIRBlock::new());

		return Ok(MIRBlockReference::new(ind, self))
	}

}