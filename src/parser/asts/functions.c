/**
 * Parsing for function related ASTs.
 */

#include <string.h>
#include <stdio.h>
#include <stdlib.h>

#include "./functions.h"

#include "../structs/functions.h"

#include "../parser.h"
#include "../ast.h"

#include "../../lexer/lexer.h"

#include "../../utils/string.h"

/**
 * Parses a function declaration into AST.
 * @param result the Lexer result.
 * @param index the index of the start of the parsing.
 */
AST_FUNCTION_DEC* parseFunctionDeclaration(LEXER_RESULT result, int index) {
    AST_FUNCTION_DEC* func = malloc(sizeof(AST_FUNCTION_DEC));
    func->returnType = malloc(1);

    func->type = AST_TYPE_FUNCTION_DECLARATION;

    int offset;

    switch(result.tokens[index + 2].type) {
        case PAREN_OPEN:
            if(result.tokens[index + 1].type != KEYWORD) {
                printf("Error: Excepted a keyword as function name!\n");
                return NULL;
            }

            func->funcName = result.tokens[index + 1].value;
            func->returnType[0] = 0x00;
            offset = 3;
            break;
        case KEYWORD:
            if(result.tokens[index + 1].type != TYPE_INT32) {
                printf("Error: Invalid type as function return type!\n");
                return NULL;
            }

            func->funcName = result.tokens[index + 2].value;
            func->returnType[0] = 0x01;
            offset = 4;
            break;
        
        default:
            printf("Error: couldn't parse function declaration!\n");
            return NULL;
    }

    index += offset;

    parseFunctionParameters(func, result, index);

    if(result.tokens[func->endingIndex + 1].type != BRACKETS_OPEN) {
        printf("Error: Excepted function body!\n");
        return NULL;
    }

    void* root = parseRoot(result, func->endingIndex + 2, AST_TYPE_FUNC_ROOT);

    if(root == NULL) {
        printf("Error: couldn't parse function body!\n");
        return NULL;
    }

    func->body = root;
    func->endingIndex = ((AST_TREE_BRANCH*)root)->endingIndex;

    return func;
}


/**
 * Parses an ASM function declaration into AST.
 * @param result the Lexer result.
 * @param index the index of the start of the parsing.
 */
AST_ASM_FUNCTION_DEC* parseASMFunctionDeclaration(LEXER_RESULT result, int index) {
    AST_ASM_FUNCTION_DEC* func = malloc(sizeof(AST_ASM_FUNCTION_DEC));

    if(result.tokens[index + 1].type != KEYWORD) {
        printf("Error: Excepted keyword as ASM function name!\n");
        return NULL;
    }

    func->type = AST_TYPE_ASM_FUNCTION_DECLARATION;
    func->funcName = result.tokens[index + 1].value;

    parseFunctionParameters((AST_FUNCTION_DEC*)func, result, index + 3);

    index = func->endingIndex;

    if(result.tokens[index + 1].type != BRACKETS_OPEN) {
        printf("Error: Badly made ASM function body!\n");
        return NULL;
    }


    int allocatedBuff = 512;

    func->buff = malloc(allocatedBuff);
    func->buffIndex = 0;

    index += 2;
    for(; index < result.size; ++index) {
        TOKEN t = result.tokens[index];

        if(t.type == BRACKETS_CLOSE) {
            func->endingIndex = index;
            return func;
        }
        else if(t.type == STRING) {
            func->buffIndex = fast_strcat(func->buff, t.value, func->buffIndex) + 1; // Adds one to the index to keep the seperator.
            func->buff[func->buffIndex - 1] = '\n';

            if(func->buffIndex >= allocatedBuff) {
                allocatedBuff *= 1.5;
                func->buff = realloc(func->buff, allocatedBuff);
            }

        }
        else {
            printf("Error: disallowed token %d in ASM function! Only string are allowed in body!\n", t.type);
            printf("%s", t.value);
            return NULL;
        }

    }

    func->endingIndex = index;
    return func;
}


/**
 * Parses the parameters of a function into AST.
 * @param result the Lexer result.
 * @param index the index of the start of the parsing.
 */
void parseFunctionParameters(AST_FUNCTION_DEC* func, LEXER_RESULT result, int index) {
    int allocated = 10;
    func->parameterIndex = 0;

    for(; index < result.size; ++index) {
        TOKEN t = result.tokens[index];

        switch(t.type) {
            case TYPE_INT32:
                if(result.tokens[index + 1].type != KEYWORD) {
                    printf("Error: Excepted keyword as parameter name!\n");
                    return;
                }

                func->parameters[func->parameterIndex].name = result.tokens[index + 1].value;
                func->parameters[func->parameterIndex].type[0] = 0x01; // i32

                func->parameterIndex++;
                if(func->parameterIndex > allocated) {
                    allocated *= 1.25;
                    func->parameters = realloc(func->parameters, sizeof(AST_PARAMETER) * allocated);
                }
                break;
            case KEYWORD:
                if(result.tokens[index + 1].type != COMMA) {
                    printf("Error: Excepted comma after parameter!\n");
                    return;
                }

                if(result.tokens[index - 1].type == COMMA) {
                    func->parameters[func->parameterIndex].name = result.tokens[index].value;
                    func->parameters[func->parameterIndex].type[0] = 0x00;
                    func->parameterIndex++;
                    if(func->parameterIndex > allocated) {
                        allocated *= 1.25;
                        func->parameters = realloc(func->parameters, sizeof(AST_PARAMETER) * allocated);
                    }
                }

                break;
            
            case PAREN_CLOSE:
                func->endingIndex = index;
                return;

            default:
                printf("Error: Disallowed token type %d in parameters!\n", t.type);
                return;
        }
    }
}