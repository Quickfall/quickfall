//! Declarations for stated types.

use crate::tree::Type;

/// Represents a variable type. Can either be inferred or fully enforced
pub struct StatedType {
	pub raw_type: Type,
	pub infered: bool
}