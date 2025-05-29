use crate::lexer::Token;

use super::expression::Expression;

#[derive(Debug)]
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
}

#[derive(Debug)]
pub struct IfThenBranch {
    pub condition: Expression,
    pub then_branch: Box<Statement>,
}