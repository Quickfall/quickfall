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

    // todo: add multi block handling.

    ctx->blockOffsets[0] = 0;

    for(int i = 0; i < out->blocks[0]->instructionCount; ++i) {
        compileInstruction(buff, ctx, out->blocks[0]->instructions[i]);
    }

    return buff;
}