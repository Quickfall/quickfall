//! Collection of error & warning messages to avoid repetition

// Lexer & AST
#[macro_export]
macro_rules! EXPECTED_TOKEN {
	() => {
		"Expected a {:#?} but got a {:#?}"
	};
}

#[macro_export]
macro_rules! UNEXPECTED_TOKEN {
	() => {
		"Unexpected {:#?} token"
	};
}

// Lexer
#[macro_export]
macro_rules! PARSE_INT {
	() => {
		"Couldn't parse int literal"
	};
}

#[macro_export]
macro_rules! PARSE_FLOAT {
	() => {
		"Couldn't parse floating literal"
	};
}

#[macro_export]
macro_rules! PARSE_STRING {
	() => {
		"Couldn't parse string literal"
	};
}

#[macro_export]
macro_rules! PARSE_OPERATOR {
	() => {
		"Invalid math operator part! Got char {}"
	};
}

// AST

#[macro_export]
macro_rules! PARSE_VALUE {
	() => {
		"Couldn't parse the following as value."
	};
}

#[macro_export]
macro_rules! UNUSED_VAR_ACCESS {
	() => {
		"Unused variable access. Consider removing this"
	};
}

// IO
#[macro_export]
macro_rules! IO_ERROR_READ {
	() => {
		"Couldn't read the file!"
	};
}

// Math

#[macro_export]
macro_rules! MATH_OP_NO_ASSIGN {
	() => {
		"This context enforces the usage of assigments in math operations. Please remove this operation or make it assigned"
	};
}

// IR

#[macro_export]
macro_rules! INVALID_EXPR {
	() => {
		"Invalid expression here. Please correct it"
	};
}

#[macro_export]
macro_rules! IR_OBTAIN_TYPE {
	() => {
		"Cannot use this value as a {}"
	};
}

#[macro_export]
macro_rules! IR_DIFF_TYPE {
	() => {
		"Both types aren't matching!"
	};
}


#[macro_export]
macro_rules! IR_EXPECTED_TYPE {
	() => {
		"Expected type {} but got type {}"
	};
}

#[macro_export]
macro_rules! IR_OBTAIN_COND {
	() => {
		"Cannot use this value as a condition boolean."
	};
}

#[macro_export]
macro_rules! IR_ALREADY_EXISTING_ELEM {
	() => {
		"Cannot use this name for an element. An element named like this already exists!"
	};
}

#[macro_export]
macro_rules! IR_FIND_FUNCTION {
	() => {
		"Cannot find function in the current context."
	};
}

#[macro_export]
macro_rules! IR_FIND_VARIABLE {
	() => {
		"Cannot find variable in the current context."
	};
}

#[macro_export]
macro_rules! IR_OUTSIDE_ERA {
	() => {
		"Tried invoking element outside of it's definition era."
	};
}

#[macro_export]
macro_rules! IR_TYPE_UNSIGNED {
	() => {
		"Expected an unsigned numeric data type. Change the type here.\nPlease send a bug report at https://github.com/quickfall/quickfall"
	};
}

#[macro_export]
macro_rules! IR_TYPE_SIGNED {
	() => {
		"Expected an unsigned numeric data type. Change the type here.\nPlease send a bug report at https://github.com/quickfall/quickfall"
	};
}

#[macro_export]
macro_rules! IR_TYPE_BOOL {
	() => {
		"Expected a boolean here. Change the type.\nPlease send a bug report at https://github.com/quickfall/quickfall"
	};
}

#[macro_export]
macro_rules! IR_STATIC_STR_TYPE {
	() => {
		"Expected type staticstr for static string variable values. Change the type of this variable to staticstr"
	};
}

#[macro_export]
macro_rules! IR_INVALID_INT_COMP_VAL {
	() => {
		"Cannot perform comparing on given values. Cannot use {} as an integer"
	};
}

#[macro_export]
macro_rules! IR_INVALID_MATH_VAL {
	() => {
		"Cannot perform operations on given values. Cannot use {} as in integer"
	};
}

#[macro_export]
macro_rules! IR_ASSIGN_NOVAR {
	() => {
		"Cannot perform math assignment operations if the left value is not a variable."
	};
}

#[macro_export]
macro_rules! IR_FIND_TYPE {
	() => {
		"Cannot find said type."
	};
}

#[macro_export]
macro_rules! IR_REQ_VARIABLE_ASSIGN {
	() => {
		"A variable is required here to use assigments"
	};
}

#[macro_export]
macro_rules! IR_STATIC_VAR_WRONG_OP {
	() => {
		"Cannot obtain string reference if value is compile-time replacable.\nPlease send a bug report at https://github.com/quickfall/quickfall"
	};
}

#[macro_export]
macro_rules! IR_LAYOUT_FUNCS {
	() => {
		"Cannot declare member functions inside of a layout. Consider removing the functions or switching the type to be a struct."
	};
}

#[macro_export]
macro_rules! IR_LAYOUT_FUNC_USAGE {
	() => {
		"Cannot use member functions inside of a layout.\nPlease send a bug report at https://github.com/quickfall/quickfall"
	};
}

#[macro_export]
macro_rules! IR_FIELD {
	() => {
		"Unknown field inside of the current context."
	};
}

#[macro_export]
macro_rules! NO_PERMITTED_OUTSIDE_FUNC {
	() => {
		"The following is not permitted outside of a function. Please move this inside of a function"
	};
}

#[macro_export]
macro_rules! FUNC_RETVOID_USE_VAL {
	() => {
		"Cannot use this function's return as a value as said function returns void."
	};
}

#[macro_export]
macro_rules! IR_FIND_PRIMITIVE_TYPE {
	() => {
		"Cannot find said primitive type in the type storage.\nPlease send a bug report at https://github.com/quickfall/quickfall"
	};
}

// Internal & Critical Errors
#[macro_export]
macro_rules! INKWELL_FUNC_FAILED {
	() => {
		"Inkwell function {} failed.\nPlease send a bug report at https://github.com/quickfall/quickfall, err: {}"
	};
}

#[macro_export]
macro_rules! IR_TYPE_NO_INKWELL_TYPE {
	() => {
		"The given IR type doesn't have an Inkwell / LLVM IR type equivalent. This is a near-impossible error.\nPlease send a bug report at https://github.com/quickfall/quickfall immeditately"
	};
}

#[macro_export]
macro_rules! IR_TYPE_WRONG_KIND {
	() => {
		"Assumed wrong kind of IR type!\nPlease send a bug report at https://github.com/quickfall/quickfall"
	};
}

#[macro_export]
macro_rules! AST_INVALID_TREE {
	() => {
		"Invalid AST tree node in AST tree! Got node {:#?}.\nPlease send a bug report at https://github.com/quickfall/quickfall"
	};
}

#[macro_export]
macro_rules! IR_VALUE_REF_TEMP_TYPE {
	() => {
		"Cannot perform this action on the given IR value type.\nPlease send a bug report at https://github.com/quickfall/quickfall"
	};
}

#[macro_export]
macro_rules! IR_INVALID_NODE_TYPE {
	() => {
		"Cannot use singular IR parse function on said AST node. Got a {}.\nPlease send a bug report at https://github.com/quickfall/quickfall"
	};
}

#[macro_export]
macro_rules! INKWELL_TYPE_GATHER {
	() => {
		"Cannot use {} on the given IR type.\nPlease send a bug report at https://github.com/quickfall/quickfall"
	};
}
