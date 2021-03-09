#include "stdlib.h"
#include "stdio.h"
#include "string.h"
#include "assert.h"

// Q1: make add concatenate strings

// Q2: subexpressions with parentheses

// Q3: while loop

// Q4: how would you go about implementing function calls in this?

typedef enum
{
    OP_PUSH,
    OP_GETLOCAL,
    OP_SETLOCAL,
    OP_LT,
    OP_BRANCHIF,
    OP_JUMP,
    OP_ADD,
    OP_SUB,
    OP_READ,
    OP_PRINT,
    OP_EXIT
} opcode_t;

typedef struct
{
    opcode_t op;

    union
    {
        int64_t int_val;
        const char* str_val;
    };

} instr_t;

instr_t* parse(const char* file_name)
{
    FILE* file = fopen(file_name, "r");

    if (!file)
    {
        fprintf(stderr, "failed to open source file \"%s\"\n", file_name);
        exit(-1);
    }

    // Read the entire file into a string buffer
    fseek(file, 0, SEEK_END);
    long fsize = ftell(file);
    fseek(file, 0, SEEK_SET);
    char* pstr = malloc(fsize + 1);
    fread(pstr, 1, fsize, file);
    fclose(file);
    pstr[fsize] = 0;

    fprintf(stderr, "read %ld bytes\n", fsize);




    uint32_t num_locals = 0;


    // TODO: read line by line




    free(pstr);


    return NULL;
}

int64_t tag(int64_t val)
{
    return (val << 1) | 1;
}

int64_t untag(int64_t word)
{
    assert (word & 1);
    return word >> 1;
}

void eval(const instr_t* insns)
{
    // TODO: locals

    // TODO: stack

    for (const instr_t* pc = insns; pc != NULL; ++pc)
    {
        switch (pc->op)
        {
            case OP_PUSH:
            break;



            case OP_EXIT:
            return;
        }
    }
}

int main(int argc, char** argv)
{
    if (argc == 2)
    {
        instr_t* insns = parse(argv[1]);


    }

    return 0;
}
