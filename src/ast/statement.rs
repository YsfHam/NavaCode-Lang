use crate::lexer::Token;

use super::expression::Expression;

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
}

pub struct IfThenBranch {
    pub condition: Expression,
    pub then_branch: Box<Statement>,
}