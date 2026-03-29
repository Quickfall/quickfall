use compiler_errors::errs::{BaseResult, base::BaseError};

#[derive(Clone, Debug)]
pub enum MIRConstantValue {
	Int(u128),
	Float(f64)
}

impl MIRConstantValue {
	pub fn as_int(&self) -> BaseResult<u128> {
		return match self {
			MIRConstantValue::Int(v) => Ok(*v),
			_ => Err(BaseError::err("Cannot use as_int on MIRConstantValue if it isn't int!".to_string()))
		}
	}

	pub fn as_float(&self) -> BaseResult<f64> {
		return match self {
			MIRConstantValue::Float(v) => Ok(*v),
			_ => Err(BaseError::err("Cannot use as_float on MIRConstantValue if it isn't float!".to_string()))
		}
	}
}