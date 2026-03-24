use std::{fmt::Debug};

use crate::token;

pub struct Binary<'a> {
    pub left: Box<Expr<'a>>,
    pub operator: token::Token<'a>,
    pub right: Box<Expr<'a>>,
} 

pub struct Grouping<'a> {
    pub expression: Box<Expr<'a>>,
}

pub struct Literal<'a> {
    pub value: token::Literal<'a>,
}

pub struct Unary<'a> {
    pub operator: token::Token<'a>,
    pub right: Box<Expr<'a>>
}

pub enum Expr<'a> {
    Binary(Binary<'a>),
    Grouping(Grouping<'a>),
    Literal(Literal<'a>),
    Unary(Unary<'a>),
}

impl<'a> Expr<'a> {
    pub fn binary(left: Expr<'a>, operator: token::Token<'a>, right: Expr<'a>) -> Self {
        Expr::Binary(Binary {
            left: Box::new(left),
            operator: operator,
            right: Box::new(right),
        })
    }

    pub fn grouping(expression: Expr<'a>) -> Self {
        Expr::Grouping(Grouping {
            expression: Box::new(expression),
        })
    }

    pub fn literal(value: token::Literal<'a>) -> Self {
        Expr::Literal(Literal {
            value: value,
        })
    }

    pub fn unary(operator: token::Token<'a>, right: Expr<'a>) -> Self {
        Expr::Unary(Unary {
            operator: operator,
            right: Box::new(right),
        })
    }
}

impl<'a> Expr<'a> {
    pub fn pretty_print(&self) -> String {
        match self {
            Expr::Binary(binary) => format!("({} {} {})", binary.operator.lexeme.as_ref().unwrap(), binary.left.pretty_print(), binary.right.pretty_print()),
            Expr::Grouping(grouping) => format!("(group {})", grouping.expression.pretty_print()),
            Expr::Literal(literal) => format!("{:?}", literal.value),
            Expr::Unary(unary) => format!("({} {})", unary.operator.lexeme.as_ref().unwrap(), unary.right.pretty_print()),
        }
    }
}

impl<'a> Debug for Expr<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.pretty_print())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expr_pretty_print() {
        let expr = Expr::binary(
            Expr::unary(
                token::Token {
                    token_type: token::TokenType::Minus,
                    lexeme: Some("-"),
                    literal: None,
                    line: 1,
                },
                Expr::literal(token::Literal::Number(123.0)),
            ),
            token::Token {
                token_type: token::TokenType::Star,
                lexeme: Some("*"),
                literal: None,
                line: 1,
            },
            Expr::grouping(
                Expr::literal(token::Literal::Number(45.67)),
            ),
        );
        assert_eq!(expr.pretty_print(), "(* (- 123) (group 45.67))");
    }
}