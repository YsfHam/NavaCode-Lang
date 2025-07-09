pub mod lexer;
pub mod ast;
pub mod parser;
pub mod utils;
pub mod diagnostic;
pub mod compiler;
pub mod interpreter;
pub mod symbols_table;
pub mod resolver;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockType {
    IfBlock,
    WhileBlock,
    ForBlock,
    ElseBlock,
    FunctionBlock,
}