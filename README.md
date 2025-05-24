# NavaCode Lang

## Project Overview
NavaCode Lang is a simple, educational programming language and compiler project. The main goal is to learn and experiment with compiler, interpreter, and virtual machine concepts, while improving software architecture and design skills. This project is intended for fun and personal growth.

## Motivation
- Learn how compilers and interpreters work under the hood
- Explore language design and implementation
- Practice making software architecture decisions
- Build something fun and useful for learning

## Language Design (Draft)
- **Syntax:** Simple and readable, inspired by languages like Python, JavaScript, C, and some functional languages.
- **Features:**
  - Basic data types (integers, floats, strings, booleans)
  - Variables and assignment
  - Arithmetic and logical operations
  - Control flow (if, else, while, for)
  - Functions and basic scoping
  - (Planned) User-defined types and modules
- **Philosophy:** Keep it simple, easy to read, and easy to implement

## Planned Architecture
- **Lexer:** Tokenizes source code
- **Parser:** Builds an abstract syntax tree (AST)
- **Semantic Analyzer:** Checks for errors and builds symbol tables
- **Interpreter/VM:** Executes the AST or bytecode
- **(Planned) Compiler:** Compiles to C. Adding bytecode and IR later

## Syntax Overview
This section describes the syntax of NavaCode Lang with examples:

### Variable Declaration
```nava
let x be 10
let y be 20
```

### Integer Arithmetic
```nava
let sum be x + y
let diff be x - y
let prod be x * y
let quot be x / y
```

## Roadmap

### MVP Features (Current Progress)
- [x] Variable declarations

1. **Arithmetic and Logical Expressions**
  - [ ] Implement integer and float arithmetic (`+`, `-`, `*`, `/`)
  - [ ] Add logical operators (`and`, `or`, `not`)
  - [ ] Support comparison operators (`==`, `!=`, `<`, `>`, `<=`, `>=`)

2. **Control Flow**
  - [ ] Add `if` and `else` statements
  - [ ] Implement `while` loops
  - [ ] Add `for` loops

3. **Functions**
  - [ ] Support function definitions
  - [ ] Enable function calls with arguments
  - [ ] Implement return values

4. **Standard Input/Output**
  - [ ] Add basic `print` and `input` functions

### Short-Term Goals
- [ ] Add string manipulation functions
- [ ] Implement arrays and basic collections
- [ ] Support comments in code
- [ ] Improve error reporting

### Long-Term Ideas
- [ ] User-defined types (structs/records)
- [ ] Module and import system
- [ ] Pattern matching
- [ ] Lambda expressions and closures
- [ ] Compile to C or custom bytecode
- [ ] Expand the standard library


## License
MIT License

---
This project is a work in progress and open to ideas, suggestions, and contributions!

