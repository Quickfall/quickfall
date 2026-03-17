use compiler_errors::errs::{BaseResult, base::BaseError};

use crate::{blocks::{refer::MIRBlockReference}, builder::{build_load, build_store}, ctx::MIRContext, vals::{base::BaseMIRValue, ptr::MIRPointerValue}};

/// Represents a reference to a variable. This is not a reference to individual SSA values.
/// 
/// # Variables
/// Variables can be either:
/// - A pointer (using `store` and `load`) 
/// - SSA handled variable (stores variable ID -> current SSA value for the variable)
/// 
/// # Finding Guarantee
/// This reference guarantees to point to valid variable within a function. Please note that a reference can **only** be used in the function it was created in.
/// No Safety has been put in place to limit these errors as these references should never leave their origin function anyways
/// 
/// # Usage
/// This can be used to reference any true variable to modify it's content or read it. This will automatically handle diverse instructions needed such as `load` and `store`.
pub enum MIRVariableReference {
	PointerReference(MIRPointerValue),
	SSAReference(usize)
}

impl MIRVariableReference {
	pub fn read(&self, block: MIRBlockReference, ctx: &mut MIRContext) -> BaseResult<BaseMIRValue> {
		if self.is_pointer_ref() {
			let ptr_ref = self.as_pointer_ref()?;

			let res = build_load(ctx, ptr_ref)?;

			return Ok(res);
		}

		let ind = self.as_ssa_ref()?;

		return match &ctx.blocks[block].variables[&ind].hint {
			Some(v) => Ok(v.clone()),
			None => Err(BaseError::err("Cannot unpack SSA reference for variable in MIRVariableReference::read".to_string()))
		}
	}

	pub fn write(&self, block: MIRBlockReference, ctx: &mut MIRContext, val: BaseMIRValue) -> BaseResult<bool> {
		if self.is_pointer_ref() {
			let ptr_ref = self.as_pointer_ref()?;

			build_store(ctx, ptr_ref, val)?;

			return Ok(true);
		}

		let ind = self.as_ssa_ref()?;

		let block = &mut ctx.blocks[block];

		if block.variables[&ind].hint.is_some() && block.variables[&ind].hint.clone().unwrap().vtype != val.vtype {
			return Err(BaseError::err("Cannot write on this variable reference since the two types differ!".to_string()));
		}

		let mut hint = block.variables[&ind].clone();
		hint.hint = Some(val);

		block.variables.insert(ind, hint);

		return Ok(true);
	} 

	pub fn get_hint(&self) -> usize {
		return match self {
			Self::PointerReference(e) => {
				let clone: BaseMIRValue = e.clone().into();

				return clone.get_ssa_index();
			},
			Self::SSAReference(e) => *e
		}
	}

	pub fn is_pointer_ref(&self) -> bool {
		return match self {
			Self::PointerReference(_) => true,
			_ => false
		}
	}

	pub fn as_pointer_ref(&self) -> BaseResult<MIRPointerValue> {
		return match self {
			Self::PointerReference(e) => Ok(e.clone()),
			_ => Err(BaseError::err("as_pointer_ref requires a pointer var ref!".to_string()))
		}
	}

	pub fn as_ssa_ref(&self) -> BaseResult<usize> {
		return match self {
			Self::SSAReference(e) => Ok(*e),
			_ => Err(BaseError::err("as_ssa_ref requires a SSA var ref!".to_string()))
		}
	}

	pub fn is_ssa_ref(&self) -> bool {
		return !self.is_pointer_ref();
	}
}

impl From<MIRPointerValue> for MIRVariableReference {
	fn from(value: MIRPointerValue) -> Self {
		return MIRVariableReference::PointerReference(value)
	}
}

impl From<usize> for MIRVariableReference {
	fn from(value: usize) -> Self {
		return MIRVariableReference::SSAReference(value)
	}
}