use std::rc::Rc;


#[derive(Clone)]
pub enum Object {
    String(Rc<str>),
    Number(f64),
    Boolean(bool),
    Nil,
    Callable(Rc<dyn crate::callable::Callable>),
}

impl Object {
    pub fn get_type(&self) -> &'static str {
        match self {
            Object::String(_) => "string",
            Object::Number(_) => "number",
            Object::Boolean(_) => "boolean",
            Object::Nil => "nil",
            Object::Callable(_) => "callable",
        }
    }
}

impl From<&str> for Object {
    fn from(value: &str) -> Self {
        Object::String(Rc::from(value))
    }
}

impl From<f64> for Object {
    fn from(value: f64) -> Self {
        Object::Number(value)
    }
}

impl From<i32> for Object {
    fn from(value: i32) -> Self {
        Object::Number(value as f64)
    }
}

impl From<bool> for Object {
    fn from(value: bool) -> Self {
        Object::Boolean(value)
    }
}

impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Object::String(s1), Object::String(s2)) => s1 == s2,
            (Object::Number(n1), Object::Number(n2)) => n1 == n2,
            (Object::Boolean(b1), Object::Boolean(b2)) => b1 == b2,
            (Object::Nil, Object::Nil) => true,
            (Object::Callable(a), Object::Callable(b)) => Rc::ptr_eq(a, b),
            _ => false,
        }
    }
}

impl Default for Object {
    fn default() -> Self {
        Object::Nil
    }
}

impl std::fmt::Debug for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::String(s) => write!(f, "{}", s),
            Object::Number(n) => write!(f, "{}", n),
            Object::Boolean(b) => write!(f, "{}", b),
            Object::Nil => write!(f, "nil"),
            Object::Callable(_) => write!(f, "<callable>"),
        }
    }
}