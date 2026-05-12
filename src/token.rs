use crate::object::Object;

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

#[derive(Clone, Debug)]
pub struct Token {
    pub token_type: TokenType,
    // Only populated for Identifier/String/Number; other token types derive lexeme from token_type.
    pub lexeme: Option<Box<str>>,
    pub literal: Option<Object>,
    pub line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: Option<&str>, literal: Option<Object>, line: usize) -> Self {
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
