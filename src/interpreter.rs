use std::collections::HashMap;

use crate::ast::{expression::{BinaryOperator, UnaryOperator}, AstExplorer};

#[derive(Clone, Debug)]
pub enum RuntimeValue {
    Number(i64),
}

pub enum RuntimeError {
    VariableNotFound(String),
    InvalidBinaryOperation(BinaryOperator),
    InvalidUnaryOperation(UnaryOperator),
    DivisionByZero,
}


pub struct Interpreter {
    accumulator: Option<RuntimeValue>,
    variables: HashMap<String, RuntimeValue>,

    errors: Vec<RuntimeError>,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            accumulator: None,
            variables: HashMap::new(),
            errors: Vec::new(),
        }
    }

    pub fn display_state(&self) {
        println!("Current Variables:");
        for (name, value) in &self.variables {
            match value {
                RuntimeValue::Number(n) => println!("{}: {}", name, n),
            }
        }

        if !self.errors.is_empty() {
            println!("Errors:");
            for error in &self.errors {
                match error {
                    RuntimeError::VariableNotFound(name) => println!("Variable not found: {}", name),
                    RuntimeError::InvalidBinaryOperation(op) => println!("Invalid binary operation: {:?}", op),
                    RuntimeError::InvalidUnaryOperation(op) => println!("Invalid unary operation: {:?}", op),
                    RuntimeError::DivisionByZero => println!("Error: Division by zero"),
                }
            }
        }
    }

    fn register_variable(&mut self, name: String, value: RuntimeValue) {
        self.variables.insert(name, value);
    }

    fn get_variable(&self, name: &str) -> Option<&RuntimeValue> {
        self.variables.get(name)
    }

    fn report_error(&mut self, error: RuntimeError) {
        self.errors.push(error);
    }
}

impl AstExplorer for Interpreter {
    fn visit_variable_declaration(&mut self, name: &crate::lexer::Token, value: &crate::ast::expression::Expression) {
        self.visit_expression(value);
        let expr_value = self.accumulator.take().unwrap();
        self.register_variable(name.value.clone(), expr_value);
    }

    fn visit_variable_assignement(&mut self, name: &crate::lexer::Token, value: &crate::ast::expression::Expression) {
        if self.get_variable(&name.value).is_none() {
            self.report_error(RuntimeError::VariableNotFound(name.value.clone()));
        }
        else {
            self.visit_expression(value);
            let expr_value = self.accumulator.take().unwrap();
            self.register_variable(name.value.clone(), expr_value);
        }
    }


    fn visit_number_expression(&mut self, value: i64) {
        self.accumulator = Some(RuntimeValue::Number(value));
    }

    fn visit_variable_expression(&mut self, name: &crate::lexer::Token) {
        if let Some(value) = self.get_variable(&name.value) {
            self.accumulator = Some(value.clone());
        } else {
            self.report_error(RuntimeError::VariableNotFound(name.value.clone()));
        }
    }

    fn visit_binary_operation(&mut self, left: &crate::ast::expression::Expression, operator: &crate::ast::expression::BinaryOperator, right: &crate::ast::expression::Expression) {

        self.visit_expression(left);
        let left_value = self.accumulator.take().unwrap();

        self.visit_expression(right);
        let right_value = self.accumulator.take().unwrap();

        let result = match (left_value, right_value, operator) {
            (RuntimeValue::Number(l), RuntimeValue::Number(r), crate::ast::expression::BinaryOperator::Add) => {
                RuntimeValue::Number(l + r)
            }
            (RuntimeValue::Number(l), RuntimeValue::Number(r), crate::ast::expression::BinaryOperator::Subtract) => {
                RuntimeValue::Number(l - r)
            }
            (RuntimeValue::Number(l), RuntimeValue::Number(r), crate::ast::expression::BinaryOperator::Multiply) => {
                RuntimeValue::Number(l * r)
            }
            (RuntimeValue::Number(l), RuntimeValue::Number(r), crate::ast::expression::BinaryOperator::Divide) => {
                if r == 0 {
                    self.report_error(RuntimeError::DivisionByZero);
                    RuntimeValue::Number(0)
                }
                else {
                    RuntimeValue::Number(l / r)
                }
            }
            _ => {
                self.report_error(RuntimeError::InvalidBinaryOperation(*operator));
                return;
            }
        };

        self.accumulator = Some(result);
    }

    fn visit_unary_operation(&mut self, operator: &crate::ast::expression::UnaryOperator, operand: &crate::ast::expression::Expression) {
        self.visit_expression(operand);
        let operand_value = self.accumulator.take().unwrap();

        let result = match (operator, operand_value) {
            (crate::ast::expression::UnaryOperator::Negate, RuntimeValue::Number(v)) => {
                RuntimeValue::Number(-v)
            }
            _ => {
                self.report_error(RuntimeError::InvalidUnaryOperation(*operator));
                return;
            }
        };

        self.accumulator = Some(result);
    }
}