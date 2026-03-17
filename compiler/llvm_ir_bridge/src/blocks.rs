use astoir_mir::ctx::MIRContext;

use crate::ctx::LLVMBridgeContext;

pub fn bridge_llvm_blocks(mir: &MIRContext, bridge: &mut LLVMBridgeContext) {
	for func in &mir.functions {
		for block in &mir.blocks {
			//bridge.ctx.append_basic_block(function, name)
		}
	}
}