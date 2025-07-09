use crate::{ast::{Ast, AstExplorer}, diagnostic::{Diagnostic, Diagnostics}, symbols_table::{FunctionSymbol, ScopeId, SymbolsTable, VariableSymbol}, types::{self, Type}, BlockType};

pub struct Resolver {
    symbols_table: SymbolsTable,
    current_scope_id: ScopeId,
    diagnostics: Diagnostics,
    block_type_stack: Vec<BlockType>,
    current_block_type: Option<BlockType>,
    type_accumulator: Type,
}

impl Resolver {
    pub fn new() -> Self {
        Self {
            symbols_table: SymbolsTable::new(),
            current_scope_id: ScopeId(0),
            diagnostics: Diagnostics::new(),
            block_type_stack: Vec::new(),
            current_block_type: None,
            type_accumulator: Type::Unresolved,
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

    fn enter_scope(&mut self) {
        self.current_scope_id = self.symbols_table.enter_scope(self.current_scope_id);
    }
    fn exit_scope(&mut self) {
        self.current_scope_id = self.symbols_table.exit_scope(self.current_scope_id);
    }

    fn is_inside_block(&self, block_type: BlockType) -> bool {
        self.block_type_stack.iter().any(|&bt| bt == block_type)
    }
}

impl AstExplorer for Resolver {
    fn visit_variable_declaration(&mut self, name: &crate::lexer::Token, value: &crate::ast::expression::Expression) {
        
        if self.symbols_table.lookup_variable_in_scope_only(&name.value, self.current_scope_id).is_some() {
            self.diagnostics.report(Diagnostic::variable_redefinition(name.clone()));
        }
        
        self.visit_expression(value);

        self.symbols_table.define_variable(VariableSymbol {
            identifier: name.value.clone(),
            sym_type: self.type_accumulator.clone(),
        }, self.current_scope_id);
        
    }

    fn visit_variable_assignement(&mut self, name: &crate::lexer::Token, value: &crate::ast::expression::Expression) {
        self.visit_expression(value);

        if let Some(variable_symbol) = self.symbols_table.lookup_variable(&name.value, self.current_scope_id) {
            if variable_symbol.sym_type != self.type_accumulator {
                self.diagnostics.report(Diagnostic::variable_type_mismatch(name.clone(), variable_symbol.sym_type.clone(), self.type_accumulator.clone()));
            }
        }
        else {
            self.diagnostics.report(Diagnostic::undefined_variable(name.clone()));
        }
    }

    fn visit_if_statement(&mut self, condition: &crate::ast::expression::Expression, then_branch: &crate::ast::statement::Statement, else_branch: Option<&crate::ast::statement::Statement>) {
        self.current_block_type = Some(BlockType::IfBlock);
        self.visit_expression(condition);

        if self.type_accumulator != Type::Bool {
            self.diagnostics.report(Diagnostic::expression_type_mismatch(Type::Bool, self.type_accumulator.clone(), condition.span()));
        }

        self.visit_statement(then_branch);
        if let Some(else_branch) = else_branch {
            self.current_block_type = Some(BlockType::ElseBlock);
            self.visit_statement(else_branch);
        }
    }

    fn visit_while_statement(&mut self, condition: &crate::ast::expression::Expression, body: &crate::ast::statement::Statement) {
        self.current_block_type = Some(BlockType::WhileBlock);
        self.visit_expression(condition);
        if self.type_accumulator != Type::Bool {
            self.diagnostics.report(Diagnostic::expression_type_mismatch(Type::Bool, self.type_accumulator.clone(), condition.span()));
        }
        self.visit_statement(body);

    }

    fn visit_for_statement(&mut self, variable: &crate::lexer::Token, start: &crate::ast::expression::Expression, end: &crate::ast::expression::Expression, step: &Option<crate::ast::expression::Expression>, body: &crate::ast::statement::Statement) {
        self.current_block_type = Some(BlockType::ForBlock);

        self.visit_expression(start);
        let start_type = self.type_accumulator.clone();
        self.visit_expression(end);
        let end_type = self.type_accumulator.clone();
        
        if start_type != end_type {
            self.diagnostics.report(Diagnostic::variable_type_mismatch(variable.clone(), start_type.clone(), end_type.clone()));
        }

        if let Some(step_expr) = step {
            self.visit_expression(step_expr);

            let step_type = self.type_accumulator.clone();
            if step_type != start_type {
                self.diagnostics.report(Diagnostic::variable_type_mismatch(variable.clone(), start_type.clone(), step_type.clone()));
            }

            if end_type != step_type {
                self.diagnostics.report(Diagnostic::expression_type_mismatch(end_type.clone(), step_type.clone(), step_expr.span()));
            }
        }
        self.enter_scope();
        self.symbols_table.define_variable(VariableSymbol {
            identifier: variable.value.clone(),
            sym_type: start_type,
        }, self.current_scope_id);
        self.visit_statement(body);
        self.exit_scope();
    }

    fn block_statement_on_enter(&mut self) {
        self.enter_scope();
        if let Some(block_type) = self.current_block_type.take() {
            self.block_type_stack.push(block_type);
        }
    }
    

    fn block_statement_on_exit(&mut self) {
        self.exit_scope();
        self.block_type_stack.pop();
    }

    fn visit_number_expression(&mut self, _value: i64) {
        self.type_accumulator = Type::Int;
    }

    fn visit_boolean_expression(&mut self, _value: bool) {
        self.type_accumulator = Type::Bool;
    }

    fn visit_variable_expression(&mut self, name: &crate::lexer::Token) {
        if let Some(symbol) = self.symbols_table.lookup_variable(&name.value, self.current_scope_id) {
            self.type_accumulator = symbol.sym_type.clone();
        } else {
           self.diagnostics.report(Diagnostic::undefined_variable(name.clone()));
        }
    }

    fn visit_binary_operation(&mut self, left: &crate::ast::expression::Expression, operator: &crate::ast::expression::BinaryOperator, right: &crate::ast::expression::Expression) {
        self.visit_expression(left);
        let left_type = self.type_accumulator.clone();
        self.visit_expression(right);
        let right_type = self.type_accumulator.clone();

        self.type_accumulator = types::resolve_binary_operation_type(&left_type, &right_type, operator);

        if self.type_accumulator == Type::Unresolved {
            self.diagnostics.report(Diagnostic::incompatible_binary_operation(left_type, right_type, *operator, left.span().union(&right.span())));
        }
    }

    fn visit_unary_operation(&mut self, operator: &crate::ast::expression::UnaryOperator, operand: &crate::ast::expression::Expression) {
        self.visit_expression(operand);
        let operand_type = self.type_accumulator.clone();
        self.type_accumulator = types::resolve_unary_operation_type(&operand_type, operator);
        if self.type_accumulator == Type::Unresolved {
            self.diagnostics.report(Diagnostic::incompatible_unary_operation(operand_type, *operator, operand.span()));
        }
    }
    
    fn visit_function_definition(&mut self, name: &crate::lexer::Token, arguments: &[crate::lexer::Token], body: &crate::ast::statement::Statement) {
        self.symbols_table.define_function(FunctionSymbol {
            identifier: name.value.clone(),
            parameters: arguments.iter().map(|arg| arg.value.clone()).collect(),
        });

        self.enter_scope();
        self.current_block_type = Some(BlockType::FunctionBlock);

        arguments
            .iter()
            .for_each(|argument| 
            self.symbols_table.define_variable(VariableSymbol {
            identifier: argument.value.clone(),
            sym_type: Type::Unresolved, // Type will be inferred later
        }, self.current_scope_id));
        
        self.visit_statement(body);
        self.exit_scope();
    }
    
    fn visit_function_call(&mut self, function_name: &crate::lexer::Token, arguments: &[crate::ast::expression::Expression]) {
        if let Some(function_symbol) = self.symbols_table.lookup_function(&function_name.value) {
            if function_symbol.parameters.len() != arguments.len() {
                self.diagnostics.report(Diagnostic::function_arguments_mismatch(function_name.clone(), function_symbol.parameters.len(), arguments.len()));
            }
        } 
        else {
            self.diagnostics.report(Diagnostic::undefined_function(function_name.clone()));
        }

        for argument in arguments {
            self.visit_expression(argument);
        }
    }

    fn visit_return_statement(&mut self, span: crate::lexer::TextSpan, expression: &Option<crate::ast::expression::Expression>) {
        if self.is_inside_block(BlockType::FunctionBlock) {
            if let Some(expr) = expression {
                self.visit_expression(expr);
            }
        } else {
            self.diagnostics.report(Diagnostic::return_outside_function(span));
        }
    }
}