#include <stdio.h>

#include "../structs/misc.h"
#include "../ast.h"
#include "../../lexer/lexer.h"

AST_USE_STD* parseASTUseSTDStatement(LEXER_RESULT result, int index) {
    if(result.tokens[index + 1].type != STRING) {
        printf("Error: Excepted string as use value!\n");
        return NULL;
    }

    AST_USE_STD* node = malloc(sizeof(AST_USE_STD));
    node->type = AST_TYPE_USE_STD;
    node->endingIndex = index + 1;
    node->stdPath = result.tokens[index + 1].value;
    node->next = NULL;

    return node;
}


AST_IMPORT_FILE* parseASTUseImportStatement(LEXER_RESULT result, int index) {
    if(result.tokens[index + 1].type != STRING) {
        printf("Error: Excepted string as IMPORT value!\n");
        return NULL;
    }

    AST_IMPORT_FILE* node = malloc(sizeof(AST_IMPORT_FILE));
    node->type = AST_TYPE_IMPORT;
    node->endingIndex = index + 1;
    node->path = result.tokens[index + 1].value;
    node->next = NULL;

    return node;
}