//! The bridge between Quickfall MIR and LLVM IR.

use std::rc::Rc;

use astoir_mir::ctx::MIRContext;
use compiler_errors::errs::BaseResult;
use inkwell::context::Context;

use crate::{ctx::LLVMBridgeContext, funcs::bridge_llvm_functions};

pub mod ctx;
pub mod utils;
pub mod blocks;
pub mod types;
pub mod funcs;

pub fn bridge_llvm(mir: &MIRContext) -> BaseResult<LLVMBridgeContext> {
	let ctx = Rc::new(Context::create());

	let mut ctx = LLVMBridgeContext::new(ctx);

	bridge_llvm_functions(mir, &mut ctx)?;

	return Ok(ctx);
}