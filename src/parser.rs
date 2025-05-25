use std::iter::Peekable;

use crate::{ast::{expression::{BinaryOperator, Expression, UnaryOperator}, statement::Statement, Ast}, diagnostic::{self, Diagnostic}, lexer::{Token, TokenKind}};


pub struct Parser<I: Iterator<Item = Token>> {
    tokens: Peekable<I>,
}

impl<I: Iterator<Item = Token>> Parser<I> {
    pub fn new(tokens: I) -> Self {
        Parser {
            tokens: tokens.peekable(),
        }
    }

    pub fn parse(&mut self) -> Result<Ast, Diagnostic> {
        let mut ast = Ast::new();

        let mut diagnostic = Diagnostic::new();

        loop {
            match self.parse_statement() {
                Ok(Some(stmt)) => ast.add_statement(stmt),
                Ok(None) => break,
                Err(err) => {
                    diagnostic.report_error(err);
                    self.recover();
                }
            }
        }

        if diagnostic.has_errors() {
            return Err(diagnostic);
        }

        Ok(ast)
    }

    fn advance(&mut self) -> Token {
        self.tokens.next().unwrap()
    }

    fn peek(&mut self) -> &Token {
        self.tokens.peek().unwrap()
    }

    fn expect(&mut self, expected_tokens: &[TokenKind]) -> Result<Token, diagnostic::Error> {

        let token = self.peek();
        if expected_tokens.contains(&token.kind) {
            Ok(self.advance())
        } else {
            Err(diagnostic::Error::UnexpectedToken {
                expected: expected_tokens.to_vec(),
                found: token.clone(),
            })
        }

    }

    fn recover(&mut self) {

        loop {
            let token = self.peek();
            match token.kind {
                TokenKind::EndOfFile |
                TokenKind::LetKeyword |
                TokenKind::SetKeyword
                => {
                    break;
                }
                _ => {
                    self.advance();
                }
            }
        }

    }


    fn parse_statement(&mut self) -> Result<Option<Statement>, diagnostic::Error> {
        let next_token_kind = self.peek().kind;

        if next_token_kind == TokenKind::EndOfFile {
            return Ok(None);
        }


        match next_token_kind {
            TokenKind::LetKeyword => Ok(Some(self.parse_variable_declaration()?)),
            TokenKind::SetKeyword => Ok(Some(self.parse_variable_assignement()?)),
            _ => {
                return Err(diagnostic::Error::UnexpectedToken {
                    expected: vec![TokenKind::LetKeyword],
                    found: self.advance(),
                });
            }
        }
    }

    fn parse_variable_declaration(&mut self) -> Result<Statement, diagnostic::Error> {
        self.expect(&[TokenKind::LetKeyword])?;
        let name_token = self.expect(&[TokenKind::Identifier])?;
        self.expect(&[TokenKind::BeKeyword])?;
        let value = self.parse_expression()?;

        Ok(Statement::VariableDeclaration {
            name: name_token,
            value,
        })
    }

    fn parse_variable_assignement(&mut self) -> Result<Statement, diagnostic::Error> {
        self.expect(&[TokenKind::SetKeyword])?;
        let name_token = self.expect(&[TokenKind::Identifier])?;
        self.expect(&[TokenKind::ToKeyword])?;
        let value = self.parse_expression()?;

        Ok(Statement::VariableAssignment {
            name: name_token,
            value,
        })
    }

    fn parse_expression(&mut self) -> Result<Expression, diagnostic::Error> {
        self.parse_expression_with_precedence(0)
    }

    fn parse_expression_with_precedence(&mut self, min_precedence: u8) -> Result<Expression, diagnostic::Error> {
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

    fn parse_unary_expression(&mut self) -> Result<Expression, diagnostic::Error> {
        
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

    fn parse_primary_expression(&mut self) -> Result<Expression, diagnostic::Error> {

        let next_token = self.peek();
        
        if next_token.kind == TokenKind::LeftParen {
            return self.parse_grouped_expression();
        }

        self.parse_literal_expression()
    }

    fn parse_grouped_expression(&mut self) -> Result<Expression, diagnostic::Error> {
        self.expect(&[TokenKind::LeftParen])?;
        let expr = self.parse_expression()?;
        self.expect(&[TokenKind::RightParen])?;
        Ok(Expression::Grouped(Box::new(expr)))
    }

    fn parse_literal_expression(&mut self) -> Result<Expression, diagnostic::Error> {
        let next_token = self.peek();

        match next_token.kind {
            TokenKind::Number => {
                let number_token: Token = self.advance();
                Ok(Expression::Number(number_token.value.parse().unwrap()))
            }
            TokenKind::Identifier => {
                let identifier_token = self.advance();
                Ok(Expression::Variable(identifier_token))
            }
            _ => {
                Err(diagnostic::Error::UnexpectedToken {
                    expected: vec![TokenKind::Number, TokenKind::Identifier],
                    found: next_token.clone(),
                })
            }
        }
    }

    
}