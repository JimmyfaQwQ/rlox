use crate::token::{self, Token, Literal, TokenType};
use crate::error::error;
use std::rc::Rc;
use std::result::Result;

pub struct Scanner {
    source: Rc<str>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize
}

impl Scanner {
    pub fn new(source: &str) -> Self {
        Scanner {
            source: Rc::from(source),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1
        }
    }

    pub fn scan_tokens(&mut self) -> Result<&[Token], &'static str> {
        self.tokens.clear();
        let mut error: Option<&'static str> = None;
        while !self.is_at_end() {
            self.start = self.current;
            let result = self.scan_token();
            match result {
                Ok(_) => (),
                Err(e) => {
                    error = Some(e);
                    break;
                }
            }
        }
        if let Some(e) = error {
            return Err(e);
        }
        self.add_token(token::TokenType::EOF, None);
        return Ok(&self.tokens);
    }

    fn scan_token(&mut self) -> Result<(), &'static str> {
        let c = self.advance();
        match c {
            '(' => self.add_token(token::TokenType::LeftParen, None),
            ')' => self.add_token(token::TokenType::RightParen, None),
            '{' => self.add_token(token::TokenType::LeftBrace, None),
            '}' => self.add_token(token::TokenType::RightBrace, None),
            ',' => self.add_token(token::TokenType::Comma, None),
            '.' => self.add_token(token::TokenType::Dot, None),
            '-' => self.add_token(token::TokenType::Minus, None),
            '+' => self.add_token(token::TokenType::Plus, None),
            ';' => self.add_token(token::TokenType::Semicolon, None),
            '*' => self.add_token(token::TokenType::Star, None),
            '!' => {
                if self.match_next('=') {
                    self.add_token(token::TokenType::BangEqual, None);
                } else {
                    self.add_token(token::TokenType::Bang, None);
                }
            }
            '=' => {
                if self.match_next('=') {
                    self.add_token(TokenType::EqualEqual, None);
                } else {
                    self.add_token(TokenType::Equal, None);
                }
            }
            '<' => {
                if self.match_next('=') {
                    self.add_token(TokenType::LessEqual, None);
                } else {
                    self.add_token(TokenType::Less, None);
                }
            }
            '>' => {
                if self.match_next('=') {
                    self.add_token(TokenType::GreaterEqual, None);
                } else {
                    self.add_token(TokenType::Greater, None);
                }
            }
            '/' => {
                if self.match_next('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash, None);
                }
            }
            ' ' | '\r' | '\t' => (),
            '\n' => self.line += 1,
            '"' => self.string(),
            '1'..='9' => self.number(),
            'a'..='z' | 'A'..='Z' | '_' => self.identifier(),
            _ => { 
                error(self.line, "Unexpected character.");
                return Err("The scanner encountered unexpected character.");
            }
        }
        return Ok(());
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }
        if self.is_at_end() {
            error(self.line, "Unterminated string.");
            return;
        }
        self.advance();
        let value = &self.source[self.start + 1..self.current - 1];
        self.add_token(TokenType::String, Some(Literal::from(value)));
    }

    fn number(&mut self) {
        while self.peek().is_digit(10) {
            self.advance();
        }
        if self.peek() == '.' && self.peek_next().is_digit(10) {
            self.advance();
            while self.peek().is_digit(10) {
                self.advance();
            }
        }
        let value = self.source[self.start..self.current].parse::<f64>().unwrap();
        self.add_token(TokenType::Number, Some(Literal::from(value)));
    }

    fn identifier(&mut self) {
        while self.peek().is_alphanumeric() || self.peek() == '_' {
            self.advance();
        }
        let text = &self.source[self.start..self.current];
        let token_type = token::KEYWORDS.iter().find(|(k, _)| *k == text).map(|(_, v)| *v).unwrap_or(TokenType::Identifier);
        self.add_token(token_type, Some(Literal::from(text)));
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char {
        let c = self.source.chars().nth(self.current).unwrap();
        self.current += 1;
        c
    }

    fn match_next(&mut self, expected: char) -> bool {
        // Only advance if the next character is the expected one. This is used for two-character tokens.
        if self.is_at_end() {
            return false;
        }
        if self.source.chars().nth(self.current).unwrap() != expected {
            return false;
        }
        self.current += 1;
        return true;
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source.chars().nth(self.current).unwrap()
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        self.source.chars().nth(self.current + 1).unwrap()
    }

    fn add_token(&mut self, token_type: token::TokenType, literal: Option<Literal>) {
        let text = &self.source[self.start..self.current];
        self.tokens.push(Token::new(token_type, Some(text), literal, self.line));
    }
}