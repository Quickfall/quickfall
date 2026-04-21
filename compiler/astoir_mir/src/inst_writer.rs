//! A way to avoid Rust borrowing errors by delegating the Instruction appending logic directly to the MIRContext instead of the builder functions

use crate::blocks::refer::MIRBlockReference;

#[derive(Clone)]
pub enum BlockPosition {
    START,
    END,
}

/// The main instruction. Contains the current position
pub struct InstructionWriterPosition {
    pub curr_block: MIRBlockReference,
    pub curr_inst: BlockPosition,
}

impl InstructionWriterPosition {
    pub fn move_start(&mut self, block: MIRBlockReference) {
        self.curr_block = block;
        self.curr_inst = BlockPosition::START;
    }

    pub fn move_end(&mut self, block: MIRBlockReference) {
        self.curr_block = block;

        self.curr_inst = BlockPosition::END;
    }
}
