#include "stdio.h"
#include "string.h"
#include "assert.h"

typedef enum
{
    OP_PUSH_IMM,
    OP_LT,
    OP_BRANCHIF,
    OP_JUMP,
    OP_ADD,
    OP_SUB,
    OP_PRINT
    OP_EXIT
} opcode_t;

typedef struct
{
    opcode_t op;
  
    union imm
    {
    }
} instr_t;

instr_t* parse(const char* file_name)
{
    return NULL;
}

int64_t tag(int64_t int)
{
    return (int << 1) | 1;
}

int64_t untag(int64_t word)
{
    assert (word & 1);
    return word >> 1;
}

void eval(const instr_t* insns)
{
    for (const instr_t* pc = insns; pc != NULL; ++pc)
    {
        switch (pc->op)
        {



            case OP_EXIT:
            return;
        }
    }    
}

int main(int argc, char** argv)
{
    if (argc == 2)
    {
        
        
        
    }
    
    return 0;
}
