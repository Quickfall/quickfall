use ast::tree::{ASTTreeNode, ASTTreeNodeKind};
use compiler_utils::hash::HashedString;
use diagnostics::DiagnosticResult;
use lexer::token::{LexerToken, LexerTokenType};

pub fn parse_use_statement(tokens: &Vec<LexerToken>, ind: &mut usize) -> DiagnosticResult<Box<ASTTreeNode>> {
	let start = tokens[*ind].pos.clone();

	*ind += 1;

	let mut steps = vec![];
	let mut uses = vec![];

	while tokens[*ind].is_keyword() {
		let kwd = tokens[*ind].expects_keyword()?;
		*ind += 1;

		steps.push(HashedString::new(kwd.0));
		
		tokens[*ind].expects(LexerTokenType::Collon)?;
		*ind += 1;
	}

	tokens[*ind].expects(LexerTokenType::ArrayOpen)?;

	while tokens[*ind].is_keyword() {
		let kwd = tokens[*ind].expects_keyword()?;
		*ind += 1;

		uses.push(HashedString::new(kwd.0));

		if tokens[*ind].tok_type == LexerTokenType::ArrayClose {
			break;
		}

		tokens[*ind].expects(LexerTokenType::Comma)?;
	}

	*ind += 1;

	return Ok((Box::new(ASTTreeNode::new(ASTTreeNodeKind::UseStatement { shards: steps, use_clauses: uses }, start, tokens[*ind].get_end_pos()))))
}