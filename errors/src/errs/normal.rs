use crate::{errs::{ErrorKind, base::BaseError}, pos::ErrorPosition};

/// The normal type of error used by the Quickfall compiler. Can be cleanly formatted or passed to the language server.
pub struct CompilerError {
	pub kind: ErrorKind,
	pub str: String,

	pub pos: Option<ErrorPosition>
}

impl CompilerError {
	pub fn from_base(base: BaseError, pos: ErrorPosition) -> Self {
		return CompilerError { kind: base.kind, str: base.str, pos: Some(pos) }
	}

	pub fn from_base_posless(base: BaseError) -> Self {
		return CompilerError { kind: base.kind, str: base.str, pos: None }
	}
}