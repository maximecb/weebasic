/****************************************************************************
*
* WEEBASIC
*
* Implementation of a parser and stack-based interpreter for a toy
* programming language. This language was built by Maxime Chevalier-Boisvert
* in 2021 as a study or interview, and is released under the unlicense.
*
****************************************************************************/

#include "stdbool.h"
#include "stdint.h"
#include "stdlib.h"
#include "stdio.h"
#include "string.h"
#include "assert.h"

// 64K instructions should be enough for anybody
#define MAX_INSTRS 65536
#define MAX_IDENT_LEN 64
#define MAX_LOCALS 128
#define MAX_STACK 32

// Kinds of instructions we support
typedef enum
{
    OP_EXIT = 0,
    OP_PUSH,
    OP_GETLOCAL,
    OP_SETLOCAL,
    OP_LT,
    OP_IFNOT,
    OP_ADD,
    OP_SUB,
    OP_READINT,
    OP_PRINT
} opcode_t;

// Format of the instructions we implement
typedef struct
{
    opcode_t op;

    union
    {
        uint64_t idx;
        int64_t int_val;
        const char* str_val;
    };

} instr_t;

// Local variable declaration
typedef struct LocalVar
{
    // Name of the variable
    char ident[MAX_IDENT_LEN];

    // Index of the local variable
    size_t idx;

    // Next variable in the list
    struct LocalVar* next;

} local_t;

// Consume whitespace chars in the input
void eat_ws(char** pstr)
{
    while (true)
    {
        char ch = **pstr;

        switch (ch)
        {
            // Keep reading as long as we see whitespace
            case ' ':
            case '\t':
            case '\r':
            case '\n':
            break;

            // Not whitespace, stop
            default:
            return;
        }

        // Move to the next character
        (*pstr)++;
    }
}

// Consume single-line comments
void eat_comment(char** pstr)
{
    while (true)
    {
        char ch = **pstr;

        if (ch == '\n')
        {
            // Move to the next character
            (*pstr)++;
            break;
        }

        if (ch == '\0')
        {
            break;
        }

        // Move to the next character
        (*pstr)++;
    }
}

// Match a keyword in the input
bool match_keyword(char** pstr, const char* keyword)
{
    eat_ws(pstr);

    size_t len_kw = strlen(keyword);
    if (strncmp(*pstr, keyword, len_kw) == 0)
    {
        *pstr += len_kw;
        eat_ws(pstr);
        return true;
    }

    return false;
}

// Fail to parse if a given keyword is not there
void expect_keyword(char** pstr, const char* keyword)
{
    if (!match_keyword(pstr, keyword))
    {
        fprintf(stderr, "expected keyword \"%s\"\n", keyword);
        exit(-1);
    }
}

// Parse an identifier
void parse_ident(char** pstr, char* ident_out)
{
    size_t ident_len = 0;

    while (true)
    {
        char ch = **pstr;

        if (ident_len >= MAX_IDENT_LEN - 1)
        {
            fprintf(stderr, "identifier too long\n");
            exit(-1);
        }

        if (ch >= 'a' && ch <= 'z')
        {
            ident_out[ident_len] = ch;
            ident_len++;

            // Move to the next character
            (*pstr)++;
            continue;
        }

        break;
    }

    if (ident_len == 0)
    {
        fprintf(stderr, "expected identifier\n");
        exit(-1);
    }

    assert(ident_len <= MAX_IDENT_LEN - 1);
    ident_out[ident_len] = '\0';
}

// Parse a positive integer constant
int64_t parse_int(char** pstr)
{
    int64_t num = 0;

    while (true)
    {
        char ch = **pstr;

        if (ch >= '0' && ch <= '9')
        {
            int64_t digit = ch - '0';
            num = 10 * num + digit;

            // Move to the next character
            (*pstr)++;
            continue;
        }

        break;
    }

    return num;
}

// Try to find a local variable declaration
local_t* find_local(local_t* local_vars, const char* ident)
{
    for (local_t* var = local_vars; var != NULL; var = var->next)
    {
        if (strcmp(var->ident, ident) == 0)
        {
            return var;
        }
    }

    return NULL;
}

// Macros to append an instruction to the program
#define APPEND_INSN(op_type) ( insns[(*insn_idx)++] = (instr_t){ op_type }, &insns[(*insn_idx)-1] )
#define APPEND_INSN_IMM(op_type, imm) ( insns[(*insn_idx)++] = (instr_t){ op_type, imm }, &insns[(*insn_idx)-1] )

// Parse an atomic expression
void parse_atom(char** pstr, instr_t* insns, size_t* insn_idx, local_t* locals)
{
    char ch = **pstr;

    // Read an integer from the console
    if (match_keyword(pstr, "read_int"))
    {
        APPEND_INSN(OP_READINT);
        return;
    }

    // Integer constant
    if (ch >= '0' && ch <= '9')
    {
        int64_t num = parse_int(pstr);
        APPEND_INSN_IMM(OP_PUSH, num);
        return;
    }

    // Reference to a variable
    if (ch >= 'a' && ch <= 'z')
    {
        // Parse the variable name
        char ident[MAX_IDENT_LEN];
        parse_ident(pstr, ident);

        // Try to find the declaration
        local_t* local = find_local(locals, ident);

        if (!local)
        {
            fprintf(stderr, "reference to undeclared variable \"%s\"\n", ident);
            exit(-1);
        }

        APPEND_INSN_IMM(OP_GETLOCAL, local->idx);
        return;
    }

    fprintf(stderr, "invalid expression");
    exit(-1);
}

// Parse an expression
void parse_expr(char** pstr, instr_t* insns, size_t* insn_idx, local_t* locals)
{
    // Parse a first expression
    parse_atom(pstr, insns, insn_idx, locals);

    eat_ws(pstr);

    char ch = **pstr;

    if (match_keyword(pstr, "+"))
    {
        // Parse the RHS expression
        parse_atom(pstr, insns, insn_idx, locals);

        // Add the result
        APPEND_INSN(OP_ADD);
        return;
    }

    if (match_keyword(pstr, "-"))
    {
        // Parse the RHS expression
        parse_atom(pstr, insns, insn_idx, locals);

        // Subtract the result
        APPEND_INSN(OP_SUB);
        return;
    }

    if (match_keyword(pstr, "<"))
    {
        // Parse the RHS expression
        parse_atom(pstr, insns, insn_idx, locals);

        // Compare the arguments
        APPEND_INSN(OP_LT);
        return;
    }
}

// Parse a statement
void parse_stmt(char** pstr, instr_t* insns, size_t* insn_idx, local_t** plocals)
{
    // Consume whitespace
    eat_ws(pstr);

    // Single-line comments
    if (match_keyword(pstr, "#"))
    {
        eat_comment(pstr);
        return;
    }

    // Local variable declaration
    if (match_keyword(pstr, "let"))
    {
        // Parse the variable name
        char ident[MAX_IDENT_LEN];
        parse_ident(pstr, ident);

        expect_keyword(pstr, "=");

        // Parse the expression we are assigning
        parse_expr(pstr, insns, insn_idx, *plocals);

        // Make sure this isn't a redeclaration
        local_t* first_local = *plocals;
        if (find_local(first_local, ident))
        {
            fprintf(stderr, "local variable \"%s\" already declared\n", ident);
            exit(-1);
        }

        // Create a new local variable
        local_t* new_local = malloc(sizeof(local_t));
        strcpy(new_local->ident, ident);
        new_local->idx = first_local? (first_local->idx + 1):0;
        new_local->next = first_local;
        *plocals = new_local;

        // Set the local to the expression's value
        APPEND_INSN_IMM(OP_SETLOCAL, new_local->idx);

        return;
    }

    if (match_keyword(pstr, "if"))
    {
        // Parse the test expression
        parse_expr(pstr, insns, insn_idx, *plocals);

        expect_keyword(pstr, "then");

        // If the result is false, jump past the if clause
        instr_t* ifnot_insn = APPEND_INSN_IMM(OP_IFNOT, 0);

        // Parse the body of the if statement
        parse_stmt(pstr, insns, insn_idx, plocals);

        // If the condition is false, we jump after the body of the if
        instr_t* pfalse = &insns[*insn_idx];
        ifnot_insn->int_val = (pfalse - ifnot_insn) - 1;

        return;
    }

    // Sequencing of statements
    if (match_keyword(pstr, "begin"))
    {
        while (true)
        {
            if (match_keyword(pstr, "end"))
            {
                break;
            }

            parse_stmt(pstr, insns, insn_idx, plocals);
        }

        return;
    }

    // Print to stdout
    if (match_keyword(pstr, "print"))
    {
        parse_expr(pstr, insns, insn_idx, *plocals);
        APPEND_INSN(OP_PRINT);
        return;
    }

    // Cap the string length for printing
    if (strlen(*pstr) > 10)
    {
        (*pstr)[10] = '\0';
    }

    fprintf(stderr, "invalid statement: \"%s [...]\"\n", *pstr);
    exit(-1);
}

// Parse a source file into a sequence of instructions
instr_t* parse_file(const char* file_name)
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
    char* input_str = malloc(fsize + 1);
    fread(input_str, 1, fsize, file);
    input_str[fsize] = 0;
    fclose(file);

    // We will use a doubly-indirected pointer for parsing
    char* current_ch = input_str;
    char** pstr = &current_ch;

    // Instruction array
    instr_t* insns = malloc(sizeof(instr_t) * MAX_INSTRS);
    size_t insn_idx = 0;

    // Table of local variables
    local_t* local_vars = NULL;

    // Until we reach the end of the input
    while (true)
    {
        // End of input
        if (*current_ch == '\0')
        {
            break;
        }

        parse_stmt(pstr, insns, &insn_idx, &local_vars);
    }

    free(input_str);

    return insns;
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

// Stack manipulation primitives
#define PUSH(v) ( stack[stack_size] = v, stack_size++ )
#define POP() ( stack_size--, stack[stack_size] )

void eval(const instr_t* insns)
{
    // Local variables
    int64_t vars[MAX_LOCALS];

    // Stack of temporary values
    int64_t stack[MAX_STACK];
    size_t stack_size = 0;

    for (const instr_t* pc = insns; pc != NULL; ++pc)
    {
        //printf("stack_size=%zu\n", stack_size);

        switch (pc->op)
        {
            // Exit the program
            case OP_EXIT:
            return;

            case OP_PUSH:
            PUSH(pc->int_val);
            break;

            case OP_SETLOCAL:
            vars[pc->idx] = POP();
            break;

            case OP_GETLOCAL:
            PUSH(vars[pc->idx]);
            break;

            case OP_LT:
            {
                int64_t arg1 = POP();
                int64_t arg0 = POP();
                PUSH((arg0 < arg1)? 1:0);
            }
            break;

            case OP_IFNOT:
            {
                int64_t test_val = POP();

                if (!test_val)
                {
                    uint64_t jump_offset = pc->int_val;
                    pc += jump_offset;
                }
            }
            break;

            case OP_ADD:
            {
                int64_t arg1 = POP();
                int64_t arg0 = POP();
                PUSH(arg0 + arg1);
            }
            break;

            case OP_SUB:
            {
                int64_t arg1 = POP();
                int64_t arg0 = POP();
                PUSH(arg0 - arg1);
            }
            break;

            case OP_READINT:
            {
                printf("Input an integer value:\n");
                printf("> ");

                int64_t int_val = 0;
                while (true)
                {
                    char c = fgetc(stdin);
                    if (c == EOF || c < '0' || c > '9')
                        break;
                    int64_t digit = c - '0';
                    int_val = 10 * int_val + digit;
                }

                PUSH(int_val);
            }
            break;

            case OP_PRINT:
            {
                int64_t int_val = POP();
                printf("print: %ld\n", int_val);
            }
            break;

            default:
            fprintf(stderr, "unknown bytecode instruction\n");
            exit(-1);
        }
    }
}

int main(int argc, char** argv)
{
    if (argc == 2)
    {
        instr_t* insns = parse_file(argv[1]);
        eval(insns);
    }

    return 0;
}
