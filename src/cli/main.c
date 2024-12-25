/*
 * The file of the Quickfall CLI.
 * A modern, fast and lightweight programming language.
 * Usage:
 * quickfall build <input> -o <output> - Builds the input files into an output.
 * quickfall help - Shows the help command.
 */

#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <stdint.h>

#include "../lexer/lexer.h"

#include "../parser/ast.h"
#include "../parser/parser.h"

#include "../compiler/compiler.h"
#include "../compiler/pe/pe.h"
#include "../compiler/ir.h"

#include "../utils/logging.c"

// Version
#define VERSION "0.1.0"

void showCommandEntry(char* commandName, char* description, int argumentCount, char* argumentNames[], char* argumentDescriptions[]) {
	printf("\n    >  %s\n\n       %s%sDescription%s: %s\n", commandName, STYLE_BOLD, STYLE_UNDERLINE, RESET, description);

	if(argumentCount > 0) {
		printf("\n\n       %s%sArguments%s: ", STYLE_BOLD, STYLE_UNDERLINE, RESET);
		for(int i = 0; i < argumentCount; ++i) {
			printf("\n         > %s%s%s: %s", TEXT_GRAY, argumentNames[i], RESET, argumentDescriptions[i]);
		}
		printf("\n");
	}
}

void showHelpMessage() {
	printf("\n%sQuickfall%s - The programming language.\n\nCommands:\n", TEXT_CYAN, RESET);
	
	char** arguments = malloc(5 * 24);
	char** argumentDescriptions = malloc(5 * 256);
	
	arguments[0] = "-p";
	argumentDescriptions[0] = "Determines the targeted platform. Defaults to the current platform.";
	
	arguments[1] = "-o";
	argumentDescriptions[1] = "Determines the output file of the compiling. Is required";

	showCommandEntry("compile, c", "Compiles the Quickfall program into an executable for the targeted platform.", 2, arguments, argumentDescriptions);
	
	showCommandEntry("version, v", "Shows the current version of Quickfall.", 0, NULL, NULL);

	showCommandEntry("help, h", "Shows a message with helpful Quickfall information.", 0, NULL, NULL);

}


int main(int argc, char* argv[]) {
	if(argc < 2) {
		showHelpMessage();
		return -1;
	}

	switch(argv[1][0]) {
		case 'c':
			if(strlen(argv[1]) > 1 && strcmp(argv[1], "compile") != 0) {
				printf("Unknown command!\n");
				showHelpMessage();
				break;
			}
					
			if(argc < 3) {
				printf("%sInvalid Usage! Correct command usage: quickfall compile <target>%s\n", TEXT_RED, RESET);
				return -1;
			}

			FILE* fptr = fopen(argv[2], "r");

			if(fptr == NULL) {
				printf("%sCouldn't open the %s file! Are you sure it exists?%s\n", TEXT_RED, argv[2], RESET);
				return -1;
			}

			char* outputFile = NULL;
			int foundOutput = 0;

			for(int i = 2; i < argc; ++i) {
				if(foundOutput) {
					outputFile = argv[i];
					break;
				}

				if(argv[i][1] == 'o') {
					foundOutput = 1;
				}
			}
			
			if(outputFile == NULL) {
				printf("%sError: the output file destination wasn't provided! Please use the -o argument to add an output!%s", TEXT_RED, RESET);
				return -1;
			}

			printf("%sReading source file...%s\n", TEXT_CYAN, RESET);

			fseek(fptr, 0, SEEK_END);
			int size = ftell(fptr);
			printf("  File size: %d bytes\n", size);
			fseek(fptr, 0, SEEK_SET);

			char* buff = malloc(size + 1); // Allocates one more byte for the \0 char.
			if (!buff) {
				printf("%sError: Failed to allocate buffer for source file%s\n", TEXT_RED, RESET);
				return -1;
			}

			size = fread(buff, 1, size, fptr);
			buff[size] = '\0';
			fclose(fptr);

			printf("%sLexing source code...%s\n", TEXT_CYAN, RESET);
			LEXER_RESULT result = runLexer(buff, size);
			printf("  Generated %d tokens\n", result.size);
			free(buff);

			printf("%sParsing tokens...%s\n", TEXT_CYAN, RESET);
			AST_NODE* root = parseNodes(result, 0, AST_ROOT);
			if (!root) {
				printf("%sError: Parser failed to create AST%s\n", TEXT_RED, RESET);
				free(result.tokens);
				return -1;
			}

			printf("%sGenerating IR context...%s\n", TEXT_CYAN, RESET);
			IR_CTX* ctx = makeContext(root);

			if(ctx == NULL) {
				printf("Error: the IR context is null! Something went wrong during compiling! Please check any logs for errors\n");
				return -1;
			}

			fptr = fopen(outputFile, "w");
			compile(ctx, fptr);
			
			break;
		case 'v':
			if(strlen(argv[1]) > 1 && strcmp(argv[1], "version") != 0) {
				printf("Unknown command!\n");
				showHelpMessage();
				break;
			}
			printf("Currently installed Quickfall version: %s\n", VERSION);
			break;
		case 'h':
			if(strlen(argv[1]) > 1 && strcmp(argv[1], "help") != 0) {
				printf("Unknown command!\n");
				showHelpMessage();
				break;
			}
			showHelpMessage();
			break;
		default:
			printf("Unknown command!\n");
			showHelpMessage();
	}

}

