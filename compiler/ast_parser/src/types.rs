//! Parsing for type related features

use std::collections::HashMap;

use ast::types::ASTType;
use compiler_utils::hash::HashedString;
use diagnostics::{
    DiagnosticResult,
    builders::{make_expected_single_simple_error, make_unexpected_simple_error},
};
use lexer::token::{LexerToken, LexerTokenType};

#[derive(Clone, Debug)]
pub enum ParsingASTTypeMember {
    Generic(String, Vec<Box<ASTType>>, Vec<usize>, Option<String>),
    Pointer(bool),
    Reference,
    Array(usize),
}

/// Parses the type size specifiers
pub fn parse_type_size_specifiers(
    tokens: &Vec<LexerToken>,
    ind: &mut usize,
) -> DiagnosticResult<Vec<usize>> {
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

pub fn parse_type_type_parameters(
    tokens: &Vec<LexerToken>,
    ind: &mut usize,
) -> DiagnosticResult<Vec<Box<ASTType>>> {
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
        *ind += 1;
    }

    *ind += 1;

    return Ok(types);
}

pub fn parse_type_generic(
    tokens: &Vec<LexerToken>,
    ind: &mut usize,
) -> DiagnosticResult<ParsingASTTypeMember> {
    let type_name = tokens[*ind].expects_keyword()?;

    *ind += 1;

    let specifier;

    if tokens[*ind].tok_type == LexerTokenType::Collon {
        *ind += 1;

        tokens[*ind].expects(LexerTokenType::Collon)?;
        *ind += 1;

        specifier = Some(tokens[*ind].expects_keyword()?.0);
        *ind += 1;
    } else {
        specifier = None;
    }

    let sizes = parse_type_size_specifiers(tokens, ind)?;
    let types = parse_type_type_parameters(tokens, ind)?;

    return Ok(ParsingASTTypeMember::Generic(
        type_name.0,
        types,
        sizes,
        specifier,
    ));
}

pub fn parse_type_member(
    tokens: &Vec<LexerToken>,
    ind: &mut usize,
    took_generic: bool,
) -> DiagnosticResult<Option<ParsingASTTypeMember>> {
    match &tokens[*ind].tok_type {
        LexerTokenType::Keyword(_, _) => {
            if took_generic {
                return Ok(None);
            }

            return Ok(Some(parse_type_generic(tokens, ind)?));
        }

        LexerTokenType::ParenOpen => {
            *ind += 1;

            let res = parse_type_member(tokens, ind, took_generic)?;
            *ind += 1;

            tokens[*ind].expects(LexerTokenType::ParenClose)?;
            *ind += 1;

            return Ok(res);
        }

        LexerTokenType::Ampersand => {
            *ind += 1;

            return Ok(Some(ParsingASTTypeMember::Reference));
        }

        LexerTokenType::Asterisk => {
            *ind += 1;

            if tokens[*ind].tok_type == LexerTokenType::ArrayOpen {
                *ind += 1;

                tokens[*ind].expects(LexerTokenType::ArrayClose)?;

                *ind += 1;

                return Ok(Some(ParsingASTTypeMember::Pointer(true)));
            }

            return Ok(Some(ParsingASTTypeMember::Pointer(false)));
        }

        LexerTokenType::ArrayOpen => {
            *ind += 1;

            let size = tokens[*ind].expects_int_lit()?;

            *ind += 1;

            tokens[*ind].expects(LexerTokenType::ArrayClose)?;

            *ind += 1;

            return Ok(Some(ParsingASTTypeMember::Array(size.0 as usize)));
        }

        _ => {
            if took_generic {
                return Ok(None);
            }

            return Err(make_unexpected_simple_error(&tokens[*ind], &tokens[*ind].tok_type).into());
        }
    }
}

pub fn parse_type(tokens: &Vec<LexerToken>, ind: &mut usize) -> DiagnosticResult<ASTType> {
    let mut members = vec![];
    let mut took_generic = false;

    loop {
        let parsed_member = parse_type_member(tokens, ind, took_generic)?;

        if let Some(value) = parsed_member {
            if let ParsingASTTypeMember::Generic(_, _, _, _) = &value {
                took_generic = true;
            }

            members.push(value);
        } else {
            break;
        }
    }

    let mut child = None;

    for i in 0..members.len() {
        let converted_member = match members[i].clone() {
            ParsingASTTypeMember::Generic(t, types, sizes, specifier) => {
                ASTType::Generic(t, types, sizes, specifier)
            }
            ParsingASTTypeMember::Pointer(array) => ASTType::Pointer(array, child.unwrap()),
            ParsingASTTypeMember::Reference => ASTType::Reference(child.unwrap()),
            ParsingASTTypeMember::Array(size) => ASTType::Array(size, child.unwrap()),
        };

        child = Some(Box::new(converted_member));
    }

    return match child {
        Some(v) => Ok(*v),
        None => {
            return Err(
                make_expected_single_simple_error(&tokens[*ind], &"type".to_string()).into(),
            );
        }
    };
}

pub fn parse_type_parameters_declaration(
    tokens: &Vec<LexerToken>,
    ind: &mut usize,
) -> DiagnosticResult<HashMap<HashedString, usize>> {
    if tokens[*ind].tok_type != LexerTokenType::AngelBracketOpen {
        return Ok(HashMap::new());
    }

    let mut container = HashMap::new();

    *ind += 1;

    while tokens[*ind].is_keyword() {
        let param = tokens[*ind].expects_keyword()?;

        container.insert(HashedString::new(param.0), container.len());

        *ind += 1;

        if tokens[*ind].tok_type == LexerTokenType::AngelBracketClose {
            break;
        }

        tokens[*ind].expects(LexerTokenType::Comma)?;
        *ind += 1;
    }

    *ind += 1;

    return Ok(container);
}
