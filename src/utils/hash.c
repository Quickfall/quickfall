/**
 * Hashing related utilities.
 */

unsigned int hashstr(char* str) {
    unsigned int hash = 2166136261u;  // FNV offset basis
    unsigned char *p = (unsigned char *)str;
    
    while (*p) {
        char c = *p++;
        if(c == '\0') break;
        
        hash ^= c;
        hash *= 16777619u;  // FNV prime
        hash &= 0x0FFF;     // Keep only 12 bits to maintain smaller values
    }
    
    // Final mixing for better distribution in small range
    hash ^= hash >> 6;
    hash *= 0x9E3779B1;    // Golden ratio prime
    hash &= 0x0FFF;        // Final mask to 12 bits
    
    return hash;
}
