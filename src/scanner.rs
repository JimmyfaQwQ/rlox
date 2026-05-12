use crate::token::{self, Token, TokenType};
use crate::object::Object;
use crate::error::{error, Error};
use std::result::Result;

pub fn scan_tokens(source: &str) -> Result<Vec<Token>, Error> {
    let mut s = ScanState {
        source,
        tokens: Vec::new(),
        start: 0,
        current: 0,
        line: 1,
    };
    while !s.is_at_end() {
        s.start = s.current;
        s.scan_token()?;
    }
    s.add_token(TokenType::EOF, None);
    Ok(s.tokens)
}

struct ScanState<'a> {
    source: &'a str,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl<'a> ScanState<'a> {
    fn scan_token(&mut self) -> Result<(), Error> {
        let c = self.advance();
        match c {
            b'(' => self.add_token(TokenType::LeftParen, None),
            b')' => self.add_token(TokenType::RightParen, None),
            b'{' => self.add_token(TokenType::LeftBrace, None),
            b'}' => self.add_token(TokenType::RightBrace, None),
            b',' => self.add_token(TokenType::Comma, None),
            b'.' => self.add_token(TokenType::Dot, None),
            b'-' => self.add_token(TokenType::Minus, None),
            b'+' => self.add_token(TokenType::Plus, None),
            b';' => self.add_token(TokenType::Semicolon, None),
            b'*' => self.add_token(TokenType::Star, None),
            b'!' => {
                let t = if self.match_next(b'=') { TokenType::BangEqual } else { TokenType::Bang };
                self.add_token(t, None);
            }
            b'=' => {
                let t = if self.match_next(b'=') { TokenType::EqualEqual } else { TokenType::Equal };
                self.add_token(t, None);
            }
            b'<' => {
                let t = if self.match_next(b'=') { TokenType::LessEqual } else { TokenType::Less };
                self.add_token(t, None);
            }
            b'>' => {
                let t = if self.match_next(b'=') { TokenType::GreaterEqual } else { TokenType::Greater };
                self.add_token(t, None);
            }
            b'/' => {
                if self.match_next(b'/') {
                    while self.peek() != b'\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash, None);
                }
            }
            b' ' | b'\r' | b'\t' => (),
            b'\n' => self.line += 1,
            b'"' => self.string(),
            b'0'..=b'9' => self.number(),
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => self.identifier(),
            _ => {
                error(self.line, "Scanner", "Unexpected character.");
                return Err(Error::Scanner);
            }
        }
        Ok(())
    }

    fn string(&mut self) {
        while self.peek() != b'"' && !self.is_at_end() {
            if self.peek() == b'\n' {
                self.line += 1;
            }
            self.advance();
        }
        if self.is_at_end() {
            error(self.line, "Scanner", "Unterminated string.");
            return;
        }
        self.advance();
        let value = &self.source[self.start + 1..self.current - 1];
        let literal = Object::from(value);
        self.add_token(TokenType::String, Some(literal));
    }

    fn number(&mut self) {
        while self.peek().is_ascii_digit() {
            self.advance();
        }
        if self.peek() == b'.' && self.peek_next().is_ascii_digit() {
            self.advance();
            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }
        let value = self.source[self.start..self.current].parse::<f64>().unwrap();
        self.add_token(TokenType::Number, Some(Object::from(value)));
    }

    fn identifier(&mut self) {
        while {
            let c = self.peek();
            c.is_ascii_alphanumeric() || c == b'_'
        } {
            self.advance();
        }
        let text = &self.source[self.start..self.current];
        let token_type = token::keyword_token(text).unwrap_or(TokenType::Identifier);
        self.add_token(token_type, None);
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> u8 {
        let c = self.source.as_bytes()[self.current];
        self.current += 1;
        c
    }

    fn match_next(&mut self, expected: u8) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source.as_bytes()[self.current] != expected {
            return false;
        }
        self.current += 1;
        true
    }

    fn peek(&self) -> u8 {
        if self.is_at_end() {
            return 0;
        }
        self.source.as_bytes()[self.current]
    }

    fn peek_next(&self) -> u8 {
        if self.current + 1 >= self.source.len() {
            return 0;
        }
        self.source.as_bytes()[self.current + 1]
    }

    fn add_token(&mut self, token_type: TokenType, literal: Option<Object>) {
        let lexeme = match token_type {
            TokenType::Identifier | TokenType::String | TokenType::Number => {
                Some(&self.source[self.start..self.current])
            }
            _ => None,
        };
        self.tokens.push(Token::new(token_type, lexeme, literal, self.line));
    }
}
