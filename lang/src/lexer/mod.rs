use logos::Logos;
mod utils;

use utils::{parse_float, parse_identifier, parse_int, parse_string};

#[derive(Logos, Debug, PartialEq, Clone)]
pub enum Token {
    // ===== Literals =====
    #[regex(r"[0-9]+", parse_int)]
    IntLiteral(i64),
    #[regex(r"[0-9]+\.[0-9]+", parse_float)]
    FloatLiteral(f64),
    #[token("true")]
    BoolTrue,
    #[token("false")]
    BoolFalse,
    #[regex(r#""([^"\\]|\\.)*""#, parse_string)]
    StringLiteral(String),

    // ===== Identifiers =====
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", parse_identifier)]
    Identifier(String),

    // ===== Keywords =====
    #[token("let")]
    Let,
    #[token("const")]
    Const,
    #[token("fn")]
    Fn,
    #[token("return")]
    Return,
    #[token("if")]
    If,
    #[token("else")]
    Else,
    #[token("import")]
    Import,
    #[token("export")]
    Export,
    #[token("while")]
    While,
    #[token("for")]
    For,
    #[token("continue")]
    Continue,
    #[token("break")]
    Break,


    // ===== Operators =====
    // Basic
    #[token("=")]
    Equal,
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Star,
    #[token("/")]
    Slash,
    #[token("%")]
    Modulo,
    #[token("^")]
    Caret,
    #[token("**")]
    DoubleStar,

    // Compound assignment
    #[token("+=")]
    PlusEqual,
    #[token("-=")]
    MinusEqual,
    #[token("*=")]
    StarEqual,
    #[token("/=")]
    SlashEqual,
    #[token("%=")]
    ModuloEqual,
    #[token("^=")]
    CaretEqual,

    // Increment/decrement
    #[token("++")]
    PlusPlus,
    #[token("--")]
    MinusMinus,

    // Comparison
    #[token("==")]
    EqualEqual,
    #[token("!=")]
    BangEqual,
    #[token("<")]
    Less,
    #[token("<=")]
    LessEqual,
    #[token(">")]
    Greater,
    #[token(">=")]
    GreaterEqual,

    // Logic
    #[token("!")]
    Bang,
    #[token("&&")]
    And,
    #[token("||")]
    Or,

    // Bitwise
    #[token("&")]
    BitAnd,
    #[token("|")]
    BitOr,
    #[token("<<")]
    ShiftLeft,
    #[token(">>")]
    ShiftRight,
    #[token("&=")]
    BitAndEqual,
    #[token("|=")]
    BitOrEqual,
    #[token("<<=")]
    ShiftLeftEqual,
    #[token(">>=")]
    ShiftRightEqual,

    // ===== Grouping & Structure =====
    #[token("(")]
    LeftParen,
    #[token(")")]
    RightParen,
    #[token("{")]
    LeftBrace,
    #[token("}")]
    RightBrace,
    #[token("[")]
    LeftBracket,
    #[token("]")]
    RightBracket,
    #[token(",")]
    Comma,
    #[token(";")]
    Semicolon,
    #[token(":")]
    Colon,
    #[token(".")]
    Dot,

    // ===== Whitespace =====
    #[regex(r"[ \t\r\n]+", logos::skip)]
    Whitespace,

    // ===== Placeholders for future (commented) =====
    // #[token("..")] Range,
    // #[token("..=")] RangeInclusive,
    // #[token("?")] Question,
    // #[token("??")] NullCoalesce,
    // #[token("|>")] Pipe,
    // #[token("...")] Ellipsis,
}
