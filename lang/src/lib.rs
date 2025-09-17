mod error;
mod vscode;

pub mod lexer;
pub mod parser;

pub mod ast {
    pub mod expr;
    pub mod op;
    pub mod stmt;
    pub mod value;
}

