use crate::lexer::Token;

pub enum Expression {
    Number(i64),

    Variable(Token),

    BinaryOperation {
        left: Box<Expression>,
        operator: BinaryOperator,
        right: Box<Expression>,
    },

    UnaryOperation {
        operator: UnaryOperator,
        operand: Box<Expression>,
    },

    Grouped(Box<Expression>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl BinaryOperator {
    pub fn precedence(&self) -> u8 {
        match self {
            BinaryOperator::Add | BinaryOperator::Subtract => 1,
            BinaryOperator::Multiply | BinaryOperator::Divide => 2,
        }
    }
}


impl TryFrom<crate::lexer::TokenKind> for BinaryOperator {
    type Error = ();

    fn try_from(kind: crate::lexer::TokenKind) -> Result<Self, Self::Error> {
        match kind {
            crate::lexer::TokenKind::Plus => Ok(BinaryOperator::Add),
            crate::lexer::TokenKind::Minus => Ok(BinaryOperator::Subtract),
            crate::lexer::TokenKind::Star => Ok(BinaryOperator::Multiply),
            crate::lexer::TokenKind::Slash => Ok(BinaryOperator::Divide),
            _ => Err(()),
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOperator {
    Negate,
}

impl TryFrom<crate::lexer::TokenKind> for UnaryOperator {
    type Error = ();

    fn try_from(kind: crate::lexer::TokenKind) -> Result<Self, Self::Error> {
        match kind {
            crate::lexer::TokenKind::Minus => Ok(UnaryOperator::Negate),
            _ => Err(()),
        }
    }
}