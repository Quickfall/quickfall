use compiler_utils::{Position, hash::HashedString};

use ast::{make_node, tree::{ASTTreeNode, ASTTreeNodeKind}};
use compiler_errors::{PARSE_VALUE, UNEXPECTED_TOKEN, errs::{CompilerResult, ErrorKind, normal::CompilerError}, pos::BoundPosition};
use lexer::token::{LexerToken, LexerTokenType};

use crate::{arrays::parse_array_access, functions::parse_function_call, structs::val::parse_struct_initialize};
use crate::literals::{parse_integer_literal, parse_string_literal};
use crate::math::parse_math_operation;

pub fn parse_ast_value_dotacess(tokens: &Vec<LexerToken>, ind: &mut usize, original: CompilerResult<Box<ASTTreeNode>>) -> CompilerResult<Box<ASTTreeNode>> {
	match &tokens[*ind].tok_type {
		LexerTokenType::Dot => {
			let original = original?;
			if !original.kind.is_function_call() && !original.kind.is_var_access() {
				return Err(tokens[*ind].make_err(format!(UNEXPECTED_TOKEN!(), original), ErrorKind::Error));
			}

			*ind += 1;
			let r = parse_ast_value_dotacess_chain_member(tokens, ind, Ok(original))?;

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
		LexerTokenType::Keyword(s, _) => {
			if tokens[*ind + 1].tok_type == LexerTokenType::ParenOpen {
				let r_member = parse_function_call(tokens, ind)?;
				let start = original.as_ref().unwrap().start.clone();
				let end = r_member.end.clone();

				return Ok(Box::new(ASTTreeNode::new(ASTTreeNodeKind::StructLRFunction { l: original?, r: r_member }, start, end)))
			}

			match original {
				Ok(_) => {},
				Err(_) => return Err(CompilerError::new(ErrorKind::Error, "original was false".to_string(), BoundPosition::from_size(tokens[*ind].pos.clone(), 1)))
			}

			let start = original.as_ref().unwrap().start.clone();
			let end = tokens[*ind].get_end_pos();

			let r_member = Box::new(ASTTreeNode::new(ASTTreeNodeKind::VariableReference(HashedString::new(s.clone())), start.clone(), end));

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

		LexerTokenType::ArrayOpen => {
			return parse_array_access(tokens, ind, original?)
		},

		LexerTokenType::EqualSign => {
			*ind += 1;

			let start = original.clone()?.start.clone();
		
			let right_val = parse_ast_value(tokens, ind)?;

			let end = right_val.end.clone();

			let kind = ASTTreeNodeKind::VarValueChange { var: original?, value: right_val };
			return Ok(Box::new(ASTTreeNode::new(kind, start, end)));
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
				let end = ast.end.clone();
				return Ok(Box::new(ASTTreeNode::new(ASTTreeNodeKind::BooleanBasedConditionMember { val: ast, negate: true }, Position::clone(&tokens[*ind].pos), end)))
			}

			return Err(tokens[*ind].make_err(format!(UNEXPECTED_TOKEN!(), ast), ErrorKind::Error));
		},

		LexerTokenType::BracketOpen | LexerTokenType::ArrayOpen => {
			return parse_ast_array_init(tokens, ind);
		}

		LexerTokenType::IntLit(_, _) => {
			let int = parse_integer_literal(tokens, ind);
			return parse_ast_value_post_l(tokens, ind, int, false);
		},

		LexerTokenType::StringLit(_) => {
			let str = parse_string_literal(tokens, ind);
			return parse_ast_value_post_l(tokens, ind, str, false);
		},

		LexerTokenType::New => {
			return parse_struct_initialize(tokens, ind);
		}

		LexerTokenType::Keyword(str, _) => {
			if tokens[*ind + 1].tok_type == LexerTokenType::ParenOpen {
				let call = parse_function_call(tokens, ind);
				return parse_ast_value_post_l(tokens, ind, call, false);
			}

			let n = Ok(make_node!(ASTTreeNodeKind::VariableReference(HashedString::new(str.clone())), &tokens[*ind], &tokens[*ind]));

			*ind += 1;

			let chain = parse_ast_value_dotacess(tokens, ind, n);

			return parse_ast_value_post_l(tokens, ind, chain, false);
		}

		_ => return Err(tokens[*ind].make_err(PARSE_VALUE!().to_string(), ErrorKind::Error))
	}	
}

pub fn parse_ast_array_init(tokens: &Vec<LexerToken>, ind: &mut usize) -> CompilerResult<Box<ASTTreeNode>> {
	let start = tokens[*ind].pos.clone();

	if tokens[*ind].tok_type == LexerTokenType::BracketOpen {
		*ind += 1;

		let int_lit = tokens[*ind].expects_int_lit()?;

		*ind += 1;

		tokens[*ind].expects(LexerTokenType::Dot)?;

		*ind += 1;

		let val = parse_ast_value(tokens, ind)?;

		tokens[*ind].expects(LexerTokenType::BracketClose)?;

		*ind += 1;

		return Ok(Box::new(ASTTreeNode::new(ASTTreeNodeKind::ArrayVariableInitializerValueSameValue { size: int_lit.0 as usize, v: val }, start, tokens[*ind].get_end_pos())));
	}

	tokens[*ind].expects(LexerTokenType::ArrayOpen)?;
	*ind += 1;

	let mut vals = vec![];

	loop {
		vals.push(parse_ast_value(tokens, ind)?);

		if tokens[*ind].tok_type == LexerTokenType::ArrayClose {
			break;
		}

		tokens[*ind].expects(LexerTokenType::Comma)?;
		*ind += 1;
	}

	*ind += 1;

	return Ok(Box::new(ASTTreeNode::new(ASTTreeNodeKind::ArrayVariableInitializerValue { vals }, start, tokens[*ind].get_end_pos())))
}