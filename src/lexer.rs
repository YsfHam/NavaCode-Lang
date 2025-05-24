use std::{iter::Peekable, str::Chars};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenKind {
    // Literals
    Number,

    // Keywords
    LetKeyword,
    BeKeyword,

    // Operators
    Plus,
    Minus,
    Star,
    Slash,

    Identifier,

    Unknown,

    EndOfFile,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TokenPosition {
    line: usize,
    column: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub value: String,
    pub position: TokenPosition,
}

pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
    position: TokenPosition,
    is_eof_encountered: bool,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            input: input.chars().peekable(),
            position: TokenPosition { line: 1, column: 1 },
            is_eof_encountered: false,
        }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        // Skip whitespace and update position
        self.handle_whitespaces();

        let next_char_opt = self.peek();

        if next_char_opt.is_none() {
            return self.end_of_file_token();
        }
        let next_char = *next_char_opt.unwrap();

        if next_char.is_ascii_digit() {
            return Some(self.number_token());
        }
        
       return 
        self.operator_token()
        .or_else(|| self.identifier_token())
        .or_else(|| self.unknown_token());
       
    }

    fn advance(&mut self) -> char {
        let c = self.input.next().unwrap();
        if c == '\n' {
            self.position.line += 1;
            self.position.column = 1;
        } else {
            self.position.column += 1;
        }
        c
    }

    fn peek(&mut self) -> Option<&char> {
        self.input.peek()
    }

    fn handle_whitespaces(&mut self) {

        while self.peek().is_some_and(|c| c.is_whitespace()) {
            self.advance();
        }
    }

    fn number_token(&mut self) -> Token {
        let mut number = String::new();
        let start_pos = self.position.clone();
        while let Some(&c) = self.peek().filter(|c| c.is_ascii_digit()) {
            number.push(c);
            self.advance();
        }
        Token {
            kind: TokenKind::Number,
            value: number,
            position: start_pos,
        }
    }

    fn operator_token(&mut self) -> Option<Token> {
        let start_pos = self.position.clone();
        let mut operator = String::new();
        let mut last_operator_kind= None;
        while let Some(&c) = self.peek() {
            operator.push(c);
            if let Some(operator_kind) = self.match_operator(&operator) {
                last_operator_kind = Some(operator_kind);
                self.advance();
            }
            else {
                operator.pop();
                break;
            }
        }

        last_operator_kind
        .map(|op_kind| Token { 
            kind: op_kind,
            value: operator,
            position: start_pos 
        })
    }

    fn match_operator(&self, operator: &str) -> Option<TokenKind> {
        match operator {
            "+" => Some(TokenKind::Plus),
            "-" => Some(TokenKind::Minus),
            "*" => Some(TokenKind::Star),
            "/" => Some(TokenKind::Slash),
            _ => None
        }
    }

    fn identifier_token(&mut self) -> Option<Token> {
        let mut identifier = String::new();
        let start_pos = self.position.clone();
        if let Some(&c) = self.peek().filter(|c| c.is_alphabetic() || **c == '_') {
            identifier.push(c);
            self.advance();
        } 
        else {
            return None;
        }
        while let Some(&c) = self.peek().filter(|c| c.is_alphanumeric()) {
            identifier.push(c);
            self.advance();
        }


        Some(Token {
            kind: self.match_identifier_with_keyword(&identifier),
            value: identifier,
            position: start_pos,
        })
    }

    fn match_identifier_with_keyword(&self, identifier: &str) -> TokenKind {
        match identifier {
            "let" => TokenKind::LetKeyword,
            "be" => TokenKind::BeKeyword,
            _ => TokenKind::Identifier,
        }
    }

    // Group consecutive unknown characters into a single Unknown token
    fn unknown_token(&mut self) -> Option<Token> {
        let mut unknown = String::new();
        let start_pos = self.position.clone();
        
        while let Some(&c) = self.peek() {
            if !self.is_char_known(c) {
                unknown.push(c);
                self.advance();
            } 
            else {
                break;
            }
        }
        if !unknown.is_empty() {
            Some(Token {
                kind: TokenKind::Unknown,
                value: unknown,
                position: start_pos,
            })
        } 
        else {
            None
        }
    }

    fn end_of_file_token(&mut self) -> Option<Token> {

        if self.is_eof_encountered {
            return None;
        }

        self.is_eof_encountered = true;
        Some(Token {
            kind: TokenKind::EndOfFile,
            value: String::new(),
            position: self.position.clone(),
        })
    }

    fn is_char_known(&self, c: char) -> bool {
        c.is_ascii_digit() || 
        c.is_alphabetic()  || 
        c.is_whitespace()  ||
        ['-', '+', '-', '*', '/'].contains(&c)
    }
}

// Implement Iterator for Lexer
impl<'a> Iterator for Lexer<'a> {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}