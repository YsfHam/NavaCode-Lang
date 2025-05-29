use std::fmt;

use crate::lexer::{Token, TokenKind, TokenPosition};


#[derive(Debug)]
enum DiagnosticError {
    UnexpectedToken {
        expected: Vec<TokenKind>,
        found: String,
    },
    UnexpectedElseAfterEnd,
    UnexpectedEndToken,
    UnexpectedElseToken,
}

impl fmt::Display for DiagnosticError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DiagnosticError::UnexpectedToken { expected, found } => {
                        let expected_str = 
                            expected.iter()
                            .map(|k| format!("{}", k))
                            .collect::<Vec<_>>()
                            .join(", ");
                        write!(f, "Unexpected token '{}'. expected one of [{}]", found, expected_str)
                    }
            DiagnosticError::UnexpectedElseAfterEnd => {
                        write!(f, "Unexpected 'else' after 'end'")
                    }
            DiagnosticError::UnexpectedEndToken => {
                        write!(f, "'end' present without a matching block")
                    }
            DiagnosticError::UnexpectedElseToken => write!(f, "'else' present without a matching 'if'"),
        }
    }
}

#[derive(Debug)]
enum DiagnosticType {
    Error(DiagnosticError),
}

#[derive(Debug)]
pub struct Diagnostic {
    diagnostic_type: DiagnosticType,
    position: TokenPosition,
}

impl Diagnostic {
    pub fn unexpected_token(expected: Vec<TokenKind>, found: Token) -> Self {
        Self {
            diagnostic_type: DiagnosticType::Error(DiagnosticError::UnexpectedToken {
                expected,
                found: found.value,
            }),
            position: found.position,
        }
    }

    pub fn unexpected_else_after_end(position: TokenPosition) -> Self {
        Self {
            diagnostic_type: DiagnosticType::Error(DiagnosticError::UnexpectedElseAfterEnd),
            position,
        }
    } 

    pub fn unexpected_end_token(position: TokenPosition) -> Self {
        Self {
            diagnostic_type: DiagnosticType::Error(DiagnosticError::UnexpectedEndToken),
            position,
        }
    }

    pub fn unexpected_else_token(position: TokenPosition) -> Self {
        Self {
            diagnostic_type: DiagnosticType::Error(DiagnosticError::UnexpectedElseToken),
            position,
        }
    }
}

impl fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.diagnostic_type {
            DiagnosticType::Error(err) => {
                write!(f, "ERROR: at {}:{}: {}", self.position.line, self.position.column, err)
            }
        }
    }
}

#[derive(Debug)]
pub struct Diagnostics {
    pub diagnostics: Vec<Diagnostic>,
}

impl Diagnostics {
    pub fn new() -> Self {
        Self { diagnostics: Vec::new() }
    }

    pub fn report(&mut self, diag: Diagnostic) {
        self.diagnostics.push(diag);
    }

    pub fn has_errors(&self) -> bool {
        self.diagnostics.iter().any(|d| matches!(d.diagnostic_type, DiagnosticType::Error(_)))
    }
}

impl fmt::Display for Diagnostics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        for diag in &self.diagnostics {
            writeln!(f, "{}", diag)?;
        }
        Ok(())
    }
}