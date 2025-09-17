use serde::{Deserialize, Serialize};

#[cfg(feature = "wasm")]
use crate::parser::{ParseError, ParseResult};

#[derive(Serialize, Deserialize)]
pub struct VsCodeError {
    pub message: String,
    pub line: usize,
    pub column: usize,
    pub length: usize,
}

#[cfg(feature = "wasm")]
impl From<ParseError> for VsCodeError {
    fn from(value: ParseError) -> Self {
        match &value {
            ParseError::BuildError { span, context, .. } => VsCodeError {
                message: format!("{value}"),
                line: context.line_number,
                column: context.column_start,
                length: span.end - span.start,
            },
            ParseError::UnexpectedToken { span, context, .. } => VsCodeError {
                message: format!("{value}"),
                line: context.line_number,
                column: context.column_start,
                length: span.end - span.start,
            },
            ParseError::UnexpectedEof { context, .. } => VsCodeError {
                message: format!("{value}"),
                line: context.line_number,
                column: context.column_start,
                length: 1,
            },
            ParseError::InvalidSyntax { context, span, .. } => VsCodeError {
                message: format!("{value}"),
                line: context.line_number,
                column: context.column_start,
                length: span.end - span.start,
            },
            ParseError::MissingToken {
                span,
                source_context,
                ..
            } => VsCodeError {
                message: format!("{value}"),
                line: source_context.clone().map(|x| x.line_number).unwrap_or(0),
                column: source_context.clone().map(|x| x.column_start).unwrap_or(0),
                length: span.end - span.start,
            },
            ParseError::TooMuchRecursion { position, .. } => VsCodeError {
                message: format!("{value}"),
                line: *position,
                column: 0,
                length: 1,
            },
        }
    }
}

#[cfg(feature = "wasm")]
#[derive(Serialize, Deserialize)]
pub struct VsCodeResult {
    success: bool,
    errors: Vec<VsCodeError>,
}

#[cfg(feature = "wasm")]
impl From<Result<ParseResult, ParseError>> for VsCodeResult {
    fn from(value: Result<ParseResult, ParseError>) -> Self {
        match value {
            Ok(_) => VsCodeResult {
                success: true,
                // ast: Some(format!("{:#?}", res.statements())),
                errors: vec![],
            },
            Err(error) => {
                VsCodeResult {
                    // ast: None,
                    success: false,
                    errors: vec![VsCodeError::from(error)],
                }
            }
        }
    }
}
