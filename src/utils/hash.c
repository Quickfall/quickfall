/**
 * Hashing related utilities.
 */

#include <stdio.h>

unsigned int hashstr(char* str) {
	unsigned int result = 0;
	unsigned char* p = (unsigned char*) str;

	printf("str: %s -> ", str);

	while(*p != '\0') {
		printf("0x%x ", *p);

		result = ((*p - 97)  << 5) + result + 1;
		++p;
	}

	printf("\n");

	return result;
}
