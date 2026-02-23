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

use commons::Position;
use utils::hash;
use errors::{PARSE_VALUE, UNEXPECTED_TOKEN, UNUSED_VAR_ACCESS, errs::{CompilerResult, ErrorKind, normal::CompilerError}};
use lexer::token::{self, LexerToken, LexerTokenType};
use utils::hash::{WithHash};

use crate::{ast::{control::{forloop::parse_for_loop, ifelse::parse_if_statement, whileblock::parse_while_block}, func::{call::parse_function_call, decl::{parse_function_declaraction, parse_shadow_function_declaration}, parse_function_return_statement}, literals::{parse_integer_literal, parse_string_literal}, math::parse_math_operation, tree::{ASTTreeNode, ASTTreeNodeKind}, types::parse_type_declaration, var::{decl::parse_variable_declaration, staticdecl::parse_static_function_declaration}}, make_node};

pub mod tree;
pub mod func;
pub mod var;
pub mod literals;
pub mod control;
pub mod math;
pub mod types;

pub fn parse_ast_value_dotacess(tokens: &Vec<LexerToken>, ind: &mut usize, original: CompilerResult<Box<ASTTreeNode>>) -> CompilerResult<Box<ASTTreeNode>> {
	match &tokens[*ind].tok_type {
		LexerTokenType::Dot => {
			let original = original?;
			if !original.kind.is_function_call() && !original.kind.is_var_access() {
				return Err(tokens[*ind].make_err(format!(UNEXPECTED_TOKEN!(), original), ErrorKind::Error));
			}

			*ind += 1;
			let r = parse_ast_value_dotacess_chain_member(tokens, ind, Ok(original))?;

			println!("Tok: {:#?}", tokens[*ind].tok_type);

			if tokens[*ind].tok_type == LexerTokenType::Dot {
				return parse_ast_value_dotacess(tokens, ind, Ok(r)); // Continue the chain until finished
			}

			return Ok(r);
		},

		_ => return original
	}
}

pub fn parse_ast_value_dotacess_chain_member(tokens: &Vec<LexerToken>, ind: &mut usize, original: CompilerResult<Box<ASTTreeNode>>) -> CompilerResult<Box<ASTTreeNode>> {
	match &tokens[*ind].tok_type {
		LexerTokenType::KEYWORD(s, hsh) => {
			if tokens[*ind + 1].tok_type == LexerTokenType::ParenOpen {

				println!("Calling function call parsing on kwd {}", s);

				let r_member = parse_function_call(tokens, ind)?;
				let start = original.as_ref().unwrap().start.clone();
				let end = r_member.end.clone();

				return Ok(Box::new(ASTTreeNode::new(ASTTreeNodeKind::StructLRFunction { l: original?, r: r_member }, start, end)))
			}

			let start = original?.start.clone();
			let end = tokens[*ind].get_end_pos();

			let r_member = Box::new(ASTTreeNode::new(ASTTreeNodeKind::VariableReference(WithHash::new(s.clone())), start, end));

			*ind += 1;

			let end_r = r_member.end.clone();

			return Ok(Box::new(ASTTreeNode::new(ASTTreeNodeKind::StructLRVariable { l: original?, r: r_member }, start, end_r)));
		},

		_ => return original
	};
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
pub fn parse_ast_value_post_l(tokens: &Vec<LexerToken>, ind: &mut usize, original: CompilerResult<Box<ASTTreeNode>>, invoked_on_body: bool) -> CompilerResult<Box<ASTTreeNode>> {
	match &tokens[*ind].tok_type {
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

			let start_pos = k.start.clone();
			let end_pos = right_val.end.clone();

			return Ok(Box::new(ASTTreeNode::new(ASTTreeNodeKind::OperatorBasedConditionMember { lval: k, rval: right_val, operator }, start_pos, end_pos)));
		},

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
pub fn parse_ast_value(tokens: &Vec<LexerToken>, ind: &mut usize) -> CompilerResult<Box<ASTTreeNode>> {
	match &tokens[*ind].tok_type {

		LexerTokenType::ExclamationMark => {
			*ind += 1;
			let ast = parse_ast_value(tokens, ind)?;

			if ast.kind.is_function_call() || ast.kind.is_var_access() {
				return Ok(Box::new(ASTTreeNode::new(ASTTreeNodeKind::BooleanBasedConditionMember { val: ast, negate: true }, Position::clone(&tokens[*ind].pos), ast.end.clone())))
			}

			return Err(tokens[*ind].make_err(format!(UNEXPECTED_TOKEN!(), ast), ErrorKind::Error));
		},

		LexerTokenType::IntLit(_, _) => {
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

			let n = Ok(make_node!(ASTTreeNodeKind::VariableReference(hash!(str.clone())), &tokens[*ind], &tokens[*ind]));

			*ind += 1;

			let chain = parse_ast_value_dotacess(tokens, ind, n);

			return parse_ast_value_post_l(tokens, ind, chain, false);
		}

		_ => return Err(tokens[*ind].make_err(PARSE_VALUE!().to_string(), ErrorKind::Error))
	}	
}

/// Parses an AST node outside of any other node.
/// 
/// # Examples
/// `parse_ast_node` is used to parse:
/// - Function declarations
/// - Struct declarations
/// - Layout declarations
pub fn parse_ast_node(tokens: &Vec<LexerToken>, ind: &mut usize) -> CompilerResult<Box<ASTTreeNode>> {
	match &tokens[*ind].tok_type {
		LexerTokenType::Function => {
			return parse_function_declaraction(tokens, ind);
		},

		LexerTokenType::ShadowFunction => {
			return parse_shadow_function_declaration(tokens, ind);
		}

		LexerTokenType::Struct => {
			return parse_type_declaration(tokens, ind, false);
		},

		LexerTokenType::Static => {
			return parse_static_function_declaration(tokens, ind);
		}

		LexerTokenType::Layout => {
			return parse_type_declaration(tokens, ind, true);
		},

		_ => {
			return Err(tokens[*ind].make_err(format!(UNEXPECTED_TOKEN!(), &tokens[*ind].tok_type), ErrorKind::Error));
		}
	}	
}

/// Parses an AST node inside of another compatible node (functions, control bodies)
pub fn parse_ast_node_in_body(tokens: &Vec<LexerToken>, ind: &mut usize) -> CompilerResult<Box<ASTTreeNode>> {
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
		},
		
		LexerTokenType::Return  => {
			return parse_function_return_statement(tokens, ind);
		}

		LexerTokenType::KEYWORD(str, _) => {
			if tokens[*ind + 1].tok_type == LexerTokenType::ParenOpen {
				let call = parse_function_call(tokens, ind);
				return parse_ast_value_post_l(tokens, ind, call, true);
			}

			let n = Ok(make_node!(ASTTreeNodeKind::VariableReference(hash!(str.clone())), &tokens[*ind], &tokens[*ind]));

			*ind += 1;

			let new =  parse_ast_value_post_l(tokens, ind, n, true)?;

			if new.kind.is_var_access() {
				return Err(tokens[*ind].make_err(UNUSED_VAR_ACCESS!().to_string(), ErrorKind::Warn));
			}

			return Ok(new);
		},

		_ => {
			return Err(tokens[*ind].make_err(format!(UNEXPECTED_TOKEN!(), tokens[*ind].tok_type), ErrorKind::Error));
		}
	}
}