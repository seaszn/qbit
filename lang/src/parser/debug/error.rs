use std::ops::Range;
use thiserror::Error;

use crate::parser::DebugContext;

#[derive(Debug, Clone, Error)]
pub enum ParseError {
    /// Lexer encountered an invalid token
    BuildError {
        message: String,
        invalid_text: String,
        span: Range<usize>,
        context: DebugContext,
    },
    /// Unexpected token during parsing
    UnexpectedToken {
        expected: Option<String>,
        found: String,
        span: Range<usize>,
        context: DebugContext,
    },

    /// Unexpected end of file
    UnexpectedEof {
        expected: String,
        position: usize,
        context: DebugContext,
    },

    /// Invalid syntax
    InvalidSyntax {
        message: String,
        span: Range<usize>,
        context: DebugContext,
    },

    /// Missing required token
    MissingToken {
        expected: String,
        span: Range<usize>,
        source_context: Option<DebugContext>,
    },

    /// Too much recursion (stack overflow prevention)
    TooMuchRecursion { max_depth: usize, position: usize },
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::BuildError {
                message,
                invalid_text,
                context,
                ..
            } => {
                write!(f, "Lexer error: {} ('{}')", message, invalid_text)?;
                write!(f, "\n{context:?}")?;

                Ok(())
            }
            ParseError::UnexpectedToken {
                expected,
                found,
                context,
                ..
            } => {
                match expected {
                    Some(exp) => write!(f, "Expected {}, found {}", exp, found)?,
                    None => write!(f, "Unexpected token {found}")?,
                }

                write!(f, "\n{context}")?;

                Ok(())
            }
            ParseError::UnexpectedEof {
                expected, context, ..
            } => {
                write!(f, "Unexpected end of file, expected {}", expected)?;
                write!(f, "\n{context}")?;

                Ok(())
            }
            ParseError::InvalidSyntax {
                message, context, ..
            } => {
                write!(f, "Syntax error: {}", message)?;
                write!(f, "\n{context}")?;

                Ok(())
            }
            ParseError::MissingToken {
                expected,
                source_context,
                ..
            } => {
                write!(f, "Missing {}", expected)?;

                if let Some(context) = source_context {
                    write!(f, "\n{context}")?;
                }

                Ok(())
            }
            ParseError::TooMuchRecursion {
                max_depth,
                position,
            } => {
                write!(
                    f,
                    "Maximum recursion depth ({max_depth}) exceeded at position {position}"
                )
            }
        }
    }
}