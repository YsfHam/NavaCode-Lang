use navacodelang::lexer::{Lexer, Token, TokenKind, TokenPosition};
use navacodelang::parser::Parser;
use navacodelang::ast::{Ast, statement::Statement, expression::Expression};
use navacodelang::ast::expression::{BinaryOperator, UnaryOperator};

fn lex_all(input: &str) -> Vec<(TokenKind, String)> {
    Lexer::new(input)
        .map(|t| (t.kind, t.value))
        .collect()
}

#[test]
fn test_lexer_number() {
    let tokens = lex_all("12345");
    assert_eq!(tokens, vec![
        (TokenKind::Number, "12345".to_string()),
        (TokenKind::EndOfFile, "EOF".to_string()),
    ]);
}

#[test]
fn test_lexer_identifier() {
    let tokens = lex_all("foo bar");
    assert_eq!(tokens, vec![
        (TokenKind::Identifier, "foo".to_string()),
        (TokenKind::Identifier, "bar".to_string()),
        (TokenKind::EndOfFile, "EOF".to_string()),
    ]);
}

#[test]
fn test_lexer_keywords() {
    let tokens = lex_all("let be");
    assert_eq!(tokens, vec![
        (TokenKind::LetKeyword, "let".to_string()),
        (TokenKind::BeKeyword, "be".to_string()),
        (TokenKind::EndOfFile, "EOF".to_string()),
    ]);
}

#[test]
fn test_lexer_operators() {
    let tokens = lex_all("+ - * / == != <= >= < >");
    let expected = vec![
        (TokenKind::Plus, "+".to_string()),
        (TokenKind::Minus, "-".to_string()),
        (TokenKind::Star, "*".to_string()),
        (TokenKind::Slash, "/".to_string()),
        (TokenKind::EqualEqual, "==".to_string()),
        (TokenKind::NotEqual, "!=".to_string()),
        (TokenKind::LessThanOrEqual, "<=".to_string()),
        (TokenKind::GreaterThanOrEqual, ">=".to_string()),
        (TokenKind::LessThan, "<".to_string()),
        (TokenKind::GreaterThan, ">".to_string()),
        (TokenKind::EndOfFile, "EOF".to_string()),
    ];
    assert_eq!(tokens, expected);
}

#[test]
fn test_lexer_symbols() {
    let tokens = lex_all("( )");
    assert_eq!(tokens, vec![
        (TokenKind::LeftParen, "(".to_string()),
        (TokenKind::RightParen, ")".to_string()),
        (TokenKind::EndOfFile, "EOF".to_string()),
    ]);
}

#[test]
fn test_lexer_unknown() {
    let tokens = lex_all("foo$bar");
    assert_eq!(tokens, vec![
        (TokenKind::Identifier, "foo".to_string()),
        (TokenKind::Unknown, "$".to_string()),
        (TokenKind::Identifier, "bar".to_string()),
        (TokenKind::EndOfFile, "EOF".to_string()),
    ]);
}

#[test]
fn test_lexer_mixed() {
    let tokens = lex_all("let x = 42 + y");
    assert_eq!(tokens, vec![
        (TokenKind::LetKeyword, "let".to_string()),
        (TokenKind::Identifier, "x".to_string()),
        (TokenKind::Unknown, "=".to_string()),
        (TokenKind::Number, "42".to_string()),
        (TokenKind::Plus, "+".to_string()),
        (TokenKind::Identifier, "y".to_string()),
        (TokenKind::EndOfFile, "EOF".to_string()),
    ]);
}

#[test]
fn test_lexer_logical_operators() {
    let tokens = lex_all("and or");
    assert_eq!(tokens, vec![
        (TokenKind::AndKeyword, "and".to_string()),
        (TokenKind::OrKeyword, "or".to_string()),
        (TokenKind::EndOfFile, "EOF".to_string()),
    ]);
}

// Parser tests
fn parse_program(input: &str) -> Result<Ast, String> {
    let tokens: Vec<_> = Lexer::new(input).collect();
    let mut parser = Parser::new(tokens.into_iter());
    parser.parse().map_err(|d| format!("{d:?}"))
}

#[test]
fn test_parser_variable_declaration() {
    let ast = parse_program("let x be 42").unwrap();
    assert_eq!(ast.statements().len(), 1);
    let stmt = &ast.statements()[0];
    let (name, value) = match stmt {
        Statement::VariableDeclaration { name, value } => (name, value),
        _ => panic!("Expected variable declaration"),
    };
    assert_eq!(name.value, "x");
    assert_eq!(*value, Expression::Number(42));
}

#[test]
fn test_parser_arithmetic_expression() {
    let ast = parse_program("let y be 1 + 2").unwrap();
    assert_eq!(ast.statements().len(), 1);
    let stmt = &ast.statements()[0];
    let (name, value) = match stmt {
        Statement::VariableDeclaration { name, value } => (name, value),
        _ => panic!("Expected variable declaration"),
    };
    assert_eq!(name.value, "y");
    match value {
        Expression::BinaryOperation { left, operator: _, right } => {
            assert_eq!(**left, Expression::Number(1));
            assert_eq!(**right, Expression::Number(2));
        }
        _ => panic!("Expected binary operation"),
    }
}

#[test]
fn test_parser_multiple_statements() {
    let ast = parse_program("let a be 1\nlet b be 2").unwrap();
    assert_eq!(ast.statements().len(), 2);
}

#[test]
fn test_parser_error() {
    let result = parse_program("let be 42");
    assert!(result.is_err());
}

#[test]
fn test_parser_comparison_expressions() {
    let ast = parse_program("let a be 1 == 2").unwrap();
    let stmt = &ast.statements()[0];
    let (name, value) = match stmt {
        Statement::VariableDeclaration { name, value } => (name, value),
        _ => panic!("Expected variable declaration"),
    };
    assert_eq!(name.value, "a");
    match value {
        Expression::BinaryOperation { left, operator, right } => {
            assert_eq!(**left, Expression::Number(1));
            assert_eq!(*operator, BinaryOperator::Equal);
            assert_eq!(**right, Expression::Number(2));
        }
        _ => panic!("Expected binary operation"),
    }

    let ast = parse_program("let b be 3 != 4").unwrap();
    let stmt = &ast.statements()[0];
    let (name, value) = match stmt {
        Statement::VariableDeclaration { name, value } => (name, value),
        _ => panic!("Expected variable declaration"),
    };
    assert_eq!(name.value, "b");
    match value {
        Expression::BinaryOperation { left, operator, right } => {
            assert_eq!(**left, Expression::Number(3));
            assert_eq!(*operator, BinaryOperator::NotEqual);
            assert_eq!(**right, Expression::Number(4));
        }
        _ => panic!("Expected binary operation"),
    }

    let ast = parse_program("let c be 5 < 6").unwrap();
    let stmt = &ast.statements()[0];
    let (name, value) = match stmt {
        Statement::VariableDeclaration { name, value } => (name, value),
        _ => panic!("Expected variable declaration"),
    };
    assert_eq!(name.value, "c");
    match value {
        Expression::BinaryOperation { left, operator, right } => {
            assert_eq!(**left, Expression::Number(5));
            assert_eq!(*operator, BinaryOperator::LessThan);
            assert_eq!(**right, Expression::Number(6));
        }
        _ => panic!("Expected binary operation"),
    }

    let ast = parse_program("let d be 7 >= 8").unwrap();
    let stmt = &ast.statements()[0];
    let (name, value) = match stmt {
        Statement::VariableDeclaration { name, value } => (name, value),
        _ => panic!("Expected variable declaration"),
    };
    assert_eq!(name.value, "d");
    match value {
        Expression::BinaryOperation { left, operator, right } => {
            assert_eq!(**left, Expression::Number(7));
            assert_eq!(*operator, BinaryOperator::GreaterThanOrEqual);
            assert_eq!(**right, Expression::Number(8));
        }
        _ => panic!("Expected binary operation"),
    }
}

#[test]
fn test_parser_logical_operators() {
    let ast = parse_program("let a be 1 and 2").unwrap();
    let stmt = &ast.statements()[0];
    let (name, value) = match stmt {
        Statement::VariableDeclaration { name, value } => (name, value),
        _ => panic!("Expected variable declaration"),
    };
    assert_eq!(name.value, "a");
    match value {
        Expression::BinaryOperation { left, operator, right } => {
            assert_eq!(**left, Expression::Number(1));
            assert_eq!(*operator, BinaryOperator::And);
            assert_eq!(**right, Expression::Number(2));
        }
        _ => panic!("Expected binary operation"),
    }

    let ast = parse_program("let b be 3 or 4").unwrap();
    let stmt = &ast.statements()[0];
    let (name, value) = match stmt {
        Statement::VariableDeclaration { name, value } => (name, value),
        _ => panic!("Expected variable declaration"),
    };
    assert_eq!(name.value, "b");
    match value {
        Expression::BinaryOperation { left, operator, right } => {
            assert_eq!(**left, Expression::Number(3));
            assert_eq!(*operator, BinaryOperator::Or);
            assert_eq!(**right, Expression::Number(4));
        }
        _ => panic!("Expected binary operation"),
    }
}

#[test]
fn test_parser_unary_operators() {
    use navacodelang::ast::expression::UnaryOperator;
    // Unary minus
    let ast = parse_program("let x be -5").unwrap();
    let stmt = &ast.statements()[0];
    let (name, value) = match stmt {
        Statement::VariableDeclaration { name, value } => (name, value),
        _ => panic!("Expected variable declaration"),
    };
    assert_eq!(name.value, "x");
    match value {
        Expression::UnaryOperation { operator, operand } => {
            assert_eq!(*operator, UnaryOperator::Negate);
            assert_eq!(**operand, Expression::Number(5));
        }
        _ => panic!("Expected unary operation"),
    }

    // Nested unary
    let ast = parse_program("let a be - -5").unwrap();
    let stmt = &ast.statements()[0];
    let (_, value) = match stmt {
        Statement::VariableDeclaration { name, value } => (name, value),
        _ => panic!("Expected variable declaration"),
    };
    match value {
        Expression::UnaryOperation { operator, operand } => {
            assert_eq!(*operator, UnaryOperator::Negate);
            match **operand {
                Expression::UnaryOperation { operator: ref op2, operand: ref opnd2 } => {
                    assert_eq!(*op2, UnaryOperator::Negate);
                    assert_eq!(**opnd2, Expression::Number(5));
                }
                _ => panic!("Expected nested unary operation"),
            }
        }
        _ => panic!("Expected unary operation"),
    }

    // Unary operator with grouped expression
    let ast = parse_program("let b be - (2 + 3)").unwrap();
    let stmt = &ast.statements()[0];
    let (_, value) = match stmt {
        Statement::VariableDeclaration { name, value } => (name, value),
        _ => panic!("Expected variable declaration"),
    };
    match value {
        Expression::UnaryOperation { operator, operand } => {
            assert_eq!(*operator, UnaryOperator::Negate);
            match **operand {
                Expression::Grouped(ref inner) => match **inner {
                    Expression::BinaryOperation { ref left, ref operator, ref right } => {
                        assert_eq!(**left, Expression::Number(2));
                        assert_eq!(*operator, BinaryOperator::Add);
                        assert_eq!(**right, Expression::Number(3));
                    }
                    _ => panic!("Expected binary operation inside group"),
                },
                _ => panic!("Expected grouped expression as operand"),
            }
        }
        _ => panic!("Expected unary operation"),
    }
}

#[test]
fn test_parser_grouped_and_precedence() {
    // Grouped expression changes precedence
    let ast = parse_program("let x be (1 + 2) * 3").unwrap();
    let stmt = &ast.statements()[0];
    let (_, value) = match stmt {
        Statement::VariableDeclaration { name, value } => (name, value),
        _ => panic!("Expected variable declaration"),
    };
    match value {
        Expression::BinaryOperation { left, operator, right } => {
            assert_eq!(*operator, BinaryOperator::Multiply);
            match **left {
                Expression::Grouped(ref inner) => match **inner {
                    Expression::BinaryOperation { ref left, ref operator, ref right } => {
                        assert_eq!(**left, Expression::Number(1));
                        assert_eq!(*operator, BinaryOperator::Add);
                        assert_eq!(**right, Expression::Number(2));
                    }
                    _ => panic!("Expected binary operation inside group"),
                },
                _ => panic!("Expected grouped expression as left operand"),
            }
            assert_eq!(**right, Expression::Number(3));
        }
        _ => panic!("Expected binary operation"),
    }

    // Grouped expression as right operand
    let ast = parse_program("let y be 4 / (2 - 1)").unwrap();
    let stmt = &ast.statements()[0];
    let (_, value) = match stmt {
        Statement::VariableDeclaration { name, value } => (name, value),
        _ => panic!("Expected variable declaration"),
    };
    match value {
        Expression::BinaryOperation { left, operator, right } => {
            assert_eq!(*operator, BinaryOperator::Divide);
            assert_eq!(**left, Expression::Number(4));
            match **right {
                Expression::Grouped(ref inner) => match **inner {
                    Expression::BinaryOperation { ref left, ref operator, ref right } => {
                        assert_eq!(**left, Expression::Number(2));
                        assert_eq!(*operator, BinaryOperator::Subtract);
                        assert_eq!(**right, Expression::Number(1));
                    }
                    _ => panic!("Expected binary operation inside group"),
                },
                _ => panic!("Expected grouped expression as right operand"),
            }
        }
        _ => panic!("Expected binary operation"),
    }
}

#[test]
fn test_parser_variable_and_identifier() {
    // Variable as right operand
    let ast = parse_program("let x be y").unwrap();
    let stmt = &ast.statements()[0];
    let (name, value) = match stmt {
        Statement::VariableDeclaration { name, value } => (name, value),
        _ => panic!("Expected variable declaration"),
    };

    let expected_value = Expression::Variable(Token {
        kind: TokenKind::Identifier,
        value: "y".to_string(),
        position: TokenPosition {
            line: 1,
            column: 10,
        },
    });
    assert_eq!(name.value, "x");
    assert_eq!(*value, expected_value);
}

#[test]
fn test_parser_number_literal() {
    let ast = parse_program("let x be 123").unwrap();
    let stmt = &ast.statements()[0];
    let (name, value) = match stmt {
        Statement::VariableDeclaration { name, value } => (name, value),
        _ => panic!("Expected variable declaration"),
    };
    assert_eq!(name.value, "x");
    assert_eq!(*value, Expression::Number(123));
}

#[test]
fn test_parser_error_cases() {
    // Missing right operand
    let result = parse_program("let x be 1 +");
    assert!(result.is_err());
    // Unexpected token
    let result = parse_program("let x be @");
    assert!(result.is_err());
    // Unmatched parenthesis
    let result = parse_program("let x be (1 + 2");
    assert!(result.is_err());
    // Invalid variable declaration
    let result = parse_program("let be 5");
    assert!(result.is_err());
}

#[test]
fn test_parser_variable_assignment() {
    let ast = parse_program("let x be 10\nset x to 20").unwrap();
    assert_eq!(ast.statements().len(), 2);

    // Check variable declaration
    let stmt = &ast.statements()[0];
    let (name, value) = match stmt {
        Statement::VariableDeclaration { name, value } => (name, value),
        _ => panic!("Expected variable declaration"),
    };
    assert_eq!(name.value, "x");
    assert_eq!(*value, Expression::Number(10));

    // Check variable assignment
    let stmt = &ast.statements()[1];
    let (name, value) = match stmt {
        Statement::VariableAssignment { name, value } => (name, value),
        _ => panic!("Expected variable assignment"),
    };
    assert_eq!(name.value, "x");
    assert_eq!(*value, Expression::Number(20));
}
    
// Test for `if` statement parsing
#[test]
fn test_parser_if_statement() {
    let ast = parse_program("if x > 0 then\nset y to 1\nend").unwrap();
    assert_eq!(ast.statements().len(), 1);

    let stmt = &ast.statements()[0];
    match stmt {
        Statement::IfStatement { if_then_branch: if_then_block, else_branch } => {
            // Check condition
            match if_then_block.condition {
                Expression::BinaryOperation { ref left, ref operator, ref right } => {
                    assert_eq!(**left, Expression::Variable(Token { kind: TokenKind::Identifier, value: "x".to_string(), position: TokenPosition { line: 1, column: 4 } }));
                    assert_eq!(*operator, BinaryOperator::GreaterThan);
                    assert_eq!(**right, Expression::Number(0));
                }
                _ => panic!("Expected binary operation in condition"),
            }

            // Check then branch
            match *if_then_block.then_branch {
                Statement::BlockStatement { ref statements } => {
                    assert_eq!(statements.len(), 1);
                    match &statements[0] {
                        Statement::VariableAssignment { name, value } => {
                            assert_eq!(name.value, "y");
                            assert_eq!(*value, Expression::Number(1));
                        }
                        _ => panic!("Expected variable assignment in then branch"),
                    }
                }
                _ => panic!("Expected block statement in then branch"),
            }

            // Check else branch
            assert!(else_branch.is_none());
        }
        _ => panic!("Expected if statement"),
    }
}

#[test]
fn test_parser_if_else_statement() {
    let ast = parse_program("if x > 0 then\nset y to 1\nelse\nset y to -1\nend").unwrap();
    assert_eq!(ast.statements().len(), 1);

    let stmt = &ast.statements()[0];
    match stmt {
        Statement::IfStatement { if_then_branch: if_then_block, else_branch } => {
            // Check condition
            match &if_then_block.condition {
                Expression::BinaryOperation { left, operator, right } => {
                    assert_eq!(**left, Expression::Variable(Token { kind: TokenKind::Identifier, value: "x".to_string(), position: TokenPosition { line: 1, column: 4 } }));
                    assert_eq!(*operator, BinaryOperator::GreaterThan);
                    assert_eq!(**right, Expression::Number(0));
                }
                _ => panic!("Expected binary operation in condition"),
            }

            // Check then branch
            match &*if_then_block.then_branch {
                Statement::BlockStatement { statements } => {
                    assert_eq!(statements.len(), 1);
                    match &statements[0] {
                        Statement::VariableAssignment { name, value } => {
                            assert_eq!(name.value, "y");
                            assert_eq!(*value, Expression::Number(1));
                        }
                        _ => panic!("Expected variable assignment in then branch"),
                    }
                }
                _ => panic!("Expected block statement in then branch"),
            }

            // Check else branch
            match else_branch {
                Some(else_branch) => match &**else_branch {
                    Statement::BlockStatement { statements } => {
                        assert_eq!(statements.len(), 1);
                        match &statements[0] {
                            Statement::VariableAssignment { name, value } => {
                                assert_eq!(name.value, "y");
                                assert_eq!(*value, crate::Expression::UnaryOperation { operator: UnaryOperator::Negate, operand: Box::new(Expression::Number(1)) });
                            }
                            _ => panic!("Expected variable assignment in else branch"),
                        }
                    }
                    _ => panic!("Expected block statement in else branch"),
                }
                None => panic!("Expected else branch"),
            }
        }
        _ => panic!("Expected if statement"),
    }
}

#[test]
fn test_parser_nested_if_statement() {
    let ast = parse_program("if x > 0 then\nif y < 0 then\nset z to 1\nend\nend").unwrap();
    assert_eq!(ast.statements().len(), 1);

    let stmt = &ast.statements()[0];
    match stmt {
        Statement::IfStatement { if_then_branch, else_branch } => {
            // Check outer condition
            match if_then_branch.condition {
                Expression::BinaryOperation { ref left, ref operator, ref right } => {
                    assert_eq!(**left, Expression::Variable(Token { kind: TokenKind::Identifier, value: "x".to_string(), position: TokenPosition { line: 1, column: 4 } }));
                    assert_eq!(*operator, BinaryOperator::GreaterThan);
                    assert_eq!(**right, Expression::Number(0));
                }
                _ => panic!("Expected binary operation in outer condition"),
            }

            // Check outer then branch
            match *if_then_branch.then_branch {
                Statement::BlockStatement { ref statements } => {
                    assert_eq!(statements.len(), 1);
                    match &statements[0] {
                        Statement::IfStatement { if_then_branch, else_branch } => {
                            // Check inner condition
                            match if_then_branch.condition {
                                Expression::BinaryOperation { ref left, ref operator, ref right } => {
                                    assert_eq!(**left, Expression::Variable(Token { kind: TokenKind::Identifier, value: "y".to_string(), position: TokenPosition { line: 2, column: 4 } }));
                                    assert_eq!(*operator, BinaryOperator::LessThan);
                                    assert_eq!(**right, Expression::Number(0));
                                }
                                _ => panic!("Expected binary operation in inner condition"),
                            }

                            // Check inner then branch
                            match *if_then_branch.then_branch {
                                Statement::BlockStatement { ref statements } => {
                                    assert_eq!(statements.len(), 1);
                                    match &statements[0] {
                                        Statement::VariableAssignment { name, value } => {
                                            assert_eq!(name.value, "z");
                                            assert_eq!(*value, Expression::Number(1));
                                        }
                                        _ => panic!("Expected variable assignment in inner then branch"),
                                    }
                                }
                                _ => panic!("Expected block statement in inner then branch"),
                            }

                            // Check inner else branch
                            assert!(else_branch.is_none());
                        }
                        _ => panic!("Expected inner if statement"),
                    }
                }
                _ => panic!("Expected block statement in outer then branch"),
            }

            // Check outer else branch
            assert!(else_branch.is_none());
        }
        _ => panic!("Expected outer if statement"),
    }
}

