use std::collections::HashMap;
use std::rc::Rc;
use crate::object::Object;
use crate::callable::Callable;

pub struct Enviorment {
    enclosing: Option<Box<Enviorment>>,
    values: HashMap<Box<str>, Object>,
}

struct ClockFn {}

impl Callable for ClockFn {
    fn arity(&self) -> usize {
        0
    }

    fn call(&self, _arguments: Vec<Object>) -> Result<Object, String> {
        Ok(Object::Number(std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs_f64()))
    }
}

fn native(env: Enviorment) -> Enviorment {
    let mut env = env;
    env.define("clock", Object::Callable(Rc::new(ClockFn {})));
    env
}

impl Enviorment {
    pub fn new(enclosing: Option<Box<Enviorment>>) -> Self {
        let mut env = Enviorment {
            enclosing,
            values: HashMap::new(),
        };
        env = native(env);
        env
    }

    pub fn take_enclosing(&mut self) -> Option<Box<Enviorment>> {
        self.enclosing.take()
    }

    pub fn define(&mut self, name: &str, value: impl Into<Object>) {
        self.values.insert(Box::from(name), value.into());
    }

    pub fn get(&mut self, name: &str) -> Result<&Object, String> {
        if !self.values.contains_key(name) {
            if let Some(enclosing) = &mut self.enclosing {
                return enclosing.get(name);
            }
            return Err(format!("Undefined variable '{}'.", name));
        }
        Ok(self.values.get(name).unwrap())
    }

    pub fn assign(&mut self, name: &str, value: impl Into<Object>) -> Result<(), String> {
        if !self.values.contains_key(name) {
            if let Some(enclosing) = &mut self.enclosing {
                return enclosing.assign(name, value);
            }
            return Err(format!("Undefined variable '{}'.", name));
        }
        self.values.insert(Box::from(name), value.into());
        Ok(())
    }
}

impl Default for Enviorment {
    fn default() -> Self {
        Enviorment::new(None)
    }
}