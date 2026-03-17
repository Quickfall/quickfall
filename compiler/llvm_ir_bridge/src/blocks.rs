use astoir_mir::ctx::MIRContext;
use compiler_errors::errs::{BaseResult};

use crate::{ctx::LLVMBridgeContext, insts::bridge_llvm_instruction, llvm_to_base};

pub fn bridge_llvm_blocks(mir: &MIRContext, bridge: &mut LLVMBridgeContext) -> BaseResult<()> {
	for block in &mir.blocks {
		bridge.builder.position_at_end(bridge.blocks[&block.self_ref].clone().inner);

		for inst in block.instructions.clone() {
			println!("res: {}", inst);

			let res = bridge_llvm_instruction(inst.clone(), bridge, mir)?;

			if res.is_some() {
				bridge.values.insert(inst.as_valuedindex()?, res.unwrap());
			}
		}

		llvm_to_base!(bridge.builder.build_return(None));
	}

	return Ok(());
}