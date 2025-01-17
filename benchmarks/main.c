/**
 * Compiling Benchmark of Quickfall.
 */

#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <windows.h>
#include <math.h>

#include "../src/compiler/compiler.h"
#include "../src/compiler/structs.h"
#include "../src/compiler/pe/pe.h"

#include "../src/ir/ir.h"
#include "../src/ir/instructions.h"
#include "../src/ir/structs.h"

#include "../src/parser/parser.h"
#include "../src/parser/structs/tree.h"
#include "../src/parser/ast.h"
#include "../src/utils/logging.c"

// Benchmark Settings
#define BENCH_DEFAULT_RUNS 100
#define BENCH_BAR_LENGTH 25

//
// Timing Utilities
//
#ifdef WIN32

#include <windows.h>
double get_time()
{
    LARGE_INTEGER t, f;
    QueryPerformanceCounter(&t);
    QueryPerformanceFrequency(&f);
    return ((double)t.QuadPart/(double)f.QuadPart) * 1000000;
}

#else

#include <sys/time.h>
#include <sys/resource.h>

double get_time()
{
    struct timeval t;
    struct timezone tzp;
    gettimeofday(&t, &tzp);
    return t.tv_sec + t.tv_usec*1e-6;
}

#endif

char** categories;

double totalTimeTaken;
double startTime;

struct CategoryStatistics {
	double max;
	double low;
	double total;
	double* runs;
};

struct CategoryStatistics stats[5];

void startTimer() {
    double time = get_time();
    if(time > 0) startTime = time;
}

void endTimer(int run, int category) {
    double time = (get_time() - startTime);
    totalTimeTaken += time;
    stats[category].total += time;
    stats[category].runs[run] = time;
    if(stats[category].max < time) stats[category].max = time;
    if(stats[category].low > time) stats[category].low = time;
}

/**
 * Starts the monitoring
 */

void main(int argc, char* argv[]) {
    if(argc < 2) {
        printf("Usage: <executable> <file> [runs, default = 100]");
        return;
    }

    int runs = BENCH_DEFAULT_RUNS;

    if(argc > 2) {
        runs = atoi(argv[2]);
    }

    char* c[6] = {"File IO (Open)", "Lexer", "AST", "IR", "Compiler", "Artifact Generator"};
    categories = c;

    for(int i = 0; i < 6; ++i) {
        stats[i].total = 0;
        stats[i].max = 0;
        stats[i].low = 1000000;
        stats[i].runs = malloc(sizeof(double) * runs);

        for(int ii = 0; ii < runs; ++ii) {
            stats[i].runs[ii] = 0;
        }
    }

    for(int i = 0; i < runs; ++i) {

        startTimer();
        FILE* fptr = fopen(argv[1], "r");

        fseek(fptr, 0, SEEK_END);
        int size = ftell(fptr);
        fseek(fptr, 0, SEEK_SET);

        char* buff = (char*)malloc(size + 1);

        fread(buff, 1, size, fptr);
	    buff[size] = '\0';
        fclose(fptr);

        endTimer(i, 0);
        startTimer();

        LEXER_RESULT result = runLexer(buff, size);

	    free(buff);

        endTimer(i, 1);

        startTimer();

        void* node = parseRoot(result, 0, AST_TYPE_ROOT);
	
        endTimer(i, 2);
        startTimer();

        IR_OUTPUT* out = parseIR((AST_TREE_BRANCH*)node);

        endTimer(i, 3);
        startTimer();

	    BYTECODE_BUFFER* b = compile(out);

        endTimer(i, 4);
        startTimer();

	    fptr = fopen("output.exe", "w");
        compilePE(fptr, b);
        
	    fclose(fptr);

        endTimer(i, 5);
    }

    printf("========= Benchmarking Results =========\n");
    printf("Total time taken: %.3f micros, Average time per run: %.3f\n micros\n\n", totalTimeTaken, totalTimeTaken / runs);

    for(int i = 0; i < 6; ++i) {
	    printf("Benchmarking Results of %s:\n", categories[i]);
	    printf("  Total time duration: %s%.2fus%s (%s%.1f%%%s over total running time)\n", TEXT_HCYAN, stats[i].total, RESET, TEXT_CYAN, (stats[i].total / totalTimeTaken) * 100, RESET);
	    printf("  Average: %s%.2fus%s\n", TEXT_HCYAN, stats[i].total / runs, RESET);
	    printf("  Range (%sFastest%s, %sLowest%s): %s%0.fus%s  ... %s%.2fus%s\n\n", TEXT_HGREEN, RESET, TEXT_HRED, RESET, TEXT_HGREEN, stats[i].low, RESET, TEXT_HRED, stats[i].max, RESET);
	
	    double averages[10];
	    double highestAverage = 0;

        for(int ii = 0; ii < 10; ++ii) {
            averages[ii] = 0;
        }

        for(int ii = 0; ii < runs; ++ii) {
            averages[ii / 10] += stats[i].runs[ii] / 10;
        }

        for(int ii = 0; ii < 10; ++ii) {
            if(averages[ii] > highestAverage) highestAverage = averages[ii];
        }	

        char spacing[3];

        for(int ii = 0; ii < 10; ++ii) {
            spacing[0] = '\0';
            if(ii == 0) strcat(spacing, "  \0");
            else if(ii != 9) strcat(spacing, " \0");

            int clean = (averages[ii] / highestAverage) * BENCH_BAR_LENGTH;

            printf("  %d%% to %d%% Percentile:%s%s ", ii * 10, (ii + 1) * 10, spacing, TEXT_GRAY);
            for(int c = 0; c < clean; ++c) {
                printf("#");
            }
            printf("%s (%s%.2fus%s average)\n", RESET, TEXT_HCYAN, averages[ii], RESET);
        }

        double timeWithoutHighest = stats[i].total - highestAverage * 10;
        printf("\n  Total time duration without highest avg: %s%.1fus%s (%s%.1f%%%s over total running time)", TEXT_HCYAN, timeWithoutHighest, RESET, TEXT_CYAN, (timeWithoutHighest / totalTimeTaken) * 100, RESET);
        printf("\n  Average without highest avg: %s%.2fus%s\n\n", TEXT_HCYAN, (timeWithoutHighest / runs), RESET);
    }
}
