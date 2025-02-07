#ifndef HEAP_DEBUG_H
#define HEAP_DEBUG_H

#include <stdlib.h>
#include <stdio.h>

#include "../src/utils/hashmap.h"

int current;

void *tracked_malloc(size_t size, const char *file, int line) {
    void* ptr = malloc(size);
    current += size;
    printf("Performed allocation at address %p of size %d (currently allocated: %d) at line %d of file %s\n", ptr, size, current, line, file);
    return ptr;
}

void tracked_free(void* ptr, const char* file, int line) {
    current -= sizeof(ptr);
    printf("Performed de-allocation of address %p of size %d (currently allocated: %d) at line %d of file %s)\n", ptr, sizeof(ptr), current, line, file);
    free(ptr);
}


// Macro to override malloc
#define malloc(size) tracked_malloc(size, __FILE__, __LINE__)
#define free(ptr) tracked_malloc(ptr, __FILE__, __LINE__)

#endif 