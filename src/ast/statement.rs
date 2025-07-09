use crate::{ast::expression::FunctionCallData, lexer::Token};

use super::expression::Expression;

#[derive(Debug, Clone)]
pub enum Statement {
    VariableDeclaration {
        name: Token,
        value: Expression,
    },

    VariableAssignment {
        name: Token,
        value: Expression,
    },

    IfStatement {
        if_then_branch: IfThenBranch,
        else_branch: Option<Box<Statement>>,
    },

    BlockStatement {
        statements: Vec<Statement>,
    },
    WhileStatement { condition: Expression, body: Box<Statement> },

    ForStatement {
        variable: Token,
        start: Expression,
        end: Expression,
        step: Option<Expression>,
        body: Box<Statement>,
    },

    FunctionDefinition {
        name: Token,
        arguments: Vec<Token>,
        body: Box<Statement>,
    },

    FunctionCall(FunctionCallData),
}

#[derive(Debug, Clone)]
pub struct IfThenBranch {
    pub condition: Expression,
    pub then_branch: Box<Statement>,
}