use super::*;
use std::fmt;
#[derive(Debug)]
pub enum Object {
    Null,
    Integer(i64),
    Boolean(bool),
    String(String),
    // TODO: List
    List(Vec<Object>),
    // TODO: Type (represented as an enum?)
    // Type(Vec<Identifier>),
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Object::Null => write!(f, "null"),
            Object::Integer(ref value) => write!(f, "{}", value),
            Object::Boolean(ref value) => write!(f, "{}", value),
            Object::String(ref value) => write!(f, "{}", value),
            Object::List(ref value) => write!(f, "{:?}", value),
        }
    }
}
