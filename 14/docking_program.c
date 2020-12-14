#include "stdio.h"
#include "stdlib.h"
#include "string.h"
#include "stdint.h"
#include "assert.h"

#define BIT_DEPTH 36

int main(void)
{
    FILE* input = fopen("input.txt", "r");
    if (input == NULL) { printf("Could not open file\n"); exit(EXIT_FAILURE); }

    uint64_t memSize = 65535;
    uint64_t* mem = (uint64_t*)calloc(memSize, sizeof(uint64_t));
    if (mem == NULL) { printf("Could not allocate memory\n"); goto CLEANUP; }

    uint64_t finalSum = 0;

    char line[80];
    char maskStr[BIT_DEPTH + 1];

    uint64_t address, number;
    uint64_t onesMask = 0, zerosMask = 0;

    while (fgets(line, 80, input)) {

        // This is a mask write
        if (0 != sscanf(line, "mask = %s", maskStr)) {
            onesMask = 0;
            zerosMask = 0;
            for (int i = 0; i < BIT_DEPTH; i++) {
                zerosMask <<= 1;
                onesMask <<= 1;
                switch(maskStr[i]) {
                    case '0': zerosMask |= 1UL; break;
                    case '1': onesMask  |= 1UL; break;
                }
            }
        }

        // This is a memory write
        if (0 != sscanf(line, "mem[%lu] = %lu", &address, &number)) {
            number |= onesMask;
            number &= ~zerosMask;
            assert(address < memSize);
            mem[address] = number;
        }
    }

    for (uint64_t i = 0; i < memSize; i++)
        finalSum += mem[i];

    printf("Memory Sum: %lu\n", finalSum);

CLEANUP:
    free(mem);
    fclose(input);
}
