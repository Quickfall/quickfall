/**
 * The Win-x64 support for the Quickfall compiler.
 */

#include <stdio.h>
#include <stdlib.h>

#include "../ir/structs.h"
#include "../ir/instructions.h"

#include "./structs.h"
#include "./compiler.h"

#include "../utils/hashmap.h"
#include "../utils/hash.h"

const unsigned char WIN_REGISTERS[] = {
    0x48, 0x4D,
    0x48, 0x55,
    0x4C, 0x45,
    0x4C, 0x4D
};

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
            buff->buff[buff->size + 1] = 0x2B;
            buff->buff[buff->size + 2] = 0x64;
            buff->buff[buff->size + 3] = 0x24;

            int i = getAddressFromPointer(ctx, instruction->params[0]);
            //int i = (((unsigned char*)instruction->params[0])[0] << 24) | (((unsigned char*)instruction->params[0])[1] << 16) | (((unsigned char*)instruction->params[0])[2] << 8) | ((unsigned char*)instruction->params[0])[3];

            ctx->currStack += i;
            ctx->stackSize += i;

            int* ptr = malloc(sizeof(int));
            *ptr = ctx->stackSize;

            hashPut(ctx->map, hashstr(instruction->params[1]), ptr);

            buff->buff[buff->size + 4] = i;

            buff->size += 5;
            break;

        case PTR_SET:
            buff->buff[buff->size] = 0xC6;
            buff->buff[buff->size + 1] = 0x45;

            int addr = getAddressFromPointer(ctx, instruction->params[0]);
            buff->buff[buff->size + 2] = (uint8_t) addr;

            buff->buff[buff->size + 3] = ((unsigned char*)instruction->params[1])[0];

            buff->size += 4;
            break;

        case DUO_SET:
            buff->buff[buff->size] = 0x66;
            buff->buff[buff->size + 1] = 0x45;
            
            addr = getAddressFromPointer(ctx, instruction->params[0]);
            buff->buff[buff->size + 2] = (uint8_t) addr;

            buff->buff[buff->size + 3] = ((unsigned char*)instruction->params[1])[3];
            buff->buff[buff->size + 4] = ((unsigned char*)instruction->params[1])[2];

            buff->size += 5;

        case QUAD_SET: //dword
            buff->buff[buff->size] = 0xC7;
            buff->buff[buff->size + 1] = 0x45;

            addr = getAddressFromPointer(ctx, instruction->params[0]);
            buff->buff[buff->size + 2] = (uint8_t) addr;

            buff->buff[buff->size + 3] = ((unsigned char*)instruction->params[1])[3];
            buff->buff[buff->size + 4] = ((unsigned char*)instruction->params[1])[2];
            buff->buff[buff->size + 5] = ((unsigned char*)instruction->params[1])[1];
            buff->buff[buff->size + 6] = ((unsigned char*)instruction->params[1])[0];

            buff->size += 7;
            break;

        case OCT_SET: //qword
            buff->buff[buff->size] = 0x48;
            buff->buff[buff->size + 1] = 0xC7;
            buff->buff[buff->size + 2] = 0x45;

            addr = getAddressFromPointer(ctx, instruction->params[0]);
            buff->buff[buff->size + 2] = (uint8_t) addr;

            buff->buff[buff->size + 4] = ((unsigned char*)instruction->params[1])[7];
            buff->buff[buff->size + 5] = ((unsigned char*)instruction->params[1])[6];
            buff->buff[buff->size + 6] = ((unsigned char*)instruction->params[1])[5];
            buff->buff[buff->size + 7] = ((unsigned char*)instruction->params[1])[4];
            buff->buff[buff->size + 8] = ((unsigned char*)instruction->params[1])[3];
            buff->buff[buff->size + 9] = ((unsigned char*)instruction->params[1])[2];
            buff->buff[buff->size + 10] = ((unsigned char*)instruction->params[1])[1];
            buff->buff[buff->size + 11] = ((unsigned char*)instruction->params[1])[0];

            buff->size += 12;
            break;

        case STACK_FREE_FUNC:
            buff->buff[buff->size] = 0x48;
            buff->buff[buff->size + 1] = 0x83;
            buff->buff[buff->size + 2] = 0xC4;

            buff->buff[buff->size + 3] = (uint8_t) ctx->currStack;
            
            buff->size += 4;
            break;

        case PTR_DEC:
            int* ii = malloc(sizeof(int));
            *ii = (((unsigned char*)instruction->params[1])[0] << 24) | (((unsigned char*)instruction->params[1])[1] << 16) | (((unsigned char*)instruction->params[1])[2] << 8) | ((unsigned char*)instruction->params[1])[3];

            hashPut(ctx->map, hashstr(instruction->params[0]), ii);
            break;

        case PTR_DEC_OFF:
            ii = malloc(sizeof(int));
            *ii = (((unsigned char*)instruction->params[2])[0] << 24) | (((unsigned char*)instruction->params[2])[1] << 16) | (((unsigned char*)instruction->params[2])[2] << 8) | ((unsigned char*)instruction->params[2])[3];
            
            addr = getAddressFromPointer(ctx, instruction->params[1]);


            *ii += addr;

            hashPut(ctx->map, hashstr(instruction->params[0]), ii);
            break;

        case BLOCK_SWAP:
            buff->buff[buff->size] = 0xE9;

            i = (((unsigned char*)instruction->params[0])[0] << 24) | (((unsigned char*)instruction->params[0])[1] << 16) | (((unsigned char*)instruction->params[0])[2] << 8) | ((unsigned char*)instruction->params[0])[3];

            int off = ctx->blockOffsets[i] - buff->size;

            buff->buff[buff->size + 1] = off & 0xFF;
            buff->buff[buff->size + 2] = (off >> 8) & 0xFF;
            buff->buff[buff->size + 3] = (off >> 16) & 0xFF;
            buff->buff[buff->size + 4] = (off >> 24) & 0xFF;

            buff->size += 5;
            break;

        case COND_BLOCK_SWAP:
            buff->buff[buff->size] = 0x80;
            buff->buff[buff->size + 1] = 0x7d;

            i = (((unsigned char*)instruction->params[0])[0] << 24) | (((unsigned char*)instruction->params[0])[1] << 16) | (((unsigned char*)instruction->params[0])[2] << 8) | ((unsigned char*)instruction->params[0])[3];

            off = ctx->blockOffsets[i] - buff->size;

            addr = getAddressFromPointer(ctx, instruction->params[1]);

            buff->buff[buff->size + 2] = addr & 0xFF;
            buff->buff[buff->size + 3] = 0x01;

            buff->buff[buff->size + 4] = 0x74;
            buff->buff[buff->size + 5] = off & 0xFF;
            
            buff->size += 6;
            break;
        case LOGICAL_BLOCK_SWAP:
            int trueBlockId = (((unsigned char*)instruction->params[0])[0] << 24) | (((unsigned char*)instruction->params[0])[1] << 16) | (((unsigned char*)instruction->params[0])[2] << 8) | ((unsigned char*)instruction->params[0])[3];
            int falseBlockId = (((unsigned char*)instruction->params[1])[0] << 24) | (((unsigned char*)instruction->params[1])[1] << 16) | (((unsigned char*)instruction->params[1])[2] << 8) | ((unsigned char*)instruction->params[1])[3];

            // Instead of doing if else, just jump to the block if true and continue if not

            addr = getAddressFromPointer(ctx, instruction->params[2]);


            buff->buff[buff->size] = 0x80;
            buff->buff[buff->size + 1] = 0x7d;

            buff->buff[buff->size + 2] = addr & 0xFF;
            buff->buff[buff->size + 3] = 0x01;

            buff->buff[buff->size + 4] = 0x74;
            buff->buff[buff->size + 5] = (ctx->blockOffsets[trueBlockId] - buff->size) & 0xFF;
            
            // else
            off = ctx->blockOffsets[falseBlockId] - buff->size;

            buff->buff[buff->size + 6] = off & 0xFF;
            buff->buff[buff->size + 7] = (off >> 8) & 0xFF;
            buff->buff[buff->size + 8] = (off >> 16) & 0xFF;
            buff->buff[buff->size + 9] = (off >> 24) & 0xFF;

            buff->size += 10;
            break;

        case PRM_LOAD:
            int index = (((unsigned char*)instruction->params[0])[0] << 24) | (((unsigned char*)instruction->params[0])[1] << 16) | (((unsigned char*)instruction->params[0])[2] << 8) | ((unsigned char*)instruction->params[0])[3];

            buff->buff[buff->size] = WIN_REGISTERS[2 * index];
            buff->buff[buff->size + 1] = 0x89;
            buff->buff[buff->size + 2] = WIN_REGISTERS[(2 * index) + 1];

            int i = getAddressFromPointer(ctx, instruction->params[1]);

            buff->buff[buff->size + 3] = (uint8_t) i;
            buff->size += 4;
            break;

            
    }
}