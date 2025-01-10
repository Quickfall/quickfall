/**
 * The writer of QuickAssembly. The inline assembly of Quickfall.
 */

#include <stdio.h>

#include "./values.h"

#include "../../ir/instructions.h"
#include "../../ir/structs.h"

const char* formatedInstructions[] = {"blck_swap", "cblk_swap", "lblck_swap", "salloc", "ptr_load", "iadd", "isub", "imul", "idiv", "icmp", "icmp_h", "icmp_l", "prm_push", "ret_push", "call", "ret", "st_push", "st_load", "st_freef", "ptr_set", "qd_set", "dd_set", "od_set", "ptr_dec"};

/**
 * Writes some standart QuickAssembly code equivalent to the parsed IR.
 * @param fptr the file output.
 * @param output the output of the IR convertor.
 */
void writeQASM(FILE* fptr, IR_OUTPUT* output) {
    for(int i = 0; i < output->blockCount + 1; ++i) {
        fprintf(fptr, "%d:", i);
        for(int index = 0; index < output->blocks[i]->instructionCount; ++index) {
            IR_INSTRUCTION* instruction = output->blocks[i]->instructions[index];
            
            fprintf(fptr, "\n  %s", formatedInstructions[instruction->opCode]);

            switch(instruction->opCode) {
                case BLOCK_SWAP:
                case COND_BLOCK_SWAP:
                case LOGICAL_BLOCK_SWAP:
                    writeInt32(fptr, instruction->params, 0);
                    break;

                case PTR_SET:
                    writeVarName(fptr, instruction->params, 0);
                    writeInt32(fptr, instruction->params, 1);
                    break;

                case PTR_LOAD:
                case S_ALLOC:
                    writeInt32(fptr, instruction->params, 0);
                    writeVarName(fptr, instruction->params, 1);
                    break;

                case IADD:
                case ISUB:
                case IMUL:
                case IDIV:
                case ICMP:
                case ICMP_H:
                case ICMP_L:
                    writeVarName(fptr, instruction->params, 0);
                    writeVarName(fptr, instruction->params, 1);
                    writeVarName(fptr, instruction->params, 2);
                    break;

                case PRM_PUSH:
                    writeVarName(fptr, instruction->params, 0);
                    writeInt32(fptr, instruction->params, 1);
                    break;

                case RET_PUSH:
                    writeVarName(fptr, instruction->params, 0);
                    break;
            }   

        }
    }
}