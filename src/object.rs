use std::fmt::Display;
use crate::error::RuntimeError;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Object {
    Integer(i64),
    Boolean(bool),
    Symbol(String),
    Function(Vec<String>, Vec<Object>),
    PrimitiveFunction(String, usize),
    SpecialForm(String, Option<usize>),
    List(Vec<Object>),
}

impl Object {
    pub fn into_integer(self) -> Result<i64, RuntimeError> {
        match self {
            Object::Integer(i) => Ok(i),
            _ => Err(RuntimeError {
                message: "Expected integer".to_string(),
            }),
        }
    }

    pub fn into_boolean(self) -> Result<bool, RuntimeError> {
        match self {
            Object::Boolean(b) => Ok(b),
            _ => Err(RuntimeError {
                message: "Expected boolean".to_string(),
            }),
        }
    }

    pub fn into_symbol(self) -> Result<String, RuntimeError> {
        match self {
            Object::Symbol(s) => Ok(s),
            _ => Err(RuntimeError {
                message: "Expected symbol".to_string(),
            }),
        }
    }

    pub fn into_list(self) -> Result<Vec<Object>, RuntimeError> {
        match self {
            Object::List(l) => Ok(l),
            _ => Err(RuntimeError {
                message: "Expected list".to_string(),
            }),
        }
    }
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Object::Integer(i) => write!(f, "{}", i),
            Object::Boolean(b) => write!(f, "{}", b),
            Object::Symbol(s) => write!(f, "{}", s),
            Object::Function(params, body) => {
                let body = body.iter().map(|o| format!("{}", o)).collect::<Vec<String>>().join(" ");
                write!(f, "(fn ({}) ({}))", params.join(" "), body)
            }
            Object::PrimitiveFunction(s, arity) => write!(f, "{}/{}", s, arity),
            Object::SpecialForm(s, arity) => write!(f, "{}/{}", s, arity.map(|i| i.to_string()).unwrap_or("*".to_string())),
            Object::List(l) => {
                let s = l.iter().map(|o| format!("{}", o)).collect::<Vec<String>>().join(" ");
                write!(f, "({})", s)
            }
        }
    }
}
