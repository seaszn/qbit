use crate::{
    lexer::Token,
    parser::{ParseContext, Parse, ParseError, Parser},
};

use super::{
    op::{BinaryOp, Precedence, UnaryOp},
    value::Value,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    // Literals and variables
    Literal(Value),
    Variable(String),

    // Binary operations
    Binary {
        op: BinaryOp,
        left: Box<Expr>,
        right: Box<Expr>,
    },

    // Unary operations
    Unary {
        op: UnaryOp,
        operand: Box<Expr>,
    },

    // Grouping
    Group(Box<Expr>),

    // Function calls
    Call {
        callee: Box<Expr>,
        args: Vec<Expr>,
    },

    // Member access
    Member {
        object: Box<Expr>,
        property: String,
    },

    // Array/object indexing
    Index {
        object: Box<Expr>,
        index: Box<Expr>,
    },

    // Array literal
    Array {
        elements: Vec<Expr>,
    },

    // Assignment
    Assignment {
        target: Box<Expr>,
        value: Box<Expr>,
    },

    // Compound assignment (+=, -=, etc.)
    CompoundAssignment {
        target: Box<Expr>,
        op: BinaryOp,
        value: Box<Expr>,
    },

    // Increment/Decrement
    PreIncrement {
        operand: Box<Expr>,
    },
    PostIncrement {
        operand: Box<Expr>,
    },
    PreDecrement {
        operand: Box<Expr>,
    },
    PostDecrement {
        operand: Box<Expr>,
    },
}

impl Expr {
    fn parse_expression(parser: &mut Parser, min_precedence: u8) -> Result<Self, ParseError> {
        parser.safe_call(|parser| {
            let mut left = Self::parse_unary(parser)?;

            while let Some(token) = parser.peek() {
                match BinaryOp::from_token(token) {
                    Some(op) => {
                        let precedence = op.precedence();

                        // Check if we should continue parsing at this precedence level
                        if precedence < min_precedence {
                            break;
                        }

                        parser.advance(); // consume the operator

                        // For right-associative operators, use same precedence
                        // For left-associative, use precedence + 1
                        let next_min_precedence = match op.is_right_associative() {
                            true => precedence,
                            false => precedence + 1,
                        };

                        let right = Self::parse_expression(parser, next_min_precedence)?;

                        left = Expr::Binary {
                            op,
                            left: Box::new(left),
                            right: Box::new(right),
                        };
                    }
                    None => break,
                }
            }

            Ok(left)
        })
    }

    fn parse_assignment(parser: &mut Parser) -> Result<Self, ParseError> {
        parser.safe_call(|parser| {
            let expr = Self::parse_expression(parser, 0)?; // Start with minimum precedence

            // Handle assignment operators
            match parser.peek() {
                Some(Token::Equal) => {
                    parser.advance();
                    let value = Self::parse_assignment(parser)?;
                    Ok(Expr::Assignment {
                        target: Box::new(expr),
                        value: Box::new(value),
                    })
                }
                Some(
                    Token::PlusEqual
                    | Token::MinusEqual
                    | Token::StarEqual
                    | Token::SlashEqual
                    | Token::ModuloEqual
                    | Token::CaretEqual
                    | Token::BitAndEqual
                    | Token::BitOrEqual
                    | Token::ShiftLeftEqual
                    | Token::ShiftRightEqual,
                ) => {
                    let op_token = parser.peek().unwrap().clone();
                    parser.advance();
                    let value = Self::parse_assignment(parser)?;
                    let binary_op = match op_token {
                        Token::PlusEqual => BinaryOp::Add,
                        Token::MinusEqual => BinaryOp::Sub,
                        Token::StarEqual => BinaryOp::Mul,
                        Token::SlashEqual => BinaryOp::Div,
                        Token::ModuloEqual => BinaryOp::Mod,
                        Token::CaretEqual => BinaryOp::Pow,
                        Token::BitAndEqual => BinaryOp::BitAnd,
                        Token::BitOrEqual => BinaryOp::BitOr,
                        Token::ShiftLeftEqual => BinaryOp::Shl,
                        Token::ShiftRightEqual => BinaryOp::Shr,
                        _ => unreachable!(),
                    };
                    Ok(Expr::CompoundAssignment {
                        target: Box::new(expr),
                        op: binary_op,
                        value: Box::new(value),
                    })
                }
                _ => Ok(expr),
            }
        })
    }

    fn parse_unary(parser: &mut Parser) -> Result<Self, ParseError> {
        match parser.peek() {
            Some(token) => match UnaryOp::from_token(token) {
                Some(unary_op) => {
                    parser.advance();
                    Ok(Expr::Unary {
                        op: unary_op,
                        operand: Box::new(Self::parse_unary(parser)?),
                    })
                }
                None => match token {
                    Token::PlusPlus => {
                        parser.advance();
                        Ok(Expr::PreIncrement {
                            operand: Box::new(Self::parse_postfix(parser)?),
                        })
                    }
                    Token::MinusMinus => {
                        parser.advance();
                        Ok(Expr::PreDecrement {
                            operand: Box::new(Self::parse_postfix(parser)?),
                        })
                    }
                    _ => Self::parse_postfix(parser),
                },
            },
            None => Self::parse_postfix(parser),
        }
    }

    fn parse_postfix(parser: &mut Parser) -> Result<Self, ParseError> {
        let mut expr = Self::parse_call(parser)?;

        loop {
            match parser.peek() {
                Some(Token::PlusPlus) => {
                    parser.advance();
                    expr = Expr::PostIncrement {
                        operand: Box::new(expr),
                    };
                }
                Some(Token::MinusMinus) => {
                    parser.advance();
                    expr = Expr::PostDecrement {
                        operand: Box::new(expr),
                    };
                }
                Some(Token::LeftBracket) => {
                    parser.advance();
                    let index = Self::parse(parser)?;
                    parser.expect(Token::RightBracket)?;
                    expr = Expr::Index {
                        object: Box::new(expr),
                        index: Box::new(index),
                    };
                }
                Some(Token::Dot) => {
                    let source = parser.source;

                    parser.advance();

                    match parser.advance() {
                        Some(token_span) => match &token_span.token {
                            Token::Identifier(name) => {
                                expr = Expr::Member {
                                    object: Box::new(expr),
                                    property: name.clone(),
                                };
                            }
                            _ => {
                                return Err(ParseError::UnexpectedToken {
                                    expected: Some("identifier".to_string()),
                                    found: format!("{:?}", token_span.token),
                                    span: token_span.span.clone(),
                                    context: ParseContext::from_span(source, &token_span.span),
                                });
                            }
                        },
                        None => {
                            return Err(parser.error("", Some("property name")));
                        }
                    }
                }
                _ => break,
            }
        }

        Ok(expr)
    }

    fn parse_call(parser: &mut Parser) -> Result<Self, ParseError> {
        let mut expr = Self::parse_primary(parser)?;

        while parser.peek() == Some(&Token::LeftParen) {
            parser.advance();
            let args = Self::parse_argument_list(parser)?;
            parser.expect(Token::RightParen)?;

            expr = Expr::Call {
                callee: Box::new(expr),
                args,
            };
        }

        Ok(expr)
    }

    fn parse_argument_list(parser: &mut Parser) -> Result<Vec<Expr>, ParseError> {
        let mut args = Vec::new();

        while parser.peek() != Some(&Token::RightParen) {
            args.push(Self::parse(parser)?);

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

        Ok(args)
    }

    fn parse_primary(parser: &mut Parser) -> Result<Self, ParseError> {
        let source = parser.source;

        match parser.advance() {
            Some(token_span) => match &token_span.token {
                Token::IntLiteral(i) => Ok(Expr::Literal(Value::Int(*i))),
                Token::FloatLiteral(f) => Ok(Expr::Literal(Value::Float(*f))),
                Token::BoolTrue => Ok(Expr::Literal(Value::Bool(true))),
                Token::BoolFalse => Ok(Expr::Literal(Value::Bool(false))),
                Token::StringLiteral(s) => Ok(Expr::Literal(Value::Str(s.clone()))),
                Token::Identifier(name) => Ok(Expr::Variable(name.clone())),
                Token::LeftParen => {
                    let expr = Self::parse(parser)?;

                    parser.expect(Token::RightParen)?;

                    Ok(Expr::Group(Box::new(expr)))
                }
                Token::LeftBracket => {
                    // Need to backtrack since we consumed the bracket
                    parser.pos -= 1;

                    Self::parse_array_literal(parser)
                }
                _ => Err(ParseError::UnexpectedToken {
                    expected: Some("expression".to_string()),
                    found: format!("{:?}", token_span.token),
                    span: token_span.span.clone(),
                    context: ParseContext::from_span(source, &token_span.span),
                }),
            },
            None => Err(parser.error("", Some("expression"))),
        }
    }

    fn parse_array_literal(parser: &mut Parser) -> Result<Self, ParseError> {
        parser.expect(Token::LeftBracket)?;
        let mut elements = Vec::new();

        while parser.peek() != Some(&Token::RightBracket) {
            elements.push(Self::parse(parser)?);

            match parser.peek() {
                Some(Token::Comma) => {
                    parser.advance();
                    // Handle trailing comma if configured
                    if parser.config.allow_trailing_commas()
                        && parser.peek() == Some(&Token::RightBracket)
                    {
                        break;
                    }
                }
                Some(Token::RightBracket) => break,
                _ => return Err(parser.error("", Some("',' or ']'"))),
            }
        }

        parser.expect(Token::RightBracket)?;
        Ok(Expr::Array { elements })
    }
}

impl Parse for Expr {
    fn parse(parser: &mut Parser) -> Result<Self, ParseError> {
        Self::parse_assignment(parser)
    }
}
