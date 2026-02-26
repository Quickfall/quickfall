//! Module for parsing arguments

use ast::tree::FunctionDeclarationArgument;
use compiler_errors::errs::CompilerResult;
use lexer::token::{LexerToken, LexerTokenType};

pub fn parse_function_arguments(tokens: &Vec<LexerToken>, ind: &mut usize) -> CompilerResult<Vec<FunctionDeclarationArgument>> {
	*ind += 1;

	let mut args: Vec<FunctionDeclarationArgument> = Vec::new();
	
	while *ind < tokens.len() && tokens[*ind].is_keyword() {
		let var_type = tokens[*ind].expects_keyword()?;

		*ind += 1;
		let var_name = tokens[*ind].expects_keyword()?;

		args.push(FunctionDeclarationArgument::new(var_name.0, var_type.1));

		*ind += 1;

		if tokens[*ind].tok_type == LexerTokenType::ParenClose {
			break;
		}

		tokens[*ind].expects(LexerTokenType::Comma)?;

		*ind += 1;
	}

	tokens[*ind].expects(LexerTokenType::ParenClose)?;

	Ok(args)
}
