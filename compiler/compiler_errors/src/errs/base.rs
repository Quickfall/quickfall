//! Base-error related declarations

use crate::errs::{ErrorKind, IS_MIR_STAGE, normal::CompilerError};

/// Base errors are errors originating from the IR. They do not contain positions or other things
#[derive(Clone, Debug)]
pub struct BaseError {
	pub kind: ErrorKind,
	pub str: String,
}

impl BaseError {
	pub fn new(kind: ErrorKind, str: String) -> Self {
		let base = BaseError { kind, str };

		IS_MIR_STAGE.with(|e| {
			if *e.borrow() {
				CompilerError::from_base_posless(base.clone());
			}
		});

		return base;
	}

	pub fn err(str: String) -> Self {
		return BaseError::new(ErrorKind::Error, str);
	}

	pub fn warn(str: String) -> Self {
		return BaseError::new(ErrorKind::Warn, str);
	}

	pub fn critical(str: String) -> Self {
		return BaseError::new(ErrorKind::Critical, str);
	}
}