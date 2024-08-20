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

