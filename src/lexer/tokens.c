/**
 * Data related to tokens
 */

#define longestKeywordSize 32 // Pumped up to handle longer values like numbers
#define smallestKeywordSize 4

/**
 * The token types.
 */
enum TokenType {
    FUNCTION = 1,
    RETURN = 2,
    VAR = 3,
    BRACKETS_OPEN = 4,
    BRACKETS_CLOSE = 5,
    PAREN_OPEN = 6,
    PAREN_CLOSE = 7,
    ARRAY_OPEN = 8,
    ARRAY_CLOSE = 9,
    NUMBER = 10,
    STRING = 11,
    BOOLEAN = 12,
    NU = 13,
    CLASS = 14,
    EXTENDS = 15,
    IMPLEMENTS = 16,
    ABSTRACT = 17,
    INTERFACE = 18,
    NEW = 19,
    KEYWORD = 20,
    SEMICOLON = 21,
    COMMA = 22,
    DECLARE = 23
};

