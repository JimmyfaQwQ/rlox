use std::{fmt::Debug};

use crate::token;

pub struct BinaryExpr {
    pub left: Box<Expr>,
    pub operator: token::Token,
    pub right: Box<Expr>,
} 

pub struct GroupingExpr {
    pub expression: Box<Expr>,
}

pub struct LiteralExpr {
    pub value: token::Literal,
}

pub struct UnaryExpr {
    pub operator: token::Token,
    pub right: Box<Expr>
}

pub enum Expr {
    BinaryExprs(BinaryExpr),
    GroupingExprs(GroupingExpr),
    LiteralExprs(LiteralExpr),
    UnaryExprs(UnaryExpr),
}

impl Expr {
    pub fn binary(left: Expr, operator: token::Token, right: Expr) -> Self {
        Expr::BinaryExprs(BinaryExpr {
            left: Box::new(left),
            operator: operator,
            right: Box::new(right),
        })
    }

    pub fn grouping(expression: Expr) -> Self {
        Expr::GroupingExprs(GroupingExpr {
            expression: Box::new(expression),
        })
    }

    pub fn literal(value: token::Literal) -> Self {
        Expr::LiteralExprs(LiteralExpr {
            value: value,
        })
    }

    pub fn unary(operator: token::Token, right: Expr) -> Self {
        Expr::UnaryExprs(UnaryExpr {
            operator: operator,
            right: Box::new(right),
        })
    }
}

impl Expr {
    pub fn pretty_print(&self) -> String {
        match self {
            Expr::BinaryExprs(binary) => format!("(operator({}) {} {})", 
                binary.operator.lexeme.as_ref().unwrap(),
                binary.left.pretty_print(),
                binary.right.pretty_print()
            ),
            Expr::GroupingExprs(grouping) => format!("(group {})", grouping.expression.pretty_print()),
            Expr::LiteralExprs(literal) => format!("{}({:?})", literal.value.get_type(), literal.value),
            Expr::UnaryExprs(unary) => format!("(operator({}) {})", 
                unary.operator.lexeme.as_ref().unwrap(),
                unary.right.pretty_print()
            ),
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