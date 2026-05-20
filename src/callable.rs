use std::rc::Rc;

use crate::{interpreter::Interpreter, object::Object};


pub trait Callable {
    fn arity(&self) -> usize;
    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Object>) -> Result<Object, String>;
}

impl Into<Object> for Rc<dyn Callable> {
    fn into(self) -> Object {
        Object::Callable(self)
    }
}
