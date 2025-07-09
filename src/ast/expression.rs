use core::fmt;

use crate::lexer::Token;

#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    Number(i64),
    Boolean(bool),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Literal {
        value: Literal,
        span: crate::lexer::TextSpan,
    },

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

    FunctionCall(FunctionCallData),
}

impl Expression {
    pub fn span(&self) -> crate::lexer::TextSpan {
        match self {
            Expression::Literal { span, .. } => span.clone(),
            Expression::Variable(token) => token.span(),
            Expression::BinaryOperation { left, right, .. } => left.span().union(&right.span()),
            Expression::UnaryOperation { operand, .. } => operand.span(),
            Expression::Grouped(expression) => expression.span(),
            Expression::FunctionCall(data) => data.function_name.span(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionCallData {
    pub function_name: Token,
    pub arguments: Vec<Expression>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BinaryOperator {
    /// Arithmetic Operators
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulus,

    /// Comparison Operators
    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
    LessThanOrEqual,
    GreaterThanOrEqual,
    /// Logical Operators
    And,
    Or,
}

impl fmt::Display for BinaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let symbol = match self {
            BinaryOperator::Add => "+",
            BinaryOperator::Subtract => "-",
            BinaryOperator::Multiply => "*",
            BinaryOperator::Divide => "/",
            BinaryOperator::Modulus => "%",
            BinaryOperator::Equal => "==",
            BinaryOperator::NotEqual => "!=",
            BinaryOperator::LessThan => "<",
            BinaryOperator::GreaterThan => ">",
            BinaryOperator::LessThanOrEqual => "<=",
            BinaryOperator::GreaterThanOrEqual => ">=",
            BinaryOperator::And => "and",
            BinaryOperator::Or => "or",
        };
        write!(f, "{}", symbol)
    }
}

impl BinaryOperator {
    pub fn precedence(&self) -> u8 {
        match self {
            // Logical operators (lowest precedence)
            BinaryOperator::Or => 0,
            BinaryOperator::And => 1,

            // Comparison operators
            BinaryOperator::Equal
            | BinaryOperator::NotEqual
            | BinaryOperator::LessThan
            | BinaryOperator::GreaterThan
            | BinaryOperator::LessThanOrEqual
            | BinaryOperator::GreaterThanOrEqual => 2,

            // Arithmetic operators
            BinaryOperator::Add | BinaryOperator::Subtract => 3,

              BinaryOperator::Multiply 
            | BinaryOperator::Divide 
            | BinaryOperator::Modulus
            => 4,
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
            crate::lexer::TokenKind::Percent => Ok(BinaryOperator::Modulus),
            crate::lexer::TokenKind::EqualEqual => Ok(BinaryOperator::Equal),
            crate::lexer::TokenKind::NotEqual => Ok(BinaryOperator::NotEqual),
            crate::lexer::TokenKind::LessThan => Ok(BinaryOperator::LessThan),
            crate::lexer::TokenKind::GreaterThan => Ok(BinaryOperator::GreaterThan),
            crate::lexer::TokenKind::LessThanOrEqual => Ok(BinaryOperator::LessThanOrEqual),
            crate::lexer::TokenKind::GreaterThanOrEqual => Ok(BinaryOperator::GreaterThanOrEqual),
            crate::lexer::TokenKind::AndKeyword => Ok(BinaryOperator::And),
            crate::lexer::TokenKind::OrKeyword => Ok(BinaryOperator::Or),
            _ => Err(()),
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UnaryOperator {
    Negate,
    Not,
}

impl TryFrom<crate::lexer::TokenKind> for UnaryOperator {
    type Error = ();

    fn try_from(kind: crate::lexer::TokenKind) -> Result<Self, Self::Error> {
        match kind {
            crate::lexer::TokenKind::Minus => Ok(UnaryOperator::Negate),
            crate::lexer::TokenKind::NotKeyword | crate::lexer::TokenKind::Bang => Ok(UnaryOperator::Not),
            _ => Err(()),
        }
    }
}

impl fmt::Display for UnaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let symbol = match self {
            UnaryOperator::Negate => "-",
            UnaryOperator::Not => "not/!",
        };
        write!(f, "{}", symbol)
    }
}