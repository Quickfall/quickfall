/**
 * Structures used by the Quickfall compiler.
 */

#include "../utils/hashmap.h"

#include <stdint.h>

#ifndef COMPILER_STRUCTS_H
#define COMPILER_STRUCTS_H

/**
 * A bytecode buffer, is used to transmit byte codes around.
 */
typedef struct BYTECODE_BUFFER {

    uint8_t* buff;
    int size;
    int allocSize;

} BYTECODE_BUFFER;

/**
 * A structure holding all of the necessary information during the compiling process.
 * For example, value pointers
 */
typedef struct COMPILER_CONTEXT {

    struct Hashmap* map;
    int stackSize;
    int currStack;

} COMPILER_CONTEXT;


#endif