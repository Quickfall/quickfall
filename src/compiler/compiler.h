/**
 * The compiler of Quickfall.
 */

#ifndef COMPILER_H
#define COMPILER_H

/**
 * A bytecode buffer, is used to transmit byte codes around.
 */
typedef struct BYTECODE_BUFFER {

    uint8_t* buff;
    int size;
    int allocSize;

} BYTECODE_BUFFER;

/**
 * Compiles the Context tree to an executable named the provided name.
 * @param ctx the IR context.
 * @param char the output file.
 */
void compile(FILE* outputFileName);

#endif
