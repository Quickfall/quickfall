//!
//! The main AST part of the parser.
//! The AST parsing is responsible for putting tokens into structures such as functions and other stuff.
//! It is an extremely important step.
//! 
//! Indexes passed to parsing functions SHOULD be the "detected" token rather than the next one.
//! 

use std::fmt::Debug;

use commons::err::PositionedResult;
use lexer::token::{LexerToken, LexerTokenType};
use utils::hash::WithHash;

use crate::{ParserError, ParserResult, ast::{cond::operators::parse_condition_operator, control::{forloop::parse_for_loop, ifelse::parse_if_statement, whileblock::parse_while_block}, func::{call::parse_function_call, decl::parse_function_declaraction}, literals::{parse_integer_literal, parse_string_literal}, math::parse_math_operation, tree::ASTTreeNode, var::decl::parse_variable_declaration}};

pub mod tree;
pub mod func;
pub mod var;
pub mod literals;
pub mod cond;
pub mod control;
pub mod math;

pub fn parse_ast_value_post_l(tokens: &Vec<LexerToken>, ind: &mut usize, original: PositionedResult<Box<ASTTreeNode>>) -> PositionedResult<Box<ASTTreeNode>> {
	match &tokens[*ind].tok_type {
		LexerTokenType::DOT => {
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

		LexerTokenType::MATH_OPERATOR(_, _) => {
			let o = &original?;
			let k = Box::new(ASTTreeNode::clone(o.as_ref()));

			return Ok(parse_math_operation(tokens, ind, k, false)?);
		},

		LexerTokenType::ANGEL_BRACKET_CLOSE | LexerTokenType::EQUAL_SIGN | LexerTokenType::ANGEL_BRACKET_OPEN => {
			let operator = parse_condition_operator(tokens, ind)?;

			let o = &original?;
			let k = Box::new(ASTTreeNode::clone(o.as_ref()));

			*ind += 1;
			let right_val = parse_ast_value(tokens, ind)?;

			return Ok(Box::new(ASTTreeNode::OperatorBasedConditionMember { lval: k, rval: right_val, operator }));
		}

		_ => return original
	}
}

pub fn parse_ast_value(tokens: &Vec<LexerToken>, ind: &mut usize) -> PositionedResult<Box<ASTTreeNode>> {
	match &tokens[*ind].tok_type {

		LexerTokenType::EXCLAMATION_MARK => {
			*ind += 1;
			let ast = parse_ast_value(tokens, ind)?;

			if ast.is_function_call() || ast.is_var_access() {
				return Ok(Box::new(ASTTreeNode::BooleanBasedConditionMember { val: ast, negate: true }))
			}

			return Err(tokens[*ind].make_err("Boolean negative requires either func or var access!"));
		},

		LexerTokenType::INT_LIT(_) => {
			let int = parse_integer_literal(tokens, ind);
			return parse_ast_value_post_l(tokens, ind, int);
		},

		LexerTokenType::STRING_LIT(_) => {
			let str = parse_string_literal(tokens, ind);
			return parse_ast_value_post_l(tokens, ind, str);
		},

		LexerTokenType::KEYWORD(str, _) => {
			if tokens[*ind + 1].tok_type == LexerTokenType::PAREN_OPEN {
				let call = parse_function_call(tokens, ind);
				return parse_ast_value_post_l(tokens, ind, call);
			}

			let n = Ok(Box::new(ASTTreeNode::VariableReference(WithHash::new(String::clone(str)))));

			*ind += 1;

			return parse_ast_value_post_l(tokens, ind, n);
		}

		_ => return Err(tokens[*ind].make_err("Cannot be parsed as value!"))
	}	
}

pub fn parse_ast_node(tokens: &Vec<LexerToken>, ind: &mut usize) -> PositionedResult<Box<ASTTreeNode>> {
	println!("Ind: {}, tok at: {:#?}", ind, tokens[*ind].tok_type);

	match &tokens[*ind].tok_type {
		LexerTokenType::FUNCTION => {
			return parse_function_declaraction(tokens, ind);
		}

		LexerTokenType::VAR => {
			return parse_variable_declaration(tokens, ind);
		},

		LexerTokenType::IF => {
			return parse_if_statement(tokens, ind);
		},
		
		LexerTokenType::WHILE => {
			return parse_while_block(tokens, ind);
		},

		LexerTokenType::FOR => {
			return parse_for_loop(tokens, ind);
		}

		_ => {
			return Err(tokens[*ind].make_err("Invalid token type! Shouldn't be there!"));
		}

	}
}