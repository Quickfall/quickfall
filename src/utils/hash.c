/**
 * Hashing related utilities.
 */

unsigned int hashstr(char* str) {
	unsigned int result = 0;
	unsigned char* p = (unsigned char*) str;

	while(*p != '\0') {
		result = ((*p - 97)  << 5) + result + 1;
		++p;
	}

	return result;
}
