use crate::expr::Expr;
use crate::stmt::Stmt;
use crate::token::{Token, TokenType, Literal};
use crate::error::{error_at_token};
use std::result::Result;
use std::rc::Rc;

fn evaluate(expr: &Expr) -> Result<Literal, Rc<str>> {
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
                        let error_message = format!("Operand must be a number, found: {}({:?})", right.get_type(), right);
                        Err(error_from_string(&unary_expr.operator, error_message))
                    }
                },
                TokenType::Bang => Ok(Literal::Boolean(!is_truthy(&right))),
                _ => Err(error_from_string(&unary_expr.operator, format!("Invalid unary operator: {}", unary_expr.operator.lexeme.as_ref().unwrap()))),
            }
        },
        Expr::BinaryExprs(binary_expr) => {
            let left = evaluate(&binary_expr.left)?;
            let right = evaluate(&binary_expr.right)?;
            match binary_expr.operator.token_type {
                TokenType::Plus => {
                    match (&left, &right) {
                        (Literal::Number(l), Literal::Number(r)) => Ok(Literal::Number(l + r)),
                        (Literal::String(l), Literal::String(r)) => Ok(Literal::String(Rc::from(format!("{}{}", l, r)))),
                        _ => Err(error_from_string(&binary_expr.operator, format!("Operands must be two numbers or two strings, found: {}({:?}) and {}({:?})", left.get_type(), left, right.get_type(), right))),
                    }
                },
                TokenType::Minus => {
                    if let (Literal::Number(l), Literal::Number(r)) = (&left, &right) {
                        Ok(Literal::Number(l - r))
                    } else {
                        Err(error_from_string(&binary_expr.operator, format!("Operands must be numbers, found: {}({:?}) and {}({:?})", left.get_type(), left, right.get_type(), right)))
                    }
                },
                TokenType::Star => {
                    if let (Literal::Number(l), Literal::Number(r)) = (&left, &right) {
                        Ok(Literal::Number(l * r))
                    } else {
                        Err(error_from_string(&binary_expr.operator, format!("Operands must be numbers, found: {}({:?}) and {}({:?})", left.get_type(), left, right.get_type(), right)))
                    }
                },
                TokenType::Slash => {
                    if let (Literal::Number(l), Literal::Number(r)) = (&left, &right) {
                        if *r == 0.0 {
                            Err(error(&binary_expr.operator, "Division by zero."))
                        } else {
                            Ok(Literal::Number(l / r))
                        }
                    } else {
                        Err(error_from_string(&binary_expr.operator, format!("Operands must be numbers, found: {}({:?}) and {}({:?})", left.get_type(), left, right.get_type(), right)))
                    }
                },
                TokenType::EqualEqual => {
                    if left.get_type() == right.get_type() {
                        Ok(Literal::Boolean(left == right))
                    } else {
                        Err(error_from_string(&binary_expr.operator, format!("Operands must be of the same type for equality comparison, found: {}({:?}) and {}({:?})", left.get_type(), left, right.get_type(), right)))
                    }
                },
                TokenType::BangEqual => {
                    if left.get_type() == right.get_type() {
                        Ok(Literal::Boolean(left != right))
                    } else {
                        Err(error_from_string(&binary_expr.operator, format!("Operands must be of the same type for inequality comparison, found: {}({:?}) and {}({:?})", left.get_type(), left, right.get_type(), right)))
                    }
                },
                TokenType::Greater => {
                    if let (Literal::Number(l), Literal::Number(r)) = (&left, &right) {
                        Ok(Literal::Boolean(l > r))
                    } else {
                        Err(error_from_string(&binary_expr.operator, format!("Operands must be numbers for '>' comparison, found: {}({:?}) and {}({:?})", left.get_type(), left, right.get_type(), right)))
                    }
                },
                TokenType::GreaterEqual => {
                    if let (Literal::Number(l), Literal::Number(r)) = (&left, &right) {
                        Ok(Literal::Boolean(l >= r))
                    } else {
                        Err(error_from_string(&binary_expr.operator, format!("Operands must be numbers for '>=' comparison, found: {}({:?}) and {}({:?})", left.get_type(), left, right.get_type(), right)))
                    }
                },
                TokenType::Less => {
                    if let (Literal::Number(l), Literal::Number(r)) = (&left, &right) {
                        Ok(Literal::Boolean(l < r))
                    } else {
                        Err(error_from_string(&binary_expr.operator, format!("Operands must be numbers for '<' comparison, found: {}({:?}) and {}({:?})", left.get_type(), left, right.get_type(), right)))
                    }
                },
                TokenType::LessEqual => {
                    if let (Literal::Number(l), Literal::Number(r)) = (&left, &right) {
                        Ok(Literal::Boolean(l <= r))
                    } else {
                        Err(error_from_string(&binary_expr.operator, format!("Operands must be numbers for '<=' comparison, found: {}({:?}) and {}({:?})", left.get_type(), left, right.get_type(), right)))
                    }
                },
                _ => Err(error_from_string(&binary_expr.operator, format!("Invalid binary operator: {}", binary_expr.operator.lexeme.as_ref().unwrap()))),
            }
        },
    }
}

pub fn interpret(stmt: Rc<[Stmt]>) -> Result<(), Rc<str>> {
    for statement in stmt.iter() {
        execute(statement)?;
    }
    Ok(())
}

fn execute(stmt: &Stmt) -> Result<(), Rc<str>> {
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

fn error<'a> (token: &Token, message: &'a str) -> Rc<str> {
    error_at_token(token, "Runtime", message);
    Rc::from(message)
}

fn error_from_string(token: &Token, message: String) -> Rc<str> {
    error_at_token(token, "Runtime", message.as_str());
    Rc::from(message)
}