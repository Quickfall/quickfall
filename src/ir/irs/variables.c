/**
 * IR for variable related.
 */

#include <stdlib.h>
#include <stdio.h>
#include <string.h>

#include "../../parser/structs/variables.h"

#include "./values.h"

#include "../ir.h"

/**
 * Parses a variable declaration.
 * @param block the IR basic block to append to.
 * @param node the AST node representing the variable.
 */
void parseVariableDeclaration(IR_BASIC_BLOCK* block, AST_VARIABLE_DEC* node) {
    int allocSize = 0;
    if(node->type[0] == 0x01) allocSize = 32; // int32

    void** params = malloc(sizeof(void*) * 2);

    char* name = node->name;

    params[0] = &allocSize;
    params[1] = node->name;

    appendInstruction(block, S_ALLOC, params, 2);

    if(node->value != NULL) {
        params = malloc(sizeof(void*) * 2);
        params[0] = node->name;
    
        parseValue(params, 1, node->value);

        appendInstruction(block, PTR_SET, params, 2);
    }
}

/**
 * Parses a variable modification.
 * @param block the IR basic block to append to.
 * @param node the AST node representing the variable.
 */
inline void parseVariableModification(IR_BASIC_BLOCK* block, AST_VARIABLE_MOD* node) {
    void** params = malloc(sizeof(void*) * 2);

    params[0] = node->name;

    parseValue(params, 1, node->value);
    appendInstruction(block, PTR_SET, params, 2);
}