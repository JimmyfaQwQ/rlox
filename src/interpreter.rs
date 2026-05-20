use crate::environment::Environment;
use crate::expr::Expr;
use crate::function::Function;
use crate::stmt::Stmt;
use crate::token::{Token, TokenType};
use crate::object::Object;
use crate::error::{error_at_token, Error};
use std::cell::RefCell;
use std::result::Result;
use std::rc::Rc;

pub struct Interpreter {
    pub environment: Rc<RefCell<Environment>>,
}

impl Interpreter {
    pub fn evaluate(&mut self, expr: &Expr) -> Result<Object, Error> {
        match expr {
            Expr::LiteralExprs(literal_expr) => Ok(literal_expr.value.clone()),
            Expr::GroupingExprs(grouping_expr) => self.evaluate(&grouping_expr.expression),
            Expr::UnaryExprs(unary_expr) => {
                let right = self.evaluate(&unary_expr.right)?;
                match unary_expr.operator.token_type {
                    TokenType::Minus => match right {
                        Object::Number(n) => Ok(Object::Number(-n)),
                        _ => Err(type_mismatch_unary(&unary_expr.operator, "a number", &right)),
                    },
                    TokenType::Bang => Ok(Object::Boolean(!is_truthy(&right))),
                    _ => Err(error(&unary_expr.operator, &format!("Invalid unary operator: {}", unary_expr.operator.lexeme()))),
                }
            },
            Expr::BinaryExprs(binary_expr) => {
                let left = self.evaluate(&binary_expr.left)?;
                let right = self.evaluate(&binary_expr.right)?;
                let op = &binary_expr.operator;
                match op.token_type {
                    TokenType::Plus => match (&left, &right) {
                        (Object::Number(l), Object::Number(r)) => Ok(Object::Number(l + r)),
                        (Object::String(l), Object::String(r)) => Ok(Object::String(Rc::from(format!("{}{}", l, r)))),
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
                    TokenType::EqualEqual => Ok(Object::Boolean(left == right)),
                    TokenType::BangEqual  => Ok(Object::Boolean(left != right)),
                    _ => Err(error(op, &format!("Invalid binary operator: {}", op.lexeme()))),
                }
            },
            Expr::LogicalExprs(logical_expr) => {
                let left = self.evaluate(&logical_expr.left)?;
                if logical_expr.operator.token_type == TokenType::Or {
                    if is_truthy(&left) {
                        return Ok(left);
                    }
                } else {
                    if !is_truthy(&left) {
                        return Ok(left);
                    }
                }
                self.evaluate(&logical_expr.right)
            },
            Expr::VariableExprs(variable_expr) => {
                let name = variable_expr.name.lexeme();
                match self.environment.borrow().get(name) {
                    Ok(value) => Ok(value),
                    Err(msg) => Err(error(&variable_expr.name, &msg)),
                }
            },
            Expr::AssignExprs(assign_expr) => {
                let value = self.evaluate(&assign_expr.value)?;
                let name = assign_expr.name.lexeme();
                match self.environment.borrow_mut().assign(name, value.clone()) {
                    Ok(()) => Ok(value),
                    Err(msg) => Err(error(&assign_expr.name, &msg)),
                }
            },
            Expr::CallExprs(call_expr) => {
                let callee = self.evaluate(&call_expr.callee)?;
                let mut arguments = Vec::new();
                for arg in &call_expr.arguments {
                    arguments.push(self.evaluate(arg)?);
                }
                match callee {
                    Object::Callable(function) => {
                        if arguments.len() != function.arity() {
                            return Err(error(&call_expr.paren, &format!(
                                "Expected {} arguments but got {}.",
                                function.arity(),
                                arguments.len()
                            )));
                        }
                        function.call(self, arguments).map_err(|msg| error(&call_expr.paren, &msg))
                    },
                    _ => Err(error(&call_expr.paren, "Can only call functions and classes.")),
                }
            },
        }
    }

    pub fn interpret(&mut self, stmts: &[Stmt]) -> Result<(), Error> {
        for statement in stmts {
            self.execute(statement)?;
        }
        Ok(())
    }

    pub fn execute(&mut self, stmt: &Stmt) -> Result<(), Error> {
        match stmt {
            Stmt::Expression(expr_stmt) => {
                self.evaluate(&expr_stmt.expression)?;
                Ok(())
            },
            Stmt::Print(print_stmt) => {
                let value = self.evaluate(&print_stmt.expression)?;
                println!("{:?}", value);
                Ok(())
            },
            Stmt::Var(var_stmt) => {
                let value = if let Some(initializer) = &var_stmt.initializer {
                    self.evaluate(initializer)?
                } else {
                    Object::Nil
                };
                self.environment.borrow_mut().define(var_stmt.name.lexeme(), value);
                Ok(())
            },
            Stmt::Block(block_stmt) => {
                let new_env = Environment::new(Some(Rc::clone(&self.environment)));
                self.execute_block(&block_stmt.statements, new_env)
            },
            Stmt::If(if_stmt) => {
                let condition = self.evaluate(&if_stmt.condition)?;
                if is_truthy(&condition) {
                    self.execute(&if_stmt.then_branch)
                } else if let Some(else_branch) = &if_stmt.else_branch {
                    self.execute(else_branch)
                } else {
                    Ok(())
                }
            },
            Stmt::While(while_stmt) => {
                while is_truthy(&self.evaluate(&while_stmt.condition)?) {
                    self.execute(&while_stmt.body)?;
                } 
                Ok(())
            },
            Stmt::Function(function_stmt) => {
                let function = Function {
                    declaration: Rc::clone(function_stmt),
                    closure: Rc::clone(&self.environment),
                };
                self.environment.borrow_mut().define(function_stmt.name.lexeme(), Object::Callable(Rc::new(function)));
                Ok(())
            },
            Stmt::Return(return_stmt) => {
                let value = if let Some(expr) = &return_stmt.value {
                    Some(self.evaluate(expr)?)
                } else {
                    None
                };
                return Err(Error::Return(value));
            },
        }
    }

    pub fn execute_block(&mut self, statements: &[Stmt], env: Rc<RefCell<Environment>>) -> Result<(), Error> {
        let previous = std::mem::replace(&mut self.environment, env);
        let mut result = Ok(());
        for statement in statements {
            result = self.execute(statement);
            if result.is_err() {
                break;
            }
        }
        self.environment = previous;
        result
    }
}

fn is_truthy(literal: &Object) -> bool {
    match literal {
        Object::Nil => false,
        Object::Boolean(b) => *b,
        _ => true,
    }
}

fn numeric_binop<F>(op: &Token, left: &Object, right: &Object, f: F) -> Result<Object, Error>
where
    F: FnOnce(f64, f64) -> Result<f64, &'static str>,
{
    match (left, right) {
        (Object::Number(l), Object::Number(r)) => f(*l, *r)
            .map(Object::Number)
            .map_err(|msg| error(op, msg)),
        _ => Err(type_mismatch(op, "numbers", left, right)),
    }
}

fn numeric_compare<F>(op: &Token, left: &Object, right: &Object, f: F) -> Result<Object, Error>
where
    F: FnOnce(f64, f64) -> bool,
{
    match (left, right) {
        (Object::Number(l), Object::Number(r)) => Ok(Object::Boolean(f(*l, *r))),
        _ => Err(type_mismatch(op, "numbers", left, right)),
    }
}

fn type_mismatch(op: &Token, expected: &str, left: &Object, right: &Object) -> Error {
    error(op, &format!(
        "Operands must be {}, found: {}({:?}) and {}({:?})",
        expected, left.get_type(), left, right.get_type(), right,
    ))
}

fn type_mismatch_unary(op: &Token, expected: &str, value: &Object) -> Error {
    error(op, &format!(
        "Operand must be {}, found: {}({:?})",
        expected, value.get_type(), value,
    ))
}

fn error(token: &Token, message: &str) -> Error {
    error_at_token(token, "Runtime", message);
    Error::Runtime
}
