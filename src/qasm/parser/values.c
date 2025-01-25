/**
 * QuickAssembly Value types parsing.
 */

#include <stdlib.h>
#include <stdio.h>
#include <string.h>

/**
 * Parses a 32-bit integer into bytes.
 * @param buff the buff to append the bytes to.
 * @param index the index of where to append the bytes.
 * @param str the string containing the integer.
 */
void parseInt32(void** buff, int index, char* str) {
    int i = atoi(str);

    buff[index] = malloc(4);

    ((unsigned char*)buff[index])[0] = (i >> 24) & 0xFF;
    ((unsigned char*)buff[index])[1] = (i >> 16) & 0xFF;
    ((unsigned char*)buff[index])[2] = (i >> 8) & 0xFF;
    ((unsigned char*)buff[index])[3] = i & 0xFF;
}


/**
 * Parses a variable name.
 * @param buff the buff to append the bytes to.
 * @param index the index of where to append the bytes.
 * @param str the string containing the variable name.
 */
void parseVariableName(void** buff, int index, char* str) {
    if(str[0] != '%') {
        printf("Error: Variable names must start with %%! Got %s\n", str);
        return;
    }

    int size = strlen(str);
    buff[index] = malloc(size);

    char c;
    int i = 0;
    while(c == *str++) {
        ((unsigned char*)buff[index])[i] = c;
        
        ++i;

        if(c == ',') {
            ((unsigned char*)buff[index])[i] = ',';
            int t = atoi(str++);

            ((unsigned char*)buff[index])[i + 1] = (i >> 24) & 0xFF;
            ((unsigned char*)buff[index])[i + 2] = (i >> 16) & 0xFF;
            ((unsigned char*)buff[index])[i + 3] = (i >> 8) & 0xFF;
            ((unsigned char*)buff[index])[i + 4] = i & 0xFF;
        }

        if(c == '\0') break;
    }

}