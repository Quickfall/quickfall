use std::cell::{RefCell};

use crate::errs::{base::BaseError, normal::CompilerError};

pub mod base;
pub mod normal;

pub type BaseResult<K> = Result<K, BaseError>;
pub type CompilerResult<K> = Result<K, CompilerError>;

thread_local! {
	static ERR_STORAGE: RefCell<ErrorStorage> = RefCell::new(ErrorStorage { errs: Vec::new() });
}

#[derive(Clone)]
pub enum ErrorKind {
	Warn,
	Error,
	Critical
}

pub struct ErrorStorage {
	pub errs: Vec<CompilerError>
}