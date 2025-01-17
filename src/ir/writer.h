/**
 * The writer of IR. Allows to compile Quickfall files into a lower level format than Assembly for speed.
 */

#include <stdio.h>

#include "./structs.h"
#include "./ir.h"

#ifndef IR_WRITER_H
#define IR_WRITER_H

/**
 * Writes an IR block into the compiled Quickfall format.
 * @param block the block.
 * @param file the file stream.
 */
void writeIRBlock(IR_BASIC_BLOCK* block, FILE* file);

#endif