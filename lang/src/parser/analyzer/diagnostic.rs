use crate::parser::ParseWarning;

use super::ParseError;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[repr(u8)]
pub enum DiagnosticLevel {
    Error = 0,
    Warn = 1,
    Info = 2,
    Hint = 3,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Diagnostic {
    line: usize,
    length: usize,
    column: usize,
    message: String,
    level: DiagnosticLevel,
}

impl From<ParseError> for Diagnostic {
    fn from(value: ParseError) -> Self {
        match &value {
            ParseError::BuildError { span, context, .. } => Diagnostic {
                level: DiagnosticLevel::Error,
                message: format!("{value}"),
                line: context.line_number,
                column: context.column_start,
                length: span.end - span.start,
            },
            ParseError::UnexpectedToken { span, context, .. } => Diagnostic {
                level: DiagnosticLevel::Error,
                message: format!("{value}"),
                line: context.line_number,
                column: context.column_start,
                length: span.end - span.start,
            },
            ParseError::UnexpectedEof { context, .. } => Diagnostic {
                level: DiagnosticLevel::Error,
                message: format!("{value}"),
                line: context.line_number,
                column: context.column_start,
                length: 1,
            },
            ParseError::InvalidSyntax { context, span, .. } => Diagnostic {
                level: DiagnosticLevel::Error,
                message: format!("{value}"),
                line: context.line_number,
                column: context.column_start,
                length: span.end - span.start,
            },
            ParseError::MissingToken {
                span,
                context: source_context,
                ..
            } => Diagnostic {
                level: DiagnosticLevel::Error,
                message: format!("{value}"),
                line: source_context.line_number,
                column: source_context.column_start,
                length: span.end - span.start,
            },
            ParseError::TooMuchRecursion { position, .. } => Diagnostic {
                level: DiagnosticLevel::Error,
                message: format!("{value}"),
                line: *position,
                column: 0,
                length: 1,
            },
        }
    }
}

impl From<ParseWarning> for Diagnostic {
    fn from(value: ParseWarning) -> Self {
        match &value {
            ParseWarning::UnusedVariable { span, context, .. } => Diagnostic {
                level: DiagnosticLevel::Warn,
                message: format!("{value}"),
                line: context.line_number,
                column: context.column_start,
                length: span.end - span.start,
            },
            ParseWarning::UnusedFunction { span, context, .. } => Diagnostic {
                level: DiagnosticLevel::Warn,
                message: format!("{value}"),
                line: context.line_number,
                column: context.column_start,
                length: span.end - span.start,
            },
            ParseWarning::UnreachableCode { span, context } => Diagnostic {
                level: DiagnosticLevel::Warn,
                message: format!("{value}"),
                line: context.line_number,
                column: context.column_start,
                length: span.end - span.start,
            },
            ParseWarning::NamingConvention { span, context, .. } => Diagnostic {
                level: DiagnosticLevel::Warn,
                message: format!("{value}"),
                line: context.line_number,
                column: context.column_start,
                length: span.end - span.start,
            },
        }
    }
}
