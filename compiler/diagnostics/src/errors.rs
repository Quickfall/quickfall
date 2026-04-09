macro_rules! declare_error {
	($expr: ident, $ind: literal, $err: literal) => {	
		pub const $expr: (usize, &str) = ($ind, $err);
	}
}

declare_error!(EXPECTED_TOKEN, 0, "expected a {} but got a {}");
declare_error!(UNEXPECTED_TOKEN, 1, "unexpected {}");
declare_error!(MISMATCHED_LITERAL, 2, "expected literal of type {} but got {} instead");
declare_error!(MATH_OPERATION_ASSIGNS, 3, "math operations here require assignments");
declare_error!(FIND_TYPE, 4, "cannot find type {} in the current scope");
declare_error!(EXPECTED_TYPE, 5, "expected a value of type {} but got a value of type {} instead");
declare_error!(ALREADY_IN_SCOPE, 6, "{} was already found in this scope");
declare_error!(ERA_NOT_EXIST, 7, "{} doesn't exist in this era");
declare_error!(ENUM_PARENT_FIELDS, 8, "fields are not supported in enum parents");
declare_error!(ENUM_CHILD_FUNCTIONS, 9, "functions are not supported in enum children");
declare_error!(LAYOUT_FUNCTIONS, 10, "functions are not supported in layouts");
declare_error!(INVALID_POINTING, 11, "cannot point to a non-variable");
declare_error!(TRAIT_MISSING, 12, "bound trait {} cannot be applied onto {}");
declare_error!(BOUND_MISSING, 13, "bound {} cannot be applied onto {}");
declare_error!(VARIABLE_UNINIT, 14, "variable {} must be initialized every usage");
declare_error!(TYPE_RESOLVE, 15, "type cannot be fully resolved here");
declare_error!(DIFF_SIZE_SPECIFIERS, 16, "expected {} size specifiers on this type, got {}");
declare_error!(DIFF_TYPE_SPECIFIERS, 18, "expected {} type specifiers on this type, got {}");
declare_error!(IR_CAST, 19, "cannot cast IR value to given type!");
declare_error!(EXPECTED_FREE, 20, "expected a {}");
declare_error!(FIND_TYPE_FUNCTION, 21, "cannot find function {} in type {}");
declare_error!(FIND_TYPE_FIELD, 22, "cannot find field {} in type {}");
declare_error!(INDEX_USAGE, 23, "cannot use index access on this type");
declare_error!(FIELD_STRUCT_INIT, 24, "field {} of type {} is missing in the initializer");
declare_error!(FIELD_MISSING, 25, "field {} was not found in type {}");
declare_error!(FUNC_MISSING, 25, "function {} was not found in type {}");
declare_error!(FIND_VAR, 25, "cannot find variable {} in the current context");

// IR internals
declare_error!(LOWER_TYPE_IR, 26, "cannot cast MIR variable to given type");
declare_error!(ASSIGN_DIFF_TYPE_IR, 27, "cannot write on this value since the two types differ");
declare_error!(IR_DIFF_SSA_INDEXES, 28, "cannot hint SSA val for pointers! indexes are different");
declare_error!(IR_INSTRUCTION_HELD_VAL, 29, "cannot extract held value from instruction since it doesnt exist");

// Misc
declare_error!(INVALID_TYPE_REQ, 30, "this operation requires a {} type");
declare_error!(TYPE_NOT_PART, 31, "type {} is not part of type {}");