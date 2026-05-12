use std::collections::HashMap;
use crate::token::Literal;

pub struct Enviorment {
    enclosing: Option<Box<Enviorment>>,
    values: HashMap<Box<str>, Literal>,
}

impl Enviorment {
    pub fn new(enclosing: Option<Box<Enviorment>>) -> Self {
        Enviorment {
            enclosing,
            values: HashMap::new(),
        }
    }

    pub fn take_enclosing(&mut self) -> Option<Box<Enviorment>> {
        self.enclosing.take()
    }

    pub fn define(&mut self, name: &str, value: impl Into<Literal>) {
        self.values.insert(Box::from(name), value.into());
    }

    pub fn get(&mut self, name: &str) -> Result<&Literal, String> {
        if !self.values.contains_key(name) {
            if let Some(enclosing) = &mut self.enclosing {
                return enclosing.get(name);
            }
            return Err(format!("Undefined variable '{}'.", name));
        }
        Ok(self.values.get(name).unwrap())
    }

    pub fn assign(&mut self, name: &str, value: impl Into<Literal>) -> Result<(), String> {
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