//! The bridge between Quickfall MIR and LLVM IR.

use std::rc::Rc;

use astoir_mir::ctx::MIRContext;
use compiler_errors::errs::{BaseResult, dump_errors};
use inkwell::context::Context;

use crate::{blocks::bridge_llvm_blocks, ctx::LLVMBridgeContext, funcs::bridge_llvm_functions};

pub mod ctx;
pub mod utils;
pub mod blocks;
pub mod types;
pub mod funcs;
pub mod insts;

pub fn bridge_llvm(mir: &MIRContext) -> BaseResult<LLVMBridgeContext> {
	let ctx = Rc::new(Context::create());

	let mut ctx = LLVMBridgeContext::new(ctx);

	bridge_llvm_functions(mir, &mut ctx)?;
	bridge_llvm_blocks(mir, &mut ctx)?;

	dump_errors();

	return Ok(ctx);
}

#[macro_export]
macro_rules! llvm_to_base  {
	($exp:expr) => {
		match $exp {
			Ok(v) => v,
			Err(e) => return Err(compiler_errors::errs::base::BaseError::new(compiler_errors::errs::ErrorKind::Critical, format!(compiler_errors::INKWELL_FUNC_FAILED!(), "unamed", e)))
		}
	};
}

#[macro_export]
macro_rules! llvm_to_base_returnless  {
	($exp:expr) => {
		match $exp {
			Ok(_) => {},
			Err(e) => return Err(compiler_errors::errs::base::BaseError::new(compiler_errors::errs::ErrorKind::Critical, format!(compiler_errors::INKWELL_FUNC_FAILED!(), "unamed", e)))
		}
	};
}