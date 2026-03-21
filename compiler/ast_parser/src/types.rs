//! Parsing for type related features

use ast::types::ASTType;
use compiler_errors::errs::CompilerResult;
use lexer::token::{LexerToken, LexerTokenType};

pub enum ParsingASTTypeMember {
	Generic(String, Vec<Box<ASTType>>, Vec<usize>),
	Pointer(bool),
	Array(bool)	
}

/// Parses the type size specifiers
pub fn parse_type_size_specifiers(tokens: &Vec<LexerToken>, ind: &mut usize) -> CompilerResult<Vec<usize>> {
	tokens[*ind].expects(LexerTokenType::Dot)?;

	let mut sizes = vec![];

	while tokens[*ind].tok_type == LexerTokenType::Dot {
		*ind += 1;

		sizes.push(tokens[*ind].expects_int_lit()?.0 as usize);

		*ind += 1;
	}

	return Ok(sizes);
}

pub fn parse_type_member(tokens: &Vec<LexerToken>, ind: &mut usize, took_generic: bool) -> CompilerResult<ParsingASTTypeMember> {

}

pub fn parse_type(tokens: &Vec<LexerToken>, ind: &mut usize) -> CompilerResult<ASTType> {
	
}