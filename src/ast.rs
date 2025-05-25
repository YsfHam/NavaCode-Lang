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
        }
    }

    fn visit_variable_declaration(&mut self, name: &Token, value: &Expression);

    fn visit_expression(&mut self, expression: &Expression) {
        match expression {
            Expression::Number(value) => self.visit_number_expression(*value),
            Expression::Variable(name) => self.visit_variable_expression(name),
            Expression::BinaryOperation { left, operator, right } => 
                    self.visit_binary_operation(left, operator, right),
            Expression::UnaryOperation { operator, operand } =>
                    self.visit_unary_operation(operator, operand),
            Expression::Grouped(expression) => self.visit_expression(expression),
                    }
    }
    
    fn visit_number_expression(&mut self, value: i64);
    fn visit_variable_expression(&mut self, name: &Token);
    fn visit_binary_operation(&mut self, left: &Expression, operator: &expression::BinaryOperator, right: &Expression);
    fn visit_unary_operation(&mut self, operator: &expression::UnaryOperator, operand: &Expression);
}
