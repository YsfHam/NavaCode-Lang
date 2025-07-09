use crate::{ast::{Ast, AstExplorer}, diagnostic::{Diagnostic, Diagnostics}, symbols_table::{FunctionSymbol, ScopeId, SymbolsTable, VariableSymbol}};

pub struct Resolver {
    symbols_table: SymbolsTable,
    current_scope_id: ScopeId,
    diagnostics: Diagnostics,
}

impl Resolver {
    pub fn new() -> Self {
        Self {
            symbols_table: SymbolsTable::new(),
            current_scope_id: ScopeId(0),
            diagnostics: Diagnostics::new(),
        }
    }

    pub fn resolve(mut self, ast: &Ast) -> Result<SymbolsTable, Diagnostics> {
        self.explore_ast(ast);

        if self.diagnostics.has_errors() {
            Err(self.diagnostics)
        } else {
            Ok(self.symbols_table)
        }
    }
}

impl AstExplorer for Resolver {
    fn visit_variable_declaration(&mut self, name: &crate::lexer::Token, value: &crate::ast::expression::Expression) {
        
        if self.symbols_table.lookup_variable_in_scope_only(&name.value, self.current_scope_id).is_some() {
            self.diagnostics.report(Diagnostic::variable_redifinition(name.clone()));
        }
        
        self.visit_expression(value);

        self.symbols_table.define_variable(VariableSymbol {
            identifier: name.value.clone(),
        }, self.current_scope_id);
        
    }

    fn visit_variable_assignement(&mut self, name: &crate::lexer::Token, value: &crate::ast::expression::Expression) {

        if self.symbols_table.lookup_variable(&name.value, self.current_scope_id).is_none() {
            self.diagnostics.report(Diagnostic::undefined_variable(name.clone()));
        }
        self.visit_expression(value);
    }

    fn visit_if_statement(&mut self, condition: &crate::ast::expression::Expression, then_branch: &crate::ast::statement::Statement, else_branch: Option<&crate::ast::statement::Statement>) {
        self.visit_expression(condition);
        self.visit_statement(then_branch);
        if let Some(else_branch) = else_branch {
            self.visit_statement(else_branch);
        }
    }

    fn visit_while_statement(&mut self, condition: &crate::ast::expression::Expression, body: &crate::ast::statement::Statement) {
        self.visit_expression(condition);
        self.visit_statement(body);
    }

    fn visit_for_statement(&mut self, variable: &crate::lexer::Token, start: &crate::ast::expression::Expression, end: &crate::ast::expression::Expression, step: Option<&crate::ast::expression::Expression>, body: &crate::ast::statement::Statement) {
        self.visit_expression(start);
        self.visit_expression(end);
        if let Some(step_expr) = step {
            self.visit_expression(step_expr);
        }
        self.current_scope_id = self.symbols_table.enter_scope(self.current_scope_id);
        self.symbols_table.define_variable(VariableSymbol {
            identifier: variable.value.clone(),
        }, self.current_scope_id);
        self.visit_statement(body);
        self.current_scope_id = self.symbols_table.exit_scope(self.current_scope_id);
    }

    fn block_statement_on_enter(&mut self) {
        self.current_scope_id = self.symbols_table.enter_scope(self.current_scope_id);
    }
    

    fn block_statement_on_exit(&mut self) {
        self.current_scope_id = self.symbols_table.exit_scope(self.current_scope_id);
    }

    fn visit_number_expression(&mut self, _value: i64) {

    }

    fn visit_boolean_expression(&mut self, _value: bool) {
    }

    fn visit_variable_expression(&mut self, name: &crate::lexer::Token) {
        if let Some(_) = self.symbols_table.lookup_variable(&name.value, self.current_scope_id) {
            // Symbol found, do nothing for now
        } else {
           self.diagnostics.report(Diagnostic::undefined_variable(name.clone()));
        }
    }

    fn visit_binary_operation(&mut self, left: &crate::ast::expression::Expression, _operator: &crate::ast::expression::BinaryOperator, right: &crate::ast::expression::Expression) {
        self.visit_expression(left);
        self.visit_expression(right);
    }

    fn visit_unary_operation(&mut self, _operator: &crate::ast::expression::UnaryOperator, operand: &crate::ast::expression::Expression) {
        self.visit_expression(operand);
    }
    
    fn visit_function_definition(&mut self, name: &crate::lexer::Token, arguments: &[crate::lexer::Token], body: &crate::ast::statement::Statement) {
        self.symbols_table.define_function(FunctionSymbol {
            identifier: name.value.clone(),
            parameters: arguments.iter().map(|arg| arg.value.clone()).collect(),
        });

        self.current_scope_id = self.symbols_table.enter_scope(self.current_scope_id);
        
        arguments
            .iter()
            .for_each(|argument| 
            self.symbols_table.define_variable(VariableSymbol {
            identifier: argument.value.clone(),
        }, self.current_scope_id));
        
        self.visit_statement(body);
        self.current_scope_id = self.symbols_table.exit_scope(self.current_scope_id);
    }
    
    fn visit_function_call(&mut self, function_name: &crate::lexer::Token, arguments: &[crate::ast::expression::Expression]) {
        if let Some(function_symbol) = self.symbols_table.lookup_function(&function_name.value) {
            if function_symbol.parameters.len() != arguments.len() {
                self.diagnostics.report(Diagnostic::function_arguments_mismatch(function_name.clone(), function_symbol.parameters.len(), arguments.len()));
            }
        } else {
            self.diagnostics.report(Diagnostic::undefined_function(function_name.clone()));
        }

        for argument in arguments {
            self.visit_expression(argument);
        }
    }
}

