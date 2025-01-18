/**
 * The writer of IR. Allows to compile Quickfall files into a lower level format than Assembly for speed.
 */

#include <stdio.h>

#include "./structs.h"
#include "./ir.h"

/**
 * Writes an IR block into the compiled Quickfall format.
 * @param block the block.
 * @param file the file stream.
 */
void writeIRBlock(IR_BASIC_BLOCK* block, FILE* file) {
    fwrite(&block->instructionCount, sizeof(int), 1, file);
    
    for(int i = 0; i < block->instructionCount; ++i) {
        IR_INSTRUCTION* instruction = block->instructions[i];

        fwrite(instruction->opCode, 1, 1, file);
        fwrite(instruction->paramCount, sizeof(int), 1, file);
        
        for(int i = 0; i < 
    }
}