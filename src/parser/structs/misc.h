#ifndef PARSER_STURCTS_MISC_H
#define PARSER_STRUCTS_MISC_H

typedef struct AST_USE_STD {

    unsigned char* type;
    void* next; 
    int endingIndex;

    char* stdPath;

} AST_USE_STD;

typedef struct AST_IMPORT_FILE {

    unsigned char* type;
    void* next; 
    int endingIndex;

    char* path;

} AST_IMPORT_FILE;

#endif