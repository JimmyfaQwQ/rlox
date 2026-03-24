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


pub enum Literal {
    String(String),
    Number(f64),
    Boolean(bool),
    Nil,
}

impl From<String> for Literal {
    fn from(value: String) -> Self {
        Literal::String(value)
    }
}

impl From<&str> for Literal {
    fn from(value: &str) -> Self {
        Literal::String(value.to_string())
    }
}

impl From<f64> for Literal {
    fn from(value: f64) -> Self {
        Literal::Number(value)
    }
}

impl From<bool> for Literal {
    fn from(value: bool) -> Self {
        Literal::Boolean(value)
    }
}

impl Default for Literal {
    fn default() -> Self {
        Literal::Nil
    }
}

impl std::fmt::Debug for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Literal::String(s) => write!(f, "{}", s),
            Literal::Number(n) => write!(f, "{}", n),
            Literal::Boolean(b) => write!(f, "{}", b),
            Literal::Nil => write!(f, "nil"),
        }
    }
}

#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    lexeme: Option<String>,
    literal: Option<Literal>,
    line: usize
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: Option<String>, literal: Option<Literal>, line: usize) -> Self {
        Token {
            token_type,
            lexeme,
            literal,
            line
        }
    }
}
