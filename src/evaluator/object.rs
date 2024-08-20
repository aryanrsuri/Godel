use super::*;
#[derive(Debug)]
pub enum Object {
    Null,
    Integer(i64),
    Boolean(bool),
    String(String),
    // TODO: List
    List(Vec<Object>),
    // TODO: Type (represented as an enum?)
    Type(Vec<Identifier>),
}

impl Object {
    pub fn inspect(&self) {
        return match self {
            Object::Integer(value) => println!("OK {:?}", value),
            Object::Boolean(value) => println!("OK {:?}", value),
            Object::String(value) => println!("OK {:?}", value),
            _ => println!("ERR Object Not Parsed"),
        };
    }
}
