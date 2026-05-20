use crate::object::Object;

#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    Scanner,
    Parser,
    Runtime,
    Return(Option<Object>),
}

impl Error {
    /// Exit code following Crafting Interpreters: 65 for compile-time, 70 for runtime.
    pub fn exit_code(self) -> i32 {
        match self {
            Error::Scanner | Error::Parser => 65,
            Error::Runtime => 70,
            Error::Return(_) => 0, // Not an actual error, just a control flow mechanism
        }
    }
}

pub fn error(line: usize, prefix: &str, message: &str) {
    eprintln!("[line {}] {} Error: {}", line, prefix, message);
}

pub fn error_at_token(token: &crate::token::Token, prefix: &str, message: &str) {
    if token.token_type == crate::token::TokenType::EOF {
        eprintln!("[line {}] {} Error at end: {}", token.line, prefix, message);
    } else {
        eprintln!("[line {}] {} Error at '{}': {}", token.line, prefix, token.lexeme(), message);
    }
}
