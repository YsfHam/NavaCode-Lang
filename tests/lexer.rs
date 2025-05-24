use navacodelang::lexer::{Lexer, TokenKind};

fn lex_all(input: &str) -> Vec<(TokenKind, String)> {
    let mut tokens: Vec<(TokenKind, String)> = Lexer::new(input)
        .map(|t| (t.kind, t.value))
        .collect();
    // Add EndOfFile token at the end if not already present
    if tokens.last().map_or(true, |(k, _)| *k != TokenKind::EndOfFile) {
        tokens.push((TokenKind::EndOfFile, String::new()));
    }
    tokens
}

#[test]
fn test_number() {
    let tokens = lex_all("12345");
    assert_eq!(tokens, vec![
        (TokenKind::Number, "12345".to_string()),
        (TokenKind::EndOfFile, String::new()),
    ]);
}

#[test]
fn test_identifier() {
    let tokens = lex_all("foo bar");
    assert_eq!(tokens, vec![
        (TokenKind::Identifier, "foo".to_string()),
        (TokenKind::Identifier, "bar".to_string()),
        (TokenKind::EndOfFile, String::new()),
    ]);
}

#[test]
fn test_keywords() {
    let tokens = lex_all("let be");
    assert_eq!(tokens, vec![
        (TokenKind::LetKeyword, "let".to_string()),
        (TokenKind::BeKeyword, "be".to_string()),
        (TokenKind::EndOfFile, String::new()),
    ]);
}

#[test]
fn test_operators() {
    let tokens = lex_all("+ - * /");
    assert_eq!(tokens, vec![
        (TokenKind::Plus, "+".to_string()),
        (TokenKind::Minus, "-".to_string()),
        (TokenKind::Star, "*".to_string()),
        (TokenKind::Slash, "/".to_string()),
        (TokenKind::EndOfFile, String::new()),
    ]);
}

#[test]
fn test_unknown() {
    let tokens = lex_all("foo$bar");
    assert_eq!(tokens, vec![
        (TokenKind::Identifier, "foo".to_string()),
        (TokenKind::Unknown, "$".to_string()),
        (TokenKind::Identifier, "bar".to_string()),
        (TokenKind::EndOfFile, String::new()),
    ]);
}

#[test]
fn test_mixed() {
    let tokens = lex_all("let x = 42 + y");
    assert_eq!(tokens, vec![
        (TokenKind::LetKeyword, "let".to_string()),
        (TokenKind::Identifier, "x".to_string()),
        (TokenKind::Unknown, "=".to_string()),
        (TokenKind::Number, "42".to_string()),
        (TokenKind::Plus, "+".to_string()),
        (TokenKind::Identifier, "y".to_string()),
        (TokenKind::EndOfFile, String::new()),
    ]);
}
