use std::mem::transmute;

use astoir_mir::ctx::MIRContext;
use compiler_errors::errs::BaseResult;
use inkwell::{basic_block::BasicBlock, types::BasicType};

use crate::{ctx::LLVMBridgeContext, utils::{LLVMBlock, get_block_name}};

pub fn bridge_llvm_functions(mir: &MIRContext, bridge: &mut LLVMBridgeContext) -> BaseResult<()> {
	for func in &mir.functions {
		let mut args = vec![];

		for arg in &func.arguments {
			args.push(bridge.types.convert(arg.base.clone())?.inner.into());
		}

		let t = match &func.return_type {
			Some(ret) => bridge.types.convert(ret.base.clone())?.fn_type(&args, false),
			None => bridge.void_type.fn_type(&args, false)
		};

		let ff = bridge.module.add_function(&func.name.val, t, None);

		for block in &func.blocks {
			let b = bridge.ctx.append_basic_block(ff, &get_block_name());

			bridge.blocks.insert(*block, LLVMBlock::new(unsafe { transmute::<BasicBlock, BasicBlock<'static>>(b) }));
		}
	}

	Ok(())
}