use navacodelang::lexer::{Lexer, TokenKind};
use navacodelang::parser::Parser;
use navacodelang::ast::{Ast, statement::Statement, expression::Expression};
use navacodelang::ast::expression::BinaryOperator;

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
        (TokenKind::And, "and".to_string()),
        (TokenKind::Or, "or".to_string()),
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

