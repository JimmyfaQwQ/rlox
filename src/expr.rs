use std::{fmt::Debug};

use crate::token;

pub struct Binary {
    pub left: Box<Expr>,
    pub operator: token::Token,
    pub right: Box<Expr>,
} 

pub struct Grouping {
    pub expression: Box<Expr>,
}

pub struct Literal {
    pub value: token::Literal,
}

pub struct Unary {
    pub operator: token::Token,
    pub right: Box<Expr>
}

pub enum Expr {
    Binary(Binary),
    Grouping(Grouping),
    Literal(Literal),
    Unary(Unary),
}

impl Expr {
    pub fn binary(left: Expr, operator: token::Token, right: Expr) -> Self {
        Expr::Binary(Binary {
            left: Box::new(left),
            operator: operator,
            right: Box::new(right),
        })
    }

    pub fn grouping(expression: Expr) -> Self {
        Expr::Grouping(Grouping {
            expression: Box::new(expression),
        })
    }

    pub fn literal(value: token::Literal) -> Self {
        Expr::Literal(Literal {
            value: value,
        })
    }

    pub fn unary(operator: token::Token, right: Expr) -> Self {
        Expr::Unary(Unary {
            operator: operator,
            right: Box::new(right),
        })
    }
}

impl Expr {
    pub fn pretty_print(&self) -> String {
        match self {
            Expr::Binary(binary) => format!("({} {} {})", binary.operator.lexeme.as_ref().unwrap(), binary.left.pretty_print(), binary.right.pretty_print()),
            Expr::Grouping(grouping) => format!("(group {})", grouping.expression.pretty_print()),
            Expr::Literal(literal) => format!("{:?}", literal.value),
            Expr::Unary(unary) => format!("({} {})", unary.operator.lexeme.as_ref().unwrap(), unary.right.pretty_print()),
        }
    }
}

impl Debug for Expr {
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
                token::Token::operator(token::TokenType::Minus, Some("-"), 1),
                Expr::literal(token::Literal::Number(123.0)),
            ),
            token::Token::operator(token::TokenType::Star, Some("*"), 1),
            Expr::grouping(
                Expr::literal(token::Literal::Number(45.67)),
            ),
        );
        assert_eq!(expr.pretty_print(), "(* (- 123) (group 45.67))");
    }
}