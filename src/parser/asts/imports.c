/**
 * Import/Use-related AST parsing.
 */

#include <stdlib.h>
#include "../../lexer/lexer.h"
#include "../ast.h"
#include "../../utils/module_resolver.h"
#include "../../utils/logging.c"

/**
 * Parses a use/import declaration.
 * @param result the lexer result.
 * @param index the starting index.
 */
AST_NODE* parseUseDeclaration(LEXER_RESULT result, int index) {
    AST_NODE* node = createNode(AST_USE_DECLARATION);
    printf("  Parsing use declaration at index %d\n", index);
    
    // Check if next token is a string containing the path
    if(result.tokens[index + 1].type != STRING) {
        printf("%sError: Expected string path after 'use' keyword, got type %d%s\n", 
               TEXT_RED, result.tokens[index + 1].type, RESET);
        freeNode(node);
        return NULL;
    }
    
    printf("  Resolving module path: %s\n", result.tokens[index + 1].value);
    char* resolvedPath = resolveModulePath(result.tokens[index + 1].value);
    if (resolvedPath == NULL) {
        freeNode(node);
        return NULL;
    }
    
    // Create a string literal node for the path
    AST_NODE* pathNode = createValueNode(AST_STRING_LITERAL, resolvedPath);
    free(resolvedPath);  // createValueNode makes a copy
    
    if (!pathNode) {
        freeNode(node);
        return NULL;
    }
    
    // Add path as child of use declaration
    addChild(node, pathNode);
    
    // Track source location

    node->endingIndex = index + 1;
    
    return node;
} 