/**
 * IR for variable related.
 */

#include <stdlib.h>
#include <stdio.h>
#include <string.h>

#include "../../parser/structs/variables.h"
#include "../../parser/structs/values.h"

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
        case BIT: // bit is 8 for now, only for now
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
        AST_VALUE* val = (AST_VALUE*) node->value;
        if(node->type[0] == BIT && val->valueType == BIT) {
            params = malloc(sizeof(void*) * 2);
            params[0] = node->name;

            params[1] = malloc(1);
            ((unsigned char*)params[1])[0] = strcmp(val->value, "true") == 0;

            appendInstruction(block, PTR_SET, params, 2);
        }
        else if(allocSize == 32) { // if allocates 32 bits, use qd_set
            params = malloc(sizeof(void*) * 2);
            params[0] = node->name;
    
            parseValue(params, 1, node->value);

            appendInstruction(block, QUAD_SET, params, 2);
        }
        else {
            unsigned char* equiv = getByteEquivalent(node->value);

            for(int i = 4; i > (4 - allocSize / 8); --i) {
                int size = strlen(node->name) + 2;
                char* ptrName = malloc(size);
                ptrName[size] = '\0';
                ptrName[size - 1] = (char) i + 97;

                params = malloc(sizeof(void*) * 3);
                params[0] = ptrName;
                params[1] = node->name;

                params[2] = malloc(4);
                unsigned char* buff = (unsigned char*) params[2];

                buff[0] = ((i - 4) * 8 - 1 >> 24) & 0xFF;
                buff[1] = ((i - 4) * 8 - 1 >> 16) & 0xFF;
                buff[2] = ((i - 4) * 8 - 1 >> 8) & 0xFF;
                buff[3] = (i - 4) * 8 - 1 & 0xFF;

                appendInstruction(block, PTR_DEC_OFF, params, 3);
                
                params = malloc(sizeof(void*) * 2);
                params[0] = ptrName;
                params[1] = malloc(1);
                ((unsigned char*)params[1])[0] = equiv[i - 1];

                appendInstruction(block, PTR_SET, params, 2);
            }
        }
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