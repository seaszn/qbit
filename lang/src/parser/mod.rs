use crate::{ast::stmt::Stmt, lexer::Token};

mod error;

pub use error::ParseError;

pub trait Parse: Sized {
    fn parse(parser: &mut Parser) -> Result<Self, ParseError>;
}

#[derive(Clone)]
pub struct Parser<'a> {
    pub tokens: &'a [Token],
    pub pos: usize,
}
impl<'a> Parser<'a> {


    pub fn new(tokens: &'a [Token]) -> Self {
        Self { tokens, pos: 0 }
    }

    pub fn current(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    pub fn advance(&mut self) -> Option<&Token> {
        let tok = self.tokens.get(self.pos);
        self.pos += 1;
        tok
    }

    pub fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    pub fn peek_ahead(&self, n: usize) -> Option<&Token> {
        self.tokens.get(self.pos + n)
    }

    pub fn expect(&mut self, expected: Token) -> Result<(), ParseError> {
        let current_pos = self.pos;
        let got = self.advance();
        match got {
            Some(token) if token == &expected => Ok(()),
            Some(token) => Err(ParseError::UnexpectedToken {
                position: current_pos,
                expected: Some(format!("{:?}", expected)),
                found: format!("{:?}", token),
            }),
            None => Err(ParseError::UnexpectedEof {
                position: self.pos,
                expected: format!("{:?}", expected),
            }),
        }
    }

    pub fn consume(&mut self, token: &Token) -> bool {
        if self.peek() == Some(token) {
            self.advance();
            true
        } else {
            false
        }
    }

    pub fn is_at_end(&self) -> bool {
        self.pos >= self.tokens.len()
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, ParseError> {
        let mut statements = Vec::new();

        while !self.is_at_end() {
            statements.push(Stmt::parse(self)?);
        }

        Ok(statements)
    }

    pub fn error(&self, message: &str, expected: Option<&str>) -> ParseError {
        match (self.peek(), expected) {
            (Some(token), Some(exp)) => ParseError::UnexpectedToken {
                position: self.pos,
                expected: Some(exp.to_string()),
                found: format!("{:?}", token),
            },
            (Some(_), None) => ParseError::InvalidSyntax {
                message: message.to_string(),
                position: self.pos,
            },
            (None, Some(exp)) => ParseError::UnexpectedEof {
                position: self.pos,
                expected: exp.to_string(),
            },
            (None, None) => ParseError::UnexpectedEof {
                position: self.pos,
                expected: "token".to_string(),
            },
        }
    }
}
