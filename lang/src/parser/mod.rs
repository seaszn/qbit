use crate::{
    ast::{expr::Expr, stmt::Stmt},
    lexer::Token,
    parser::debug::ParseWarning,
};
use std::ops::{Deref, Range};

mod builder;
mod config;
mod debug;

pub use builder::ParserBuilder;
pub use config::ParserConfig;
pub use debug::{DebugContext, ParseError};
use inflections::Inflect;

/// Enhanced token with source position information
#[derive(Debug, Clone)]
pub struct TokenSpan {
    pub token: Token,
    pub span: Range<usize>,
}

impl Deref for TokenSpan {
    type Target = Token;

    fn deref(&self) -> &Self::Target {
        &self.token
    }
}

#[derive(Debug)]
pub struct ParseResult {
    statements: Vec<Stmt>,
    warnings: Vec<ParseWarning>,
}

impl ParseResult {
    pub fn statements(&self) -> &[Stmt] {
        &self.statements
    }
    pub fn warnings(&self) -> &[ParseWarning] {
        &self.warnings
    }
}

/// Parser with configuration and safety features
#[derive(Clone)]
pub struct Parser<'a> {
    pub tokens: Vec<TokenSpan>,
    pub config: ParserConfig,
    pub source: &'a str,
    pub pos: usize,
    depth: usize,
}

impl<'a> Parser<'a> {
    pub fn builder(source: &'a str) -> ParserBuilder<'a> {
        ParserBuilder::new(source)
    }

    pub fn parse_src(source: &'a str) -> Result<ParseResult, ParseError> {
        let mut parser = Self::builder(source).build()?;
        parser.parse()
    }

    pub fn parse_expr(source: &'a str) -> Result<Expr, ParseError> {
        let mut parser = Self::builder(source).build()?;
        let expr = parser.safe_call(|p| crate::ast::expr::Expr::parse(p))?;

        match parser.eof() {
            true => Ok(expr),
            false => Err(parser.error("unexpected tokens after expression", Some("end of input"))),
        }
    }

    pub fn parse_stmt(source: &'a str) -> Result<Stmt, ParseError> {
        let mut parser = Self::builder(source).build()?;
        parser.safe_call(|p| Stmt::parse(p))
    }

    fn eof_position(&self) -> usize {
        self.tokens.last().map(|pt| pt.span.end).unwrap_or(0)
    }

    pub fn eof(&self) -> bool {
        self.current().is_none() // Use current() which already skips comments
    }

    pub fn current(&self) -> Option<&TokenSpan> {
        let mut pos = self.pos;
        while let Some(token_span) = self.tokens.get(pos) {
            match token_span.is_comment() {
                true => pos += 1,
                false => return Some(token_span),
            }
        }
        None
    }

    pub fn current_span(&self) -> Option<Range<usize>> {
        self.current().map(|pt| pt.span.clone())
    }

    pub fn current_position(&self) -> usize {
        match self.current() {
            Some(ts) => ts.span.start,
            None => self.eof_position(),
        }
    }

    pub fn peek(&self) -> Option<&Token> {
        self.current().map(|ts| &ts.token)
    }

    pub fn peek_ahead(&self, n: usize) -> Option<&Token> {
        let mut pos = self.pos;
        let mut count = 0;

        while let Some(token_span) = self.tokens.get(pos) {
            match token_span.is_comment() {
                false => match count == n {
                    true => return Some(&token_span.token),
                    false => count += 1,
                },
                true => {}
            }

            pos += 1;

            match pos > self.pos + self.tokens.len() {
                true => break,
                false => {}
            }
        }

        None
    }

    pub fn advance(&mut self) -> Option<&TokenSpan> {
        let start_pos = self.pos;

        loop {
            match self.pos >= self.tokens.len() {
                true => return None,
                false => {}
            }

            let span = &self.tokens[self.pos];
            self.pos += 1;

            match span.is_comment() {
                false => return Some(span),
                true => {}
            }

            match self.pos > start_pos + self.tokens.len() {
                true => return None,
                false => {}
            }
        }
    }

    pub fn consume(&mut self, token: &Token) -> bool {
        match self.peek() == Some(token) {
            true => {
                self.advance();
                true
            }
            false => false,
        }
    }

    pub fn expect(&mut self, expected: Token) -> Result<(), ParseError> {
        let source = self.source;

        match self.advance() {
            Some(token) => match token.token == expected {
                true => Ok(()),
                false => Err(ParseError::UnexpectedToken {
                    expected: Some(format!("{:?}", expected)),
                    found: format!("{:?}", token.token),
                    span: token.span.clone(),
                    context: DebugContext::from_span(source, &token.span),
                }),
            },
            None => {
                let position = self.eof_position();

                Err(ParseError::UnexpectedEof {
                    position,
                    expected: format!("{:?}", expected),
                    context: DebugContext::from_span(source, &(position..position)),
                })
            }
        }
    }

    pub fn parse(&mut self) -> Result<ParseResult, ParseError> {
        let mut statements = Vec::new();
        let mut warnings = vec![];

        while !self.eof() {
            let statement = self.safe_call(|parser| Stmt::parse(parser))?;

            match (self.current(), &statement) {
                (token_span, Stmt::Let { name, .. }) if !name.is_snake_case() => {
                    let span = match token_span.map(|x| &x.span) {
                        Some(res) => res,
                        None => &(self.pos..self.pos),
                    };

                    warnings.push(ParseWarning::NamingConvention {
                        message: format!("expected {}", name.to_snake_case()),
                        span: span.clone(),
                        context: DebugContext::from_span(self.source, span),
                    });
                }
                (token_span, Stmt::Const { name, .. }) if !name.is_constant_case() => {
                    let span = match token_span.map(|x| &x.span) {
                        Some(res) => res,
                        None => &(self.pos..self.pos),
                    };

                    warnings.push(ParseWarning::NamingConvention {
                        message: format!("expected {}", name.to_constant_case()),
                        span: span.clone(),
                        context: DebugContext::from_span(self.source, span),
                    });
                }
                (token_span, Stmt::Function { name, .. }) if !name.is_snake_case() => {
                    let span = match token_span.map(|x| &x.span) {
                        Some(res) => res,
                        None => &(self.pos..self.pos),
                    };

                    warnings.push(ParseWarning::NamingConvention {
                        message: format!("expected {}", name.is_snake_case()),
                        span: span.clone(),
                        context: DebugContext::from_span(self.source, &span),
                    });
                }
                _ => (),
            };

            statements.push(statement);
        }

        let result = ParseResult {
            statements,
            warnings,
        };

        Ok(result)
    }

    pub fn safe_call<T, F>(&mut self, f: F) -> Result<T, ParseError>
    where
        F: FnOnce(&mut Self) -> Result<T, ParseError>,
    {
        self.depth += 1;

        let result = match self.depth > self.config.max_recursion_depth {
            true => Err(ParseError::TooMuchRecursion {
                max_depth: self.config.max_recursion_depth,
                position: self.current_position(),
            }),
            false => f(self),
        };

        self.depth = self.depth.saturating_sub(1);
        result
    }

    fn error(&self, message: &str, expected: Option<&str>) -> ParseError {
        match (self.current(), expected) {
            (Some(token_span), Some(exp)) => ParseError::UnexpectedToken {
                expected: Some(exp.to_string()),
                found: format!("{:?}", token_span.token),
                span: token_span.span.clone(),
                context: DebugContext::from_span(self.source, &token_span.span),
            },
            (Some(token_span), None) => ParseError::InvalidSyntax {
                message: message.to_string(),
                span: token_span.span.clone(),
                context: DebugContext::from_span(self.source, &token_span.span),
            },
            (None, Some(exp)) => {
                let position = self.eof_position();
                ParseError::UnexpectedEof {
                    expected: exp.to_string(),
                    position,
                    context: DebugContext::from_span(self.source, &(position..position)),
                }
            }
            (None, None) => {
                let position = self.eof_position();

                ParseError::UnexpectedEof {
                    expected: "token".to_string(),
                    position,
                    context: DebugContext::from_span(self.source, &(position..position)),
                }
            }
        }
    }

    pub fn expected(&self, expected: &str) -> ParseError {
        self.error("", Some(expected))
    }
}

pub trait Parse: Sized {
    fn parse(parser: &mut Parser) -> Result<Self, ParseError>;
}
