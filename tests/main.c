/**
 * Quickfall's Testing suite.
 */

#include <string.h>
#include <stdlib.h>
#include <stdio.h>

#include "../src/utils/logging.c"

#include "./lexer.h"
#include "./parser.h"
#include "./ir.h"

void runTest(char* buff, char* testName) {
	int code = 0;

	if(strcmp(testName, "lexer") == 0) {
		code = runLexerTest(buff);	
	}
	else if(strcmp(testName, "parser") == 0) {
		code = runParserTest(buff);
	}
	else if(strcmp(testName, "ir") == 0) {
		code = runIRTest(buff);
	}
	else {
		printf("Unknown Test!\n");
	}

	if(code == -1) printf("[%sFAILED%s] The test %s failed!\n", TEXT_HRED, RESET, testName);
	else printf("[%sOK%s] The test %s was OK!\n", TEXT_HGREEN, RESET, testName);
}

int mainsdsd(int argc, char** argv) {
	if(argc < 3) {
		printf("Usage: <executable> <file> <tests...>\0");
		return -1;
	}

	FILE* fptr = fopen(argv[1], "r");
	
	fseek(fptr, 0, SEEK_END);
	int size = ftell(fptr);
	fseek(fptr, 0, SEEK_SET);

	char* buff = malloc(size + 1);

	fread(buff, 1, size, fptr);
	fclose(fptr);

	buff[size] = '\0';

	for(int i = 2; i < argc; ++i) {
		char* testName = argv[i];

		if(strcmp(testName, "all") == 0) {
			char* tests[] = {"lexer", "parser", "ir"};
			
			for(int i = 0; i < 3; ++i) {
				runTest(buff, tests[i]);
			}
		}
		else {
			runTest(buff, testName);
		}
	}
}
