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
    BOOLEAN_VALUE = 12,
    NU = 13,
    KEYWORD = 14,
    SEMICOLON = 15,
    COMMA = 16,
    DECLARE = 17,
    USE = 18,

    MATH_OPERATOR = 19
};
