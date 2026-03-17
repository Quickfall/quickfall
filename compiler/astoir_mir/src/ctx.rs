use compiler_errors::errs::BaseResult;

use crate::{blocks::{MIRBlock, hints::{HintStorage, MIRValueHint}, refer::MIRBlockReference}, builder::build_phi, funcs::MIRFunction, inst_writer::{BlockPosition, InstructionWriterPosition}, insts::{MIRInstruction, val::InstructionValue}, vals::{base::BaseMIRValue}};


pub struct MIRContext {
	pub functions: Vec<MIRFunction>,
	pub blocks: Vec<MIRBlock>,
	pub writer: InstructionWriterPosition,

	pub ssa_hints: HintStorage,
}

impl MIRContext {
	pub fn new() -> Self {
		MIRContext { functions: vec![], ssa_hints: HintStorage::new(), blocks: vec![], writer: InstructionWriterPosition { curr_block: 0, curr_inst: BlockPosition::START } }
	}

	pub fn create_block(&mut self) -> MIRBlockReference {
		let ind = self.blocks.len();

		self.blocks.push(MIRBlock::new(ind));

		return ind;
	}

	pub fn append_function(&mut self, func: MIRFunction) -> usize {
		let ind = self.functions.len();

		self.functions.push(func);

		return ind;
	}

	pub fn append_inst(&mut self, inst: MIRInstruction) -> InstructionValue {
		match self.writer.curr_inst {
			BlockPosition::START => self.blocks[self.writer.curr_block].append_start(inst.clone()),
			BlockPosition::END => self.blocks[self.writer.curr_block].append(inst.clone())
		};

		if inst.has_return(self) {
			let ret = inst.get_return_type(self);

			if !inst.should_hint() {
				let hint_ind = self.ssa_hints.vec.len();

				return InstructionValue::new(Some(BaseMIRValue::new(hint_ind, ret)))
			}

			let hint_ind = self.ssa_hints.append_hint(MIRValueHint::Value(ret.clone()));

			return InstructionValue::new(Some(BaseMIRValue::new(hint_ind, ret)));
		}

		return InstructionValue::new(None);
	}

	/// Resolve the different SSA values for the given merge blocks
	pub fn resolve_ssa(&mut self, block: MIRBlockReference) -> BaseResult<bool> {
		let mut vals = vec![];

		let b = &self.blocks[block]; 

		for (ind, hint) in b.variables.iter() {
			let mut choices: Vec<(MIRBlockReference, BaseMIRValue)> = vec![];

			for block_ref in &b.merge_blocks {
				let block = &self.blocks[*block_ref];
				let block_hint = &block.variables[ind];

				if hint != block_hint && block_hint.hint.is_some() {
					choices.push((*block_ref, block_hint.hint.clone().unwrap()));
				}
			}

			vals.push((*ind, choices));
		}

		self.writer.move_end(block);
		
		for val in vals {
			let res = build_phi(self, val.1)?;

			let mut hint = self.blocks[block].variables[&val.0].clone();
			hint.hint = Some(res);

			self.blocks[block].variables.insert(val.0, hint);
		}

		return Ok(true);
	}

}