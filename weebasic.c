typedef enum
{
    OP_PUSH_IMM,
    OP_LT,
    OP_BRANCHIF,
    OP_JUMP,
    OP_ADD,
    OP_SUB,
    OP_PRINT
} opcode_t;

typedef struct
{
    opcode_t op;
  
    union imm
    {
    }
} instr_t;

void eval()
{
}

int main(int argc, char** argv)
{
    return 0;
}
