/**
 * String related utilities.
 */

/**
 * Adds a char array (string) into another one.
 * @param output the output string.
 * @param input the input string.
 * @param index the starting index.
 * @return the last index.
 */
int fast_strcat(char* output, char* input, int index) {
    char c;
    while(c = *input++) {
        output[index] = c;
        
        if(c == '\0') return index;

        index++;
    }

    return index;
}