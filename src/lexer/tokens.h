#ifndef TOKENS_H
#define TOKENS_H

#define longestKeywordSize 32
#define smallestKeywordSize 4

enum TokenType {
    /* Keywords */
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

    /* Other */
    KEYWORD = 19,
    SEMICOLON = 20,
    COMMA = 21,
    DECLARE = 22
};

struct KeywordResult {
    int count;
    char* keywords[10];
    enum TokenType types[10];
};

void initKeywords();
struct KeywordResult getKeywords(char start);

#endif
