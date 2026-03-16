use crate::{blocks::{MIRBlock, hints::{HintStorage, MIRValueHint}, refer::MIRBlockReference}, funcs::MIRFunction, inst_writer::{BlockPosition, InstructionWriterPosition}, insts::{MIRInstruction, val::InstructionValue}, vals::{base::BaseMIRValue, ptr::MIRPointerValue}};


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

}