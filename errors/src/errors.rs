//! Collection of error & warning messages to avoid repetition

// Lexer & AST
pub const EXPECTED_TOKEN: &str = "Expected a {} but got a {}";
pub const UNEXPECTED_TOKEN: &str = "Unexpected {} token";

// Lexer
pub const PARSE_INT: &str = "Couldn't parse int literal";
pub const PARSE_FLOAT: &str = "Couldn't parse floating literal";
pub const PARSE_STRING: &str = "Couldn't parse string literal";

pub const PARSE_OPERATOR: &str = "Invalid math operator part! Got char {}";

// AST
pub const PARSE_VALUE: &str = "Couldn't parse the following as value.";
pub const UNUSED_VAR_ACCESS: &str = "Unused variable access. Consider removing this";

// IO
pub const IO_ERROR_READ: &str = "Couldn't read the file!";

// Math
pub const MATH_OP_NO_ASSIGN: &str = "This context enforces the usage of assigments in math operations. Please remove this operation or make it assigned";

// IR
pub const IR_OBTAIN_TYPE: &str = "Cannot use this vaue as a {}";
pub const IR_EXPECTED_TYPE: &str = "Expected type {} but got type {}";
pub const IR_OBTAIN_COND: &str = "Cannot use this value as a condition boolean.";

pub const IR_ALREADY_EXISTING_ELEM: &str = "Cannot use this name for an element. An element named like this already exists!";

pub const IR_FIND_FUNCTION: &str = "Cannot find function in the current context.";
pub const IR_FIND_VARIABLE: &str = "Cannot find variable in the current context.";

pub const IR_TYPE_UNSIGNED: &str = "Expected an unsigned numeric data type. Change the type here.\nPlease send a bug report at https://github.com/quickfall/quickfall";
pub const IR_TYPE_SIGNED: &str = "Expected an unsigned numeric data type. Change the type here.\nPlease send a bug report at https://github.com/quickfall/quickfall";
pub const IR_TYPE_BOOL: &str = "Expected a boolean here. Change the type.\nPlease send a bug report at https://github.com/quickfall/quickfall";

pub const IR_STATIC_STR_TYPE: &str = "Expected type staticstr for static string variable values. Change the type of this variable to staticstr";

pub const IR_INVALID_INT_COMP_VAL: &str = "Cannot perform comparing on given values. Cannot use {} as an integer";
pub const IR_INVALID_MATH_VAL: &str = "Cannot perform operations on given values. Cannot use {} as in integer";

pub const IR_ASSIGN_NOVAR: &str = "Cannot perform math assignment operations if the left value is not a variable.";

pub const IR_FIND_TYPE: &str = "Cannot find said type.";

pub const IR_STATIC_VAR_WRONG_OP: &str = "Cannot obtain string reference if value is compile-time replacable.\nPlease send a bug report at https://github.com/quickfall/quickfall";

pub const IR_LAYOUT_FUNCS: &str = "Cannot declare member functions inside of a layout. Consider removing the functions or switching the type to be a struct.";
pub const IR_LAYOUT_FUNC_USAGE: &str = "Cannot use member functions inside of a layout.\nPlease send a bug report at https://github.com/quickfall/quickfall";

pub const IR_FIELD: &str = "Unknown field inside of the current context.";

pub const IR_FIND_PRIMITIVE_TYPE: &str = "Cannot find said primitive type in the type storage.\nPlease send a bug report at https://github.com/quickfall/quickfall";

// Internal & Critical Errors
pub const INKWELL_FUNC_FAILED: &str = "Inkwell function {} failed.\nPlease send a bug report at https://github.com/quickfall/quickfall, err: {}";
pub const AST_INVALID_TREE: &str = "Invalid AST tree node in AST tree! Got node {}.\nPlease send a bug report at https://github.com/quickfall/quickfall";

pub const IR_VALUE_REF_TEMP_TYPE: &str = "Cannot perform this action on the given IR value type.\nPlease send a bug report at https://github.com/quickfall/quickfall";
pub const IR_INVALID_NODE_TYPE: &str = "Cannot use singular IR parse function on said AST node. Got a {}.\nPlease send a bug report at https://github.com/quickfall/quickfall";

pub const INKWELL_TYPE_GATHER: &str = "Cannot use {} on the given IR type.\nPlease send a bug report at https://github.com/quickfall/quickfall";