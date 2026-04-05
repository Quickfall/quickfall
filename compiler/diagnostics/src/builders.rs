use std::{fmt::Display};

use crate::{DiagnosticSpanOrigin, diagnostic::{Diagnostic, Level, Span, SpanKind, SpanPosition}, errors::{ALREADY_IN_SCOPE, ASSIGN_DIFF_TYPE_IR, BOUND_MISSING, DIFF_SIZE_SPECIFIERS, ENUM_PARENT_FIELDS, ERA_NOT_EXIST, EXPECTED_FREE, EXPECTED_TOKEN, EXPECTED_TYPE, FIELD_MISSING, FIELD_STRUCT_INIT, FIND_TYPE, FIND_TYPE_FIELD, FIND_TYPE_FUNCTION, FIND_VAR, FUNC_MISSING, INDEX_USAGE, INVALID_POINTING, INVALID_TYPE_REQ, IR_CAST, IR_INSTRUCTION_HELD_VAL, MATH_OPERATION_ASSIGNS, TRAIT_MISSING, TYPE_NOT_PART, UNEXPECTED_TOKEN, VARIABLE_UNINIT}, get_current_diagnostic_pos, warnings::UNUSED_VAR};

pub fn make_expected_simple_error<K: DiagnosticSpanOrigin, E: Display, G: Display>(origin: &K, expected: &E, got: &G) -> Diagnostic {
	origin.make_simple_diagnostic(EXPECTED_TOKEN.0, Level::Error, format!("expected {} but got {}", expected, got), None, vec![], vec![], vec![])
}

pub fn make_unexpected_simple_error<K: DiagnosticSpanOrigin, E: Display>(origin: &K, got: &E) -> Diagnostic {
	origin.make_simple_diagnostic(UNEXPECTED_TOKEN.0, Level::Error, format!("unexpected {}", got), None, vec![], vec![], vec![])
}

pub fn make_unexpected_simple_error_outside<K: Display>(got: &K, pos: SpanPosition) -> Diagnostic {
	let span = Span::make_primary(pos, None);

	Diagnostic::new_base(Level::Error, EXPECTED_TOKEN.0, format!("unexpected {}", got), span, vec![], vec![], vec![])
}

pub fn make_expected_single_simple_error<K: DiagnosticSpanOrigin, E: Display>(origin: &K, got: &E) -> Diagnostic {
	origin.make_simple_diagnostic(EXPECTED_FREE.0, Level::Error, format!("expected {}", got), None, vec![], vec![], vec![])
}

pub fn make_unused_variable<K: DiagnosticSpanOrigin, E: Display>(origin: &K, var: &E) -> Diagnostic {
	origin.make_simple_diagnostic(UNUSED_VAR.0, Level::Warning, format!("unused variable {}", var), None, vec![], vec![], vec![])
}

pub fn make_already_in_scope<K: DiagnosticSpanOrigin, E: Display>(origin: &K, val: &E) -> Diagnostic {
	origin.make_simple_diagnostic(ALREADY_IN_SCOPE.0, Level::Error, format!("{} was already found in this scope", val), None, vec![], vec![], vec![])
}

pub fn make_cannot_find_var<K: DiagnosticSpanOrigin, V: Display>(origin: &K, var: &V) -> Diagnostic {
	origin.make_simple_diagnostic(FIND_VAR.0, Level::Error, format!("cannot find variable {} in the current context", var), None, vec![], vec![], vec![])
}

pub fn make_cannot_find_type<K: DiagnosticSpanOrigin, E: Display>(origin: &K, t: &E) -> Diagnostic {
	origin.make_simple_diagnostic(FIND_TYPE.0, Level::Error, format!("cannot find type {} in the current scope", t), None, vec![], vec![], vec![])
}

pub fn make_cannot_find_func<K: DiagnosticSpanOrigin, E: Display>(origin: &K, func: &E) -> Diagnostic {
	origin.make_simple_diagnostic(FIND_TYPE.0, Level::Error, format!("cannot find function {} in the current scope", func), None, vec![], vec![], vec![])
}

pub fn make_cannot_find_type_pos<E: Display>(t: &E) -> Diagnostic {
	let primary_span = Span::make_primary(get_current_diagnostic_pos(), None);

	return Diagnostic::new_base(Level::Error, FIND_TYPE.0, format!("cannot find type {} in the current scope", t), primary_span, vec![], vec![], vec![])
} 

#[must_use = "Must set the diagnostic position beforehand"]
pub fn make_cannot_find_type_function<F: Display, T: Display>(func: &F, t: &T) -> Diagnostic {
	let primary_span = Span::make_primary(get_current_diagnostic_pos(), Some(format!("function {} required here", func)));

	return Diagnostic::new_base(Level::Error, FIND_TYPE_FUNCTION.0, format!("cannot find function {} in type {}", func, t), primary_span, vec![], vec![], vec![])
}

#[must_use = "Must set the diagnostic position beforehand"]
pub fn make_cannot_find_type_field<F: Display, T: Display>(field: &F, t: &T) -> Diagnostic {
	let primary_span = Span::make_primary(get_current_diagnostic_pos(), Some(format!("field {} required here", field)));

	return Diagnostic::new_base(Level::Error, FIND_TYPE_FIELD.0, format!("cannot find field {} in type {}", field, t), primary_span, vec![], vec![], vec![])
}

pub fn make_diff_size_specifiers<K: DiagnosticSpanOrigin, A: Display, B: Display>(origin: &K, a: &A, b: &B) -> Diagnostic {
	origin.make_simple_diagnostic(DIFF_SIZE_SPECIFIERS.0, Level::Error, format!("expected {} size specifiers on this type, got {}", a, b), None, vec![], vec![], vec![])
}

pub fn make_diff_type<K: DiagnosticSpanOrigin, V: Display, T: Display, A: Display>(origin: &K, var_name: &V, req_type: &T, got: &A, var_origin: &K) -> Diagnostic {
	let decl_span = var_origin.make_span(SpanKind::Secondary, Some(format!("variable {} was declared here", var_name)));
	origin.make_simple_diagnostic(EXPECTED_TYPE.0, Level::Error, format!("expected {} but got {}", req_type, got), Some(format!("variable {} tried having a {} value here", var_name, req_type)), vec![decl_span], vec![], vec![])
}

pub fn make_diff_type_val<K: DiagnosticSpanOrigin, T: Display, A: Display>(origin: &K, req_type: &T, got: &A) -> Diagnostic {
	origin.make_simple_diagnostic(EXPECTED_TYPE.0, Level::Error, format!("expected {} but got {}", req_type, got), None, vec![], vec![], vec![])
}


#[must_use = "Must set the diagnostic position beforehand"]
pub fn make_bound_fail_function<B: Display, T: Display, F: Display>(bound: &B, target_t: &T, func: &F, arg_count: usize) -> Diagnostic {
	let primary_span = Span::make_primary(get_current_diagnostic_pos(), Some(format!("bound {} required for type {} here", bound, target_t)));

	let notes = vec![format!("function {} declared in bound doesn't match signature!", func)];
	let help = vec![format!("argument type for argument #{} is invalid!", arg_count)];

	return Diagnostic::new_base(Level::Error, BOUND_MISSING.0, format!("bound {} cannot be applied onto {}", bound, target_t), primary_span, vec![], notes, help)
}

#[must_use = "Must set the diagnostic position beforehand"]
pub fn make_bound_fail_field<B: Display, T: Display, F: Display, E: Display, G: Display>(bound: &B, target_t: &T, field: &F, expected: &E, got: &G) -> Diagnostic {
	let primary_span = Span::make_primary(get_current_diagnostic_pos(), Some(format!("bound {} required for type {} here", bound, target_t)));

	let notes = vec![format!("field {} declared in bound requires type {} but got {}", field, expected, got)];
	let help = vec![format!("replace the field type with {}", expected)];

	return Diagnostic::new_base(Level::Error, BOUND_MISSING.0, format!("bound {} cannot be applied onto {}", bound, target_t), primary_span, vec![], notes, help)
}

#[must_use = "Must set the diagnostic position beforehand"]
pub fn make_bound_trait<B: Display, T: Display>(bound_trait: &B, t: &T) -> Diagnostic {
	let primary_span = Span::make_primary(get_current_diagnostic_pos(), Some(format!("trait {} required for type {} here", bound_trait, t)));

	let notes = vec![format!("type {} doesn't support the bound trait {}", t, bound_trait)];
	let help = vec![format!("replace type {} with a type that supports {}", t, bound_trait)];

	return Diagnostic::new_base(Level::Error, TRAIT_MISSING.0, format!("bound trait {} cannot be applied onto {}", bound_trait, t), primary_span, vec![], notes, help)
}

#[must_use = "Must set the diagnostic position beforehand"]
pub fn make_enum_parent_fields() -> Diagnostic {
	let primary_span = Span::make_primary(get_current_diagnostic_pos(), None);

	let notes = vec!["enum fields are only allowed in enum children".to_string()];
	let help = vec!["move the fields into an enum child".to_string()];

	return Diagnostic::new_base(Level::Error, ENUM_PARENT_FIELDS.0, ENUM_PARENT_FIELDS.1.to_string(), primary_span, vec![], notes, help)
}

#[must_use = "Must set the diagnostic position beforehand"]
pub fn make_unsure_panic<M: Display>(msg: &M) -> Diagnostic {
	let primary_span = Span::make_primary(get_current_diagnostic_pos(), Some("panic happened here".to_string()));

	return Diagnostic::new_base(Level::Error, 99, format!("unsure panic: {}", msg), primary_span, vec![], vec!["Report this at https://github.com/Quickfall/quickfall".to_string()], vec!["create a bug report".to_string()])
}

pub fn make_index_usage<K: DiagnosticSpanOrigin, T: Display>(origin: &K, ty: &T) -> Diagnostic {
	return origin.make_simple_diagnostic(INDEX_USAGE.0, Level::Error, INDEX_USAGE.1.to_string(), Some(format!("index access used on type {} here", ty)), vec![], vec![], vec![])
}

pub fn make_struct_init_missing_field<K: DiagnosticSpanOrigin, T: Display, F: Display>(origin: &K, ty: &T, field: &F) -> Diagnostic {
	let notes = vec!["a struct initializer must contain all fields on the struct type".to_string()];
	let help = vec![format!("add the {} field in the initializer", field)];

	return origin.make_simple_diagnostic(FIELD_STRUCT_INIT.0, Level::Error, format!("field {} of type {} is missing in the initializer", field, ty), None, vec![], notes, help)
}

pub fn make_struct_missing_field<K: DiagnosticSpanOrigin, T: Display, F: Display>(origin: &K, ty: &T, field: &F) -> Diagnostic {
	return origin.make_simple_diagnostic(FIELD_MISSING.0, Level::Error, format!("field {} was not found in type {}", field, ty), None, vec![], vec![], vec![])
}

pub fn make_struct_missing_func<K: DiagnosticSpanOrigin, T: Display, F: Display>(origin: &K, ty: &T, func: &F) -> Diagnostic {
	return origin.make_simple_diagnostic(FUNC_MISSING.0, Level::Error, format!("func {} was not found in type {}", func, ty), None, vec![], vec![], vec![])
}

pub fn make_doesnt_exist_in_era<K: DiagnosticSpanOrigin, V: Display>(origin: &K, val: &V) -> Diagnostic {
	return origin.make_simple_diagnostic(ERA_NOT_EXIST.0, Level::Error, format!("{} doesn't exist in this era", val), None, vec![], vec![], vec![])
}

pub fn make_invalid_pointing<K: DiagnosticSpanOrigin>(origin: &K) -> Diagnostic {
	origin.make_simple_diagnostic(INVALID_POINTING.0, Level::Error, INVALID_POINTING.1.to_string(), None, vec![], vec![], vec![])
}

pub fn make_variable_uninit<K: DiagnosticSpanOrigin, V: Display>(origin: &K, var: &V) -> Diagnostic {
	origin.make_simple_diagnostic(VARIABLE_UNINIT.0, Level::Error, format!("variable {} must be initialized every usage", var), Some("variable not initialized here".to_string()), vec![], vec![], vec![])
}

pub fn make_invalid_var_type_ir() -> Diagnostic {
	let span = Span::make_primary(get_current_diagnostic_pos(), None);

	Diagnostic::new_base(Level::Error, IR_CAST.0, IR_CAST.1.to_string(), span, vec![], vec![], vec![])
}

pub fn make_invalid_assign_diff_type_ir() -> Diagnostic {
	let span = Span::make_primary(get_current_diagnostic_pos(), None);

	Diagnostic::new_base(Level::Error, ASSIGN_DIFF_TYPE_IR.0, ASSIGN_DIFF_TYPE_IR.1.to_string(), span, vec![], vec![], vec![])
}

pub fn make_invalid_instruction_held_val() -> Diagnostic {
	let span = Span::make_primary(get_current_diagnostic_pos(), None);

	Diagnostic::new_base(Level::Error, IR_INSTRUCTION_HELD_VAL.0, IR_INSTRUCTION_HELD_VAL.1.to_string(), span, vec![], vec![], vec![])
}

pub fn make_math_operation_req_assign<K: DiagnosticSpanOrigin>(origin: &K) -> Diagnostic {
	origin.make_simple_diagnostic(MATH_OPERATION_ASSIGNS.0, Level::Error, MATH_OPERATION_ASSIGNS.1.to_string(), None, vec![], vec![], vec![])
}

pub fn make_req_type_kind<K: DiagnosticSpanOrigin, T: Display>(origin: &K, t: &T) -> Diagnostic {
	origin.make_simple_diagnostic(INVALID_TYPE_REQ.0, Level::Error, format!("this operation requires a {} type", t), None, vec![], vec![], vec![])
}

pub fn make_type_not_partof<K: DiagnosticSpanOrigin, A: Display, B: Display>(origin: &K, a: &A, b: &B) -> Diagnostic {
	origin.make_simple_diagnostic(TYPE_NOT_PART.0, Level::Error, format!("type {} is not part of type {}", a, b), None, vec![], vec![], vec![])
}