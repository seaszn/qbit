use crate::{
    ast::{expr::Expr, stmt::Stmt},
    lexer::Token,
    parser::analyzer::Analyzer,
};
use std::ops::{Deref, Range};

mod analyzer;
mod builder;
mod config;

pub use analyzer::{Diagnostic, ParseContext, ParseError, ParseWarning};
pub use builder::ParserBuilder;
pub use config::ParserConfig;

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
    diagnostics: Vec<Diagnostic>,
}

impl ParseResult {
    pub fn statements(&self) -> &[Stmt] {
        &self.statements
    }

    pub fn diagnositcs(&self) -> &[Diagnostic] {
        &self.diagnostics
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
    fn span(&self) -> Option<&TokenSpan> {
        let mut pos = self.pos;

        while let Some(token_span) = self.tokens.get(pos) {
            match token_span.is_comment() {
                true => pos += 1,
                false => return Some(token_span),
            }
        }
        None
    }

    fn parse(&mut self) -> Result<ParseResult, ParseError> {
        let mut statements: Vec<Stmt> = vec![];
        let mut analyzer = Analyzer::new(self.source);

        while !self.eof() {
            let span = match self.span().map(|x| &x.span) {
                Some(res) => res.clone(),
                None => self.pos..self.pos,
            };

            let statement = self.safe_call(|parser| Stmt::parse(parser))?;

            analyzer.analyze(&statement, &span);
            statements.push(statement);
        }

        let diagnostics = analyzer.finalize();

        Ok(ParseResult {
            diagnostics,
            statements,
        })
    }

    pub(crate) fn eof(&self) -> bool {
        self.span().is_none() // Use current() which already skips comments
    }

    pub(crate) fn eof_position(&self) -> usize {
        self.tokens.last().map(|pt| pt.span.end).unwrap_or(0)
    }

    pub(crate) fn peek(&self) -> Option<&Token> {
        self.span().map(|ts| &ts.token)
    }

    pub(crate) fn error(&self, message: &str, expected: Option<&str>) -> ParseError {
        match (self.span(), expected) {
            (Some(token_span), Some(exp)) => ParseError::UnexpectedToken {
                expected: Some(exp.to_string()),
                found: format!("{:?}", token_span.token),
                span: token_span.span.clone(),
                context: ParseContext::from_span(self.source, &token_span.span),
            },
            (Some(token_span), None) => ParseError::InvalidSyntax {
                message: message.to_string(),
                span: token_span.span.clone(),
                context: ParseContext::from_span(self.source, &token_span.span),
            },
            (None, Some(exp)) => {
                let position = self.eof_position();
                ParseError::UnexpectedEof {
                    expected: exp.to_string(),
                    position,
                    context: ParseContext::from_span(self.source, &(position..position)),
                }
            }
            (None, None) => {
                let position = self.eof_position();

                ParseError::UnexpectedEof {
                    expected: "token".to_string(),
                    position,
                    context: ParseContext::from_span(self.source, &(position..position)),
                }
            }
        }
    }

    pub(crate) fn advance(&mut self) -> Option<&TokenSpan> {
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

    pub(crate) fn consume(&mut self, token: &Token) -> bool {
        match self.peek() == Some(token) {
            true => {
                self.advance();
                true
            }
            false => false,
        }
    }

    pub(crate) fn expect(&mut self, expected: Token) -> Result<(), ParseError> {
        let source = self.source;

        match self.advance() {
            Some(token) => match token.token == expected {
                true => Ok(()),
                false => Err(ParseError::UnexpectedToken {
                    expected: Some(format!("{:?}", expected)),
                    found: format!("{:?}", token.token),
                    span: token.span.clone(),
                    context: ParseContext::from_span(source, &token.span),
                }),
            },
            None => {
                let position = self.eof_position();

                Err(ParseError::UnexpectedEof {
                    position,
                    expected: format!("{:?}", expected),
                    context: ParseContext::from_span(source, &(position..position)),
                })
            }
        }
    }

    pub(crate) fn safe_call<T, F>(&mut self, f: F) -> Result<T, ParseError>
    where
        F: FnOnce(&mut Self) -> Result<T, ParseError>,
    {
        self.depth += 1;

        let result = match self.depth > self.config.max_recursion_depth {
            true => Err(ParseError::TooMuchRecursion {
                max_depth: self.config.max_recursion_depth,
                position: match self.span() {
                    Some(ts) => ts.span.start,
                    None => self.eof_position(),
                },
            }),
            false => f(self),
        };

        self.depth = self.depth.saturating_sub(1);

        result
    }

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
}

pub trait Parse: Sized {
    fn parse(parser: &mut Parser) -> Result<Self, ParseError>;
}
