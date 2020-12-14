#include "stdio.h"
#include "stdlib.h"
#include "string.h"
#include "stdint.h"

#define BIT_DEPTH 36

typedef struct {
    uint64_t addr;
    uint64_t value;
} Data;

typedef struct {
    Data* data;
    unsigned available;
    unsigned size;
} Mem;

Data* Mem_Find(Mem* mem, uint64_t addr)
{
    for (unsigned i = 0; i < mem->size; i++)
        if (mem->data[i].addr == addr) return mem->data + i;
    return NULL;
}

Data* Mem_Add(Mem* mem, Data d)
{
    if (mem->size >= mem->available) {
        unsigned ns = mem->available * 2 + 16;
        mem->data = realloc(mem->data, ns * sizeof (Data));
        if (mem->data == NULL) 
            {printf("Can't reallocate\n"); exit(EXIT_FAILURE);}
        mem->available = ns;
    }
    
    mem->data[mem->size].value = d.value;
    mem->data[mem->size].addr = d.addr;
    return mem->data + (mem->size++);
}

int main(void)
{
    FILE* input = fopen("input.txt", "r");
    if (input == NULL) { printf("Could not open file\n"); exit(EXIT_FAILURE); }

    Mem memt[1] = {0};

    uint64_t finalSum = 0;

    char line[80];
    char maskStr[BIT_DEPTH + 1];

    uint64_t address, number;
    uint64_t onesMask = 0, changeMask = 0;
    unsigned int xNum = 0, xComb = 0;

    int lineIdx = 0;
    while (fgets(line, 80, input)) {

        // This is a mask write
        if (0 != sscanf(line, "mask = %s", maskStr)) {
            onesMask = changeMask = 0;
            xComb = 1;
            xNum = 0;

            for (int i = 0; i < BIT_DEPTH; i++) {
                onesMask <<= 1;
                changeMask <<= 1;
                switch(maskStr[i]) {
                    case '1': onesMask   |= 1UL; break;
                    case 'X': changeMask |= 1UL; xNum++; break;
                }
            }

            xComb <<= xNum;
        }

        // This is a memory write
        if (0 != sscanf(line, "mem[%lu] = %lu", &address, &number)) {
            address &= ~changeMask;
            address |= onesMask;
            for (uint64_t x = 0; x < xComb; x++) {
                uint64_t tempAddress = address;
                for (uint64_t b = 0, d = 0; b < BIT_DEPTH; b++) {
                    if (changeMask & (1UL << b)) {
                        tempAddress |= ((x >> d & 1UL) ? 1UL : 0) << b;
                        d++;
                    }

                }
                Data* mema = Mem_Find(memt, tempAddress);
                if (mema == NULL) {
                    Data newData = {.addr = tempAddress, .value = number};
                    mema = Mem_Add(memt, newData);
                }
                mema->value = number;
            }
        }

        if (lineIdx++ >= 3)
            lineIdx--;
    }

    for (uint64_t i = 0; i < memt->size; i++)
        finalSum += memt->data[i].value;

    printf("Memory Sum: %lu\n", finalSum);

    free(memt->data);
    fclose(input);
}
