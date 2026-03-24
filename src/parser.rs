use crate::token::{Token, TokenType, Literal};
use crate::expr::Expr;
use crate::error as universal_error;
use std::result::Result;
use std::rc::Rc;

pub struct Parser {
    tokens: Rc<[Token]>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: &[Token]) -> Self {
        Parser {
            tokens: Rc::from(tokens),
            current: 0,
        }
    }
}

impl Parser {
    fn expression(&mut self) -> Result<Expr, &'static str> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, &'static str> {
        let mut expr = self.comparison()?;
        while self.match_token(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous().clone();
            let right = self.comparison()?;
            expr = Expr::binary(expr, operator, right);
        }
        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, &'static str> {
        let mut expr = self.term()?;
        while self.match_token(&[TokenType::Greater, TokenType::GreaterEqual, TokenType::Less, TokenType::LessEqual]) {
            let operator = self.previous().clone();
            let right = self.term()?;
            expr = Expr::binary(expr, operator, right);
        }
        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, &'static str> {
        let mut expr = self.factor()?;
        while self.match_token(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous().clone();
            let right = self.factor()?;
            expr = Expr::binary(expr, operator, right);
        }
        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, &'static str> {
        let mut expr = self.unary()?;
        while self.match_token(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            expr = Expr::binary(expr, operator, right);
        }
        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, &'static str> {
        if self.match_token(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            return Ok(Expr::unary(operator, right));
        }
        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, &'static str> {
        if self.match_token(&[TokenType::False]) {
            return Ok(Expr::literal(Literal::Boolean(false)));
        }
        if self.match_token(&[TokenType::True]) {
            return Ok(Expr::literal(Literal::Boolean(true)));
        }
        if self.match_token(&[TokenType::Nil]) {
            return Ok(Expr::literal(Literal::Nil));
        }
        if self.match_token(&[TokenType::Number, TokenType::String]) {
            let literal = self.previous().literal.clone();
            match literal {
                Some(literal) => return Ok(Expr::literal(literal)),
                None => { 
                    self.error(self.previous(), "Expected literal value.");
                    return Err("Expected literal value.");
                },
            }
        }
        if self.match_token(&[TokenType::LeftParen]) {
            let expr = self.expression()?;
            if !self.match_token(&[TokenType::RightParen]) {
                self.error(self.peek(), "Expected ')' after expression.");
                return Err("Expected ')' after expression.");
            }
            return Ok(Expr::grouping(expr));
        }
        self.error(self.peek(), "Expected expression.");
        Err("Expected expression.")
    }
}

impl Parser {
    fn match_token(&mut self, types: &[TokenType]) -> bool {
        for token_type in types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        &self.peek().token_type == token_type
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::EOF
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }
}

impl Parser {
    pub fn parse(&mut self) -> Result<Expr, &'static str> {
        self.current = 0;
        self.expression()
    }

    fn error(&self, token: &Token, message: &str) {
        if token.token_type == TokenType::EOF {
            universal_error::report(token.line, " at end", message);
        } else {
            universal_error::report(token.line, &format!(" at '{}'", token.lexeme.as_ref().unwrap_or(&Rc::from("unknown"))), message);
        }
    }
}
