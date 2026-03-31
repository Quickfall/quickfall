//! Normal bounds are kind of bounds that allow to require specific functions or fields on the given type. 
//! Thus allowing you to use these with generics.

use compiler_errors::{TYPE_BOUND_MISSING, errs::{BaseResult, base::BaseError}};

use crate::{TypedFunction, TypedResolvedFunction, references::TypeReference, storage::TypeStorage, tree::Type};

pub struct NormalBound {
	pub functions: Vec<(u64, TypedResolvedFunction)>,
	pub fields: Vec<(u64, Type)>
}

impl NormalBound {
	pub fn matches(&self, t: &Type, storage: &TypeStorage) -> BaseResult<()> {
		for function in &self.functions {
			let func = t.get_function(storage, function.0)?.1;

			let mut ind = 0;
			for (_, arg) in func.0 {
				if !function.1.0[ind].1.is_truly_eq(&arg.resolve(t)) {
					return Err(BaseError::err(TYPE_BOUND_MISSING!().to_string()))
				}
				
				ind += 1;
			}
		}

		for field in &self.fields {
			let ff = t.get_field(storage, field.0)?.1;

			if !field.1.is_truly_eq(&ff.resolve(t)) {
				return Err(BaseError::err(TYPE_BOUND_MISSING!().to_string()))
			}
 		}

		return Ok(())
	}
}