/**
 * IR for variable related.
 */

#include <stdlib.h>
#include <stdio.h>
#include <string.h>

#include "../../parser/structs/variables.h"

#include "./values.h"

#include "../ir.h"

#include "../../lib/types.h"

/**
 * Parses a variable declaration.
 * @param block the IR basic block to append to.
 * @param node the AST node representing the variable.
 */
void parseVariableDeclaration(IR_BASIC_BLOCK* block, AST_VARIABLE_DEC* node) {
    int allocSize = 0;
    
    switch(node->type[0]) {
        case INT32:
            allocSize = 32;
            break;
        case INT24:
            allocSize = 24;
            break;
        case INT16:
            allocSize = 16;
            break;
        case INT8:
            allocSize = 8;
            break;
        default:
            allocSize = 0;
            break;
    }

    void** params = malloc(sizeof(void*) * 2);

    char* name = node->name;

    params[0] = malloc(4);

    ((unsigned char*)params[0])[0] = (allocSize >> 24) & 0xFF;
    ((unsigned char*)params[0])[1] = (allocSize >> 16) & 0xFF;
    ((unsigned char*)params[0])[2] = (allocSize >> 8) & 0xFF;
    ((unsigned char*)params[0])[3] = allocSize & 0xFF;

    params[1] = node->name;

    appendInstruction(block, STACK_SAVE, NULL, 0);

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