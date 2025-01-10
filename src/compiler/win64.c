/**
 * The Win-x64 support for the Quickfall compiler.
 */

#include <stdio.h>
#include <stdlib.h>

#include "../ir/structs.h"
#include "../ir/instructions.h"

#include "./structs.h"

#include "../utils/hashmap.h"
#include "../utils/hash.h"

/**
 * Compiles an QAsm / IR instruction to the Win-x64 bytecode.
 * @param buff the bytecode buffer.
 * @param ctx the compiler context.
 * @param instruction the instruction to convert.
 */
void compileInstruction(BYTECODE_BUFFER* buff, COMPILER_CONTEXT* ctx, IR_INSTRUCTION* instruction) {
    switch(instruction->opCode) {
        case RET:
            buff->buff[buff->size] = 0xC3;
            buff->size++;
            break;

        case STACK_SAVE:
            buff->buff[buff->size] = 0x55;

            buff->buff[buff->size + 1] = 0x48;
            buff->buff[buff->size + 2] = 0x89;
            buff->buff[buff->size + 3] = 0xE5;

            buff->size += 4;
            break;

        case STACK_LOAD:
            buff->buff[buff->size] = 0x5D;
            buff->size++;
            break;

        case S_ALLOC:
            buff->buff[buff->size] = 0x48; 
            buff->buff[buff->size + 1] = 0x83;
            buff->buff[buff->size + 2] = 0xEC;

            int i = (((unsigned char*)instruction->params[0])[0] << 24) | (((unsigned char*)instruction->params[0])[1] << 16) | (((unsigned char*)instruction->params[0])[2] << 8) | ((unsigned char*)instruction->params[0])[3];

            ctx->currStack += i;
            ctx->stackSize += i;

            int* ptr = malloc(sizeof(int));
            *ptr = ctx->stackSize;

            hashPut(ctx->map, hashstr(instruction->params[1]), ptr);

            buff->buff[buff->size + 3] = i;

            buff->size += 4;
            break;

        case PTR_SET:
            buff->buff[buff->size] = 0xC6;
            buff->buff[buff->size + 1] = 0x45;

            int* ii = hashGet(ctx->map, hashstr(instruction->params[0]));

            buff->buff[buff->size + 2] = (uint8_t) *ii;

            buff->buff[buff->size + 3] = ((unsigned char*)instruction->params[1])[3];

            buff->size += 4;
            break;

        case QUAD_SET: //dword
            buff->buff[buff->size] = 0xC7;
            buff->buff[buff->size + 1] = 0x45;

            int* ii = hashGet(ctx->map, hashstr(instruction->params[0]));

            buff->buff[buff->size + 2] = (uint8_t) *ii;

            buff->buff[buff->size + 3] = ((unsigned char*)instruction->params[1])[3];
            buff->buff[buff->size + 4] = ((unsigned char*)instruction->params[1])[2];
            buff->buff[buff->size + 5] = ((unsigned char*)instruction->params[1])[1];
            buff->buff[buff->size + 6] = ((unsigned char*)instruction->params[1])[0];

            buff->size += 7;
            break;

        case STACK_FREE_FUNC:
            buff->buff[buff->size] = 0x48;
            buff->buff[buff->size + 1] = 0x83;
            buff->buff[buff->size + 2] = 0xC4;

            buff->buff[buff->size + 3] = (uint8_t) ctx->currStack;
            
            buff->size += 4;
            break;
        
    }
}