use crate::{
    lexer::Token,
    parser::{Parse, ParseError, Parser},
};

use super::value::Value;

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOp {
    // Arithmetic
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,

    // Comparison
    Eq,
    Neq,
    Lt,
    Le,
    Gt,
    Ge,

    // Logic
    And,
    Or,

    // Bitwise
    BitAnd,
    BitOr,
    Shl,
    Shr,
}

// impl BinaryOp {
//     pub fn precedence(&self) -> u8 {
//         match self {
//             BinaryOp::Or => 1,
//             BinaryOp::And => 2,
//             BinaryOp::BitOr => 3,
//             BinaryOp::BitAnd => 4,
//             BinaryOp::Eq | BinaryOp::Neq => 5,
//             BinaryOp::Lt | BinaryOp::Le | BinaryOp::Gt | BinaryOp::Ge => 6,
//             BinaryOp::Shl | BinaryOp::Shr => 7,
//             BinaryOp::Add | BinaryOp::Sub => 8,
//             BinaryOp::Mul | BinaryOp::Div | BinaryOp::Mod => 9,
//             BinaryOp::Pow => 10,
//         }
//     }

//     pub fn is_right_associative(&self) -> bool {
//         matches!(self, BinaryOp::Pow)
//     }
// }

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOp {
    Not, // !
    Neg, // -
}

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
    fn parse_assignment(parser: &mut Parser) -> Result<Self, ParseError> {
        let expr = Self::parse_logical_or(parser)?;

        // Handle assignment operators
        match parser.peek().cloned() {
            Some(Token::Equal) => {
                parser.advance();
                let value = Self::parse_assignment(parser)?;
                Ok(Expr::Assignment {
                    target: Box::new(expr),
                    value: Box::new(value),
                })
            }
            Some(
                op @ (Token::PlusEqual
                | Token::MinusEqual
                | Token::StarEqual
                | Token::SlashEqual
                | Token::ModuloEqual
                | Token::CaretEqual
                | Token::BitAndEqual
                | Token::BitOrEqual
                | Token::ShiftLeftEqual
                | Token::ShiftRightEqual),
            ) => {
                parser.advance();
                let value = Self::parse_assignment(parser)?;
                let binary_op = match op {
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
    }

    fn parse_logical_or(parser: &mut Parser) -> Result<Self, ParseError> {
        let mut expr = Self::parse_logical_and(parser)?;

        while parser.consume(&Token::Or) {
            let right = Self::parse_logical_and(parser)?;
            expr = Expr::Binary {
                op: BinaryOp::Or,
                left: Box::new(expr),
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn parse_logical_and(parser: &mut Parser) -> Result<Self, ParseError> {
        let mut expr = Self::parse_bitwise_or(parser)?;

        while parser.consume(&Token::And) {
            let right = Self::parse_bitwise_or(parser)?;
            expr = Expr::Binary {
                op: BinaryOp::And,
                left: Box::new(expr),
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn parse_bitwise_or(parser: &mut Parser) -> Result<Self, ParseError> {
        let mut expr = Self::parse_bitwise_and(parser)?;

        while parser.consume(&Token::BitOr) {
            let right = Self::parse_bitwise_and(parser)?;
            expr = Expr::Binary {
                op: BinaryOp::BitOr,
                left: Box::new(expr),
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn parse_bitwise_and(parser: &mut Parser) -> Result<Self, ParseError> {
        let mut expr = Self::parse_equality(parser)?;

        while parser.consume(&Token::BitAnd) {
            let right = Self::parse_equality(parser)?;
            expr = Expr::Binary {
                op: BinaryOp::BitAnd,
                left: Box::new(expr),
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn parse_equality(parser: &mut Parser) -> Result<Self, ParseError> {
        let mut expr = Self::parse_comparison(parser)?;

        loop {
            let binary_op = match parser.peek() {
                Some(Token::EqualEqual) => BinaryOp::Eq,
                Some(Token::BangEqual) => BinaryOp::Neq,
                _ => break,
            };

            parser.advance();
            let right = Self::parse_comparison(parser)?;
            expr = Expr::Binary {
                op: binary_op,
                left: Box::new(expr),
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn parse_comparison(parser: &mut Parser) -> Result<Self, ParseError> {
        let mut expr = Self::parse_shift(parser)?;

        loop {
            let binary_op = match parser.peek() {
                Some(Token::Greater) => BinaryOp::Gt,
                Some(Token::GreaterEqual) => BinaryOp::Ge,
                Some(Token::Less) => BinaryOp::Lt,
                Some(Token::LessEqual) => BinaryOp::Le,
                _ => break,
            };

            parser.advance();
            let right = Self::parse_shift(parser)?;
            expr = Expr::Binary {
                op: binary_op,
                left: Box::new(expr),
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn parse_shift(parser: &mut Parser) -> Result<Self, ParseError> {
        let mut expr = Self::parse_term(parser)?;

        loop {
            let binary_op = match parser.peek() {
                Some(Token::ShiftLeft) => BinaryOp::Shl,
                Some(Token::ShiftRight) => BinaryOp::Shr,
                _ => break,
            };

            parser.advance();
            let right = Self::parse_term(parser)?;
            expr = Expr::Binary {
                op: binary_op,
                left: Box::new(expr),
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn parse_term(parser: &mut Parser) -> Result<Self, ParseError> {
        let mut expr = Self::parse_factor(parser)?;

        loop {
            let binary_op = match parser.peek() {
                Some(Token::Plus) => BinaryOp::Add,
                Some(Token::Minus) => BinaryOp::Sub,
                _ => break,
            };

            parser.advance();
            let right = Self::parse_factor(parser)?;
            expr = Expr::Binary {
                op: binary_op,
                left: Box::new(expr),
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn parse_factor(parser: &mut Parser) -> Result<Self, ParseError> {
        let mut expr = Self::parse_power(parser)?;

        loop {
            let binary_op = match parser.peek() {
                Some(Token::Star) => BinaryOp::Mul,
                Some(Token::Slash) => BinaryOp::Div,
                Some(Token::Modulo) => BinaryOp::Mod,
                _ => break,
            };

            parser.advance();
            let right = Self::parse_power(parser)?;
            expr = Expr::Binary {
                op: binary_op,
                left: Box::new(expr),
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn parse_power(parser: &mut Parser) -> Result<Self, ParseError> {
        let mut expr = Self::parse_unary(parser)?;

        // Right-associative
        if matches!(parser.peek(), Some(Token::Caret | Token::DoubleStar)) {
            parser.advance();
            let right = Self::parse_power(parser)?; // Right-associative recursion
            expr = Expr::Binary {
                op: BinaryOp::Pow,
                left: Box::new(expr),
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn parse_unary(parser: &mut Parser) -> Result<Self, ParseError> {
        match parser.peek() {
            Some(Token::Bang) => {
                parser.advance();
                let expr = Self::parse_unary(parser)?;
                Ok(Expr::Unary {
                    op: UnaryOp::Not,
                    operand: Box::new(expr),
                })
            }
            Some(Token::Minus) => {
                parser.advance();
                let expr = Self::parse_unary(parser)?;
                Ok(Expr::Unary {
                    op: UnaryOp::Neg,
                    operand: Box::new(expr),
                })
            }
            Some(Token::PlusPlus) => {
                parser.advance();
                let expr = Self::parse_postfix(parser)?;
                Ok(Expr::PreIncrement {
                    operand: Box::new(expr),
                })
            }
            Some(Token::MinusMinus) => {
                parser.advance();
                let expr = Self::parse_postfix(parser)?;
                Ok(Expr::PreDecrement {
                    operand: Box::new(expr),
                })
            }
            _ => Self::parse_postfix(parser),
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
                    parser.advance();
                    if let Some(Token::Identifier(name)) = parser.advance() {
                        expr = Expr::Member {
                            object: Box::new(expr),
                            property: name.clone(),
                        };
                    } else {
                        return Err(ParseError::MissingToken {
                            position: parser.pos.saturating_sub(1),
                            expected: "property name".to_string(),
                        });
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
                }
                Some(Token::RightParen) => break,
                _ => return Err(parser.error("Expected ',' or ')'", Some("',' or ')'"))),
            }
        }

        Ok(args)
    }

    fn parse_primary(parser: &mut Parser) -> Result<Self, ParseError> {
        let token = parser.advance();

        match token {
            Some(Token::IntLiteral(i)) => Ok(Expr::Literal(Value::Int(*i))),
            Some(Token::FloatLiteral(f)) => Ok(Expr::Literal(Value::Float(*f))),
            Some(Token::BoolTrue) => Ok(Expr::Literal(Value::Bool(true))),
            Some(Token::BoolFalse) => Ok(Expr::Literal(Value::Bool(false))),
            Some(Token::StringLiteral(s)) => Ok(Expr::Literal(Value::Str(s.clone()))),
            Some(Token::Identifier(name)) => Ok(Expr::Variable(name.clone())),
            Some(Token::LeftParen) => {
                let expr = Self::parse(parser)?;
                parser.expect(Token::RightParen)?;
                Ok(Expr::Group(Box::new(expr)))
            }
            Some(Token::LeftBracket) => {
                // Need to backtrack since we consumed the bracket
                parser.pos -= 1;
                Self::parse_array_literal(parser)
            }
            _ => Err(parser.error("Unexpected token", Some("expression"))),
        }
    }

    fn parse_array_literal(parser: &mut Parser) -> Result<Self, ParseError> {
        parser.expect(Token::LeftBracket)?;
        let mut elements = Vec::new();

        while parser.peek() != Some(&Token::RightBracket) {
            elements.push(Self::parse(parser)?);

            if parser.peek() == Some(&Token::Comma) {
                parser.advance();
            } else if parser.peek() != Some(&Token::RightBracket) {
                return Err(parser.error("Expected ',' or ']'", Some("',' or ']'")));
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
