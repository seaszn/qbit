use logos::Logos;

use crate::lexer::Token;

use super::{ErrorContext, ParseError, Parser, ParserConfig, TokenSpan};

pub struct ParserBuilder<'a> {
    source: &'a str,
    config: ParserConfig,
}

impl<'a> ParserBuilder<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            config: ParserConfig::default(),
        }
    }

    pub fn allow_trailing_commas(mut self, allow: bool) -> Self {
        self.config.allow_trailing_commas = allow;
        self
    }

    pub fn max_recursion_depth(mut self, depth: usize) -> Self {
        self.config.max_recursion_depth = depth;
        self
    }

    pub fn build(self) -> Result<Parser<'a>, ParseError> {
        let mut lexer = Token::lexer(self.source);
        let mut tokens = Vec::new();

        while let Some(token_result) = lexer.next() {
            match token_result {
                Ok(token) => {
                    let span = lexer.span();
                    tokens.push(TokenSpan { token, span });
                }
                Err(_) => {
                    let span = lexer.span();
                    let invalid_text = &self.source[span.start..span.end.min(self.source.len())];

                    let context = ErrorContext::from_span(self.source, &span.clone());

                    return Err(ParseError::BuildError {
                        message: "Invalid token".to_string(),
                        invalid_text: invalid_text.to_string(),
                        span,
                        context,
                    });
                }
            }
        }

        Ok(Parser {
            pos: 0,
            depth: 0,
            tokens,
            source: self.source,
            config: self.config,
        })
    }
}
