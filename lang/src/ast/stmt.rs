use crate::{
    ast::expr::Expr,
    lexer::Token,
    parser::{Parse, ParseError, Parser},
};

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
    // /// Check if this statement can appear at the top level
    // pub fn is_top_level(&self) -> bool {
    //     matches!(
    //         self,
    //         Stmt::Function { .. }
    //             | Stmt::Let { .. }
    //             | Stmt::Const { .. }
    //             | Stmt::Import { .. }
    //             | Stmt::Export { .. }
    //     )
    // }

    // /// Check if this statement creates a new scope
    // pub fn creates_scope(&self) -> bool {
    //     matches!(
    //         self,
    //         Stmt::Block { .. }
    //             | Stmt::Function { .. }
    //             | Stmt::If { .. }
    //             | Stmt::While { .. }
    //             | Stmt::For { .. }
    //     )
    // }

    fn parse_let(parser: &mut Parser) -> Result<Self, ParseError> {
        parser.expect(Token::Let)?;

        let current_pos = parser.pos;
        let name = match parser.advance() {
            Some(Token::Identifier(name)) => name.clone(),
            Some(token) => {
                return Err(ParseError::UnexpectedToken {
                    position: current_pos,
                    expected: Some("identifier".to_string()),
                    found: format!("{:?}", token),
                });
            }
            None => {
                return Err(ParseError::UnexpectedEof {
                    position: current_pos,
                    expected: "identifier".to_string(),
                });
            }
        };

        parser.expect(Token::Equal)?;

        let value = Expr::parse(parser)?;

        parser.expect(Token::Semicolon)?;

        Ok(Stmt::Let { name, value })
    }

    fn parse_const(parser: &mut Parser) -> Result<Self, ParseError> {
        parser.expect(Token::Const)?;

        let current_pos = parser.pos;
        let name = match parser.advance() {
            Some(Token::Identifier(name)) => name.clone(),
            Some(token) => {
                return Err(ParseError::UnexpectedToken {
                    position: current_pos,
                    expected: Some("identifier".to_string()),
                    found: format!("{:?}", token),
                });
            }
            None => {
                return Err(ParseError::UnexpectedEof {
                    position: current_pos,
                    expected: "identifier".to_string(),
                });
            }
        };

        parser.expect(Token::Equal)?;
        let value = Expr::parse(parser)?;
        parser.expect(Token::Semicolon)?;

        Ok(Stmt::Const { name, value })
    }

    fn parse_function(parser: &mut Parser) -> Result<Self, ParseError> {
        parser.expect(Token::Fn)?;

        let curr_position = parser.pos;
        let name = match parser.advance() {
            Some(Token::Identifier(name)) => name.clone(),
            other => {
                return Err(ParseError::UnexpectedToken {
                    position: curr_position,
                    expected: Some("function name".to_string()),
                    found: other
                        .map(|t| format!("{:?}", t))
                        .unwrap_or("EOF".to_string()),
                });
            }
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
    }

    fn parse_parameter_list(parser: &mut Parser) -> Result<Vec<String>, ParseError> {
        let mut params = Vec::new();

        while parser.peek() != Some(&Token::RightParen) {
            let current_pos = parser.pos;

            match parser.advance() {
                Some(Token::Identifier(param)) => {
                    params.push(param.clone());
                }
                Some(token) => {
                    return Err(ParseError::UnexpectedToken {
                        position: current_pos,
                        expected: Some("parameter name".to_string()),
                        found: format!("{:?}", token),
                    });
                }
                None => {
                    return Err(ParseError::UnexpectedEof {
                        position: current_pos,
                        expected: "parameter name".to_string(),
                    });
                }
            }

            match parser.peek() {
                Some(Token::Comma) => {
                    parser.advance();
                }
                Some(Token::RightParen) => break,
                Some(token) => {
                    return Err(ParseError::UnexpectedToken {
                        position: parser.pos,
                        expected: Some("',' or ')'".to_string()),
                        found: format!("{:?}", token),
                    });
                }
                None => {
                    return Err(ParseError::UnexpectedEof {
                        position: parser.pos,
                        expected: "',' or ')'".to_string(),
                    });
                }
            }
        }

        Ok(params)
    }

    fn parse_if(parser: &mut Parser) -> Result<Self, ParseError> {
        parser.expect(Token::If)?;
        let condition = Expr::parse(parser)?;
        let then_branch = Self::parse_block(parser)?;

        let else_branch = if parser.consume(&Token::Else) {
            if parser.peek() == Some(&Token::If) {
                Some(Box::new(Self::parse_if(parser)?))
            } else {
                Some(Box::new(Self::parse_block(parser)?))
            }
        } else {
            None
        };

        Ok(Stmt::If {
            condition,
            then_branch: Box::new(then_branch),
            else_branch,
        })
    }

    fn parse_return(parser: &mut Parser) -> Result<Self, ParseError> {
        parser.expect(Token::Return)?;

        let value = if parser.peek() == Some(&Token::Semicolon) {
            None
        } else {
            Some(Expr::parse(parser)?)
        };

        parser.expect(Token::Semicolon)?;
        Ok(Stmt::Return { value })
    }

    fn parse_block(parser: &mut Parser) -> Result<Self, ParseError> {
        parser.expect(Token::LeftBrace)?;
        let mut statements = Vec::new();

        while parser.peek() != Some(&Token::RightBrace) && !parser.is_at_end() {
            statements.push(Self::parse(parser)?);
        }

        parser.expect(Token::RightBrace)?;
        Ok(Stmt::Block { statements })
    }

    fn parse_import(parser: &mut Parser) -> Result<Self, ParseError> {
        parser.expect(Token::Import)?;

        let curr_position = parser.pos;
        let module = match parser.advance() {
            Some(Token::StringLiteral(module)) => module.clone(),
            Some(Token::Identifier(module)) => module.clone(),
            other => {
                return Err(ParseError::UnexpectedToken {
                    position: curr_position,
                    expected: Some("module name".to_string()),
                    found: other
                        .map(|t| format!("{:?}", t))
                        .unwrap_or("EOF".to_string()),
                });
            }
        };

        parser.expect(Token::Semicolon)?;
        Ok(Stmt::Import { module })
    }

    fn parse_export(parser: &mut Parser) -> Result<Self, ParseError> {
        parser.expect(Token::Export)?;
        let statement = Self::parse(parser)?;
        Ok(Stmt::Export {
            statement: Box::new(statement),
        })
    }

    fn parse_expression_stmt(parser: &mut Parser) -> Result<Self, ParseError> {
        let expr = Expr::parse(parser)?;
        parser.expect(Token::Semicolon)?;
        Ok(Stmt::Expression { expr })
    }

    fn parse_while(parser: &mut Parser) -> Result<Self, ParseError> {
        parser.expect(Token::While)?;

        let condition = Expr::parse(parser)?;
        let body = Self::parse_block(parser)?;

        Ok(Stmt::While {
            condition,
            body: Box::new(body),
        })
    }

    fn parse_for(parser: &mut Parser) -> Result<Self, ParseError> {
        parser.expect(Token::For)?;
        parser.expect(Token::LeftParen)?;

        // Parse init (optional)
        let init = if parser.peek() == Some(&Token::Semicolon) {
            parser.advance(); // consume semicolon
            None
        } else {
            let init_stmt = Self::parse(parser)?;
            Some(Box::new(init_stmt))
        };

        // Parse condition (optional)
        let condition = if parser.peek() == Some(&Token::Semicolon) {
            parser.advance(); // consume semicolon
            None
        } else {
            let cond = Expr::parse(parser)?;
            parser.expect(Token::Semicolon)?;
            Some(cond)
        };

        // Parse update (optional)
        let update = if parser.peek() == Some(&Token::RightParen) {
            None
        } else {
            Some(Expr::parse(parser)?)
        };

        parser.expect(Token::RightParen)?;
        let body = Self::parse_block(parser)?;

        Ok(Stmt::For {
            init,
            condition,
            update,
            body: Box::new(body),
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
