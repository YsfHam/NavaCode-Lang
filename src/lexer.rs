use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum TokenKind {
    // Literals
    Number,

    // Keywords
    LetKeyword,
    BeKeyword,
    AndKeyword,
    OrKeyword,
    NotKeyword,
    SetKeyword,
    ToKeyword,
    IfKeyword,
    ThenKeyword,
    EndKeyword,
    ElseKeyword,
    TrueKeyword,
    FalseKeyword,

    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    EqualEqual,
    NotEqual,
    LessThan,
    GreaterThan,
    LessThanOrEqual,
    GreaterThanOrEqual,
    Bang,

    LeftParen,
    RightParen,

    Identifier,

    Unknown,

    EndOfFile,
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            TokenKind::Number => "Number",
            TokenKind::LetKeyword => "let",
            TokenKind::BeKeyword => "be",
            TokenKind::AndKeyword => "and",
            TokenKind::OrKeyword => "or",
            TokenKind::Plus => "+",
            TokenKind::Minus => "-",
            TokenKind::Star => "*",
            TokenKind::Slash => "/",
            TokenKind::EqualEqual => "==",
            TokenKind::NotEqual => "!=",
            TokenKind::LessThan => "<",
            TokenKind::GreaterThan => ">",
            TokenKind::LessThanOrEqual => "<=",
            TokenKind::GreaterThanOrEqual => ">=",
            TokenKind::LeftParen => "(",
            TokenKind::RightParen => ")",
            TokenKind::Identifier => "Identifier",
            TokenKind::Unknown => "Unknown",
            TokenKind::EndOfFile => "EndOfFile",
            TokenKind::NotKeyword => "not",
            TokenKind::Bang => "!",
            TokenKind::SetKeyword => "set",
            TokenKind::ToKeyword => "to",
            TokenKind::IfKeyword => "if",
            TokenKind::ThenKeyword => "then",
            TokenKind::EndKeyword => "end",
            TokenKind::ElseKeyword => "else",
            TokenKind::TrueKeyword => "true",
            TokenKind::FalseKeyword => "false",
        };
        write!(f, "{s}")
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TokenPosition {
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub value: String,
    pub position: TokenPosition,
}


static OPERATORS: &[(&str, TokenKind)] = &[
    ("+", TokenKind::Plus), 
    ("-", TokenKind::Minus), 
    ("*", TokenKind::Star), 
    ("/", TokenKind::Slash),
    ("==", TokenKind::EqualEqual),
    ("!=", TokenKind::NotEqual),
    ("<", TokenKind::LessThan),
    (">", TokenKind::GreaterThan),
    ("<=", TokenKind::LessThanOrEqual),
    (">=", TokenKind::GreaterThanOrEqual),
    ("!", TokenKind::Bang),
];

pub struct Lexer<'a> {
    input: LexerInputBuffer<'a>,
    position: TokenPosition,
    is_eof_encountered: bool,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            input: LexerInputBuffer::new(input),
            position: TokenPosition { line: 1, column: 1 },
            is_eof_encountered: false,
        }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        self.handle_whitespaces();
        let next_char_opt = self.peek();
        if next_char_opt.is_none() {
            return self.end_of_file_token();
        }
        let next_char = next_char_opt.unwrap();
        if next_char.is_ascii_digit() {
            return Some(self.number_token());
        }
        return self.operator_token()
            .or_else(|| self.symbol_token())
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

    fn unadvance(&mut self, count: usize) {

        if count == 0 {
            return; // No need to rewind if count is zero
        }
        // Rewind the input buffer
        self.input.rewind(count);

        // Recompute line and column by scanning from the start up to the current position
        let mut line = 1;
        let mut col = 1;
        for c in self.input.start().chars() {
            if c == '\n' {
            line += 1;
            col = 1;
            } else {
            col += 1;
            }
        }
        self.position.line = line;
        self.position.column = col;
        }

    fn peek(&mut self) -> Option<char> {
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
        while self.peek().is_some_and(|c| c.is_ascii_digit()) {
            number.push(self.advance());
        }
        Token {
            kind: TokenKind::Number,
            value: number,
            position: start_pos,
        }
    }

    fn operator_token(&mut self) -> Option<Token> {
        let start_pos = self.position.clone();
        let mut op = String::new();
        let mut last_valid_kind = None;
        let mut last_valid_len = 0;
        let mut chars_consumed = 0;
        while let Some(c) = self.peek() {
            op.push(c);
            if let Some(kind) = self.match_operator(&op) {
                last_valid_kind = Some(kind);
                last_valid_len = op.len();
            }
            if OPERATORS.iter().any(|(s, _)| s.starts_with(&op)) {
                self.advance();
                chars_consumed += 1;
            } 
            else {
                break;
            }
        }
        if let Some(kind) = last_valid_kind {
            if last_valid_len < chars_consumed {
                self.unadvance(chars_consumed - last_valid_len);
            }
            let value = op[..last_valid_len].to_string();
            Some(Token {
                kind,
                value,
                position: start_pos,
            })
        } 
        else {
            self.unadvance(chars_consumed);
            None
        }
    }

    fn match_operator(&self, operator: &str) -> Option<TokenKind> {
        OPERATORS.iter().find(|(op_str, _)| *op_str == operator).map(|(_, kind)| *kind)
    }

    fn symbol_token(&mut self) -> Option<Token> {
        let start_pos = self.position.clone();
        if let Some(c) = self.peek() {
            self.match_symbol(c).map(|kind| {
                self.advance();
                Token {
                    kind,
                    value: c.to_string(),
                    position: start_pos,
                }
            })
        } else {
            None
        }
    }

    fn identifier_token(&mut self) -> Option<Token> {
        let mut identifier = String::new();
        let start_pos = self.position.clone();
        if self.peek().is_some_and(|c| c.is_alphabetic() || c == '_') {
            identifier.push(self.advance());
        } else {
            return None;
        }
        while self.peek().is_some_and(|c| c.is_alphanumeric() || c == '_') {
            identifier.push(self.advance());
        }
        Some(Token {
            kind: self.match_identifier_or_keyword(&identifier),
            value: identifier,
            position: start_pos,
        })
    }

    // Group consecutive unknown characters into a single Unknown token
    fn unknown_token(&mut self) -> Option<Token> {
        let mut unknown = String::new();
        let start_pos = self.position.clone();
        while let Some(c) = self.peek() {
            if !self.is_char_known(c) {
                unknown.push(self.advance());
            } else {
                break;
            }
        }
        if !unknown.is_empty() {
            Some(Token {
                kind: TokenKind::Unknown,
                value: unknown,
                position: start_pos,
            })
        } else {
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
            value: "EOF".to_string(),
            position: self.position.clone(),
        })
    }

    fn is_char_known(&self, c: char) -> bool {
        c.is_ascii_digit()
            || c.is_alphabetic()
            || c.is_whitespace()
            || self.match_symbol(c).is_some()
            || OPERATORS.iter().any(|(op_str, _)| **op_str == c.to_string())
    }

    // Helper to match symbol
    fn match_symbol(&self, c: char) -> Option<TokenKind> {
        match c {
            '(' => Some(TokenKind::LeftParen),
            ')' => Some(TokenKind::RightParen),
            _ => None,
        }
    }

    // Helper to match identifier or keyword
    fn match_identifier_or_keyword(&self, identifier: &str) -> TokenKind {
        match identifier {
            "let" => TokenKind::LetKeyword,
            "be" => TokenKind::BeKeyword,
            "and" => TokenKind::AndKeyword,
            "or" => TokenKind::OrKeyword,
            "not" => TokenKind::NotKeyword,
            "set" => TokenKind::SetKeyword,
            "to" => TokenKind::ToKeyword,
            "if" => TokenKind::IfKeyword,
            "then" => TokenKind::ThenKeyword,
            "end" => TokenKind::EndKeyword,
            "else" => TokenKind::ElseKeyword,
            "true" => TokenKind::TrueKeyword,
            "false" => TokenKind::FalseKeyword,
            _ => TokenKind::Identifier,
        }
    }
}

// Implement Iterator for Lexer
impl<'a> Iterator for Lexer<'a> {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}

struct LexerInputBuffer<'a> {
    input: &'a str,
    position: usize,
}


impl LexerInputBuffer<'_> {
    fn new(input: &str) -> LexerInputBuffer {
        LexerInputBuffer {
            input,
            position: 0,
        }
    }

    fn next(&mut self) -> Option<char> {
        if self.position < self.input.len() {
            let c = self.input[self.position..].chars().next()?;
            self.position += c.len_utf8();
            Some(c)
        } else {
            None
        }
    }

    fn peek(&self) -> Option<char> {
        self.input[self.position..].chars().next()
    }

    fn rewind(&mut self, count: usize) {
        if count > self.position {
            self.position = 0;
        } else {
            let mut chars = self.input[..self.position].chars();
            chars.nth_back(count - 1);
            self.position = chars.as_str().len();
        }
    }

    fn start(&self) -> &str {
        &self.input[self.position..]
    }
}