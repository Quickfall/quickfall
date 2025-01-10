/**
 * IR for function related.
 */

#include <stdlib.h>
#include <stdio.h>

#include "../../qasm/parser/parser.h"

#include "../structs.h"

#include "../../parser/structs/functions.h"
#include "../../parser/structs/variables.h"
#include "../../parser/structs/tree.h"

#include "../../parser/ast.h"

#include "./variables.h"

#include "../../utils/hash.h"
#include "../../utils/hashmap.h"

/**
 * Parses a AST function into IR.
 * @param node the AST node representing the function.
 */
void parseFunction(IR_OUTPUT* out, AST_FUNCTION_DEC* node) {
    int hash = hashstr(node->funcName);
    IR_FUNCTION* func = malloc(sizeof(IR_FUNCTION));

    func->startBlock = out->blockCount;
    func->typeCount = node->parameterIndex;

    func->types = malloc(func->typeCount + 1);

    if(node->returnType != NULL) func->types[0] = node->returnType[0];
    else func->types[0] = 0x00;

    for(int i = 0; i < func->typeCount; ++i) {
        func->types[i + 1] = node->parameters[i].type[0];
    }

    hashPut(out->map, hash, func);

    if(out->blockCount != 0) {
        out->blocks[out->blockCount] = malloc(sizeof(IR_BASIC_BLOCK));
        out->blocks[out->blockCount]->instructions = NULL;
        out->blocks[out->blockCount]->instructionCount = 0;
        out->blocks[out->blockCount]->allocatedSize = 0;
    }

    AST_TREE_BRANCH* branch = (AST_TREE_BRANCH*) node->body;
    while(branch->next != NULL) {
        branch = ((AST_TREE_BRANCH*)branch->next);

        switch(branch->type) {
            case AST_TYPE_VARIABLE_DECLARATION:
                parseVariableDeclaration(out->blocks[out->blockCount], (AST_VARIABLE_DEC*) branch);
                break;
            case AST_TYPE_VARIABLE_MODIFICATION:
                parseVariableModification(out->blocks[out->blockCount], (AST_VARIABLE_MOD*)branch);
                break;
        }
    }

    void** params = malloc(sizeof(void*));
    params[0] = malloc(sizeof(int));
    int* p = (params[0]);
    *p = 0;

    appendInstruction(out->blocks[out->blockCount], BLOCK_SWAP, params, 1);

    appendInstruction(out->blocks[out->blockCount], STACK_FREE_FUNC, NULL, 0);
    appendInstruction(out->blocks[out->blockCount], STACK_LOAD, NULL, 0);

    appendInstruction(out->blocks[out->blockCount], RET, NULL, 0);
}

/**
 * Parses a AST Asm function into IR.
 * @param node the AST node representing the Asm function.
 */
void parseASMFunction(IR_OUTPUT* out, AST_ASM_FUNCTION_DEC* node) {
    IR_FUNCTION* func = malloc(sizeof(IR_FUNCTION));
    func->startBlock = out->blockCount;

    func->types = malloc(1);
    func->types[0] = 0x00;
    func->typeCount = 1;
    
    hashPut(out->map, hashstr(node->funcName), func);

    out->blocks[out->blockCount] = malloc(sizeof(IR_BASIC_BLOCK));
    out->blocks[out->blockCount]->instructions = NULL;
    out->blocks[out->blockCount]->instructionCount = 0;

    parseQAsmInstructions(out->blocks[out->blockCount], node->buff, node->buffIndex);
}