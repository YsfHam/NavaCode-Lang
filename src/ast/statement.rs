use crate::lexer::Token;

use super::expression::Expression;

pub enum Statement {
    VariableDeclaration {
        name: Token,
        value: Expression,
    },
}