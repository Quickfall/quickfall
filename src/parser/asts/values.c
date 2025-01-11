/**
 * Parsing for value related ASTs.
 */

#include <stdlib.h>
#include <stdio.h>

#include "../structs/values.h"

#include "../ast.h"

#include "../../lexer/lexer.h"

#include "../../lib/types.h"

/**
 * Parses an AST value on the lexer result.
 * @param result the Lexer result.
 * @param index the index of the start of the parsing.
 * @param exceptedType The excepted type of token.
 */
AST_VALUE* parseASTValue(LEXER_RESULT result, int index, LIB_TYPES exceptedType) {
    AST_VALUE* value = malloc(sizeof(AST_VALUE));
    value->astType = AST_TYPE_VALUE;

    switch(exceptedType) {
        case INT32:
        case INT24:
        case INT16:
        case INT8:
            if(result.tokens[index].type != NUMBER) {
                printf("Excepted number as value!\n");
                return NULL;
            }

            value->valueType = malloc(1);
            value->valueType[0] = exceptedType;
            break;
        case BIT:
            if(result.tokens[index].type != BOOLEAN_VALUE) {
                printf("Excepted boolean as bit value!\n");
                return NULL;
            }

            value->valueType = malloc(1);
            value->valueType[0] = BIT;
            break;
    }

    value->endingIndex = index + 1;
    value->value = result.tokens[index].value;

    return value;
}

/**
 * Parses an actual expression as value instead of just one token.
 * @param result the Lexer result.
 * @param index the index of the start of the parsing.
 * @param exceptedType The excepted type of token.
 */
void* parseValueGroup(LEXER_RESULT result, int index, LIB_TYPES exceptedType) {
    switch(result.tokens[index].type) {
        case NUMBER:
            if(result.tokens[index + 1].type == MATH_OP) {
                //todo: parse math op.
            }
            
            return parseASTValue(result, index, exceptedType);
            break;
        default:
            printf("Error: couldn't parse value token group!\n");
            return NULL;
    }
}