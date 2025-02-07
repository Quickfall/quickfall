#include "../src/qasm/parser/parser.h"
#include "../src/ir/structs.h"

#include <stdlib.h>
#include <stdio.h>

int main(int argc, char** argv) {
    FILE* fptr = fopen(argv[1], "r");

    fseek(fptr, 0, SEEK_END);
    int size = ftell(fptr);
    rewind(fptr);

    char* buff = malloc(size + 1);
    fread(buff, 1, size, fptr);
    buff[size] = '\0';

    IR_FUNCTION* func = malloc(sizeof(IR_FUNCTION));

    func->blocks = malloc(sizeof(IR_BASIC_BLOCK*));
    
    func->blocks[0] = malloc(sizeof(IR_BASIC_BLOCK));

    func->blocks[0]->allocatedSize = 0;
    func->blocks[0]->instructions = NULL;
    func->blocks[0]->instructionCount = 0;

    parseQAsmInstructions(func, buff, size);

    printf("Detected instructions: %d!\n", func->blocks[0]->instructionCount);

    return 0;
}