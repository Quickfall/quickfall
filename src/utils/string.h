/**
 * String related utilities.
 */

#ifndef STRING_UTILS_H
#define STRING_UTILS_H

/**
 * Adds a char array (string) into another one.
 * @param output the output string.
 * @param input the input string.
 * @param index the starting index.
 * @return the last index.
 */
int fast_strcat(char* output, char* input, int index);

#endif