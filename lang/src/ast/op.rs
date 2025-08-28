use crate::lexer::Token;

pub trait Precedence {
    fn precedence(&self) -> u8;
}

#[derive(Debug, Clone, PartialEq, Copy)]
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

impl BinaryOp {
    pub fn is_right_associative(&self) -> bool {
        matches!(self, BinaryOp::Pow)
    }

    pub fn from_token(token: &Token) -> Option<Self>{
        match token {
            Token::Plus => Some(BinaryOp::Add),
            Token::Minus => Some(BinaryOp::Sub),
            Token::Star => Some(BinaryOp::Mul),
            Token::Slash => Some(BinaryOp::Div),
            Token::Modulo => Some(BinaryOp::Mod),
            Token::Caret | Token::DoubleStar => Some(BinaryOp::Pow),
            Token::EqualEqual => Some(BinaryOp::Eq),
            Token::BangEqual => Some(BinaryOp::Neq),
            Token::Less => Some(BinaryOp::Lt),
            Token::LessEqual => Some(BinaryOp::Le),
            Token::Greater => Some(BinaryOp::Gt),
            Token::GreaterEqual => Some(BinaryOp::Ge),
            Token::And => Some(BinaryOp::And),
            Token::Or => Some(BinaryOp::Or),
            Token::BitAnd => Some(BinaryOp::BitAnd),
            Token::BitOr => Some(BinaryOp::BitOr),
            Token::ShiftLeft => Some(BinaryOp::Shl),
            Token::ShiftRight => Some(BinaryOp::Shr),
            _ => None,
        }
    }
}

impl Precedence for BinaryOp {
    fn precedence(&self) -> u8 {
        match self {
            BinaryOp::Or => 1,
            BinaryOp::And => 2,
            BinaryOp::BitOr => 3,
            BinaryOp::BitAnd => 4,
            BinaryOp::Eq | BinaryOp::Neq => 5,
            BinaryOp::Lt | BinaryOp::Le | BinaryOp::Gt | BinaryOp::Ge => 6,
            BinaryOp::Shl | BinaryOp::Shr => 7,
            BinaryOp::Add | BinaryOp::Sub => 8,
            BinaryOp::Mul | BinaryOp::Div | BinaryOp::Mod => 9,
            BinaryOp::Pow => 10,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOp {
    Not, // !
    Neg, // -
}

impl UnaryOp{
    pub fn from_token(token: &Token) -> Option<Self>{
         match token {
            Token::Bang => Some(UnaryOp::Not),
            Token::Minus => Some(UnaryOp::Neg),
            _ => None,
        }
    }
}

impl Precedence for UnaryOp {
    fn precedence(&self) -> u8 {
        15
    }
}