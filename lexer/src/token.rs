//! 
//! Module containing lexer token-based utilities and classes
//! 

/// The token type for the lexer
pub enum LexerToken {
    /// Represent the func keyword
    FUNCTION,

    /// Represent the ret keyword
    RETURN,

    EQUAL_SIGN,

    SEMICOLON,
    COMMA,
    DOT,

    BRACKET_OPEN,
    BRACKET_CLOSE,

    PAREN_OPEN,
    PAREN_CLOSE,

    ARRAY_OPEN,
    ARRAY_CLOSE,

    INT_LIT(i64),
    STRING_LIT(String),

    ANGEL_BRACKET_OPEN,
    ANGEL_BRACKET_CLOSE,

    KEYWORD(String, u64),
    END_OF_FILE
}