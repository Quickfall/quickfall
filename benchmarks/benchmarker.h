#include <stdlib.h>
#include <stdio.h>

#ifndef BENCHMARKER_H
#define BENCHMARKER_H

#define DEBUG_CHILD_DEFAULT_ALLOC 10

#include <windows.h>
double get_time()
{
    LARGE_INTEGER t, f;
    QueryPerformanceCounter(&t);
    QueryPerformanceFrequency(&f);
    return ((double)t.QuadPart/(double)f.QuadPart) * 1000000;
}

typedef struct CallStack {

    char* functionName;
    long timing;

    struct CallStack* children;
    struct CallStack* parent;
    int childAlloc;
    int childSize;

} CALL_STACK;

CALL_STACK* debugNode = NULL;
CALL_STACK* currDebugNode = NULL;

CALL_STACK* createCallStack(char* funcName, CALL_STACK* parent) {
    CALL_STACK* stack = (CALL_STACK*)malloc(sizeof(CALL_STACK));
    stack->parent = parent;
    stack->functionName = funcName;
    stack->children = (CALL_STACK*) malloc(sizeof(CALL_STACK) * DEBUG_CHILD_DEFAULT_ALLOC);
    stack->childAlloc = DEBUG_CHILD_DEFAULT_ALLOC;
    stack->childSize = 0;

    return stack;
}

void recordFunctionCall(char* functionName) {
    if(debugNode == NULL) {
        debugNode = createCallStack("main", NULL);
        currDebugNode = debugNode;
    }

    CALL_STACK* stack = createCallStack(functionName, currDebugNode);


    if(currDebugNode->childSize >= currDebugNode->childAlloc) {
        currDebugNode->childAlloc *= 1.25;
        currDebugNode->children = (CALL_STACK*) realloc(currDebugNode->children, sizeof(CALL_STACK) * currDebugNode->childAlloc);
    }

    currDebugNode->children[currDebugNode->childSize] = *stack;
    currDebugNode->childSize++;

    currDebugNode = stack;

    stack->timing = get_time();
}

void finishFunctionCall() {
    currDebugNode->timing = get_time() - currDebugNode->timing;
    currDebugNode = currDebugNode->parent;
}

void printCallStack(CALL_STACK* stack, int depth) {
    if(!stack) return;

    for(int i = 0; i < depth; ++i) {
        printf(" ");
    }

    printf("%s [%.2fus]\n", stack->functionName, stack->timing);

    for(int i = 0; i < stack->childSize; ++i) {
        printCallStack(&stack->children[i], depth + 1);
    }
}

#endif

// Returning functions
#define fopen(...) \
    ({ \
        recordFunctionCall("fopen"); \
        __auto_type ptr = fopen(__VA_ARGS__); \
        finishFunctionCall(); \
        return ptr; \
    }) 

#define malloc(...) \
    ({ \
        recordFunctionCall("malloc"); \
        __auto_type ptr = malloc(__VA_ARGS__); \
        finishFunctionCall(); \
        return ptr;  \
    }) 


#define fread(...) \
    recordFunctionCall("fread"); \
    __auto_type ptr = fread(__VA_ARGS__); \
    finishFunctionCall(); \
    return ptr; 


#define fclose(...) \
    recordFunctionCall("fclose"); \
    fclose(__VA_ARGS__); \
    finishFunctionCall(); 