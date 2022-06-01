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

use std::io;
use std::io::Write;
use std::env;
use std::fs;
use std::collections::HashMap;

// Kinds of instructions (opcodes) we support
#[derive(Debug)]
enum Op
{
    Exit,
    Error,
    Push,
    GetLocal,
    SetLocal,
    Equal,
    LessThan,
    IfTrue,
    IfNot,
    Add,
    Sub,
    ReadInt,
    Print
}

#[derive(Clone, Debug)]
enum Value
{
    None,
    Idx(usize),
    IntVal(i64),
    Str(String),
}

impl Value
{
    fn unwrap_idx(&self) -> usize {
        match self {
            Value::Idx(idx) => *idx,
            _ => panic!("value is not an index")
        }
    }

    fn unwrap_int(&self) -> i64 {
        match self {
            Value::IntVal(int_val) => *int_val,
            _ => panic!("value is not an integer")
        }
    }
}

// Format of the instructions we implement
struct Insn
{
    op: Op,
    imm: Value,
}

struct Program
{
    /// List of instructions
    insns: Vec<Insn>,

    /// Mapping of identifiers to local variable indices
    local_idxs: HashMap<String, usize>,
}

impl Program
{
    fn new() -> Self
    {
        Program {
            insns: Vec::default(),
            local_idxs: HashMap::default(),
        }
    }

    /// Append an instruction with no argument
    fn append_insn(&mut self, op: Op)
    {
        self.insns.push(Insn {
            op: op,
            imm: Value::None
        });
    }

    /// Append an instruction with an immediate argument
    fn append_insn_imm(&mut self, op: Op, imm: Value)
    {
        self.insns.push(Insn {
            op: op,
            imm: imm
        });
    }

    /// Try to find the index for local variable declaration
    fn find_local(&self, ident: &str) -> Option<usize>
    {
        match self.local_idxs.get(ident) {
            Some(idx) => Some(*idx),
            None => None,
        }
    }

    /// Declare a new local variable
    fn declare_local(&mut self, ident: &str) -> usize
    {
        assert!(self.find_local(ident).is_none());
        let local_idx = self.local_idxs.len();
        self.local_idxs.insert(ident.to_owned(), local_idx);
        return local_idx;
    }
}

/// Stream of input characters to be parsed
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

    /// Peek at the current input character
    fn peek_char(&self) -> char
    {
        if self.pos >= self.chars.len() {
            return '\0';
        }

        self.chars[self.pos]
    }

    /// Consume one character of the input
    fn eat_char(&mut self) -> char {
        let ch = self.peek_char();

        if ch != '\0' {
            self.pos += 1;
        }

        return ch
    }

    /// Consume whitespace chars in the input
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
            self.eat_char();
        }
    }

    /// Consume single-line comments
    fn eat_comment(&mut self)
    {
        loop
        {
            let ch = self.peek_char();

            match ch
            {
                '\n' => {
                    // Move to the next character
                    self.eat_char();
                    break;
                },

                '\0' => {
                    break;
                },

                _ => {
                    self.eat_char();
                }
            }
        }
    }

    /// Check if the input starts with a given token
    fn match_token(&mut self, token: &str) -> bool
    {
        self.eat_ws();

        let token_chars: Vec<char> = token.chars().collect();
        let num_chars = token_chars.len();

        if self.pos + num_chars > self.chars.len() {
            return false;
        }

        if self.chars[self.pos..(self.pos + num_chars)] == token_chars {
            self.pos += num_chars;
            self.eat_ws();
            return true;
        }

        return false;
    }

    /// Fail to parse if a given token is not there
    fn expect_token(&mut self, token: &str)
    {
        if !self.match_token(token) {
            panic!("expected token \"{}\"", token);
        }
    }

    /// Parse an identifier at the current position
    fn parse_ident(&mut self) -> String
    {
        let mut ident_str = String::from("");

        loop
        {
            let ch = self.peek_char();

            if !ch.is_alphanumeric() && ch != '_' {
                break;
            }

            // Store this character
            ident_str.push(ch);

            // Move to the next character
            self.eat_char();
        }

        if ident_str.len() == 0 {
            panic!("expected identifier\n");
        }

        return ident_str;
    }

    /// Parse a positive decimal integer constant
    fn parse_int(&mut self) -> i64
    {
        let mut num: i64 = 0;

        loop
        {
            let ch = self.peek_char();

            if !ch.is_digit(10) {
                break;
            }

            // Store this digit
            let digit: i64 = (ch as i64) - ('0' as i64);
            num = 10 * num + digit;

            // Move to the next character
            self.eat_char();
        }

        return num;
    }
}

/// Parse an atomic expression
fn parse_atom(input: &mut Input, prog: &mut Program)
{
    let ch = input.peek_char();

    // Read an integer from the console
    if input.match_token("read_int") {
        prog.append_insn(Op::ReadInt);
        return;
    }

    // Integer constant
    if ch.is_digit(10) {
        let num = input.parse_int();
        prog.append_insn_imm(Op::Push, Value::IntVal(num));
        return;
    }

    // Reference to a variable
    if ch.is_alphabetic() || ch == '_' {
        // Parse the variable name
        let ident_str = input.parse_ident();

        // Try to find the declaration
        let local_idx = prog.find_local(&ident_str);

        if local_idx.is_none() {
            panic!("reference to undeclared variable \"{}\"\n", ident_str);
        }

        prog.append_insn_imm(Op::GetLocal, Value::Idx(local_idx.unwrap()));
        return;
    }

    panic!("invalid expression");
}

/// Parse an expression
fn parse_expr(input: &mut Input, prog: &mut Program)
{
    // Parse a first expression
    parse_atom(input, prog);

    input.eat_ws();

    let ch = input.peek_char();

    if input.match_token("+") {
        // Parse the RHS expression
        parse_atom(input, prog);

        // Add the result
        prog.append_insn(Op::Add);
        return;
    }

    if input.match_token("-") {
        // Parse the RHS expression
        parse_atom(input, prog);

        // Subtract the result
        prog.append_insn(Op::Sub);
        return;
    }

    if input.match_token("==") {
        // Parse the RHS expression
        parse_atom(input, prog);

        // Compare the arguments
        prog.append_insn(Op::Equal);
        return;
    }

    if input.match_token("<") {
        // Parse the RHS expression
        parse_atom(input, prog);

        // Compare the arguments
        prog.append_insn(Op::LessThan);
        return;
    }
}

/// Parse a statement
fn parse_stmt(input: &mut Input, prog: &mut Program)
{
    // Consume whitespace
    input.eat_ws();

    // Single-line comments
    if input.match_token("#") {
        input.eat_comment();
        return;
    }

    // Local variable declaration
    if input.match_token("let") {
        // Parse the variable name
        let ident_str = input.parse_ident();

        input.expect_token("=");

        // Parse the expression we are assigning
        parse_expr(input, prog);

        // Make sure this isn't a redeclaration
        let local_idx = prog.find_local(&ident_str);

        if local_idx.is_some() {
            panic!("local variable \"{}\" already declared\n", ident_str);
        }

        // Create a new local variable
        let local_idx = prog.declare_local(&ident_str);

        // Set the local to the expression's value
        prog.append_insn_imm(Op::SetLocal, Value::Idx(local_idx));

        return;
    }

    if input.match_token("if") {
        // Parse the test expression
        parse_expr(input, prog);

        input.expect_token("then");

        // If the result is false, jump past the if clause
        //instr_t* ifnot_insn = APPEND_INSN_IMM(OP_IFNOT, 0);
        let ifnot_insn_idx = prog.insns.len();
        prog.append_insn(Op::IfNot);

        // Parse the body of the if statement
        parse_stmt(input, prog);

        // If the condition is false, we jump after the body of the if
        let jumpto_idx = prog.insns.len();
        let jump_offset = (jumpto_idx as i64) - (ifnot_insn_idx as i64) - 1;
        prog.insns[ifnot_insn_idx].imm = Value::IntVal(jump_offset);

        return;
    }

    // Sequencing of statements
    if input.match_token("begin") {
        loop
        {
            if input.match_token("end") {
                break;
            }

            parse_stmt(input, prog);
        }

        return;
    }

    // Print to stdout
    if input.match_token("print") {
        parse_expr(input, prog);
        prog.append_insn(Op::Print);
        return;
    }

    // Assert that an expression evaluates to true
    if input.match_token("assert") {
        // Parse the condition
        parse_expr(input, prog);

        // If the result is true, jump over the error instruction
        prog.append_insn_imm(Op::IfTrue, Value::IntVal(1));

        // Exit with an error
        prog.append_insn(Op::Error);

        return;
    }

    // TODO: report more info about current position and next token
    panic!("invalid statement");
}

/// Parse a source file into a sequence of instructions
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

/// Virtual machine / interpreter
struct VM
{
    /// Local variables
    locals: Vec<Value>,

    /// Stack of temporary values
    stack: Vec<Value>,

    /// Program counter
    pc: usize,
}

impl VM
{
    fn new() -> Self {
        VM {
            locals: Vec::default(),
            stack: Vec::default(),
            pc: 0,
        }
    }

    fn push(&mut self, val: Value) {
        self.stack.push(val);
    }

    fn pop(&mut self) -> Value {
        self.stack.pop().unwrap()
    }

    // Evaluate/run a program
    fn eval(&mut self, prog: Program)
    {
        let num_locals = prog.local_idxs.len();

        self.locals.resize(num_locals, Value::None);

        self.pc = 0;

        while self.pc < prog.insns.len() {
            // Read the current instruction
            let insn = &prog.insns[self.pc];

            match insn.op
            {
                // Exit the program
                Op::Exit => {
                    return;
                }

                // Abort execution
                Op::Error => {
                    panic!("Run-time error\n");
                }

                Op::Push => {
                    self.push(insn.imm.clone());
                }

                Op::SetLocal => {
                    self.locals[insn.imm.unwrap_idx()] = self.pop();
                }

                Op::GetLocal => {
                    self.push(self.locals[insn.imm.unwrap_idx()].clone());
                }

                Op::Equal => {
                    let arg1 = self.pop().unwrap_int();
                    let arg0 = self.pop().unwrap_int();
                    let bool_val = if arg0 == arg1 { 1 } else { 0 };
                    self.push(Value::IntVal(bool_val));
                }

                Op::LessThan => {
                    let arg1 = self.pop().unwrap_int();
                    let arg0 = self.pop().unwrap_int();
                    let bool_val = if arg0 < arg1 { 1 } else { 0 };
                    self.push(Value::IntVal(bool_val));
                }

                Op::IfTrue => {
                    let test_val = self.pop().unwrap_int();

                    if test_val != 0 {
                        let jump_offset = insn.imm.unwrap_int();
                        self.pc = ((self.pc as i64) + jump_offset) as usize;
                    }
                }

                Op::IfNot => {
                    let test_val = self.pop().unwrap_int();

                    if test_val == 0 {
                        let jump_offset = insn.imm.unwrap_int();
                        self.pc = ((self.pc as i64) + jump_offset) as usize;
                    }
                }

                Op::Add => {
                    let arg1 = self.pop().unwrap_int();
                    let arg0 = self.pop().unwrap_int();
                    self.push(Value::IntVal(arg0 + arg1));
                }

                Op::Sub => {
                    let arg1 = self.pop().unwrap_int();
                    let arg0 = self.pop().unwrap_int();
                    self.push(Value::IntVal(arg0 - arg1));
                }

                // Read an integer value from stdin
                Op::ReadInt => {
                    println!("Input an integer value:");
                    print!("> ");
                    io::stdout().flush().unwrap();

                    let mut input = String::new();
                    io::stdin().read_line(&mut input).unwrap();
                    let n: i64 = input.trim().parse().unwrap();
                    self.push(Value::IntVal(n));
                }

                Op::Print => {
                    let int_val = self.pop().unwrap_int();
                    println!("print: {}\n", int_val);
                }

                #[allow(unreachable_patterns)]
                _ => {
                    panic!("unknown bytecode instruction in eval {:?}", insn.op);
                }
            }

            // Move to the next instruction
            self.pc += 1;
        }
    }
}

fn main()
{
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    if args.len() == 2 {
        // Parse the source file
        let prog = parse_file(&args[1]);

        // Evaluate the program
        let mut vm = VM::new();
        vm.eval(prog);
    }
}
