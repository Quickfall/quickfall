//! Shadow function parsing

use ast::tree::{ASTTreeNode, ASTTreeNodeKind};
use compiler_utils::hash::HashedString;
use diagnostics::DiagnosticResult;
use lexer::token::{LexerToken, LexerTokenType};

use crate::{functions::arguments::parse_function_arguments, types::parse_type};

pub fn parse_shadow_function_declaration(tokens: &Vec<LexerToken>, ind: &mut usize) -> DiagnosticResult<Box<ASTTreeNode>> {
	let start = tokens[*ind].pos.clone();

	*ind += 1;
	let function_name = tokens[*ind].expects_keyword()?;

	*ind += 1;
	tokens[*ind].expects(LexerTokenType::ParenOpen)?;

	let args = parse_function_arguments(tokens, ind, None)?;

	*ind += 1;

	let mut ret_type = None;
	let end;

	if tokens[*ind].is_keyword() {
		ret_type = Some(parse_type(tokens, ind)?);

		end = tokens[*ind].get_end_pos().clone();
	} else {
		end = tokens[*ind - 1].get_end_pos().clone();
	}

	return Ok(Box::new(ASTTreeNode::new(ASTTreeNodeKind::ShadowFunctionDeclaration { func_name: HashedString::new(function_name.0), args: args.0, return_type: ret_type }, start, end)))
}