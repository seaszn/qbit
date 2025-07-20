#[derive(Debug, Clone)]
pub enum ParseError {
    LexerError {
        message: String,
        position: usize,
        invalid_text: String,
    },
    UnexpectedToken {
        position: usize,
        expected: Option<String>,
        found: String,
    },
    UnexpectedEof {
        position: usize,
        expected: String,
    },
    InvalidSyntax {
        message: String,
        position: usize,
    },
    MissingToken {
        position: usize,
        expected: String,
    },
}
impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::LexerError {
                message,
                position,
                invalid_text,
            } => {
                write!(
                    f,
                    "Lexer error at position {}: {} ('{}')",
                    position, message, invalid_text
                )
            }
            ParseError::UnexpectedToken {
                position,
                expected,
                found,
            } => match expected {
                Some(exp) => write!(
                    f,
                    "Parse error at position {}: expected {}, found {}",
                    position, exp, found
                ),
                None => write!(
                    f,
                    "Parse error at position {}: unexpected token {}",
                    position, found
                ),
            },
            ParseError::UnexpectedEof { position, expected } => {
                write!(
                    f,
                    "Parse error at position {}: unexpected end of file, expected {}",
                    position, expected
                )
            }
            ParseError::InvalidSyntax { message, position } => {
                write!(f, "Syntax error at position {}: {}", position, message)
            }
            ParseError::MissingToken { position, expected } => {
                write!(
                    f,
                    "Parse error at position {}: missing {}",
                    position, expected
                )
            }
        }
    }
}

impl std::error::Error for ParseError {}
