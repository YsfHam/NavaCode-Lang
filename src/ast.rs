pub mod statement;
pub mod expression;

use crate::lexer::Token;
use statement::Statement;
use expression::Expression;

pub struct Ast {
    statements: Vec<Statement>,
}

impl Ast {
    pub fn new() -> Self {
        Ast {
            statements: Vec::new(),
        }
    }

    pub fn add_statement(&mut self, statement: Statement) {
        self.statements.push(statement);
    }

    // Add public accessors for Ast and Statement for testing
    pub fn statements(&self) -> &Vec<Statement> {
        &self.statements
    }
}

pub trait AstExplorer {

    fn explore_ast(&mut self, ast: &Ast) {

        for statement in &ast.statements {
            self.visit_statement(statement);
        }
    }

    fn visit_statement(&mut self, statement: &Statement) {
        match statement {
            Statement::VariableDeclaration { name, value } => {
                                                        self.visit_variable_declaration(name, value);
                                                    }
            Statement::VariableAssignment { name, value } => {
                                                        self.visit_variable_assignement(name, value);
                                                    }
            Statement::IfStatement { if_then_branch: if_then_block, else_branch } 
                                                => self.visit_if_statement(&if_then_block.condition, &if_then_block.then_branch, else_branch.as_ref().map(|b| &**b)),
            Statement::BlockStatement { statements } => {
                                                self.block_statement_on_enter();
                                                statements.iter().for_each(|s: &Statement| self.visit_statement(s));
                                                self.block_statement_on_exit();
                                            }
            Statement::WhileStatement { condition, body } => 
                                            self.visit_while_statement(condition, body),
            Statement::ForStatement { variable, start, end, step, body } => 
                                self.visit_for_statement(variable, start, end, step.as_ref(), body),
            Statement::FunctionDefinition { name, arguments, body } => 
                                self.visit_function_definition(name, arguments, body),
            Statement::FunctionCall(function_call_data) =>
                                self.visit_function_call(&function_call_data.function_name, &function_call_data.arguments),
        }
    }

    fn visit_variable_declaration(&mut self, name: &Token, value: &Expression);
    fn visit_variable_assignement(&mut self, name: &Token, value: &Expression);
    fn visit_if_statement(&mut self, condition: &Expression, then_branch: &Statement, else_branch: Option<&Statement>);
    fn visit_while_statement(&mut self, condition: &Expression, body: &Statement);
    fn visit_for_statement(&mut self, variable: &Token, start: &Expression, end: &Expression, step: Option<&Expression>, body: &Statement);
    fn visit_function_definition(&mut self, name: &Token, arguments: &[Token], body: &Statement);
    fn visit_function_call(&mut self, function_name: &Token, arguments: &[Expression]);


    fn block_statement_on_enter(&mut self);
    fn block_statement_on_exit(&mut self);
    

    fn visit_expression(&mut self, expression: &Expression) {
        match expression {
            Expression::Number(value) => self.visit_number_expression(*value),
            Expression::Boolean(value) => self.visit_boolean_expression(*value),
            Expression::Variable(name) => self.visit_variable_expression(name),
            Expression::BinaryOperation { left, operator, right } => 
                                    self.visit_binary_operation(left, operator, right),
            Expression::UnaryOperation { operator, operand } =>
                                    self.visit_unary_operation(operator, operand),
            Expression::Grouped(expression) => self.visit_expression(expression),
            Expression::FunctionCall(function_call_data) => self.visit_function_call(&function_call_data.function_name, &function_call_data.arguments),
        }
    }
    
    fn visit_number_expression(&mut self, value: i64);
    fn visit_boolean_expression(&mut self, value: bool);
    fn visit_variable_expression(&mut self, name: &Token);
    fn visit_binary_operation(&mut self, left: &Expression, operator: &expression::BinaryOperator, right: &Expression);
    fn visit_unary_operation(&mut self, operator: &expression::UnaryOperator, operand: &Expression);
}
