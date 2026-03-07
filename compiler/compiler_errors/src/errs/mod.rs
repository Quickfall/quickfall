use std::cell::{RefCell};

use crate::errs::{base::BaseError, normal::{CompilerError, HeldError}};

pub mod base;
pub mod normal;

pub type BaseResult<K> = Result<K, BaseError>;
pub type CompilerResult<K> = Result<K, CompilerError>;

thread_local! {
	static ERR_STORAGE: RefCell<ErrorStorage> = RefCell::new(ErrorStorage { errs: Vec::new() });
}

#[derive(Clone, Debug)]
pub enum ErrorKind {
	Warn,
	Error,
	Critical
}

pub struct ErrorStorage {
	pub errs: Vec<HeldError>
}

pub fn dump_errors() {
	ERR_STORAGE.with_borrow(|f| {
		for err in &f.errs {
			println!("{}", err.err);

			if err.btrace.is_some() {
				println!("Captured in: \n{}", err.btrace.as_ref().unwrap());
			}
		}
	})
}

pub fn has_errors() -> bool {
	ERR_STORAGE.with_borrow(|f| {
		!f.errs.is_empty()
	})
}