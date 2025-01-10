/**
 * Quickfall PE executable compiling.
 */

#include <stdio.h>
#include <stdint.h>

#include "../structs.h"

#ifndef COMPILER_PE
#define COMPILER_PE

/**
 * Compiles into PE format.
 */
void compilePE(FILE* fptr, BYTECODE_BUFFER* buff);

#endif
