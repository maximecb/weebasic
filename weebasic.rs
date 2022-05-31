/****************************************************************************
*
* WEEBASIC
*
* Implementation of a parser and stack-based interpreter for a toy
* programming language. This language was built by Maxime Chevalier-Boisvert
* in 2021-2022 for study or interview purposes, and is released under the
* unlicense.
*
****************************************************************************/

#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_mut)]

use std::env;
use std::fs;
use std::collections::HashMap;

// Kinds of instructions (opcodes) we support
enum Op
{
    Exit,
    Error,
    Push,
    GetLocal,
    SetLocal,
    Equal,
    LessThan,
    If,
    IfNot,
    Add,
    Sub,
    ReadInt,
    Print
}

/*
// Immutable, heap-allocated string object
typedef struct
{
    // String length, excluding null-terminator
    size_t len;

    // String data
    char* data;

} string_t;

typedef union
{
    uint64_t idx;
    int64_t int_val;
    string_t* str;

} value_t;
*/

// Format of the instructions we implement
struct Insn
{
    op: Op,

    //value_t imm;

}


struct Program
{
    insns: Vec<Insn>,

    num_locals: usize,

    local_idxs: HashMap<String, usize>,
}

impl Program
{
    fn new() -> Self
    {
        Program {
            insns: Vec::default(),
            num_locals: 0,
            local_idxs: HashMap::default(),
        }
    }
}









/*
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

bool is_int(value_t val)
{
    return val.int_val & 1;
}

value_t tag(int64_t val)
{
    return (value_t)((val << 1) | 1);
}

int64_t untag(value_t val)
{
    assert (is_int(val));
    return val.int_val >> 1;
}
*/








struct Input
{
    /// Characters of the input string
    chars: Vec<char>,

    /// Current position in the input
    pos: usize,
}

impl Input
{
    fn new(input_str: String) -> Self
    {
        Input {
            chars: input_str.chars().collect(),
            pos: 0,
        }
    }

    // Peek at the current input character
    fn peek_char(&self) -> char
    {
        self.chars[self.pos]
    }

    // Consume whitespace chars in the input
    fn eat_ws(&mut self)
    {
        loop
        {
            let ch = self.peek_char();

            match ch
            {
                // Keep reading as long as we see whitespace
                ' ' | '\t' | '\r' | '\n' => {},

                // Not whitespace, stop
                _ => return,
            }

            // Move to the next character
            self.pos += 1;
        }
    }

    // Consume single-line comments
    fn eat_comment(&mut self)
    {
        loop
        {
            let ch = self.peek_char();

            match ch
            {
                '\n' => {
                    // Move to the next character
                    self.pos += 1;
                    break;
                },

                '\0' => {
                    break;
                },

                _ => {
                    self.pos += 1;
                }
            }
        }
    }







}




















/*
// Match a token in the input
bool match_token(char** pstr, const char* token)
{
    input.eat_ws()

    size_t len_tok = strlen(token);
    if (strncmp(*pstr, token, len_tok) == 0)
    {
        *pstr += len_tok;
        input.eat_ws()
        return true;
    }

    return false;
}

// Fail to parse if a given token is not there
void expect_token(char** pstr, const char* token)
{
    if (!match_token(pstr, token))
    {
        fprintf(stderr, "expected token \"%s\"\n", token);
        exit(-1);
    }
}

// Parse an identifier
void parse_ident(char** pstr, char* ident_out)
{
    size_t ident_len = 0;

    while (true)
    {
        let ch = input.peek_char()

        if (ident_len >= MAX_IDENT_LEN - 1)
        {
            fprintf(stderr, "identifier too long\n");
            exit(-1);
        }

        if (!isalnum(ch) && ch != '_')
        {
            break;
        }

        // Store this character
        ident_out[ident_len] = ch;
        ident_len++;

        // Move to the next character
        (*pstr)++;
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
        let ch = input.peek_char()

        if (!isdigit(ch))
            break;

        // Store this digit
        int64_t digit = ch - '0';
        num = 10 * num + digit;

        // Move to the next character
        (*pstr)++;
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
    let ch = input.peek_char()

    // Read an integer from the console
    if (match_token(pstr, "read_int"))
    {
        APPEND_INSN(OP_READINT);
        return;
    }

    // Integer constant
    if (isdigit(ch))
    {
        int64_t num = parse_int(pstr);
        APPEND_INSN_IMM(OP_PUSH, num);
        return;
    }

    // Reference to a variable
    if (isalpha(ch) || ch == '_')
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

    input.eat_ws()

    let ch = input.peek_char()

    if (match_token(pstr, "+"))
    {
        // Parse the RHS expression
        parse_atom(pstr, insns, insn_idx, locals);

        // Add the result
        APPEND_INSN(OP_ADD);
        return;
    }

    if (match_token(pstr, "-"))
    {
        // Parse the RHS expression
        parse_atom(pstr, insns, insn_idx, locals);

        // Subtract the result
        APPEND_INSN(OP_SUB);
        return;
    }

    if (match_token(pstr, "=="))
    {
        // Parse the RHS expression
        parse_atom(pstr, insns, insn_idx, locals);

        // Compare the arguments
        APPEND_INSN(OP_EQ);
        return;
    }

    if (match_token(pstr, "<"))
    {
        // Parse the RHS expression
        parse_atom(pstr, insns, insn_idx, locals);

        // Compare the arguments
        APPEND_INSN(OP_LT);
        return;
    }
}
*/

// Parse a statement
fn parse_stmt(input: &mut Input, prog: &mut Program)
{
    // Consume whitespace
    input.eat_ws();



    /*
    // Single-line comments
    if (match_token(pstr, "#"))
    {
        input.eat_comment();
        return;
    }

    // Local variable declaration
    if (match_token(pstr, "let"))
    {
        // Parse the variable name
        char ident[MAX_IDENT_LEN];
        parse_ident(pstr, ident);

        expect_token(pstr, "=");

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

    if (match_token(pstr, "if"))
    {
        // Parse the test expression
        parse_expr(pstr, insns, insn_idx, *plocals);

        expect_token(pstr, "then");

        // If the result is false, jump past the if clause
        instr_t* ifnot_insn = APPEND_INSN_IMM(OP_IFNOT, 0);

        // Parse the body of the if statement
        parse_stmt(pstr, insns, insn_idx, plocals);

        // If the condition is false, we jump after the body of the if
        instr_t* pfalse = &insns[*insn_idx];
        ifnot_insn->imm.int_val = (pfalse - ifnot_insn) - 1;

        return;
    }

    // Sequencing of statements
    if (match_token(pstr, "begin"))
    {
        while (true)
        {
            if (match_token(pstr, "end"))
            {
                break;
            }

            parse_stmt(pstr, insns, insn_idx, plocals);
        }

        return;
    }

    // Print to stdout
    if (match_token(pstr, "print"))
    {
        parse_expr(pstr, insns, insn_idx, *plocals);
        APPEND_INSN(OP_PRINT);
        return;
    }

    // Print to stdout
    if (match_token(pstr, "assert"))
    {
        // Parse the condition
        parse_expr(pstr, insns, insn_idx, *plocals);

        // If the result is true, jump over the error instruction
        APPEND_INSN_IMM(OP_IF, 1);

        // Exit with an error
        APPEND_INSN(OP_ERROR);

        return;
    }

    // Cap the string length for printing
    if (strlen(*pstr) > 10)
    {
        (*pstr)[10] = '\0';
    }

    // Remove newlines from printout
    for (int i = 0;; ++i)
    {
        char ch = (*pstr)[i];
        if (ch == '\r' || ch == '\n')
            (*pstr)[i] = ' ';
        if (ch == '\0')
            break;
    }

    fprintf(stderr, "invalid statement: \"%s [...]\"\n", *pstr);
    exit(-1);
    */
}









// Parse a source file into a sequence of instructions
fn parse_file(file_name: &str) -> Program
{
    let input_str = fs::read_to_string(file_name)
        .expect("couldn't read input source file");

    // Input to be parsed
    let mut input = Input::new(input_str);

    // Program being compiled
    let mut program: Program = Program::new();

    // Until we reach the end of the input
    loop
    {
        // End of input
        if input.peek_char() == '\0' {
            break;
        }

        parse_stmt(&mut input, &mut program);
    }

    return program;
}





/*
// Stack manipulation primitives
#define PUSH(v) ( stack[stack_size] = v, stack_size++ )
#define POP() ( stack_size--, stack[stack_size] )
*/

// Evaluate/run a program
fn eval(prog: Program)
{



    /*
    // Local variables
    value_t vars[MAX_LOCALS];

    // Stack of temporary values
    value_t stack[MAX_STACK];
    size_t stack_size = 0;

    for (const instr_t* pc = insns; pc != NULL; ++pc)
    {
        //printf("stack_size=%zu\n", stack_size);

        switch (pc->op)
        {
            // Exit the program
            case OP_EXIT:
            return;

            case OP_ERROR:
            fprintf(stderr, "Run-time error\n");
            exit(-1);
            return;

            case OP_PUSH:
            PUSH(pc->imm);
            break;

            case OP_SETLOCAL:
            vars[pc->imm.idx] = POP();
            break;

            case OP_GETLOCAL:
            PUSH(vars[pc->imm.idx]);
            break;

            case OP_EQ:
            {
                int64_t arg1 = POP().int_val;
                int64_t arg0 = POP().int_val;
                int64_t bool_val = (arg0 == arg1)? 1:0;
                PUSH((value_t)bool_val);
            }
            break;

            case OP_LT:
            {
                int64_t arg1 = POP().int_val;
                int64_t arg0 = POP().int_val;
                int64_t bool_val = (arg0 < arg1)? 1:0;
                PUSH((value_t)bool_val);
            }
            break;

            case OP_IF:
            {
                int64_t test_val = POP().int_val;

                if (test_val != 0)
                {
                    int64_t jump_offset = pc->imm.int_val;
                    pc += jump_offset;
                }
            }
            break;

            case OP_IFNOT:
            {
                int64_t test_val = POP().int_val;

                if (test_val == 0)
                {
                    int64_t jump_offset = pc->imm.int_val;
                    pc += jump_offset;
                }
            }
            break;

            case OP_ADD:
            {
                int64_t arg1 = POP().int_val;
                int64_t arg0 = POP().int_val;
                PUSH((value_t)(arg0 + arg1));
            }
            break;

            case OP_SUB:
            {
                int64_t arg1 = POP().int_val;
                int64_t arg0 = POP().int_val;
                PUSH((value_t)(arg0 - arg1));
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
                    if (c == EOF || !isdigit(c))
                        break;
                    int64_t digit = c - '0';
                    int_val = 10 * int_val + digit;
                }

                PUSH((value_t)int_val);
            }
            break;

            case OP_PRINT:
            {
                int64_t int_val = POP().int_val;
                printf("print: %lld\n", (long long)int_val);
            }
            break;

            default:
            fprintf(stderr, "unknown bytecode instruction\n");
            exit(-1);
        }
    }
    */



}

fn main()
{
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    if args.len() == 2 {
        let prog = parse_file(&args[1]);
        eval(prog);
    }
}
