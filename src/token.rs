use crate::token;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen, RightParen, LeftBrace, RightBrace,
    Comma, Dot, Minus, Plus, Semicolon, Slash, Star,

    // One or two character tokens.
    Bang, BangEqual,
    Equal, EqualEqual,
    Greater, GreaterEqual,
    Less, LessEqual ,

    // Literals.
    Identifier, String, Number,

    // Keywords.
    And, Class, Else, False, Fun, For, If, Nil, Or,
    Print, Return, Super, This, True, Var, While,

    EOF
}


pub static KEYWORDS: [(&str, TokenType); 16] = [
    ("and", TokenType::And),
    ("class", TokenType::Class),
    ("else", TokenType::Else),
    ("false", TokenType::False),
    ("fun", TokenType::Fun),
    ("for", TokenType::For),
    ("if", TokenType::If),
    ("nil", TokenType::Nil),
    ("or", TokenType::Or),
    ("print", TokenType::Print),
    ("return", TokenType::Return),
    ("super", TokenType::Super),
    ("this", TokenType::This),
    ("true", TokenType::True),
    ("var", TokenType::Var),
    ("while", TokenType::While),
];

#[derive(Clone)]
pub enum Literal<'a> {
    String(&'a str),
    Number(f64),
    Boolean(bool),
    Nil,
}

impl<'a> From<&'a str> for Literal<'a> {
    fn from(value: &'a str) -> Self {
        Literal::String(value)
    }
}

impl<'a> From<f64> for Literal<'a> {
    fn from(value: f64) -> Self {
        Literal::Number(value)
    }
}

impl<'a> From<i32> for Literal<'a> {
    fn from(value: i32) -> Self {
        Literal::Number(value as f64)
    }
}

impl<'a> From<bool> for Literal<'a> {
    fn from(value: bool) -> Self {
        Literal::Boolean(value)
    }
}

impl<'a> Default for Literal<'a> {
    fn default() -> Self {
        Literal::Nil
    }
}

impl<'a> std::fmt::Debug for Literal<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Literal::String(s) => write!(f, "{}", s),
            Literal::Number(n) => write!(f, "{}", n),
            Literal::Boolean(b) => write!(f, "{}", b),
            Literal::Nil => write!(f, "nil"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Token<'a> {
    pub token_type: TokenType,
    pub lexeme: Option<&'a str>,
    pub literal: Option<Literal<'a>>,
    pub line: usize
}

impl<'a> Token<'a> {
    pub fn new(token_type: TokenType, lexeme: Option<&'a str>, literal: Option<Literal<'a>>, line: usize) -> Self {
        Token {
            token_type,
            lexeme,
            literal,
            line
        }
    }

    pub fn operator(token_type: TokenType, lexeme: Option<&'a str>, line: usize) -> Self {
        Token {
            token_type,
            lexeme,
            literal: None,
            line
        }
    }
}