use std::iter::Peekable;

use crate::{ast::{expression::{BinaryOperator, Expression, UnaryOperator}, statement::{IfThenBranch, Statement}, Ast}, diagnostic::{Diagnostic, Diagnostics}, lexer::{Token, TokenKind}};


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BlockType {
    IfBlock,
    WhileBlock,
    ForBlock,
    ElseBlock,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ErrorRecoveryState {
    RecoverFromBadBlock(BlockType),
}


// Tokens that we can recover from
static RECOVERY_END_POINTS: &[TokenKind] = &[
    TokenKind::LetKeyword,
    TokenKind::SetKeyword,
    TokenKind::IfKeyword,
    TokenKind::WhileKeyword,
    TokenKind::ForKeyword,
    TokenKind::EndKeyword,
    TokenKind::ElseKeyword,
];

pub struct Parser<I: Iterator<Item = Token>> {
    tokens: Peekable<I>,

    recovery_states: Vec<ErrorRecoveryState>,
    consumed_tokens: Vec<TokenKind>,
}

impl<I: Iterator<Item = Token>> Parser<I> {
    pub fn new(tokens: I) -> Self {
        Parser {
            tokens: tokens.peekable(),
            recovery_states: Vec::new(),
            consumed_tokens: Vec::new(),
        }
    }

    pub fn parse(mut self) -> Result<Ast, Diagnostics> {
        let mut ast = Ast::new();

        let mut diagnostic = Diagnostics::new();

        loop {
            match self.parse_statement() {
                Ok(Some(stmt)) => ast.add_statement(stmt),
                Ok(None) => break,
                Err(diag) => {
                    diagnostic.report(diag);
                    self.recover();
                }
            }
        }

        if diagnostic.has_errors() {
            return Err(diagnostic);
        }

        Ok(ast)
    }

    fn push_recovery_state(&mut self, recovery_state: ErrorRecoveryState) {
        self.recovery_states.push(recovery_state);
    }

    fn pop_recovery_state(&mut self) -> Option<ErrorRecoveryState> {
        self.recovery_states.pop()
    }

    fn current_recovery_state(&self) -> Option<&ErrorRecoveryState> {
        self.recovery_states.last()
    }

    fn advance(&mut self) -> Token {
        let token = self.tokens.next().unwrap();
        self.consumed_tokens.push(token.kind);
        token
    }

    // fn advance_if(&mut self, expected: &[TokenKind]) -> Option<Token> {
    //     if expected.contains(&self.peek().kind) {
    //         Some(self.advance())
    //     } else {
    //         None
    //     }
    // }

    fn peek(&mut self) -> &Token {
        self.tokens.peek().unwrap()
    }

    fn expect(&mut self, expected_tokens: &[TokenKind]) -> Result<Token, Diagnostic> {

        let token = self.peek();
        if expected_tokens.contains(&token.kind) {
            Ok(self.advance())
        } else {
            Err(Diagnostic::unexpected_token(expected_tokens.to_vec(), token.clone()))
        }

    }

    fn recover(&mut self) {

        loop {
            let token_kind = self.peek().kind;

            if token_kind == TokenKind::EndOfFile || RECOVERY_END_POINTS.contains(&token_kind) {
                // If we reach the end of file or a statement start token, we can stop recovering
                break;
            }
            else {
                // Otherwise, we skip the current token
                self.advance();
            }
        }

    }


    fn parse_statement(&mut self) -> Result<Option<Statement>, Diagnostic> {

        let next_token_kind = self.peek().kind;


        if next_token_kind == TokenKind::EndOfFile {
            return Ok(None);
        }

        match next_token_kind {
            TokenKind::LetKeyword => Ok(Some(self.parse_variable_declaration()?)),
            TokenKind::SetKeyword => Ok(Some(self.parse_variable_assignement()?)),
            TokenKind::IfKeyword => 
                Ok(Some(self.parse_if_statement().map_err(|diag| {
                    self.push_recovery_state(ErrorRecoveryState::RecoverFromBadBlock(BlockType::IfBlock));
                    diag
                })?)),

            TokenKind::WhileKeyword => 
                Ok(Some(self.parse_while_statement().map_err(|diag| {
                    self.push_recovery_state(ErrorRecoveryState::RecoverFromBadBlock(BlockType::WhileBlock));
                    diag
                })?)),

             TokenKind::ForKeyword => 
                Ok(Some(self.parse_for_statement().map_err(|diag| {
                    self.push_recovery_state(ErrorRecoveryState::RecoverFromBadBlock(BlockType::ForBlock));
                    diag
                })?)),

            
            // Reporting errors
            TokenKind::ElseKeyword 
                if self.current_recovery_state() == Some(&ErrorRecoveryState::RecoverFromBadBlock(BlockType::IfBlock)) => {
                self.advance();
                self.parse_statement()
            }

            TokenKind::ElseKeyword if self.consumed_tokens.last() == Some(&TokenKind::EndKeyword) => {
               self.push_recovery_state(ErrorRecoveryState::RecoverFromBadBlock(BlockType::ElseBlock));
                Err(
                    Diagnostic::unexpected_else_after_end(self.advance().position)
                )
            }

            TokenKind::ElseKeyword => {
                self.push_recovery_state(ErrorRecoveryState::RecoverFromBadBlock(BlockType::ElseBlock));
                Err(
                    Diagnostic::unexpected_else_token(self.advance().position)
                )
            }

            TokenKind::EndKeyword 
                if 
                    matches!(self.current_recovery_state(), Some(ErrorRecoveryState::RecoverFromBadBlock(_)))
                => {
                self.pop_recovery_state();
                self.advance();
                self.parse_statement()
            }
            TokenKind::EndKeyword => Err(
                Diagnostic::unexpected_end_token(self.advance().position)
            ),
            _ => {
                return Err(Diagnostic::unexpected_token(
                    RECOVERY_END_POINTS.to_vec(),
                    self.advance()
                ));
            }
        }
    }

    fn parse_statements_until(&mut self, stop_tokens: &[TokenKind]) -> Result<Statement, Diagnostic> {
        let mut statements = Vec::new();

        while !stop_tokens.contains(&self.peek().kind) {
            if let Some(stmt) = self.parse_statement()? {
                statements.push(stmt);
            } else {
                // If we reach the end of file or a stop token, we stop parsing
                break;
            }
        }
        
        Ok(Statement::BlockStatement { statements })
    }

    fn parse_variable_declaration(&mut self) -> Result<Statement, Diagnostic> {
        self.expect(&[TokenKind::LetKeyword])?;
        let name_token = self.expect(&[TokenKind::Identifier])?;
        self.expect(&[TokenKind::BeKeyword])?;
        let value = self.parse_expression()?;

        Ok(Statement::VariableDeclaration {
            name: name_token,
            value,
        })
    }

    fn parse_variable_assignement(&mut self) -> Result<Statement, Diagnostic> {
        self.expect(&[TokenKind::SetKeyword])?;
        let name_token = self.expect(&[TokenKind::Identifier])?;
        self.expect(&[TokenKind::ToKeyword])?;
        let value = self.parse_expression()?;

        Ok(Statement::VariableAssignment {
            name: name_token,
            value,
        })
    }

    fn parse_if_statement(&mut self) -> Result<Statement, Diagnostic> {
        let if_then_branch = self.parse_if_then_branch()?;

        let else_branch = if self.peek().kind == TokenKind::ElseKeyword {
            Some(self.parse_else_branch()?)
        }
        else {
            self.expect(&[TokenKind::EndKeyword])?;
            None
        }
        .map(Box::new);

        Ok(Statement::IfStatement {
            if_then_branch,
            else_branch,
        })
    }

    fn parse_if_then_branch(&mut self) -> Result<IfThenBranch, Diagnostic> {
        self.expect(&[TokenKind::IfKeyword])?;
        let condition = self.parse_expression()?;
        self.expect(&[TokenKind::ThenKeyword])?;
        let then_branch = self.parse_statements_until(&[TokenKind::ElseKeyword, TokenKind::EndKeyword])?;

        Ok(IfThenBranch { condition, then_branch: Box::new(then_branch) })
    }

    fn parse_else_branch(&mut self) ->Result<Statement, Diagnostic> {
        self.expect(&[TokenKind::ElseKeyword])?;

        if self.peek().kind == TokenKind::IfKeyword {
            return self.parse_if_statement()
        }
        
        let else_branch = self.parse_statements_until(&[TokenKind::EndKeyword])?;
        self.expect(&[TokenKind::EndKeyword])?;
        Ok(else_branch)
    }

    fn parse_while_statement(&mut self) -> Result<Statement, Diagnostic> {
        self.expect(&[TokenKind::WhileKeyword])?;
        let condition = self.parse_expression()?;
        self.expect(&[TokenKind::DoKeyword])?;
        let body = self.parse_statements_until(&[TokenKind::EndKeyword])?;
        self.expect(&[TokenKind::EndKeyword])?;

        Ok(Statement::WhileStatement {
            condition,
            body: Box::new(body),
        })
    }

    fn parse_for_statement(&mut self) -> Result<Statement, Diagnostic> {
        self.expect(&[TokenKind::ForKeyword])?;
        let variable = self.expect(&[TokenKind::Identifier])?;
        self.expect(&[TokenKind::FromKeyword])?;
        let start = self.parse_expression()?;
        self.expect(&[TokenKind::ToKeyword])?;
        let end = self.parse_expression()?;
        let step = if self.peek().kind == TokenKind::StepKeyword {
            self.advance(); // consume the 'step' keyword
            Some(self.parse_expression()?)
        } else {
            None
        };

        self.expect(&[TokenKind::DoKeyword])?;
        let body = self.parse_statements_until(&[TokenKind::EndKeyword])?;
        self.expect(&[TokenKind::EndKeyword])?;

        Ok(Statement::ForStatement {
            variable,
            start,
            end,
            step,
            body: Box::new(body),
        })
    }

    fn parse_expression(&mut self) -> Result<Expression, Diagnostic> {
        self.parse_expression_with_precedence(0)
    }

    fn parse_expression_with_precedence(&mut self, min_precedence: u8) -> Result<Expression, Diagnostic> {
        let mut left = self.parse_unary_expression()?;

        while let Ok(op) = BinaryOperator::try_from(self.peek().kind) {

            let precedence = op.precedence();
            if precedence < min_precedence {
                break;
            }

            self.advance(); // consume the operator

            // For left-associative operators, use precedence + 1 for the right operand
            let next_min_prec = precedence + 1;
            let right = self.parse_expression_with_precedence(next_min_prec)?;

            left = Expression::BinaryOperation {
                left: Box::new(left),
                operator: op,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_unary_expression(&mut self) -> Result<Expression, Diagnostic> {
        
        if let Ok(op) = UnaryOperator::try_from(self.peek().kind) {
            self.advance(); // consume the operator
            let operand = self.parse_unary_expression()?;
            return Ok(Expression::UnaryOperation {
                operator: op,
                operand: Box::new(operand),
            });
        }

        self.parse_primary_expression()
    }

    fn parse_primary_expression(&mut self) -> Result<Expression, Diagnostic> {

        let next_token = self.peek();
        
        if next_token.kind == TokenKind::LeftParen {
            return self.parse_grouped_expression();
        }

        self.parse_literal_expression()
    }

    fn parse_grouped_expression(&mut self) -> Result<Expression, Diagnostic> {

        self.expect(&[TokenKind::LeftParen])?;
        let expr = self.parse_expression()?;
        self.expect(&[TokenKind::RightParen])?;
        Ok(Expression::Grouped(Box::new(expr)))
    }

    fn parse_literal_expression(&mut self) -> Result<Expression, Diagnostic> {
        let next_token = self.peek();

        match next_token.kind {
            TokenKind::Number => {
                let number_token: Token = self.advance();
                Ok(Expression::Number(number_token.value.parse().unwrap()))
            }
            TokenKind::TrueKeyword => {
                self.advance(); // consume the 'true' keyword
                Ok(Expression::Boolean(true))
            }
            TokenKind::FalseKeyword => {
                self.advance(); // consume the 'false' keyword
                Ok(Expression::Boolean(false))
            }
            TokenKind::Identifier => {
                let identifier_token = self.advance();
                Ok(Expression::Variable(identifier_token))
            }
            _ => {
                Err(Diagnostic::unexpected_token(
                    vec![TokenKind::Number, TokenKind::Identifier],
                    next_token.clone(),
                ))
            }
        }
    }

    
}