use core::fmt;

use colored::Colorize;

use crate::{IO_ERROR_READ, errs::{ERR_STORAGE, ErrorKind, base::BaseError}, pos::BoundPosition};

/// The normal type of error used by the Quickfall compiler. Can be cleanly formatted or passed to the language server.
#[derive(Clone, Debug)]
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

	pub fn get_kind_str(&self) -> String {
		match self.kind {
			ErrorKind::Critical => return "CRITICAL ERROR".bright_red().bold().underline().to_string(),
			ErrorKind::Error => return "ERROR".bright_red().bold().to_string(),
			ErrorKind::Warn => return "WARN".yellow().bold().to_string()
		}
	}

}

impl fmt::Display for CompilerError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		if self.pos.is_none() {
			writeln!(f, "{} at ??", self.get_kind_str())?;
		} else {
			writeln!(f, "{} at {}:{}", self.get_kind_str(), self.pos.as_ref().unwrap().start, self.pos.as_ref().unwrap().end)?;

			let startLine = match self.pos.as_ref().unwrap().start.get_line_content() {
				Ok(v) => v,
				Err(_) => IO_ERROR_READ!().to_string()
			};

			let endLine = match self.pos.as_ref().unwrap().end.get_line_content() {
				Ok(v) => v,
				Err(_) => IO_ERROR_READ!().to_string()
			};

			let before = &startLine[0..self.pos.as_ref().unwrap().start.col - 1];
			let target = self.pos.as_ref().unwrap().get_bound().cyan().underline();
			let after = &endLine[self.pos.as_ref().unwrap().end.col - 1..];

			writeln!(f, "{}{}{}", before, target, after)?;
		}

		writeln!(f, "")?;
		writeln!(f, "{}", self.str.bright_red())?;
		
		Ok(())
	}
}