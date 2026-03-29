use std::mem::transmute;

use astoir_mir::ctx::MIRContext;
use compiler_errors::errs::BaseResult;
use inkwell::{basic_block::BasicBlock, types::BasicType};

use crate::{ctx::LLVMBridgeContext, utils::{LLVMBlock, LLVMFunction}};

pub fn bridge_llvm_functions(mir: &MIRContext, bridge: &mut LLVMBridgeContext) -> BaseResult<()> {
	for func in &mir.functions {
		let mut args = vec![];

		if !func.blocks.is_empty() {
			for arg in &func.arguments {
				args.push(bridge.types.convert(arg.clone())?.inner.into());
			}
		}		

		let t = match &func.return_type {
			Some(ret) => bridge.types.convert(ret.clone())?.fn_type(&args, false),
			None => bridge.void_type.fn_type(&args, false)
		};

		let ff = bridge.module.add_function(&func.name.val, t, None);

		for block in &func.blocks {
			let b = bridge.ctx.append_basic_block(ff, "");

			bridge.blocks.insert(*block, LLVMBlock::new(unsafe { transmute::<BasicBlock, BasicBlock<'static>>(b) }));
		}

		bridge.functions.push(LLVMFunction::new(ff));
	}

	Ok(())
}