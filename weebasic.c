typedef enum
{
    OP_PUSH_IMM;

} opcode_t;

typedef struct
{
    opcode_t op;
  
    union imm
    {
    }
} instr_t;
      
int main(int argc, char** argv)
{
    return 0;
}
