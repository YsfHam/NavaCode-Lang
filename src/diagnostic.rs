use std::fmt;

use crate::lexer::{Token, TokenKind};

#[derive(Debug)]
pub struct Diagnostic {
    pub errors: Vec<Error>,
}

impl Diagnostic {
    pub fn new() -> Self {
        Diagnostic { errors: Vec::new() }
    }

    pub fn report_error(&mut self, error: Error) {
        self.errors.push(error);
    }

    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }
}

impl fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        for error in &self.errors {
            writeln!(f, "{}", error)?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum Error {
    UnexpectedToken {
        expected: Vec<TokenKind>,
        found: Token,
    },
}

// You will need to implement Display for TokenKindType for this to work well.
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::UnexpectedToken { expected, found } => {
                let expected_str = expected
                    .iter()
                    .map(|kind| format!("{}", kind))
                    .collect::<Vec<_>>()
                    .join(", ");
                write!(
                    f,
                    "{}:{} - ERROR: Unexpected token: '{}'. Expected one of: [{}]",
                    found.position.line, found.position.column,
                    found.value, expected_str
                )
            }
        }
    }
}