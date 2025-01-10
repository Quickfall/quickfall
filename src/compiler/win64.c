/**
 * The Win-x64 support for the Quickfall compiler.
 */

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

            buff->buff[buff->size++] = 0x48;
            buff->buff[buff->size++] = 0x89;
            buff->buff[buff->size++] = 0xE5;
            break;

        case STACK_LOAD:
            buff->buff[buff->size] = 0x5D;
            break;

        case S_ALLOC:
            buff->buff[buff->size] = 0x48; 
            buff->buff[buff->size++] = 0x83;
            buff->buff[buff->size++] = 0xEC;

            int i = (((unsigned char*)instruction->params[0])[0] << 24) | (((unsigned char*)instruction->params[0])[1] << 16) | (((unsigned char*)instruction->params[0])[2] << 8) | ((unsigned char*)instruction->params[0])[3];

            ctx->stackSize += i;
            hashPut(ctx->map, hashstr(instruction->params[1]), ctx->stackSize);

            buff->buff[buff->size++] = i;
            break;

        case PTR_SET: //dword
            buff->buff[buff->size] = 0xC7;
            buff->buff[buff->size++] = 0x45;

            buff->buff[buff->size++] = (uint8_t) hashGet(ctx->map, hashstr(instruction->params[1]));

            buff->buff[buff->size++] = ((unsigned char*)instruction->params[0])[0];
            buff->buff[buff->size++] = ((unsigned char*)instruction->params[0])[1];
            buff->buff[buff->size++] = ((unsigned char*)instruction->params[0])[2];
            buff->buff[buff->size++] = ((unsigned char*)instruction->params[0])[3];
            break;


    }
}