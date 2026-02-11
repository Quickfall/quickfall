//!
//! The main AST part of the parser.
//! The AST parsing is responsible for putting tokens into structures such as functions and other stuff.
//! It is an extremely important step.
//! 
//! # Parsing 
//! Parsing the first AST node found in the lexer token buffer can be done using `parse_ast_node`.
//! 
//! Parsing inside of an AST node body (eg functions, if statements, ...) can be done using `parse_ast_node_in_body`
//! 
//! # Design notes
//! The initial index represents the given value of the index value (usually `ind`) passed to any AST parsing function.
//! 
//! Whenever any AST parsing function requires a keyword (for example `var`) to be called, the initial index should be the index representing said keyword and not the next one.
//! Furthermore, AST parsing functions requiring a keyword should increment index by one at the start to skip the detected keyword if needed.
//! 
//! AST parsing functions that aren't requiring a keyword will start at the actual start of the expression unless stated otherwise.
//! 

use commons::err::PositionedResult;
use lexer::token::{LexerToken, LexerTokenType};
use utils::hash::WithHash;

use crate::{ast::{control::{forloop::parse_for_loop, ifelse::parse_if_statement, whileblock::parse_while_block}, func::{call::parse_function_call, decl::parse_function_declaraction}, literals::{parse_integer_literal, parse_string_literal}, math::parse_math_operation, tree::ASTTreeNode, types::parse_type_declaration, var::decl::parse_variable_declaration}};

pub mod tree;
pub mod func;
pub mod var;
pub mod literals;
pub mod control;
pub mod math;
pub mod types;

pub fn parse_ast_value_dotacess(tokens: &Vec<LexerToken>, ind: &mut usize, original: PositionedResult<Box<ASTTreeNode>>) -> PositionedResult<Box<ASTTreeNode>> {
	match &tokens[*ind].tok_type {
		LexerTokenType::Dot => {
			let o = &original?;
			let k = Box::new(ASTTreeNode::clone(o.as_ref()));

			if !o.is_function_call() && !o.is_var_access() {
				return Err(tokens[*ind].make_err("Invalid dot access token!"));
			}

			*ind += 1;
			let r = parse_ast_value(tokens, ind)?;

			if r.is_function_call() {
				return Ok(Box::new(ASTTreeNode::StructLRFunction { l: k, r }))
			} else if r.is_var_access() {
				return Ok(Box::new(ASTTreeNode::StructLRVariable { l: k, r }))
			}

			return Err(tokens[*ind].make_err("Invalid token type to use dot access!"));
		},

		_ => return original
	}
}

/// Parses the post side of an AST node that can and WILL be intrepreted as a value.
/// 
/// This function should only be called by `parse_ast_value`
/// 
/// # Parsing Layout 
/// The `parse_ast_value` function only parses the post side of the expression (noted L) if expression is:
/// `R (pre) expression L (post) expression`
/// 
/// This layout allows us to seperate parsing from things like variable references, functions calls or even literals and
/// treat them as the same while parsing other elements such as math operations or conditions!
/// 
/// # Possible node results
/// `parse_ast_value_post_l` can possibly return the following node types:
/// - original type
/// - variable / function on type access
/// - math operation
/// - comparing
/// - boolean negation
/// 
pub fn parse_ast_value_post_l(tokens: &Vec<LexerToken>, ind: &mut usize, original: PositionedResult<Box<ASTTreeNode>>, invoked_on_body: bool) -> PositionedResult<Box<ASTTreeNode>> {
	match &tokens[*ind].tok_type {
		LexerTokenType::Dot => {
			return parse_ast_value_dotacess(tokens, ind, original);
		},

		LexerTokenType::MathOperator(_, _) => {
			let o = &original?;
			let k = Box::new(ASTTreeNode::clone(o.as_ref()));

			return Ok(parse_math_operation(tokens, ind, k, invoked_on_body)?);
		},

		LexerTokenType::ComparingOperator(op) => {
			let operator = op.clone();

			let o = &original?;
			let k = Box::new(ASTTreeNode::clone(o.as_ref()));

			*ind += 1;
			let right_val = parse_ast_value(tokens, ind)?;

			return Ok(Box::new(ASTTreeNode::OperatorBasedConditionMember { lval: k, rval: right_val, operator }));
		}

		_ => return original
	}
}

/// Parses an AST node that can and WILL be intrepreted as a value
/// 
/// # Parsing Layout 
/// The `parse_ast_value` function only parses the pre side of the expression (noted R) if expression is:
/// `R (pre) expression L (post) expression`
/// 
/// This layout allows us to seperate parsing from things like variable references, functions calls or even literals and
/// treat them as the same while parsing other elements such as math operations or conditions!
/// 
/// This function will call `parse_ast_value_post_l` to parse the L part of the expression.
/// 
/// # Recognized Nodes
/// Possible nodes recognized as values include:
/// - Function calls
/// - Variable refs
/// - Math operation results (both with or without value changing)
/// - Boolean negation result
/// - Boolean compare result
pub fn parse_ast_value(tokens: &Vec<LexerToken>, ind: &mut usize) -> PositionedResult<Box<ASTTreeNode>> {
	match &tokens[*ind].tok_type {

		LexerTokenType::ExclamationMark => {
			*ind += 1;
			let ast = parse_ast_value(tokens, ind)?;

			if ast.is_function_call() || ast.is_var_access() {
				return Ok(Box::new(ASTTreeNode::BooleanBasedConditionMember { val: ast, negate: true }))
			}

			return Err(tokens[*ind].make_err("Boolean negative requires either func or var access!"));
		},

		LexerTokenType::IntLit(_) => {
			let int = parse_integer_literal(tokens, ind);
			return parse_ast_value_post_l(tokens, ind, int, false);
		},

		LexerTokenType::StringLit(_) => {
			let str = parse_string_literal(tokens, ind);
			return parse_ast_value_post_l(tokens, ind, str, false);
		},

		LexerTokenType::KEYWORD(str, _) => {
			if tokens[*ind + 1].tok_type == LexerTokenType::ParenOpen {
				let call = parse_function_call(tokens, ind);
				return parse_ast_value_post_l(tokens, ind, call, false);
			}

			let n = Ok(Box::new(ASTTreeNode::VariableReference(WithHash::new(String::clone(str)))));

			*ind += 1;

			return parse_ast_value_post_l(tokens, ind, n, false);
		}

		_ => return Err(tokens[*ind].make_err("Invalid token to parse as a value!"))
	}	
}

/// Parses an AST node outside of any other node.
/// 
/// # Examples
/// `parse_ast_node` is used to parse:
/// - Function declarations
/// - Struct declarations
/// - Layout declarations
pub fn parse_ast_node(tokens: &Vec<LexerToken>, ind: &mut usize) -> PositionedResult<Box<ASTTreeNode>> {
	match &tokens[*ind].tok_type {
		LexerTokenType::Function => {
			return parse_function_declaraction(tokens, ind);
		},

		LexerTokenType::Struct => {
			return parse_type_declaration(tokens, ind, false);
		},

		LexerTokenType::Layout => {
			return parse_type_declaration(tokens, ind, true);
		},

		_ => {
			return Err(tokens[*ind].make_err("Expected valid token type in this context!"));
		}
	}	
}

/// Parses an AST node inside of another compatible node (functions, control bodies)
pub fn parse_ast_node_in_body(tokens: &Vec<LexerToken>, ind: &mut usize) -> PositionedResult<Box<ASTTreeNode>> {
	match &tokens[*ind].tok_type {

		LexerTokenType::Var => {
			return parse_variable_declaration(tokens, ind);
		},

		LexerTokenType::If => {
			return parse_if_statement(tokens, ind);
		},
		
		LexerTokenType::While => {
			return parse_while_block(tokens, ind);
		},

		LexerTokenType::For => {
			return parse_for_loop(tokens, ind);
		}

		LexerTokenType::KEYWORD(str, _) => {
			if tokens[*ind + 1].tok_type == LexerTokenType::ParenOpen {
				let call = parse_function_call(tokens, ind);
				return parse_ast_value_post_l(tokens, ind, call, true);
			}

			let n = Ok(Box::new(ASTTreeNode::VariableReference(WithHash::new(String::clone(str)))));

			*ind += 1;

			return parse_ast_value_post_l(tokens, ind, n, true);
		},

		_ => {
			return Err(tokens[*ind].make_err("Expected valid token type in this context!"));
		}
	}
}