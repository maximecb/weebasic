# weebasic

Toy BASIC-like language and interpreter implemented in C for study or interview purposes. The interpreter is intentionally left incomplete so that new features can be implemented as coding exercises. This software is distributed under the Unlicense, meaning it is effectively public domain, though crediting the author is appreciated.

Design:
- Top-down recursive descent parser
- The tokenizer is built directly into the parser
- Stack-based bytecode interpreter
- No operator precedence for simplicity

## Interview Format

The first thing you should do is to clone this repository and create your own branch, eg `git checkout -b yourusername`.
I would then suggest browsing the code and the example source files to get familiar with them a little bit.
I'm going to ask you to implement new features in `weebasic`. For example, I might ask you to implement new syntactic
constructs or to add support for new data types in the language.

The primary purpose is to evaluate your knowledge
of C and systems programming, and to see if you understand how a simple parser and bytecode interpreter works.
I'm also assessing your communication and problem-solving skills. You should try to roughly explain what you are doing
as you do it, and you are allowed to ask as many clarifying questions as necessary during the interview.

I'm a friendly interviewer and my goal is to help you succeed. The interview isn't designed to cause you stress.
Your solution doesn't need to be
perfect and you don't need to worry about performance. Typically, the simplest viable solution is the expected answer.

## Installation

Requirements:
- C compiler such as GCC or Clang

Clone this repository:
```
git clone https://github.com/maximecb/weebasic.git
```

To build weebasic:

```
cd weebasic
./make.sh
```

To test that weebasic is working correctly:

```
./weebasic tests.bas
```

## Usage

For syntax examples, see `example.bas` and `tests.bas`.


To execute programs, run:

```
./weebasic example.bas
```

