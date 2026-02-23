use crate::{errs::{ERR_STORAGE, ErrorKind, base::BaseError}, pos::{BoundPosition}};

/// The normal type of error used by the Quickfall compiler. Can be cleanly formatted or passed to the language server.
#[derive(Clone)]
pub struct CompilerError {
	pub kind: ErrorKind,
	pub str: String,

	pub pos: Option<BoundPosition>
}

impl CompilerError {
	pub fn from_base(base: BaseError, pos: BoundPosition) -> Self {
		let err = CompilerError { kind: base.kind, str: base.str, pos: Some(pos) };

		ERR_STORAGE.with_borrow_mut(|s| s.errs.push(err.clone()));

		return err;
	}

	pub fn new(kind: ErrorKind, str: String, pos: BoundPosition) -> Self {
		let err = CompilerError { kind, str, pos: Some(pos)};

		ERR_STORAGE.with_borrow_mut(|s| s.errs.push(err.clone()));

		return err;
	}

	pub fn from_base_posless(base: BaseError) -> Self {
		let err = CompilerError { kind: base.kind, str: base.str, pos: None };

		ERR_STORAGE.with_borrow_mut(|s| s.errs.push(err.clone()));
	
		return err;
	}
}