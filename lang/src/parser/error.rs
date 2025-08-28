use std::ops::Range;

use thiserror::Error;

#[derive(Debug, Clone)]
pub struct ErrorContext {
    pub line_number: usize,
    pub column_start: usize,
    pub column_end: usize,
    pub line_content: String,
    pub span_in_line: Range<usize>,
}

impl ErrorContext {
    pub fn from_span(source: &str, span: &Range<usize>) -> Self {
        let lines: Vec<&str> = source.lines().collect();
        let mut current_pos = 0;

        for (line_num, line) in lines.iter().enumerate() {
            let line_start = current_pos;
            let line_end = current_pos + line.len();

            if span.start >= line_start && span.start <= line_end {
                let col_start = span.start - line_start;
                let col_end = (span.end - line_start).min(line.len());

                return Self {
                    line_number: line_num + 1,
                    column_start: col_start + 1,
                    column_end: col_end + 1,
                    line_content: line.to_string(),
                    span_in_line: col_start..col_end,
                };
            }

            current_pos = line_end + 1;
        }

        Self {
            line_number: lines.len(),
            column_start: 1,
            column_end: 1,
            line_content: lines.last().unwrap_or(&"").to_string(),
            span_in_line: 0..0,
        }
    }
}

impl std::fmt::Display for ErrorContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.span_in_line.is_empty() {
            true => write!(
                f,
                "{}:{}: {}",
                self.line_number, self.column_start, self.line_content
            ),
            false => {
                let caret_line = format!(
                    "{}{}",
                    " ".repeat(self.span_in_line.start),
                    "^".repeat((self.span_in_line.end - self.span_in_line.start).max(1))
                );

                write!(
                    f,
                    "{}:{}-{}: {}\n{}",
                    self.line_number,
                    self.column_start,
                    self.column_end,
                    self.line_content,
                    caret_line
                )
            }
        }
    }
}

// #[derive(Debug, Clone)]
// pub struct BuildError {
//     pub(super) message: String,
//     pub(super) invalid_text: String,
//     pub(super) span: Range<usize>,
//     pub(super) context: ErrorContext,
// }

// impl std::fmt::Display for BuildError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "Lexer error: {} ('{}')", self.message, self.invalid_text)?;
//         write!(f, "\n{}", self.context)?;

//         Ok(())
//     }
// }

// impl std::error::Error for BuildError {}

#[derive(Debug, Clone, Error)]
pub enum ParseError {
    /// Lexer encountered an invalid token
    BuildError {
        message: String,
        invalid_text: String,
        span: Range<usize>,
        context: ErrorContext,
    },
    /// Unexpected token during parsing
    UnexpectedToken {
        expected: Option<String>,
        found: String,
        span: Range<usize>,
        context: ErrorContext,
    },

    /// Unexpected end of file
    UnexpectedEof {
        expected: String,
        position: usize,
        context: ErrorContext,
    },

    /// Invalid syntax
    InvalidSyntax {
        message: String,
        span: Range<usize>,
        context: ErrorContext,
    },

    /// Missing required token
    MissingToken {
        expected: String,
        span: Range<usize>,
        source_context: Option<ErrorContext>,
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