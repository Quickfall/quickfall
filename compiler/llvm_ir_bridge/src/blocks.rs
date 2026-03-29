use astoir_mir::{blocks::MIRBlock, ctx::MIRContext};
use compiler_errors::errs::{BaseResult};

use crate::{ctx::LLVMBridgeContext, insts::bridge_llvm_instruction};

pub fn bridge_llvm_blocks(mir: &MIRContext, bridge: &mut LLVMBridgeContext) -> BaseResult<()> {
	for block in &mir.blocks {
		for merge in &block.merge_blocks {
			let b = &mir.blocks[*merge];

			bridge_llvm_block(mir, mir.block_to_func[&block.self_ref], b, bridge)?;
		}

		bridge_llvm_block(mir, mir.block_to_func[&block.self_ref], block, bridge)?;
	}

	return Ok(());
}

pub fn bridge_llvm_block(mir: &MIRContext, func: usize, block: &MIRBlock, bridge: &mut LLVMBridgeContext) -> BaseResult<()> {
	if bridge.completed_blocks.contains(&block.self_ref) {
		return Ok(())
	}

	bridge.builder.position_at_end(bridge.blocks[&block.self_ref].clone().inner);

	for inst in block.instructions.clone() {
		let res = bridge_llvm_instruction(inst.clone(), func, bridge, mir)?;

		if res.is_some() {
			bridge.values.insert(inst.as_valuedindex()?, res.unwrap());
		}
	}

	bridge.completed_blocks.insert(block.self_ref);

	Ok(())
}