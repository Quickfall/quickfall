//! Normal bounds are kind of bounds that allow to require specific functions or fields on the given type. 
//! Thus allowing you to use these with generics.

use diagnostics::{MaybeDiagnostic, builders::{make_bound_fail_field, make_bound_fail_function}};

use crate::{TypedResolvedFunction, storage::TypeStorage, tree::Type};

pub struct NormalBound {
	pub functions: Vec<(u64, TypedResolvedFunction)>,
	pub fields: Vec<(u64, Type)>
}

impl NormalBound {
	#[must_use = "Must set the diagnostic position beforehand"]
	pub fn matches(&self, t: &Type, storage: &TypeStorage) -> MaybeDiagnostic {
		for function in &self.functions {
			let func = t.get_function(storage, function.0)?.1;

			let mut ind = 0;
			for (_, arg) in func.0 {
				if !function.1.0[ind].1.is_truly_eq(&arg.resolve(t)) {
					return Err(make_bound_fail_function(&"unnamed".to_string(), &"unnamed".to_string(), &function.0, ind).into())
				}
				
				ind += 1;
			}
		}

		for field in &self.fields {
			let ff = t.get_field(storage, field.0)?.1;

			let resolved = ff.resolve(t);

			if !field.1.is_truly_eq(&resolved) {
				return Err(make_bound_fail_field(&"unnamed".to_string(), &"unnamed".to_string(), &field.0, &resolved, &field.1).into())
			}
 		}

		return Ok(())
	}
}