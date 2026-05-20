use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use crate::interpreter;
use crate::object::Object;
use crate::callable::Callable;

pub struct Enviorment {
    enclosing: Option<Rc<RefCell<Enviorment>>>,
    values: HashMap<Box<str>, Object>,
}

struct ClockFn {}

impl Callable for ClockFn {
    fn arity(&self) -> usize {
        0
    }

    fn call(&self, _interpreter: &mut interpreter::Interpreter, _arguments: Vec<Object>) -> Result<Object, String> {
        Ok(Object::Number(std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs_f64()))
    }
}

impl Enviorment {
    pub fn new(enclosing: Option<Rc<RefCell<Enviorment>>>) -> Rc<RefCell<Self>> {
        let mut env = Enviorment {
            enclosing,
            values: HashMap::new(),
        };
        if env.enclosing.is_none() {
            env.define("clock", Object::Callable(Rc::new(ClockFn {})));
        }
        Rc::new(RefCell::new(env))
    }

    pub fn define(&mut self, name: &str, value: impl Into<Object>) {
        self.values.insert(Box::from(name), value.into());
    }

    pub fn get(&self, name: &str) -> Result<Object, String> {
        if let Some(value) = self.values.get(name) {
            return Ok(value.clone());
        }
        if let Some(enclosing) = &self.enclosing {
            return enclosing.borrow().get(name);
        }
        Err(format!("Undefined variable '{}'.", name))
    }

    pub fn assign(&mut self, name: &str, value: impl Into<Object>) -> Result<(), String> {
        if let Some(slot) = self.values.get_mut(name) {
            *slot = value.into();
            return Ok(());
        }
        if let Some(enclosing) = &self.enclosing {
            return enclosing.borrow_mut().assign(name, value);
        }
        Err(format!("Undefined variable '{}'.", name))
    }
}