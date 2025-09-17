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

// mod compiler;

// pub use engine::Engine;
pub use error::LangError;
