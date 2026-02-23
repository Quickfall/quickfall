use std::{fs, io};

use commons::Position;

#[derive(Clone, Debug)]
pub struct BoundPosition {
	pub start: Position,
	pub end: Position
}

impl BoundPosition {
	pub fn from_size(start: Position, size: usize) -> Self {
		return BoundPosition { start: start.clone(), end: start.increment_by(size) }
	}
}