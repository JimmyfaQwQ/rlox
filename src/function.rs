use std::cell::RefCell;
use std::rc::Rc;

use crate::stmt::FunctionStatement;
use crate::callable::Callable;
use crate::object::Object;
use crate::environment::Environment;
use crate::interpreter::Interpreter;

pub struct Function {
    pub declaration: Rc<FunctionStatement>,
    pub closure: Rc<RefCell<Environment>>,
}

impl Callable for Function {
    fn arity(&self) -> usize {
        self.declaration.params.len()
    }

    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Object>) -> Result<Object, String> {
        let env = Environment::new(Some(Rc::clone(&self.closure)));
        for (param, arg) in self.declaration.params.iter().zip(arguments.into_iter()) {
            env.borrow_mut().define(param.lexeme(), arg);
        }
        match interpreter.execute_block(&self.declaration.body, env) {
            Ok(()) => Ok(Object::Nil),
            Err(crate::error::Error::Return(value)) => Ok(value.unwrap_or(Object::Nil)),
            Err(e) => Err(format!("Runtime error: {:?}", e)),
        }
    }
}