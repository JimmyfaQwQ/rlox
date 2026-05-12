use crate::expr::Expr;
use crate::stmt::Stmt;
use crate::token::{Token, TokenType, Literal};
use crate::error::{error_at_token, Error};
use std::result::Result;
use std::rc::Rc;

fn evaluate(expr: &Expr) -> Result<Literal, Error> {
    match expr {
        Expr::LiteralExprs(literal_expr) => Ok(literal_expr.value.clone()),
        Expr::GroupingExprs(grouping_expr) => evaluate(&grouping_expr.expression),
        Expr::UnaryExprs(unary_expr) => {
            let right = evaluate(&unary_expr.right)?;
            match unary_expr.operator.token_type {
                TokenType::Minus => match right {
                    Literal::Number(n) => Ok(Literal::Number(-n)),
                    _ => Err(type_mismatch_unary(&unary_expr.operator, "a number", &right)),
                },
                TokenType::Bang => Ok(Literal::Boolean(!is_truthy(&right))),
                _ => Err(error(&unary_expr.operator, &format!("Invalid unary operator: {}", unary_expr.operator.lexeme()))),
            }
        },
        Expr::BinaryExprs(binary_expr) => {
            let left = evaluate(&binary_expr.left)?;
            let right = evaluate(&binary_expr.right)?;
            let op = &binary_expr.operator;
            match op.token_type {
                TokenType::Plus => match (&left, &right) {
                    (Literal::Number(l), Literal::Number(r)) => Ok(Literal::Number(l + r)),
                    (Literal::String(l), Literal::String(r)) => Ok(Literal::String(Rc::from(format!("{}{}", l, r)))),
                    _ => Err(type_mismatch(op, "two numbers or two strings", &left, &right)),
                },
                TokenType::Minus => numeric_binop(op, &left, &right, |l, r| Ok(l - r)),
                TokenType::Star  => numeric_binop(op, &left, &right, |l, r| Ok(l * r)),
                TokenType::Slash => numeric_binop(op, &left, &right, |l, r| {
                    if r == 0.0 { Err("Division by zero.") } else { Ok(l / r) }
                }),
                TokenType::Greater      => numeric_compare(op, &left, &right, |l, r| l >  r),
                TokenType::GreaterEqual => numeric_compare(op, &left, &right, |l, r| l >= r),
                TokenType::Less         => numeric_compare(op, &left, &right, |l, r| l <  r),
                TokenType::LessEqual    => numeric_compare(op, &left, &right, |l, r| l <= r),
                TokenType::EqualEqual => Ok(Literal::Boolean(left == right)),
                TokenType::BangEqual  => Ok(Literal::Boolean(left != right)),
                _ => Err(error(op, &format!("Invalid binary operator: {}", op.lexeme()))),
            }
        },
    }
}

pub fn interpret(stmts: &[Stmt]) -> Result<(), Error> {
    for statement in stmts {
        execute(statement)?;
    }
    Ok(())
}

fn execute(stmt: &Stmt) -> Result<(), Error> {
    match stmt {
        Stmt::Expression(expr_stmt) => {
            evaluate(&expr_stmt.expression)?;
            Ok(())
        },
        Stmt::Print(print_stmt) => {
            let value = evaluate(&print_stmt.expression)?;
            println!("{:?}", value);
            Ok(())
        },
    }
}

fn is_truthy(literal: &Literal) -> bool {
    match literal {
        Literal::Nil => false,
        Literal::Boolean(b) => *b,
        _ => true,
    }
}

fn numeric_binop<F>(op: &Token, left: &Literal, right: &Literal, f: F) -> Result<Literal, Error>
where
    F: FnOnce(f64, f64) -> Result<f64, &'static str>,
{
    match (left, right) {
        (Literal::Number(l), Literal::Number(r)) => f(*l, *r)
            .map(Literal::Number)
            .map_err(|msg| error(op, msg)),
        _ => Err(type_mismatch(op, "numbers", left, right)),
    }
}

fn numeric_compare<F>(op: &Token, left: &Literal, right: &Literal, f: F) -> Result<Literal, Error>
where
    F: FnOnce(f64, f64) -> bool,
{
    match (left, right) {
        (Literal::Number(l), Literal::Number(r)) => Ok(Literal::Boolean(f(*l, *r))),
        _ => Err(type_mismatch(op, "numbers", left, right)),
    }
}

fn type_mismatch(op: &Token, expected: &str, left: &Literal, right: &Literal) -> Error {
    error(op, &format!(
        "Operands must be {}, found: {}({:?}) and {}({:?})",
        expected, left.get_type(), left, right.get_type(), right,
    ))
}

fn type_mismatch_unary(op: &Token, expected: &str, value: &Literal) -> Error {
    error(op, &format!(
        "Operand must be {}, found: {}({:?})",
        expected, value.get_type(), value,
    ))
}

fn error(token: &Token, message: &str) -> Error {
    error_at_token(token, "Runtime", message);
    Error::Runtime
}
