/**
 * The compiler of Quickfall.
 */

#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>

#include "../ir/ir.h"
#include "../ir/instructions.h"

#include "./compiler.h"

#include "./win64.h"

#include "./pe/pe.h"

#include "../utils/hashmap.h"
#include "../utils/hash.h"

/**
 * Compiles the IR into actual bytecode.
 * @param out the IR output.
 */
BYTECODE_BUFFER* compile(IR_OUTPUT* out) {
    BYTECODE_BUFFER* buff = malloc(sizeof(BYTECODE_BUFFER));

    buff->allocSize = 1024;
    buff->size = 0;
    buff->buff = malloc(sizeof(uint8_t) * 1024);

    COMPILER_CONTEXT* ctx = malloc(sizeof(COMPILER_CONTEXT));
    ctx->stackSize = 0;
    ctx->currStack = 0;
    ctx->map = createHashmap(200,512);

    ctx->blockOffsets = malloc(sizeof(int) * out->blockCount);
    
    for(int blockIndex = 0; blockIndex < out->blockCount; ++blockIndex) {
        ctx->blockOffsets[blockIndex] = buff->size;

        for(int i = 0; i < out->blocks[blockIndex]->instructionCount; ++i) {
            compileInstruction(buff, ctx, out->blocks[blockIndex]->instructions[i]);
        }
    }

    return buff;
}

/**
 * Gets the compiled address from the pointer.
 * @param out the IR output.
 * @param ptr the pointer name.
 */
int getAddressFromPointer(IR_OUTPUT* out, char* ptr) {
    unsigned char* name = malloc(sizeof(ptr));

    unsigned char hasOffset = 0x00;
    int i;
    char c;
    while(c == *ptr++) {
        if(c == ',') {
            ptr++;
            hasOffset = 0x01;
            break;
        }

        if(c == '\0') {
            break;
        }


        name[i] = c;
        ++i;
    }

    int addr = hashGet(out->map, hashstr(name));

    if(hasOffset) {
        int i = (ptr[0] << 24) | (ptr[1] << 16) | (ptr[2] << 8) | ptr[3];
        addr += i;
    }

    return addr;
}