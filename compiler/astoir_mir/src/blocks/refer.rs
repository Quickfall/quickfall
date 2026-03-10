pub struct MIRBlockReference {
	block_ind: usize
}

impl MIRBlockReference {
	pub fn new(block_id: usize) -> Self {
		return MIRBlockReference { block_ind: block_id }
	}
}