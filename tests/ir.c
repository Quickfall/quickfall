/**
 * Test to check if the IR is working correctly.
 */

#include <string.h>
#include <stdio.h>
#include <stdlib.h>

#include "../src/lexer/lexer.h"
#include "../src/lexer/tokens.h"

#include "../src/parser/parser.h"
#include "../src/parser/ast.h"

#include "../src/compiler/compiler.h"
#include "../src/compiler/ir.h"

int runIRTest(char* buff) {
	LEXER_RESULT result = runLexer(buff, strlen(buff));
	AST_NODE* node = parseNodes(result, 0, AST_ROOT);

	IR_CTX* ctx = makeContext(node);

	if(ctx == NULL) return -1;

	printf("CTX dump:\n\n");

	for(int i = 0; i < ctx->nodeIndex; ++i) {
		if(ctx->nodes[i]->nodeName != NULL) printf("  %s (type: %d)\n", ctx->nodes[i]->nodeName, ctx->nodes[i]->nodeType);
	}

	return 1;
}
