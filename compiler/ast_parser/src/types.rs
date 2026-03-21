//! Parsing for type related features

use ast::types::ASTType;
use compiler_errors::{UNEXPECTED_TOKEN, errs::{CompilerResult, ErrorKind}};
use lexer::{token::{LexerToken, LexerTokenType}, toks::math::MathOperator};

#[derive(Clone)]
pub enum ParsingASTTypeMember {
	Generic(String, Vec<Box<ASTType>>, Vec<usize>),
	Pointer(bool),
	Array(usize)	
}

/// Parses the type size specifiers
pub fn parse_type_size_specifiers(tokens: &Vec<LexerToken>, ind: &mut usize) -> CompilerResult<Vec<usize>> {
	if tokens[*ind].tok_type != LexerTokenType::Dot {
		return Ok(vec![]);
	}

	let mut sizes = vec![];

	while tokens[*ind].tok_type == LexerTokenType::Dot {
		*ind += 1;

		sizes.push(tokens[*ind].expects_int_lit()?.0 as usize);

		*ind += 1;
	}

	return Ok(sizes);
}

pub fn parse_type_type_parameters(tokens: &Vec<LexerToken>, ind: &mut usize) -> CompilerResult<Vec<Box<ASTType>>> {
	if tokens[*ind].tok_type != LexerTokenType::AngelBracketOpen {
		return Ok(vec![]);
	}

	let mut types = vec![];

	*ind += 1;

	loop {
		let parsed_type = Box::new(parse_type(tokens, ind)?);

		types.push(parsed_type);

		if tokens[*ind].tok_type == LexerTokenType::AngelBracketClose {
			break;
		}

		tokens[*ind].expects(LexerTokenType::Comma)?;
	}

	return Ok(types)
}
 
pub fn parse_type_generic(tokens: &Vec<LexerToken>, ind: &mut usize) -> CompilerResult<ParsingASTTypeMember> {
	let type_name = tokens[*ind].expects_keyword()?;

	*ind += 1;

	let sizes = parse_type_size_specifiers(tokens, ind)?;
	let types = parse_type_type_parameters(tokens, ind)?;

	return Ok(ParsingASTTypeMember::Generic(type_name.0, types, sizes))
}

pub fn parse_type_member(tokens: &Vec<LexerToken>, ind: &mut usize, took_generic: bool) -> CompilerResult<Option<ParsingASTTypeMember>> {
	match &tokens[*ind].tok_type {
		LexerTokenType::Keyword(_, _) => {
			if took_generic {
				return Ok(None)
			}

			return Ok(Some(parse_type_generic(tokens, ind)?))
		},

		LexerTokenType::MathOperator(sign, assigns) => {
			if *assigns || *sign != MathOperator::MULTIPLY {
				return Err(tokens[*ind].make_err(UNEXPECTED_TOKEN!().to_string(), ErrorKind::Error))
			}

			*ind += 1;

			if tokens[*ind].tok_type == LexerTokenType::ArrayOpen {
				*ind += 1;

				tokens[*ind].expects(LexerTokenType::ArrayClose)?;

				*ind += 1;

				return Ok(Some(ParsingASTTypeMember::Pointer(true)))
			}

			return Ok(Some(ParsingASTTypeMember::Pointer(false)))
		},

		LexerTokenType::ArrayOpen => {
			*ind += 1;
			
			let size = tokens[*ind].expects_int_lit()?;

			*ind += 1;

			tokens[*ind].expects(LexerTokenType::ArrayClose)?;

			*ind += 1;

			return Ok(Some(ParsingASTTypeMember::Array(size.0 as usize)))
		},

		_ => return Err(tokens[*ind].make_err(UNEXPECTED_TOKEN!().to_string(), ErrorKind::Error))
	}
}

pub fn parse_type(tokens: &Vec<LexerToken>, ind: &mut usize) -> CompilerResult<ASTType> {
	let mut members = vec![];
	let mut took_generic = false;

	loop {
		let parsed_member = parse_type_member(tokens, ind, took_generic)?;

		if let Some(value) = parsed_member {
			if let ParsingASTTypeMember::Generic(_, _, _) = &value {
				took_generic = true;
			}

			members.push(value);
		} else {
			break;
		}
	}

	let mut child = None;

	for i in members.len()..0 {
		let converted_member = match members[i].clone() {
			ParsingASTTypeMember::Generic(t, types, sizes) => ASTType::Generic(t, types, sizes),
			ParsingASTTypeMember::Pointer(array) => ASTType::Pointer(array, child.unwrap()),
			ParsingASTTypeMember::Array(size) => ASTType::Array(size, child.unwrap())
		};

		child = Some(Box::new(converted_member));
	}

	return match child {
		Some(v) => Ok(*v),
		None => return Err(tokens[*ind].make_err("Error while parsing type! Latest child member was None!".to_string(), ErrorKind::Critical))
	};
}