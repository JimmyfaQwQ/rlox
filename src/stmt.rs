use crate::expr::Expr;
use crate::token::Token;

pub struct ExpressionStatement {
    pub expression: Expr,
}

pub struct PrintStatement {
    pub expression: Expr,
}

pub struct VarStatement {
    pub name: Token,
    pub initializer: Option<Expr>,
}

pub struct BlockStatement {
    pub statements: Vec<Stmt>,
}

pub struct IfStatement {
    pub condition: Expr,
    pub then_branch: Box<Stmt>,
    pub else_branch: Option<Box<Stmt>>,
}

pub struct WhileStatement {
    pub condition: Expr,
    pub body: Box<Stmt>,
}

pub enum Stmt {
    Expression(ExpressionStatement),
    Print(PrintStatement),
    Var(VarStatement),
    Block(BlockStatement),
    If(IfStatement),
    While(WhileStatement),
}

impl Stmt {
    pub fn expression_stmt(expression: Expr) -> Self {
        Stmt::Expression(ExpressionStatement { expression })
    }

    pub fn print_stmt(expression: Expr) -> Self {
        Stmt::Print(PrintStatement { expression })
    }

    pub fn block_stmt(statements: Vec<Stmt>) -> Self {
        Stmt::Block(BlockStatement { statements })
    }

    pub fn var_stmt(name: Token, initializer: Option<Expr>) -> Self {
        Stmt::Var(VarStatement { name, initializer })
    }

    pub fn if_stmt(condition: Expr, then_branch: Stmt, else_branch: Option<Stmt>) -> Self {
        Stmt::If(IfStatement {
            condition,
            then_branch: Box::new(then_branch),
            else_branch: else_branch.map(Box::new),
        })
    }

    pub fn while_stmt(condition: Expr, body: Stmt) -> Self {
        Stmt::While(WhileStatement {
            condition,
            body: Box::new(body),
        })
    }
}
