//! Base-error related declarations

use crate::errs::{ErrorKind, normal::CompilerError};

/// Base errors are errors originating from the IR. They do not contain positions or other things
#[derive(Clone)]
pub struct BaseError {
	pub kind: ErrorKind,
	pub str: String,
}

impl BaseError {
	pub fn err(str: String) -> Self {
		return BaseError { kind: ErrorKind::Error, str }
	}

	pub fn warn(str: String) -> Self {
		return BaseError { kind: ErrorKind::Warn, str }
	}

	pub fn critical(str: String) -> Self {
		let e =  BaseError { kind: ErrorKind::Critical, str };

		CompilerError::from_base_posless(e.clone());

		return e;
	}
}