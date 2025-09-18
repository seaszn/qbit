// lang/src/parser/warning.rs

use super::ParseContext;
use std::ops::Range;
use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum ParseWarning {
    /// Variable declared but never used
    UnusedVariable {
        name: String,
        span: Range<usize>,
        context: ParseContext,
    },

    /// Function declared but never used
    UnusedFunction {
        name: String,
        span: Range<usize>,
        context: ParseContext,
    },

    /// Code after return statement
    UnreachableCode {
        span: Range<usize>,
        context: ParseContext,
    },

    /// Naming convention violation
    NamingConvention {
        message: String,
        span: Range<usize>,
        context: ParseContext,
    },
}

impl std::fmt::Display for ParseWarning {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseWarning::UnusedVariable { name, context, .. } => {
                write!(f, "Variable '{}' is declared but never used", name)?;
                write!(f, "\n{}", context)?;
                Ok(())
            }
            ParseWarning::UnusedFunction { name, context, .. } => {
                write!(f, "Function '{}' is declared but never used", name)?;
                write!(f, "\n{}", context)?;
                Ok(())
            }
            ParseWarning::UnreachableCode { context, .. } => {
                write!(f, "Unreachable code")?;
                write!(f, "\n{}", context)?;
                Ok(())
            }
            ParseWarning::NamingConvention {
                message, context, ..
            } => {
                write!(f, "{}", message)?;
                write!(f, "\n{}", context)?;
                Ok(())
            }
        }
    }
}
