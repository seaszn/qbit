use crate::{
    ast::expr::Expr,
    lexer::Token,
    parser::{DebugContext, Parse, ParseError, Parser},
};

use super::value::Value;

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    /// let name = value;
    Let { name: String, value: Expr },

    /// const name = value;
    Const { name: String, value: Expr },

    /// fn name(params) { body }
    Function {
        name: String,
        params: Vec<String>,
        body: Box<Stmt>,
    },

    /// if condition { then_branch } else { else_branch }
    If {
        condition: Expr,
        then_branch: Box<Stmt>,
        else_branch: Option<Box<Stmt>>,
    },

    /// return value;
    Return { value: Option<Expr> },

    /// { statements }
    Block { statements: Vec<Stmt> },

    /// expr;
    Expression { expr: Expr },

    /// import "module" or import module;
    Import { module: String },

    /// export statement;
    Export { statement: Box<Stmt> },

    /// for future loop constructs
    While { condition: Expr, body: Box<Stmt> },

    /// for future loop constructs
    For {
        init: Option<Box<Stmt>>,
        condition: Option<Expr>,
        update: Option<Expr>,
        body: Box<Stmt>,
    },

    /// break;
    Break,

    /// continue;
    Continue,
}

impl Stmt {
    fn parse_let(parser: &mut Parser) -> Result<Self, ParseError> {
        let source = parser.source;

        parser.safe_call(|parser| {
            parser.expect(Token::Let)?;

            let name = match parser.advance() {
                Some(token_span) => match &token_span.token {
                    Token::Identifier(name) => name.clone(),
                    _ => {
                        return Err(ParseError::UnexpectedToken {
                            expected: Some("identifier".to_string()),
                            found: format!("{:?}", token_span.token),
                            span: token_span.span.clone(),
                            context: DebugContext::from_span(source, &token_span.span),
                        });
                    }
                },
                None => return Err(parser.error("", Some("identifier"))),
            };

            let value = match parser.peek() {
                Some(Token::Equal) => {
                    parser.expect(Token::Equal)?;
                    Expr::parse(parser)?
                }
                _ => Expr::Literal(Value::Null),
            };

            parser.expect(Token::Semicolon)?;

            Ok(Stmt::Let { name, value })
        })
    }

    fn parse_const(parser: &mut Parser) -> Result<Self, ParseError> {
        let source = parser.source;

        parser.safe_call(|parser| {
            parser.expect(Token::Const)?;

            let name = match parser.advance() {
                Some(token_span) => match &token_span.token {
                    Token::Identifier(name) => name.clone(),
                    _ => {
                        return Err(ParseError::UnexpectedToken {
                            expected: Some("identifier".to_string()),
                            found: format!("{:?}", token_span.token),
                            span: token_span.span.clone(),
                            context: DebugContext::from_span(source, &token_span.span),
                        });
                    }
                },
                None => return Err(parser.error("", Some("identifier"))),
            };

            parser.expect(Token::Equal)?;
            let value = Expr::parse(parser)?;

            parser.expect(Token::Semicolon)?;

            Ok(Stmt::Const { name, value })
        })
    }

    fn parse_function(parser: &mut Parser) -> Result<Self, ParseError> {
        let source = parser.source;

        parser.safe_call(|parser| {
            parser.expect(Token::Fn)?;

            let name = match parser.advance() {
                Some(token_span) => match &token_span.token {
                    Token::Identifier(name) => name.clone(),
                    _ => {
                        return Err(ParseError::UnexpectedToken {
                            expected: Some("function name".to_string()),
                            found: format!("{:?}", token_span.token),
                            span: token_span.span.clone(),
                            context: DebugContext::from_span(source, &token_span.span),
                        });
                    }
                },
                None => return Err(parser.error("", Some("function name"))),
            };

            parser.expect(Token::LeftParen)?;
            let params = Self::parse_parameter_list(parser)?;

            parser.expect(Token::RightParen)?;
            let body = Self::parse_block(parser)?;

            Ok(Stmt::Function {
                name,
                params,
                body: Box::new(body),
            })
        })
    }

    fn parse_parameter_list(parser: &mut Parser) -> Result<Vec<String>, ParseError> {
        let mut params = Vec::new();
        let source = parser.source;

        while parser.peek() != Some(&Token::RightParen) {
            match parser.advance() {
                Some(token_span) => match &token_span.token {
                    Token::Identifier(param) => {
                        params.push(param.clone());
                    }
                    _ => {
                        return Err(ParseError::UnexpectedToken {
                            expected: Some("parameter name".to_string()),
                            found: format!("{:?}", token_span.token),
                            span: token_span.span.clone(),
                            context: DebugContext::from_span(source, &token_span.span),
                        });
                    }
                },
                None => return Err(parser.error("", Some("parameter name"))),
            }

            match parser.peek() {
                Some(Token::Comma) => {
                    parser.advance();
                    // Handle trailing comma if configured
                    if parser.config.allow_trailing_commas()
                        && parser.peek() == Some(&Token::RightParen)
                    {
                        break;
                    }
                }
                Some(Token::RightParen) => break,
                _ => return Err(parser.error("", Some("',' or ')'"))),
            }
        }

        Ok(params)
    }

    fn parse_if(parser: &mut Parser) -> Result<Self, ParseError> {
        parser.safe_call(|parser| {
            parser.expect(Token::If)?;

            let condition = Expr::parse(parser)?;
            let then_branch = Self::parse_block(parser)?;

            let else_branch = match parser.consume(&Token::Else) {
                true => match parser.peek() {
                    Some(Token::If) => Some(Box::new(Self::parse_if(parser)?)),
                    _ => Some(Box::new(Self::parse_block(parser)?)),
                },
                false => None,
            };

            Ok(Stmt::If {
                condition,
                then_branch: Box::new(then_branch),
                else_branch,
            })
        })
    }

    fn parse_return(parser: &mut Parser) -> Result<Self, ParseError> {
        parser.safe_call(|parser| {
            parser.expect(Token::Return)?;

            let value = match parser.peek() {
                Some(Token::Semicolon) => None,
                Some(_) => Some(Expr::parse(parser)?),
                None => None,
            };

            parser.expect(Token::Semicolon)?;
            Ok(Stmt::Return { value })
        })
    }

    fn parse_block(parser: &mut Parser) -> Result<Self, ParseError> {
        parser.safe_call(|parser| {
            parser.expect(Token::LeftBrace)?;
            let mut statements = Vec::new();

            while parser.peek() != Some(&Token::RightBrace) && !parser.eof() {
                statements.push(Self::parse(parser)?);
            }

            parser.expect(Token::RightBrace)?;
            Ok(Stmt::Block { statements })
        })
    }

    fn parse_import(parser: &mut Parser) -> Result<Self, ParseError> {
        let source = parser.source;

        parser.safe_call(|parser| {
            parser.expect(Token::Import)?;

            let module = match parser.advance() {
                Some(token_span) => match &token_span.token {
                    Token::StringLiteral(module) => module.clone(),
                    Token::Identifier(module) => module.clone(),
                    _ => {
                        return Err(ParseError::UnexpectedToken {
                            expected: Some("module name".to_string()),
                            found: format!("{:?}", token_span.token),
                            span: token_span.span.clone(),
                            context: DebugContext::from_span(source, &token_span.span),
                        });
                    }
                },
                None => return Err(parser.error("", Some("module name"))),
            };

            parser.expect(Token::Semicolon)?;
            Ok(Stmt::Import { module })
        })
    }

    fn parse_export(parser: &mut Parser) -> Result<Self, ParseError> {
        parser.safe_call(|parser| {
            parser.expect(Token::Export)?;
            let statement = Self::parse(parser)?;
            Ok(Stmt::Export {
                statement: Box::new(statement),
            })
        })
    }

    fn parse_expression_stmt(parser: &mut Parser) -> Result<Self, ParseError> {
        parser.safe_call(|parser| {
            let expr = Expr::parse(parser)?;
            parser.expect(Token::Semicolon)?;
            Ok(Stmt::Expression { expr })
        })
    }

    fn parse_while(parser: &mut Parser) -> Result<Self, ParseError> {
        parser.safe_call(|parser| {
            parser.expect(Token::While)?;
            let condition = Expr::parse(parser)?;
            let body = Self::parse_block(parser)?;

            Ok(Stmt::While {
                condition,
                body: Box::new(body),
            })
        })
    }

    fn parse_for(parser: &mut Parser) -> Result<Self, ParseError> {
        parser.safe_call(|parser| {
            parser.expect(Token::For)?;
            parser.expect(Token::LeftParen)?;

            // Parse init (optional)
            let init = match parser.peek() {
                Some(Token::Semicolon) => {
                    parser.advance(); // consume semicolon
                    None
                }
                _ => {
                    let init_stmt = Self::parse(parser)?;
                    Some(Box::new(init_stmt))
                }
            };

            // Parse condition (optional)
            let condition = match parser.peek() {
                Some(Token::Semicolon) => {
                    parser.advance(); // consume semicolon
                    None
                }
                _ => {
                    let cond = Expr::parse(parser)?;
                    parser.expect(Token::Semicolon)?;
                    Some(cond)
                }
            };

            // Parse update (optional)
            let update = match parser.peek() {
                Some(Token::RightParen) => None,
                _ => Some(Expr::parse(parser)?),
            };

            parser.expect(Token::RightParen)?;
            let body = Self::parse_block(parser)?;

            Ok(Stmt::For {
                init,
                condition,
                update,
                body: Box::new(body),
            })
        })
    }

    fn parse_break(parser: &mut Parser) -> Result<Self, ParseError> {
        parser.expect(Token::Break)?;
        parser.expect(Token::Semicolon)?;
        Ok(Stmt::Break)
    }

    fn parse_continue(parser: &mut Parser) -> Result<Self, ParseError> {
        parser.expect(Token::Continue)?;
        parser.expect(Token::Semicolon)?;
        Ok(Stmt::Continue)
    }
}

// Implement Parse for Stmt enum
impl Parse for Stmt {
    fn parse(parser: &mut Parser) -> Result<Self, ParseError> {
        match parser.peek() {
            Some(Token::Let) => Self::parse_let(parser),
            Some(Token::Const) => Self::parse_const(parser),
            Some(Token::Fn) => Self::parse_function(parser),
            Some(Token::If) => Self::parse_if(parser),
            Some(Token::While) => Self::parse_while(parser),
            Some(Token::For) => Self::parse_for(parser),
            Some(Token::Break) => Self::parse_break(parser),
            Some(Token::Continue) => Self::parse_continue(parser),
            Some(Token::Return) => Self::parse_return(parser),
            Some(Token::LeftBrace) => Self::parse_block(parser),
            Some(Token::Import) => Self::parse_import(parser),
            Some(Token::Export) => Self::parse_export(parser),
            _ => Self::parse_expression_stmt(parser),
        }
    }
}
