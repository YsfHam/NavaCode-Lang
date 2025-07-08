use crate::ast::AstExplorer;

pub struct AstDebugPrinter {
    indent_level: usize,
}

impl AstDebugPrinter {
    pub fn new() -> Self {
        AstDebugPrinter { indent_level: 0 }
    }
}

impl AstExplorer for AstDebugPrinter {
    fn visit_variable_declaration(&mut self, name: &crate::lexer::Token, value: &crate::ast::expression::Expression) {
        println!("{}Variable Declaration: {}", "  ".repeat(self.indent_level), name.value);
        self.indent_level += 1;
        self.visit_expression(value);
        self.indent_level -= 1;
    }

    fn visit_number_expression(&mut self, value: i64) {
        println!("{}Number: {}", "  ".repeat(self.indent_level), value);
    }

    fn visit_variable_expression(&mut self, name: &crate::lexer::Token) {
        println!("{}Variable: {}", "  ".repeat(self.indent_level), name.value);
    }
    
    fn visit_binary_operation(&mut self, left: &crate::ast::expression::Expression, operator: &crate::ast::expression::BinaryOperator, right: &crate::ast::expression::Expression) {
        println!("{}Binary Operation:", "  ".repeat(self.indent_level));
        self.indent_level += 1;
        println!("{}Left:", "  ".repeat(self.indent_level));
        self.visit_expression(left);
        println!("{}Operator: {:?}", "  ".repeat(self.indent_level), operator);
        println!("{}Right:", "  ".repeat(self.indent_level));
        self.visit_expression(right);
        self.indent_level -= 1;
    }
    
    fn visit_unary_operation(&mut self, operator: &crate::ast::expression::UnaryOperator, operand: &crate::ast::expression::Expression) {
        println!("{}Unary Operation:", "  ".repeat(self.indent_level));
        self.indent_level += 1;
        println!("{}Operator: {:?}", "  ".repeat(self.indent_level), operator);
        println!("{}Operand:", "  ".repeat(self.indent_level));
        self.visit_expression(operand);
        self.indent_level -= 1;
    }
    
    fn visit_variable_assignement(&mut self, name: &crate::lexer::Token, value: &crate::ast::expression::Expression) {
        println!("{}Variable Assignment: {}", "  ".repeat(self.indent_level), name.value);
        self.indent_level += 1;
        self.visit_expression(value);
        self.indent_level -= 1;
    }
    
    fn visit_if_statement(&mut self, condition: &crate::ast::expression::Expression, then_branch: &crate::ast::statement::Statement, else_branch: Option<&crate::ast::statement::Statement>) {
        println!("{}If Statement:", "  ".repeat(self.indent_level));
        self.indent_level += 1;
        println!("{}Condition:", "  ".repeat(self.indent_level));
        self.visit_expression(condition);
        
        println!("{}Then Branch:", "  ".repeat(self.indent_level));
        self.visit_statement(then_branch);
        
        if let Some(else_branch) = else_branch {
            println!("{}Else Branch:", "  ".repeat(self.indent_level));
            self.visit_statement(else_branch);
        }
        
        self.indent_level -= 1;
    }
    
    fn block_statement_on_enter(&mut self) {
        println!("{}Entering Block Statement", "  ".repeat(self.indent_level));
        self.indent_level += 1;
    }
    
    fn block_statement_on_exit(&mut self) {
        self.indent_level -= 1;
        println!("{}Exiting Block Statement", "  ".repeat(self.indent_level));
    }
    
    fn visit_boolean_expression(&mut self, value: bool) {
        println!("{}Boolean: {}", "  ".repeat(self.indent_level), value);
    }
    
    fn visit_while_statement(&mut self, condition: &crate::ast::expression::Expression, body: &crate::ast::statement::Statement) {
        println!("{}While Statement:", "  ".repeat(self.indent_level));
        self.indent_level += 1;
        println!("{}Condition:", "  ".repeat(self.indent_level));
        self.visit_expression(condition);
        
        println!("{}Body:", "  ".repeat(self.indent_level));
        self.visit_statement(body);
        
        self.indent_level -= 1;
    }
    
    fn visit_for_statement(&mut self, variable: &crate::lexer::Token, start: &crate::ast::expression::Expression, end: &crate::ast::expression::Expression, step: Option<&crate::ast::expression::Expression>, body: &crate::ast::statement::Statement) {
        println!("{}For Statement:", "  ".repeat(self.indent_level));
        self.indent_level += 1;
        println!("{}Variable: {}", "  ".repeat(self.indent_level), variable.value);
        
        println!("{}Start:", "  ".repeat(self.indent_level));
        self.visit_expression(start);
        
        println!("{}End:", "  ".repeat(self.indent_level));
        self.visit_expression(end);
        
        if let Some(step) = step {
            println!("{}Step:", "  ".repeat(self.indent_level));
            self.visit_expression(step);
        }
        
        println!("{}Body:", "  ".repeat(self.indent_level));
        self.visit_statement(body);
        
        self.indent_level -= 1;
    }
    
    fn visit_function_definition(&mut self, name: &crate::lexer::Token, arguments: &[crate::lexer::Token], body: &crate::ast::statement::Statement) {
        println!("{}Function Definition: {}", "  ".repeat(self.indent_level), name.value);
        self.indent_level += 1;
        
        if !arguments.is_empty() {
            println!("{}Arguments:", "  ".repeat(self.indent_level));
            for arg in arguments {
                println!("{}- {}", "  ".repeat(self.indent_level + 1), arg.value);
            }
        } else {
            println!("{}No Arguments", "  ".repeat(self.indent_level));
        }
        
        println!("{}Body:", "  ".repeat(self.indent_level));
        self.visit_statement(body);
        
        self.indent_level -= 1;
    }
}