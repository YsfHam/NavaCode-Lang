use std::collections::HashMap;

use crate::ast::{expression::{BinaryOperator, UnaryOperator}, statement::Statement, Ast, AstExplorer};

mod builtin;


static BINARY_OPERATORS: &[(BinaryOperator, RuntimeBinaryOperator)] = &[
    (BinaryOperator::Add, builtin::add),
    (BinaryOperator::Subtract, builtin::sub),
    (BinaryOperator::Multiply, builtin::mul),
    (BinaryOperator::Divide, builtin::div),
    (BinaryOperator::Modulus, builtin::modulus),
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


    fn set_variable(&mut self, name: String, value: RuntimeValue) {
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
    InvalidCondition,
}

#[derive(Clone)]
struct FunctionInfo {
    parameters: Vec<String>,
    body: Statement,
}


pub struct Interpreter {
    accumulator: Option<RuntimeValue>,
    scopes: Vec<RuntimeScope>,
    dispatcher: RuntimeFunctionsDispatcher,
    functions: HashMap<String, FunctionInfo>,
}

impl Interpreter {
    fn new() -> Self {
        Interpreter {
            accumulator: None,
            scopes: vec![RuntimeScope::new()],
            dispatcher: RuntimeFunctionsDispatcher::new(),
            functions: HashMap::new(),
        }
    }

    pub fn interpret(ast: &Ast) {
        let mut interpreter = Self::new();

        interpreter.collect_functions(ast);

        let rust_backtrace = env!("RUST_BACKTRACE");

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

    fn collect_functions(&mut self, ast: &Ast) {
        for statement in ast.statements() {
            if let Statement::FunctionDefinition { name, arguments, body } = statement {
                let function_info = FunctionInfo {
                    parameters: arguments.iter().map(|arg| arg.value.clone()).collect(),
                    body: *body.clone(),
                };
                self.functions.insert(name.value.clone(), function_info);
            }
        }

    }

    fn call_function(&mut self, function_info: FunctionInfo, arguments: &[crate::ast::expression::Expression]) {
        
        self.push_scope();

        for (param, arg) in function_info.parameters.iter().zip(arguments) {
            self.visit_expression(arg);
            let value = self.get_accumulator_value();
            self.register_variable(param.clone(), value);
        }

        self.visit_statement(&function_info.body);

        self.pop_scope();
    }

    fn get_accumulator_value(&mut self) -> RuntimeValue {
        self.accumulator.take().expect("Expression unevaluated")
    }

    fn register_variable(&mut self, name: String, value: RuntimeValue) {
        
        self.scopes.last_mut().unwrap().set_variable(name, value);

    }

    fn set_variable_value(&mut self, name: String, value: RuntimeValue) {
        if let Some(scope) = 
            self.scopes
                .iter_mut()
                .rev()
                .find(|s| s.get_variable(&name).is_some()) 
        {
            scope.set_variable(name, value);
        } 
        else {
            self.report_error(RuntimeError::VariableNotFound(name));
        }
    }

    fn get_variable(&self, name: &str) -> &RuntimeValue {
        let value = self.scopes
            .iter()
            .rev()
            .find_map(|scope| scope.get_variable(name));

        match value {
            Some(v) => v,
            None => {
                self.report_error(RuntimeError::VariableNotFound(name.to_string()));
            }
        }
    }

    fn report_error(&self, error: RuntimeError) -> ! {
        match error {
            RuntimeError::VariableNotFound(name) => panic!("Variable not found: {}", name),
            RuntimeError::DivisionByZero => panic!("Error: Division by zero"),
            RuntimeError::InvalidCondition => panic!("Error: condition in if block must be a boolean"),
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

        self.visit_expression(value);
        let expr_value = self.get_accumulator_value();
        self.set_variable_value(name.value.clone(), expr_value);
    }


    fn visit_number_expression(&mut self, value: i64) {
        self.accumulator = Some(RuntimeValue::Number(value));
    }

    fn visit_variable_expression(&mut self, name: &crate::lexer::Token) {
        self.accumulator = Some(self.get_variable(&name.value).clone());
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
                self.report_error(RuntimeError::InvalidCondition);
            }
        }

    }
    
    fn block_statement_on_enter(&mut self) {
        self.push_scope();
    }
    
    fn block_statement_on_exit(&mut self) {
        self.pop_scope();
    }
    
    fn visit_boolean_expression(&mut self, value: bool) {
        self.accumulator = Some(RuntimeValue::Bool(value));
    }
    
    fn visit_while_statement(&mut self, condition: &crate::ast::expression::Expression, body: &crate::ast::statement::Statement) {
        loop {
            self.visit_expression(condition);
            let condition_value = self.get_accumulator_value();

            match condition_value {
                RuntimeValue::Bool(true) => {
                    self.visit_statement(body);
                }
                RuntimeValue::Bool(false) => {
                    break;
                }
                _ => {
                    self.report_error(RuntimeError::InvalidCondition);
                }
            }
        }
    }
    
    fn visit_for_statement(&mut self, variable: &crate::lexer::Token, start: &crate::ast::expression::Expression, end: &crate::ast::expression::Expression, step: Option<&crate::ast::expression::Expression>, body: &crate::ast::statement::Statement) {
        self.visit_expression(start);
        let start_value = self.get_accumulator_value();

        self.visit_expression(end);
        let end_value = self.get_accumulator_value();

        let step_value = if let Some(step_expr) = step {
            self.visit_expression(step_expr);
            self.get_accumulator_value()
        } else {
            RuntimeValue::Number(1) // Default step value
        };

        self.push_scope();
        self.register_variable(variable.value.clone(), start_value);

        loop {
            let current_value = self.get_variable(&variable.value);
            let exit = builtin::gt(current_value.clone(), end_value.clone());
            match exit {
                Ok(RuntimeValue::Bool(true)) => {
                    break;
                },

                Err(err) => {
                    self.report_error(err);
                }
                _ => {}
            }

            self.visit_statement(body);

            let current_value = self.get_variable(&variable.value);
            let new_value = match builtin::add(current_value.clone(), step_value.clone()) {
                Ok(value) => value, 
                Err(err) => self.report_error(err)
            };
            self.set_variable_value(variable.value.clone(), new_value);
        }

        self.pop_scope();
    }
    
    fn visit_function_definition(&mut self, _name: &crate::lexer::Token, _arguments: &[crate::lexer::Token], _body: &crate::ast::statement::Statement) {
    }
    fn visit_function_call(&mut self, function_name: &crate::lexer::Token, arguments: &[crate::ast::expression::Expression]) {
        if let Some(function_info) = self.functions.get(&function_name.value) {
            let function_info = function_info.clone();
            self.call_function(function_info, arguments);
        }
    }
}
