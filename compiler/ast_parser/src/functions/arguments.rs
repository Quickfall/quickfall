//! Module for parsing arguments

use ast::{tree::FunctionDeclarationArgument, types::CompleteType};
use compiler_errors::errs::{CompilerResult, ErrorKind, normal::CompilerError};
use compiler_utils::hash::HashedString;
use lexer::token::{LexerToken, LexerTokenType};

use crate::types::parse_type;

pub fn parse_function_arguments(tokens: &Vec<LexerToken>, ind: &mut usize, struct_type: Option<CompleteType>) -> CompilerResult<Vec<FunctionDeclarationArgument>> {
	*ind += 1;

	let mut args: Vec<FunctionDeclarationArgument> = Vec::new();
	
	while *ind < tokens.len() && tokens[*ind].is_keyword() {
		
		if tokens[*ind].tok_type == LexerTokenType::This {
			if struct_type.is_none() {
				return Err(CompilerError::from_ast(ErrorKind::Error, "This requires to be within a struct!".to_string(), &tokens[*ind].pos, &tokens[*ind].get_end_pos()))
			}

			*ind += 1;

			args.push(FunctionDeclarationArgument { name: HashedString::new("this".to_string()), argument_type: struct_type.clone().unwrap() })
		} else {
			let var_type = parse_type(tokens, ind)?;

			*ind += 1;
			let var_name = tokens[*ind].expects_keyword()?;
	
			args.push(FunctionDeclarationArgument::new(var_name.0, var_type));
	
			*ind += 1;
		}

		if tokens[*ind].tok_type == LexerTokenType::ParenClose {
			break;
		}

		tokens[*ind].expects(LexerTokenType::Comma)?;

		*ind += 1;
	}

	tokens[*ind].expects(LexerTokenType::ParenClose)?;

	Ok(args)
}
