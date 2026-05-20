use std::cell::RefCell;
use std::rc::Rc;

use crate::stmt::FunctionStatement;
use crate::callable::Callable;
use crate::object::Object;
use crate::enviorment::Enviorment;
use crate::interpreter::Interpreter;

pub struct Function {
    pub decleration: Rc<FunctionStatement>,
    pub closure: Rc<RefCell<Enviorment>>,
}

impl Callable for Function {
    fn arity(&self) -> usize {
        self.decleration.params.len()
    }

    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Object>) -> Result<Object, String> {
        let env = Enviorment::new(Some(Rc::clone(&self.closure)));
        for (param, arg) in self.decleration.params.iter().zip(arguments.into_iter()) {
            env.borrow_mut().define(param.lexeme(), arg);
        }
        match interpreter.execute_block(&self.decleration.body, env) {
            Ok(()) => Ok(Object::Nil),
            Err(crate::error::Error::Return(value)) => Ok(value.unwrap_or(Object::Nil)),
            Err(e) => Err(format!("Runtime error: {:?}", e)),
        }
    }
}