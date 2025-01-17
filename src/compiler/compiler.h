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

#endif
