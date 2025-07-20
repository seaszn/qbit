use logos::Logos;
use qbit_lang::{
    ast::{expr::Expr, stmt::Stmt},
    lexer::Token,
    parser::{Parse, ParseError, Parser},
};

mod expr;
mod stmt;

fn parse_expr(source: &str) -> Result<Expr, ParseError> {
    let mut lexer = Token::lexer(source);
    let mut tokens = Vec::new();

    while let Some(token) = lexer.next() {
        match token {
            Ok(tok) => tokens.push(tok),
            Err(_) => {
                let span = lexer.span();
                let invalid_text = &source[span.start..span.end.min(source.len())];
                return Err(ParseError::LexerError {
                    message: "Invalid token".to_string(),
                    position: span.start,
                    invalid_text: invalid_text.to_string(),
                });
            }
        }
    }

    let mut parser = Parser::new(&tokens);
    let expr = Expr::parse(&mut parser)?;

    if !parser.is_at_end() {
        return Err(parser.error("Unexpected token after expression", Some("end of input")));
    }

    Ok(expr)
}

fn parse_stmt(source: &str) -> Result<Stmt, ParseError> {
    let mut lexer = Token::lexer(source);
    let mut tokens = Vec::new();

    while let Some(token) = lexer.next() {
        match token {
            Ok(tok) => tokens.push(tok),
            Err(_) => {
                let span = lexer.span();
                let invalid_text = &source[span.start..span.end.min(source.len())];
                return Err(ParseError::LexerError {
                    message: "Invalid token".to_string(),
                    position: span.start,
                    invalid_text: invalid_text.to_string(),
                });
            }
        }
    }

    let mut parser = Parser::new(&tokens);
    Stmt::parse(&mut parser)
}

fn parse_src(source: &str) -> Result<Vec<Stmt>, ParseError> {
    let mut lexer = Token::lexer(source);
    let mut tokens = Vec::new();

    while let Some(token) = lexer.next() {
        match token {
            Ok(tok) => tokens.push(tok),
            Err(_) => {
                let span = lexer.span();
                let invalid_text = &source[span.start..span.end.min(source.len())];
                return Err(ParseError::LexerError {
                    message: "Invalid token".to_string(),
                    position: span.start,
                    invalid_text: invalid_text.to_string(),
                });
            }
        }
    }

    let mut parser = Parser::new(&tokens);
    parser.parse()
}
