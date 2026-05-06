//!
//! Module containing lexer token-based utilities and classes
//!

use std::fmt::Display;

use compiler_utils::Position;
use diagnostics::{
    DiagnosticResult, DiagnosticSpanOrigin,
    builders::make_expected_simple_error,
    diagnostic::{Diagnostic, Span, SpanKind, SpanPosition},
};

/// The token type for the lexer
#[derive(PartialEq, Debug)]
pub enum LexerTokenType {
    /// Represent the func keyword
    Function,
    ExternFunc,

    Comment(String),
    GlobalComment(String),

    Unwrap,
    UnwrapUnsafe,

    Use,

    Var,
    Struct,
    Layout,
    Lay,

    Enum,

    This,

    Static,

    New,

    /// Represent the ret keyword
    Return,

    True,
    False,

    For,
    If,
    Else,
    While,

    EqualSign,
    ExclamationMark,

    Asterisk,

    Comma,
    Dot,
    Ampersand,
    Collon,
    SemiCollon,

    Plus,
    Minus,
    Divide,
    Tidle,
    PercentSign,

    BracketOpen,
    BracketClose,

    ParenOpen,
    ParenClose,

    ArrayOpen,
    ArrayClose,

    IntLit(i128, u64),
    StringLit(String),

    AngelBracketOpen,
    AngelBracketClose,

    Keyword(String, u64),
    EndOfFile,
}

#[derive(Debug)]
pub struct LexerToken {
    pub tok_type: LexerTokenType,
    pub pos: Position,
    pub pos_size: usize,
}

impl LexerToken {
    pub fn make_single_sized(pos: Position, t: LexerTokenType) -> Self {
        return LexerToken {
            tok_type: t,
            pos,
            pos_size: 1,
        };
    }

    pub fn new(start: Position, size: usize, t: LexerTokenType) -> Self {
        return LexerToken {
            tok_type: t,
            pos: start,
            pos_size: size,
        };
    }

    pub fn is(&self, t: LexerTokenType) -> bool {
        return self.tok_type == t;
    }

    pub fn expects(&self, t: LexerTokenType) -> DiagnosticResult<()> {
        if self.tok_type != t {
            return Err(make_expected_simple_error(self, &t, &self.tok_type).into());
        }

        return Ok(());
    }

    pub fn expects_int_lit(&self) -> DiagnosticResult<(i128, u64)> {
        match &self.tok_type {
            LexerTokenType::IntLit(v, h) => return Ok((*v, *h)),
            _ => {
                return Err(make_expected_simple_error(
                    self,
                    &"integer literal".to_string(),
                    &self.tok_type,
                )
                .into());
            }
        };
    }

    pub fn expects_string_lit(&self) -> DiagnosticResult<String> {
        match &self.tok_type {
            LexerTokenType::StringLit(v) => return Ok(v.to_string()),
            _ => {
                return Err(make_expected_simple_error(
                    self,
                    &"string literal".to_string(),
                    &self.tok_type,
                )
                .into());
            }
        };
    }

    pub fn expects_keyword(&self) -> DiagnosticResult<(String, u64)> {
        match &self.tok_type {
            LexerTokenType::Keyword(s, h) => return Ok((s.to_string(), *h)),
            _ => {
                return Err(make_expected_simple_error(
                    self,
                    &"keyword".to_string(),
                    &self.tok_type,
                )
                .into());
            }
        };
    }

    pub fn is_keyword(&self) -> bool {
        match &self.tok_type {
            LexerTokenType::Keyword(_, _) => true,
            _ => false,
        }
    }

    pub fn get_size(&self) -> usize {
        return self.pos_size;
    }

    pub fn get_pos(&self) -> SpanPosition {
        return SpanPosition::from_pos(self.pos.clone(), self.pos_size);
    }

    pub fn get_end_pos(&self) -> Position {
        return self.pos.increment_by(self.pos_size);
    }
}

impl Display for LexerTokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Enum => "enum",
            Self::Ampersand => "&",
            Self::AngelBracketClose => ">",
            Self::AngelBracketOpen => "<",
            Self::ArrayClose => "]",
            Self::ArrayOpen => "[",
            Self::Asterisk => "*",
            Self::Collon => ":",
            Self::SemiCollon => ";",
            Self::BracketClose => "}",
            Self::BracketOpen => "{",
            Self::Comma => ",",
            Self::Comment(_) => "comment",
            Self::Dot => ".",
            Self::Else => "else",
            Self::EndOfFile => "end of file",
            Self::EqualSign => "=",
            Self::ExclamationMark => "!",
            Self::False => "false",
            Self::For => "for",
            Self::Function => "func",
            Self::GlobalComment(_) => "global comment",
            Self::If => "if",
            Self::IntLit(_, _) => "integer literal",
            Self::Keyword(_, _) => "keyword",
            Self::Lay => "lay",
            Self::Layout => "layout",
            Self::New => "new",
            Self::ParenClose => ")",
            Self::ParenOpen => "(",
            Self::Return => "ret",
            Self::ExternFunc => "externfunc",
            Self::Static => "static",
            Self::StringLit(_) => "string literal",
            Self::Var => "var",
            Self::Struct => "struct",
            Self::This => "this",
            Self::True => "true",
            Self::While => "while",
            Self::Unwrap => "unwrap",
            Self::Use => "use",
            Self::UnwrapUnsafe => "unsafe_unwrap",
            Self::Plus => "+",
            Self::Minus => "-",
            Self::Divide => "/",
            Self::Tidle => "~",
            Self::PercentSign => "%",
        };

        write!(f, "{}", s)?;
        Ok(())
    }
}

impl DiagnosticSpanOrigin for LexerToken {
    fn make_span(&self, kind: SpanKind, msg: Option<String>) -> Span {
        Span {
            kind,
            label: msg,
            start: SpanPosition::from_pos(self.pos.clone(), self.get_size()),
        }
    }

    fn get_pos(&self) -> SpanPosition {
        SpanPosition::from_pos(self.pos.clone(), self.get_size())
    }

    fn make_simple_diagnostic(
        &self,
        code: usize,
        level: diagnostics::diagnostic::Level,
        message: String,
        primary_span_msg: Option<String>,
        spans: Vec<Span>,
        notes: Vec<String>,
        help: Vec<String>,
    ) -> diagnostics::diagnostic::Diagnostic {
        let primary = Span {
            kind: SpanKind::Primary,
            label: primary_span_msg,
            start: SpanPosition::from_pos(self.pos.clone(), self.get_size()),
        };

        Diagnostic::new_base(level, code, message, primary, spans, notes, help)
    }
}
