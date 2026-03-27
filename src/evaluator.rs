use crate::expr::Expr;
use crate::token::{TokenType, Literal};
use std::result::Result;
use std::rc::Rc;

pub fn evaluate(expr: &Expr) -> Result<Literal, &'static str> {
    match expr {
        Expr::LiteralExprs(literal_expr) => Ok(literal_expr.value.clone()),
        Expr::GroupingExprs(grouping_expr) => evaluate(&grouping_expr.expression),
        Expr::UnaryExprs(unary_expr) => {
            let right = evaluate(&unary_expr.right)?;
            match unary_expr.operator.token_type {
                TokenType::Minus => {
                    if let Literal::Number(n) = right {
                        Ok(Literal::Number(-n))
                    } else {
                        Err("Operand must be a number.")
                    }
                },
                TokenType::Bang => Ok(Literal::Boolean(!is_truthy(&right))),
                _ => Err("Invalid unary operator."),
            }
        },
        Expr::BinaryExprs(binary_expr) => {
            let left = evaluate(&binary_expr.left)?;
            let right = evaluate(&binary_expr.right)?;
            match binary_expr.operator.token_type {
                TokenType::Plus => {
                    match (left, right) {
                        (Literal::Number(l), Literal::Number(r)) => Ok(Literal::Number(l + r)),
                        (Literal::String(l), Literal::String(r)) => Ok(Literal::String(Rc::from(format!("{}{}", l, r)))),
                        _ => Err("Operands must be two numbers or two strings."),
                    }
                },
                TokenType::Minus => {
                    if let (Literal::Number(l), Literal::Number(r)) = (left, right) {
                        Ok(Literal::Number(l - r))
                    } else {
                        Err("Operands must be numbers.")
                    }
                },
                TokenType::Star => {
                    if let (Literal::Number(l), Literal::Number(r)) = (left, right) {
                        Ok(Literal::Number(l * r))
                    } else {
                        Err("Operands must be numbers.")
                    }
                },
                TokenType::Slash => {
                    if let (Literal::Number(l), Literal::Number(r)) = (left, right) {
                        if r == 0.0 {
                            Err("Division by zero.")
                        } else {
                            Ok(Literal::Number(l / r))
                        }
                    } else {
                        Err("Operands must be numbers.")
                    }
                },
                _ => Err("Invalid binary operator."),
            }
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