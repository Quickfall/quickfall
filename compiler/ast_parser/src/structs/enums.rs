use ast::{tree::{ASTTreeNode, ASTTreeNodeKind}, types::ASTType};
use compiler_utils::hash::HashedString;
use diagnostics::{DiagnosticResult, diagnostic::Diagnostic};
use lexer::token::{LexerToken, LexerTokenType};

use crate::{functions::parse_function_declaraction, structs::members::parse_types_field_member, types::{parse_type_generic, parse_type_parameters_declaration}};

pub fn parse_enum_entry(tokens: &Vec<LexerToken>, ind: &mut usize) -> DiagnosticResult<Box<ASTTreeNode>> {
	let start = tokens[*ind].pos.clone();

	let name = tokens[*ind].expects_keyword()?;
	*ind += 1;

	let mut fields = vec![];

	tokens[*ind].expects(LexerTokenType::ParenOpen)?;	
	*ind += 1;

	loop {
		fields.push(parse_types_field_member(tokens, ind)?);

		if tokens[*ind].tok_type == LexerTokenType::ParenClose {
			break;
		}

		tokens[*ind].expects(LexerTokenType::Comma)?;
	}

	*ind += 1;

	return Ok(Box::new(ASTTreeNode::new(ASTTreeNodeKind::EnumEntryDeclaration { name: HashedString::new(name.0), fields }, start, tokens[*ind].get_end_pos())))
}

pub fn parse_enum_declaration(tokens: &Vec<LexerToken>, ind: &mut usize) -> DiagnosticResult<Box<ASTTreeNode>> {
	let start = tokens[*ind].pos.clone();

	*ind += 1;

	let name = tokens[*ind].expects_keyword()?;
	*ind += 1;

	let t = parse_type_parameters_declaration(tokens, ind)?;

	tokens[*ind].expects(LexerTokenType::BracketOpen)?;
	*ind += 1;

	let mut entries = vec![];
	let mut functions = vec![];

	let temp_type = ASTType::Generic(name.0.clone(), vec![], vec![], None);

	loop {
		if tokens[*ind].is_keyword() {
			entries.push(parse_enum_entry(tokens, ind)?);
		} else {
			tokens[*ind].expects(LexerTokenType::Function)?;

			functions.push(parse_function_declaraction(tokens, ind, Some(temp_type.clone()))?)
		}

		if tokens[*ind].tok_type == LexerTokenType::BracketClose {
			break;
		}
	}
	*ind += 1;

	return Ok(Box::new(ASTTreeNode::new(ASTTreeNodeKind::EnumDeclaration { name: HashedString::new(name.0), entries, functions, type_params: t }, start, tokens[*ind].get_end_pos())))
}