use crate::token::{Token, TokenType, Literal};
use crate::expr::Expr;
use crate::stmt::Stmt;
use crate::error::{error_at_token, Error};
use std::result::Result;
use std::mem;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }
}

impl Parser {
    fn expression(&mut self) -> Result<Expr, Error> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, Error> {
        let mut expr = self.comparison()?;
        while self.match_token(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.take_previous();
            let right = self.comparison()?;
            expr = Expr::binary(expr, operator, right);
        }
        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, Error> {
        let mut expr = self.term()?;
        while self.match_token(&[TokenType::Greater, TokenType::GreaterEqual, TokenType::Less, TokenType::LessEqual]) {
            let operator = self.take_previous();
            let right = self.term()?;
            expr = Expr::binary(expr, operator, right);
        }
        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, Error> {
        let mut expr = self.factor()?;
        while self.match_token(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.take_previous();
            let right = self.factor()?;
            expr = Expr::binary(expr, operator, right);
        }
        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, Error> {
        let mut expr = self.unary()?;
        while self.match_token(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.take_previous();
            let right = self.unary()?;
            expr = Expr::binary(expr, operator, right);
        }
        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, Error> {
        if self.match_token(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.take_previous();
            let right = self.unary()?;
            return Ok(Expr::unary(operator, right));
        }
        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, Error> {
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
            let literal = mem::take(&mut self.tokens[self.current - 1].literal);
            match literal {
                Some(literal) => return Ok(Expr::literal(literal)),
                None => {
                    self.error_at_previous("Expected literal value.");
                    return Err(Error::Parser);
                }
            }
        }
        if self.match_token(&[TokenType::LeftParen]) {
            let expr = self.expression()?;
            self.consume(TokenType::RightParen, "Expected ')' after expression.")?;
            return Ok(Expr::grouping(expr));
        }
        self.error_at_peek("Expected expression.");
        Err(Error::Parser)
    }
}

impl Parser {
    fn match_token(&mut self, types: &[TokenType]) -> bool {
        for token_type in types {
            if self.check(*token_type) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().token_type == token_type
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

    // Takes ownership of the previous token, leaving a stub (EOF, line 0) behind.
    // Operator tokens have no heap-allocated lexeme, so this is a cheap struct move.
    fn take_previous(&mut self) -> Token {
        let placeholder = Token::new(TokenType::EOF, None, None, 0);
        mem::replace(&mut self.tokens[self.current - 1], placeholder)
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> Result<&Token, Error> {
        if self.check(token_type) {
            return Ok(self.advance());
        }
        self.error_at_peek(message);
        Err(Error::Parser)
    }

    fn error_at_peek(&self, message: &str) {
        error_at_token(self.peek(), "Syntax", message);
    }

    fn error_at_previous(&self, message: &str) {
        error_at_token(self.previous(), "Syntax", message);
    }
}

impl Parser {
    fn statement(&mut self) -> Result<Stmt, Error> {
        if self.match_token(&[TokenType::Print]) {
            return self.print_statement();
        }
        self.expression_statement()
    }

    fn print_statement(&mut self) -> Result<Stmt, Error> {
        let value = self.expression()?;
        self.consume(TokenType::Semicolon, "Expected ';' after value.")?;
        Ok(Stmt::print_stmt(value))
    }

    fn expression_statement(&mut self) -> Result<Stmt, Error> {
        let expr = self.expression()?;
        self.consume(TokenType::Semicolon, "Expected ';' after expression.")?;
        Ok(Stmt::expression_stmt(expr))
    }

    fn synchronize(&mut self) {
        self.advance();
        while !self.is_at_end() {
            if self.previous().token_type == TokenType::Semicolon {
                return;
            }
            match self.peek().token_type {
                TokenType::Class | TokenType::Fun | TokenType::Var | TokenType::For | TokenType::If | TokenType::While | TokenType::Print | TokenType::Return => return,
                _ => (),
            }
            self.advance();
        }
    }
}

impl Parser {
    pub fn parse(&mut self) -> Result<Vec<Stmt>, Error> {
        self.current = 0;
        let mut statements: Vec<Stmt> = Vec::new();
        while !self.is_at_end() {
            match self.statement() {
                Ok(stmt) => statements.push(stmt),
                Err(e) => {
                    self.synchronize();
                    return Err(e);
                }
            }
        }
        Ok(statements)
    }
}
