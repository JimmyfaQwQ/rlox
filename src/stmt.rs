use crate::expr::Expr;

pub struct ExpressionStatement {
    pub expression: Expr,
}

pub struct PrintStatement {
    pub expression: Expr,
}

pub enum Stmt {
    Expression(ExpressionStatement),
    Print(PrintStatement),
}

impl Stmt {
    pub fn expression_stmt(expression: Expr) -> Self {
        Stmt::Expression(ExpressionStatement { expression })
    }

    pub fn print_stmt(expression: Expr) -> Self {
        Stmt::Print(PrintStatement { expression })
    }
}