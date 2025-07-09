use std::fmt;

use crate::{ast::expression::{BinaryOperator, UnaryOperator}, lexer::{TextSpan, Token, TokenKind}, types::Type};


#[derive(Debug)]
enum DiagnosticError {

    UnexpectedToken {
        expected: Vec<TokenKind>,
        found: String,
    },
    UnexpectedElseAfterEnd,
    UnexpectedEndToken,
    UnexpectedElseToken,

    VariableRedefinition {
        identifier: String,
    },

    UndefinedVariable {
        identifier: String,
    },

    FunctionArgumentsMismatch {
        function_name: String,
        expected: usize,
        found: usize,
    },

    UndefinedFunction {
        function_name: String,
    },

    ReturnOutsideFunction,

    VariableTypeMismatch {
        identifier: String,
        expected_type: Type,
        found_type: Type,
    },

    ExpressionTypeMismatch {
        expected_type: Type,
        found_type: Type,
    },

    IncompatibleBinaryOperation {
        left_type: Type,
        right_type: Type,
        operator: BinaryOperator,
    },

    IncompatibleUnaryOperation {
        operand_type: Type,
        operator: UnaryOperator,
    },
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
            DiagnosticError::VariableRedefinition { identifier } => write!(f, "Variable '{}' is already defined in the current scope", identifier),
            DiagnosticError::UndefinedVariable { identifier } => write!(f, "Variable '{}' is not defined", identifier),
            DiagnosticError::FunctionArgumentsMismatch { function_name, expected, found } => write!(f, "Function '{}' called with incorrect number of arguments: expected {}, found {}", function_name, expected, found),
            DiagnosticError::UndefinedFunction { function_name } => write!(f, "Function '{}' is not defined", function_name),
            DiagnosticError::ReturnOutsideFunction => write!(f, "Return statement outside of function"),
            DiagnosticError::VariableTypeMismatch { identifier, expected_type, found_type } => {
                                        write!(f, "Type mismatch for variable '{}': expected '{}', found '{}'", identifier, expected_type, found_type)
                                    },
            DiagnosticError::ExpressionTypeMismatch { expected_type, found_type } => {
                                        write!(f, "Type mismatch in expression: expected '{}', found '{}'", expected_type, found_type)
                                    },
            DiagnosticError::IncompatibleBinaryOperation { left_type, right_type, operator } => {
                        write!(f, "Incompatible binary operation: '{}' (left: '{}', right: '{}')", operator, left_type, right_type)
                    },
            DiagnosticError::IncompatibleUnaryOperation { operand_type, operator } => {
                        write!(f, "Incompatible unary operation: '{}' (operand: '{}')", operator, operand_type)
                    },
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
    span: TextSpan,
}

impl Diagnostic {
    pub fn unexpected_token(expected: Vec<TokenKind>, found: Token) -> Self {
        let span = found.span();
        Self {
            diagnostic_type: DiagnosticType::Error(DiagnosticError::UnexpectedToken {
                expected,
                found: found.value,
            }),
            span
        }
    }

    pub fn unexpected_else_after_end(span: TextSpan) -> Self {
        Self {
            diagnostic_type: DiagnosticType::Error(DiagnosticError::UnexpectedElseAfterEnd),
            span,
        }
    } 

    pub fn unexpected_end_token(span: TextSpan) -> Self {
        Self {
            diagnostic_type: DiagnosticType::Error(DiagnosticError::UnexpectedEndToken),
            span,
        }
    }

    pub fn unexpected_else_token(span: TextSpan) -> Self {
        Self {
            diagnostic_type: DiagnosticType::Error(DiagnosticError::UnexpectedElseToken),
            span,
        }
    }

    pub fn variable_redefinition(variable: Token) -> Self {
        let span = variable.span();
        Self {
            diagnostic_type: DiagnosticType::Error(DiagnosticError::VariableRedefinition { identifier: variable.value }),
            span,
        }
    }

    pub fn undefined_variable(variable: Token) -> Self {
        let span= variable.span();
        Self {
            diagnostic_type: DiagnosticType::Error(DiagnosticError::UndefinedVariable { identifier: variable.value }),
            span,
        }
    }

    pub fn function_arguments_mismatch(function_name: Token, expected: usize, found: usize) -> Self {
        let span = function_name.span();
        Self {
            diagnostic_type: DiagnosticType::Error(DiagnosticError::FunctionArgumentsMismatch {
                function_name: function_name.value,
                expected,
                found,
            }),
            span,
        }
    }

    pub fn undefined_function(function_name: Token) -> Self {
        let span = function_name.span();
        Self {
            diagnostic_type: DiagnosticType::Error(DiagnosticError::UndefinedFunction {
                function_name: function_name.value,
            }),
            span,
        }
    }

    pub fn return_outside_function(span: TextSpan) -> Self {
        Self {
            diagnostic_type: DiagnosticType::Error(DiagnosticError::ReturnOutsideFunction),
            span,
        }
    }

    pub fn variable_type_mismatch(variable: Token, expected_type: Type, found_type: Type) -> Self {
        let span = variable.span();
        Self {
            diagnostic_type: DiagnosticType::Error(DiagnosticError::VariableTypeMismatch {
                identifier: variable.value,
                expected_type,
                found_type,
            }),
            span,
        }
    }

    pub fn expression_type_mismatch(expected_type: Type, found_type: Type, span: TextSpan) -> Self {
        Self {
            diagnostic_type: DiagnosticType::Error(DiagnosticError::ExpressionTypeMismatch {
                expected_type,
                found_type,
            }),
            span,
        }
    }

    pub fn incompatible_binary_operation(left_type: Type, right_type: Type, operator: BinaryOperator, span: TextSpan) -> Self {
        Self {
            diagnostic_type: DiagnosticType::Error(DiagnosticError::IncompatibleBinaryOperation {
                left_type,
                right_type,
                operator,
            }),
            span,
        }
    }

    pub fn incompatible_unary_operation(operand_type: Type, operator: UnaryOperator, span: TextSpan) -> Self {
        Self {
            diagnostic_type: DiagnosticType::Error(DiagnosticError::IncompatibleUnaryOperation {
                operand_type,
                operator,
            }),
            span,
        }
    }
}

impl fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.diagnostic_type {
            DiagnosticType::Error(err) => {
                write!(f, "ERROR: at {}:{}: {}", self.span.start.line, self.span.start.column, err)
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