use std::rc::Rc;

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

pub fn keyword_token(s: &str) -> Option<TokenType> {
    use TokenType::*;
    Some(match s {
        "and"    => And,
        "class"  => Class,
        "else"   => Else,
        "false"  => False,
        "fun"    => Fun,
        "for"    => For,
        "if"     => If,
        "nil"    => Nil,
        "or"     => Or,
        "print"  => Print,
        "return" => Return,
        "super"  => Super,
        "this"   => This,
        "true"   => True,
        "var"    => Var,
        "while"  => While,
        _ => return None,
    })
}

fn fixed_lexeme(t: TokenType) -> &'static str {
    use TokenType::*;
    match t {
        LeftParen => "(", RightParen => ")",
        LeftBrace => "{", RightBrace => "}",
        Comma => ",", Dot => ".",
        Minus => "-", Plus => "+", Semicolon => ";", Slash => "/", Star => "*",
        Bang => "!", BangEqual => "!=",
        Equal => "=", EqualEqual => "==",
        Greater => ">", GreaterEqual => ">=",
        Less => "<", LessEqual => "<=",
        And => "and", Class => "class", Else => "else", False => "false",
        Fun => "fun", For => "for", If => "if", Nil => "nil", Or => "or",
        Print => "print", Return => "return", Super => "super", This => "this",
        True => "true", Var => "var", While => "while",
        EOF => "",
        // Variable-text tokens — caller should use Token::lexeme() which returns the stored slice.
        Identifier | String | Number => "",
    }
}

#[derive(Clone)]
pub enum Literal {
    String(Rc<str>),
    Number(f64),
    Boolean(bool),
    Nil,
}

impl Literal {
    pub fn get_type(&self) -> &'static str {
        match self {
            Literal::String(_) => "string",
            Literal::Number(_) => "number",
            Literal::Boolean(_) => "boolean",
            Literal::Nil => "nil",
        }
    }
}

impl From<&str> for Literal {
    fn from(value: &str) -> Self {
        Literal::String(Rc::from(value))
    }
}

impl From<f64> for Literal {
    fn from(value: f64) -> Self {
        Literal::Number(value)
    }
}

impl From<i32> for Literal {
    fn from(value: i32) -> Self {
        Literal::Number(value as f64)
    }
}

impl From<bool> for Literal {
    fn from(value: bool) -> Self {
        Literal::Boolean(value)
    }
}

impl PartialEq for Literal {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Literal::String(s1), Literal::String(s2)) => s1 == s2,
            (Literal::Number(n1), Literal::Number(n2)) => n1 == n2,
            (Literal::Boolean(b1), Literal::Boolean(b2)) => b1 == b2,
            (Literal::Nil, Literal::Nil) => true,
            _ => false,
        }
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

#[derive(Clone, Debug)]
pub struct Token {
    pub token_type: TokenType,
    // Only populated for Identifier/String/Number; other token types derive lexeme from token_type.
    pub lexeme: Option<Box<str>>,
    pub literal: Option<Literal>,
    pub line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: Option<&str>, literal: Option<Literal>, line: usize) -> Self {
        Token {
            token_type,
            lexeme: lexeme.map(Box::from),
            literal,
            line,
        }
    }

    #[allow(dead_code)]
    pub fn operator(token_type: TokenType, line: usize) -> Self {
        Token { token_type, lexeme: None, literal: None, line }
    }

    pub fn lexeme(&self) -> &str {
        match &self.lexeme {
            Some(s) => s,
            None => fixed_lexeme(self.token_type),
        }
    }
}
