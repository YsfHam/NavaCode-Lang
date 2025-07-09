use core::fmt;

use crate::ast::expression::BinaryOperator;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Int,
    Bool,

    Unresolved,
}

pub fn resolve_binary_operation_type(left: &Type, right: &Type, operator: &BinaryOperator) -> Type {
    match (left, right, operator) {
        (Type::Int, Type::Int, BinaryOperator::Add) => Type::Int,
        (Type::Int, Type::Int, BinaryOperator::Subtract) => Type::Int,
        (Type::Int, Type::Int, BinaryOperator::Multiply) => Type::Int,
        (Type::Int, Type::Int, BinaryOperator::Divide) => Type::Int,
        (Type::Int, Type::Int, BinaryOperator::Modulus) => Type::Int,


        (Type::Bool, Type::Bool, BinaryOperator::And) => Type::Bool,
        (Type::Bool, Type::Bool, BinaryOperator::Or) => Type::Bool,

        (_, _, BinaryOperator::Equal) => Type::Bool,
        (_, _, BinaryOperator::NotEqual) => Type::Int,
        (_, _, BinaryOperator::LessThan) => Type::Int,
        (_, _, BinaryOperator::GreaterThan) => Type::Int,
        (_, _, BinaryOperator::LessThanOrEqual) => Type::Int,
        (_, _, BinaryOperator::GreaterThanOrEqual) => Type::Int,
       _ => Type::Unresolved,
    }
}

pub fn resolve_unary_operation_type(operand: &Type, operator: &crate::ast::expression::UnaryOperator) -> Type {
    match (operand, operator) {
        (Type::Int, crate::ast::expression::UnaryOperator::Negate) => Type::Int,
        (Type::Bool, crate::ast::expression::UnaryOperator::Not) => Type::Bool,
        _ => Type::Unresolved,
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Int => write!(f, "int"),
            Type::Bool => write!(f, "bool"),
            Type::Unresolved => write!(f, "unresolved"),
        }
    }
}