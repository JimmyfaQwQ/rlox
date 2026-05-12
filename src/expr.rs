use std::fmt::Debug;

use crate::token;

pub struct AssignExpr {
    pub name: token::Token,
    pub value: Box<Expr>,
}

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
pub struct LogicalExpr {
    pub left: Box<Expr>,
    pub operator: token::Token,
    pub right: Box<Expr>,
}

pub struct UnaryExpr {
    pub operator: token::Token,
    pub right: Box<Expr>,
}

pub struct VariableExpr {
    pub name: token::Token,
}

pub enum Expr {
    BinaryExprs(BinaryExpr),
    GroupingExprs(GroupingExpr),
    LiteralExprs(LiteralExpr),
    LogicalExprs(LogicalExpr),
    UnaryExprs(UnaryExpr),
    VariableExprs(VariableExpr),
    AssignExprs(AssignExpr),
}

impl Expr {
    pub fn binary(left: Expr, operator: token::Token, right: Expr) -> Self {
        Expr::BinaryExprs(BinaryExpr {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        })
    }

    pub fn grouping(expression: Expr) -> Self {
        Expr::GroupingExprs(GroupingExpr {
            expression: Box::new(expression),
        })
    }

    pub fn literal(value: token::Literal) -> Self {
        Expr::LiteralExprs(LiteralExpr { value })
    }

    pub fn logical(left: Expr, operator: token::Token, right: Expr) -> Self {
        Expr::LogicalExprs(LogicalExpr {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        })
    }

    pub fn unary(operator: token::Token, right: Expr) -> Self {
        Expr::UnaryExprs(UnaryExpr {
            operator,
            right: Box::new(right),
        })
    }

    pub fn variable(name: token::Token) -> Self {
        Expr::VariableExprs(VariableExpr { name })
    }

    pub fn assign(name: token::Token, value: Expr) -> Self {
        Expr::AssignExprs(AssignExpr {
            name,
            value: Box::new(value),
        })
    }
}

impl Expr {
    pub fn pretty_print(&self) -> String {
        match self {
            Expr::BinaryExprs(binary) => format!("(operator({}) {} {})",
                binary.operator.lexeme(),
                binary.left.pretty_print(),
                binary.right.pretty_print()
            ),
            Expr::GroupingExprs(grouping) => format!("(group {})", grouping.expression.pretty_print()),
            Expr::LiteralExprs(literal) => format!("{}({:?})", literal.value.get_type(), literal.value),
            Expr::LogicalExprs(logical) => format!("(operator({}) {} {})",
                logical.operator.lexeme(),
                logical.left.pretty_print(),
                logical.right.pretty_print()
            ),
            Expr::UnaryExprs(unary) => format!("(operator({}) {})",
                unary.operator.lexeme(),
                unary.right.pretty_print()
            ),
            Expr::VariableExprs(variable) => format!("variable({})", variable.name.lexeme()),
            Expr::AssignExprs(assign) => format!("assign({} = {})", assign.name.lexeme(), assign.value.pretty_print()),
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
                token::Token::operator(token::TokenType::Minus, 1),
                Expr::literal(token::Literal::Number(123.0)),
            ),
            token::Token::operator(token::TokenType::Star, 1),
            Expr::grouping(
                Expr::literal(token::Literal::Number(45.67)),
            ),
        );
        assert_eq!(expr.pretty_print(), "(operator(*) (operator(-) number(123)) (group number(45.67)))");
    }
}
