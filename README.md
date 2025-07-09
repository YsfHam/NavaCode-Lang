# NavaCode Lang

## Project Overview
NavaCode Lang is a simple, educational programming language and compiler project. The main goal is to learn and experiment with compiler, interpreter, and virtual machine concepts, while improving software architecture and design skills. This project is intended for fun and personal growth.

## Motivation
- Learn how compilers and interpreters work under the hood
- Explore language design and implementation
- Practice making software architecture decisions
- Build something fun and useful for learning

## Language Design
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

NavaCode Lang is designed to be simple and expressive. Here are examples of all currently supported syntax features:

### Variable Declaration
```nava
let x be 10
let y be 20
```

### Variable Assignment
```nava
let x be 10
let y be 20
set x to x + y
```

### Integer Arithmetic
```nava
let sum be x + y
let diff be x - y
let prod be x * y
let quot be x / y
```

### Logical Operators
```nava
let a be 1 and 0
let b be 1 or 0
```

### Comparison Operators
```nava
let eq be x == y
let neq be x != y
let lt be x < y
let gt be x > y
let le be x <= y
let ge be x >= y
```

### Unary Operators
```nava
let neg be -x
let notx be not x
let nested be - -x
let group be - (x + 1)
```

### Grouped Expressions and Precedence
```nava
let a be (1 + 2) * 3
let b be 4 / (2 - 1)
let c be not (x and y)
```

### Function Definition and Calls
```nava
define function add with a, b as
    return (a + b)
end

define function factorial with n as
    if n <= 1 then
        return (1)
    else
        return (n * factorial(n - 1))
    end
end

let result be add(5, 7)
let fact5 be factorial(5)
```

### If/Else Statements
```nava
if x > 0 then
    set y to 1
else if x < 0 then
    set y to -1
else
    set y to 0
end
```

### While Loops
```nava
let n be 5
let fact be 1
while n > 1 then
    set fact to fact * n
    set n to n - 1
end
```

### For Loops
```nava
let sum be 0
for i from 1 to 5 then
    set sum to sum + i
end

for i from 10 to 1 step -2 then
    set sum to sum + i
end
```

---

## Technical Overview

### Parser Components
- **Lexer:** Converts source code into a stream of tokens (numbers, identifiers, operators, keywords, etc.).
- **Parser:** Uses recursive descent and operator precedence parsing to build an Abstract Syntax Tree (AST) from the token stream.
- **AST:** Represents the structure of the program (variable declarations, expressions, function definitions/calls, etc.).
- **Error Handling:** Robust error reporting and recovery for invalid syntax and semantic errors (e.g., undefined variables, return outside function, function argument mismatch).
- **Semantic Analyzer (Resolver):** Checks for variable/function definitions, scope, and correct use of return statements.
- **Interpreter:** Executes the AST, supports variables, arithmetic, logic, control flow, and function calls/returns.

### Grammar (EBNF)
```
program         ::= { statement }

statement       ::= variable_declaration
                 | variable_assignment
                 | if_statement
                 | while_statement
                 | for_statement
                 | function_definition
                 | return_statement
                 | expression_statement

variable_declaration ::= "let" identifier "be" expression
variable_assignment  ::= "set" identifier "to" expression
if_statement         ::= "if" expression "then" { statement } [ "else" { statement } ] "end"
while_statement      ::= "while" expression "then" { statement } "end"
for_statement        ::= "for" identifier "from" expression "to" expression [ "step" expression ] "then" { statement } "end"
function_definition  ::= "define function" identifier "with" [ identifier { "," identifier } ] "as" { statement } "end"
function_call        ::= identifier '(' [ expression { ',' expression } ] ')'
return_statement     ::= "return" '(' expression ')' | "return" '()'
expression_statement ::= expression

expression      ::= unary_expression [ binary_operator expression ]
unary_expression ::= unary_operator unary_expression
                  | primary_expression
primary_expression ::= grouped_expression
                    | literal_expression
                    | function_call

grouped_expression ::= '(' expression ')'
literal_expression ::= number | identifier
unary_operator   ::= '-' | 'not'
binary_operator  ::= '+' | '-' | '*' | '/' | 'and' | 'or' | '==' | '!=' | '<' | '>' | '<=' | '>='
identifier       ::= [a-zA-Z_][a-zA-Z0-9_]*
number           ::= [0-9]+(\.[0-9]+)?
```

- **Operator precedence** is handled so that arithmetic, logical, and comparison operators work as expected.
- **Unary operators** can be chained and can operate on grouped expressions.
- **Grouped expressions** (parentheses) can override precedence.
- **Functions** support definition, calls, arguments, and return values (with `return (expr)` or `return`).

---

## Roadmap

### MVP Features (Current Progress)
- [x] Variable declarations
- [x] Variable assignment

1. **Arithmetic and Logical Expressions**
  - [x] Implement integer and float arithmetic (`+`, `-`, `*`, `/`)
  - [x] Add logical operators (`and`, `or`, `not`)
  - [x] Support comparison operators (`==`, `!=`, `<`, `>`, `<=`, `>=`)
  - [x] Add boolean values `true` and `false`

2. **Control Flow**
  - [x] Add `if` and `else` statements
  - [x] Implement `while` loops
  - [x] Add `for` loops

3. **Functions**
  - [x] Support function definitions
  - [x] Enable function calls with arguments
  - [x] Implement return values

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

