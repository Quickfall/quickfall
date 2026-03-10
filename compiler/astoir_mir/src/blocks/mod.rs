use crate::insts::MIRInstruction;

pub mod refer;

/// Represents a function block or a branch.
pub struct MIRBlock {
	instructions: Vec<MIRInstruction>
}

impl MIRBlock {
	pub fn append(&mut self, instruction: MIRInstruction) {
		self.instructions.push(instruction);
	}
}