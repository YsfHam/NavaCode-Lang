use std::collections::HashMap;

use crate::ast::{expression::{BinaryOperator, UnaryOperator}, Ast, AstExplorer};

mod builtin;


static BINARY_OPERATORS: &[(BinaryOperator, RuntimeBinaryOperator)] = &[
    (BinaryOperator::Add, builtin::add),
    (BinaryOperator::Subtract, builtin::sub),
    (BinaryOperator::Multiply, builtin::mul),
    (BinaryOperator::Divide, builtin::div),
    (BinaryOperator::Equal, builtin::eq),
    (BinaryOperator::NotEqual, builtin::not_eq),
    (BinaryOperator::GreaterThan, builtin::gt),
    (BinaryOperator::GreaterThanOrEqual, builtin::gt_eq),
    (BinaryOperator::LessThan, builtin::lt),
    (BinaryOperator::LessThanOrEqual, builtin::lt_eq),

    (BinaryOperator::And, builtin::and),
    (BinaryOperator::Or, builtin::or),
];

static UNARY_OPERATORS: &[(UnaryOperator, RuntimeUnaryOperator)] = &[
    (UnaryOperator::Negate, builtin::negate),
    (UnaryOperator::Not, builtin::not)
];


type RuntimeBinaryOperator = fn (RuntimeValue, RuntimeValue) -> Result<RuntimeValue, RuntimeError>;
type RuntimeUnaryOperator = fn (RuntimeValue) -> Result<RuntimeValue, RuntimeError>;

struct RuntimeFunctionsDispatcher {
    binary_operators: HashMap<BinaryOperator, RuntimeBinaryOperator>,
    unary_operators: HashMap<UnaryOperator, RuntimeUnaryOperator>,
}

impl RuntimeFunctionsDispatcher {
    fn new() -> Self {
        Self {
            binary_operators: BINARY_OPERATORS.iter().map(|op| *op).collect(),
            unary_operators: UNARY_OPERATORS.iter().map(|op| *op).collect(),
        }
    }

    fn get_binary_operator_function(&self, operator: &BinaryOperator) -> Option<&RuntimeBinaryOperator> {
        self.binary_operators.get(operator)
    }
    
    fn get_unary_operator_function(&self, operator: &UnaryOperator) -> Option<&RuntimeUnaryOperator> {
        self.unary_operators.get(operator)
    }
}


struct RuntimeScope {
    variables: HashMap<String, RuntimeValue>,
}

impl RuntimeScope {
    fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }


    fn register_variable(&mut self, name: String, value: RuntimeValue) {
        self.variables.insert(name, value);
    }

    fn get_variable(&self, name: &str) -> Option<&RuntimeValue> {
        self.variables.get(name)
    }

}

#[derive(Clone, Debug)]
enum RuntimeValue {
    Number(i64),
    Bool(bool),
}

enum RuntimeError {
    VariableNotFound(String),
    InvalidOperation,
    DivisionByZero,
    InvalidIfCondition,
}


pub struct Interpreter {
    accumulator: Option<RuntimeValue>,
    scopes: Vec<RuntimeScope>,
    dispatcher: RuntimeFunctionsDispatcher,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            accumulator: None,
            scopes: vec![RuntimeScope::new()],
            dispatcher: RuntimeFunctionsDispatcher::new(),
        }
    }

    pub fn interpret(ast: &Ast) {
        let mut interpreter = Self::new();

        let rust_backtrace = env!("RUST_BACKTRACE");
        println!("rust backtrace level {}", rust_backtrace);

        unsafe {std::env::set_var("RUST_BACKTRACE", "0")};
        interpreter.explore_ast(ast);
        unsafe {std::env::set_var("RUST_BACKTRACE", rust_backtrace)};

        interpreter.display_state();

    }

    pub fn display_state(&self) {
        println!("Current Variables:");
        for (name, value) in &self.scopes[0].variables {
            match value {
                RuntimeValue::Number(n) => println!("{}: {}", name, n),
                RuntimeValue::Bool(b) => println!("{}: {}", name, b),
            }
        }
    }

    fn get_accumulator_value(&mut self) -> RuntimeValue {
        self.accumulator.take().expect("Expression unevaluated")
    }

    fn register_variable(&mut self, name: String, value: RuntimeValue) {
        let scope = self.scopes
            .iter_mut()
            .rev()
            .find(|scope| scope.variables.contains_key(&name));

        if let Some(scope) = scope {
            scope.register_variable(name, value);
        }
        else {
            self.scopes.last_mut().unwrap().register_variable(name, value);
        }

    }

    fn get_variable_mut(&self, name: &str) -> Option<&RuntimeValue> {
        self.scopes
            .iter()
            .rev()
            .find_map(|scope| scope.get_variable(name))
    }

    fn report_error(&mut self, error: RuntimeError) {
        match error {
            RuntimeError::VariableNotFound(name) => panic!("Variable not found: {}", name),
            RuntimeError::DivisionByZero => panic!("Error: Division by zero"),
            RuntimeError::InvalidIfCondition => panic!("Error: condition in if block must be a boolean"),
            RuntimeError::InvalidOperation => panic!("Error: invalid operation"),
        }
    }

    fn push_scope(&mut self) {
        self.scopes.push(RuntimeScope::new());
    }
    
    fn pop_scope(&mut self) {
        self.scopes.pop();
    }
}

impl AstExplorer for Interpreter {
    fn visit_variable_declaration(&mut self, name: &crate::lexer::Token, value: &crate::ast::expression::Expression) {
        self.visit_expression(value);
        let expr_value = self.get_accumulator_value();
        self.register_variable(name.value.clone(), expr_value);
    }

    fn visit_variable_assignement(&mut self, name: &crate::lexer::Token, value: &crate::ast::expression::Expression) {
        if self.get_variable_mut(&name.value).is_none() {
            self.report_error(RuntimeError::VariableNotFound(name.value.clone()));
        }
        else {
            self.visit_expression(value);
            let expr_value = self.get_accumulator_value();
            self.register_variable(name.value.clone(), expr_value);
        }
    }


    fn visit_number_expression(&mut self, value: i64) {
        self.accumulator = Some(RuntimeValue::Number(value));
    }

    fn visit_variable_expression(&mut self, name: &crate::lexer::Token) {
        if let Some(value) = self.get_variable_mut(&name.value) {
            self.accumulator = Some(value.clone());
        } else {
            self.report_error(RuntimeError::VariableNotFound(name.value.clone()));
        }
    }

    fn visit_binary_operation(&mut self, left: &crate::ast::expression::Expression, operator: &crate::ast::expression::BinaryOperator, right: &crate::ast::expression::Expression) {

        self.visit_expression(left);
        let left_value = self.get_accumulator_value();

        self.visit_expression(right);
        let right_value = self.get_accumulator_value();

        let op = self.dispatcher
            .get_binary_operator_function(operator)
            .unwrap();

        match op(left_value, right_value) {
            Ok(result) => self.accumulator = Some(result),
            Err(error) => self.report_error(error),
        }
    }

    fn visit_unary_operation(&mut self, operator: &crate::ast::expression::UnaryOperator, operand: &crate::ast::expression::Expression) {
        self.visit_expression(operand);
        let operand_value = self.get_accumulator_value();

        let op = self.dispatcher
            .get_unary_operator_function(operator)
            .unwrap();

        match op(operand_value) {
            Ok(result) => self.accumulator = Some(result),
            Err(error) => self.report_error(error),
        }
    }
    
    fn visit_if_statement(&mut self, condition: &crate::ast::expression::Expression, then_branch: &crate::ast::statement::Statement, else_branch: Option<&crate::ast::statement::Statement>) {
        self.visit_expression(condition);

        let condition_value = self.get_accumulator_value();

        match condition_value {
            RuntimeValue::Bool(v) => if v {
                self.visit_statement(then_branch);
            }
            else if let Some(else_branch) = else_branch{
                self.visit_statement(else_branch);
            },

            _ => {
                self.report_error(RuntimeError::InvalidIfCondition);
            }
        }

    }
    
    fn block_statement_on_enter(&mut self) {
        self.push_scope();
    }
    
    fn block_statement_on_exit(&mut self) {
        self.pop_scope();
    }
}