use inflections::Inflect;
use std::ops::Range;

use crate::ast::stmt::Stmt;

mod context;
mod diagnostic;
mod error;
mod warning;

pub use context::ParseContext;
pub use diagnostic::Diagnostic;
pub use error::ParseError;
pub use warning::ParseWarning;

pub struct Analyzer<'a> {
    source: &'a str,
    // position: usize,
    diagnostics: Vec<Diagnostic>,
}

impl<'a> Analyzer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            diagnostics: Vec::new(),
        }
    }

    pub fn analyze(&mut self, statement: &Stmt, span: &Range<usize>) {
        match &statement {
            Stmt::Let { name, .. } if !name.is_snake_case() => {
                self.diagnostics.push(
                    ParseWarning::NamingConvention {
                        message: format!("expected '{}'", name.to_snake_case()),
                        span: span.clone(),
                        context: ParseContext::from_span(self.source, span),
                    }
                    .into(),
                );
            }
            Stmt::Const { name, .. } if !name.is_constant_case() => {
                self.diagnostics.push(
                    ParseWarning::NamingConvention {
                        message: format!("expected '{}'", name.to_constant_case()),
                        span: span.clone(),
                        context: ParseContext::from_span(self.source, span),
                    }
                    .into(),
                );
            }
            Stmt::Function { name, body, .. } => {
                if !name.is_snake_case() {
                    self.diagnostics.push(
                        ParseWarning::NamingConvention {
                            message: format!("expected '{}'", name.to_snake_case()),
                            span: span.clone(),
                            context: ParseContext::from_span(self.source, &span),
                        }
                        .into(),
                    );
                }

                self.analyze(&body, span);
            }
            Stmt::Block { statements } => {
                for stmt in statements{
                    self.analyze(stmt, span);
                }
            },
            Stmt::For { init, body, .. } => {
                if let Some(stmt) = init{
                    self.analyze(&stmt, span);
                }

                self.analyze(&body, span);
            }
            _ => (),
        };
    }

    pub fn finalize(self) -> Vec<Diagnostic> {
        self.diagnostics
    }
}
