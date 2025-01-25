/**
 * The compiler of Quickfall.
 */

#include "../ir/structs.h"

#include "./structs.h"

#ifndef COMPILER_H
#define COMPILER_H

/**
 * Compiles the IR into actual bytecode.
 * @param out the IR output.
 */
BYTECODE_BUFFER* compile(IR_OUTPUT* out);


/**
 * Gets the compiled address from the pointer.
 * @param out the IR output.
 * @param ptr the pointer name.
 */
int getAddressFromPointer(IR_OUTPUT* out, char* ptr);

#endif
