/**
 * Quickfall PE executable compiling.
 */

#include <string.h>
#include <stdio.h>
#include <stdint.h>

#include "./format.h"
#include "../structs.h"

/**
 * Compiles into PE format.
 */
void compilePE(FILE* fptr, BYTECODE_BUFFER* buff) {
	PE_DOS_HEADER dos_header = {0};
    dos_header.e_magic = 0x5A4D;  // "MZ"
    dos_header.e_lfanew = sizeof(PE_DOS_HEADER);

    fwrite(&dos_header, sizeof(dos_header), 1, fptr);

    // Create NT headers
    PE_NT_HEADERS nt_headers = {0};
    nt_headers.Signature = 0x00004550;  // "PE\0\0"
    nt_headers.Machine = 0x8664;        // x64
    nt_headers.NumberOfSections = 1;
    nt_headers.SizeOfOptionalHeader = sizeof(PE_OPTIONAL_HEADER);
    nt_headers.Characteristics = 0x0002 | 0x0100; // Executable | 32-bit machine

    fwrite(&nt_headers, sizeof(nt_headers), 1, fptr);

    // Create Optional header
    PE_OPTIONAL_HEADER optional_header = {0};
    optional_header.Magic = 0x20B;  // PE32+
    optional_header.AddressOfEntryPoint = 0x1000;
    optional_header.ImageBase = 0x400000;
    optional_header.SectionAlignment = 0x1000;
    optional_header.FileAlignment = 0x200;
    optional_header.MajorOperatingSystemVersion = 5;
    optional_header.MinorOperatingSystemVersion = 1;
    optional_header.MajorSubsystemVersion = 5;
    optional_header.MinorSubsystemVersion = 1;
    optional_header.SizeOfImage = 0x2000;
    optional_header.SizeOfHeaders = 0x200;
    optional_header.Subsystem = 3;  // Windows CUI
    optional_header.SizeOfStackReserve = 0x100000;
    optional_header.SizeOfStackCommit = 0x1000;
    optional_header.SizeOfHeapReserve = 0x100000;
    optional_header.SizeOfHeapCommit = 0x1000;
    optional_header.NumberOfRvaAndSizes = 16;

    fwrite(&optional_header, sizeof(optional_header), 1, fptr);

    // Create Section header
    PE_SECTION_HEADER section_header = {0};
    memcpy(section_header.Name, ".text", 5);
    section_header.Misc.VirtualSize = 0x1000;
    section_header.VirtualAddress = 0x1000;
    section_header.SizeOfRawData = buff->size;
    section_header.PointerToRawData = 0x200;
    section_header.Characteristics = 0x60000020;  // Code | Execute | Read

    fwrite(&section_header, sizeof(section_header), 1, fptr);

    // Write padding to align headers
    uint8_t padding[256] = {0};
    fwrite(padding, 1, 0x200 - (sizeof(PE_DOS_HEADER) + sizeof(PE_NT_HEADERS) + sizeof(PE_OPTIONAL_HEADER) + sizeof(PE_SECTION_HEADER)), fptr);

    fwrite(buff->buff, 1, buff->size, fptr);

    fclose(fptr);
}
