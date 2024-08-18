use crate::lexer;
pub type Identifier = lexer::Token;
#[derive(PartialEq, Clone, Debug)]
pub enum Prefix {
    Plus,
    Minus,
    Not,
    Cardinal,
}

#[derive(PartialEq, Clone, Debug)]
pub enum Infix {
    Plus,
    Minus,
    Divide,
    Multiply,
    Exponent,
    Equal,
    NotEqual,
    GreaterThan,
    LessThan,
    Modulo,
    Cons,
}

#[derive(Debug)]
pub enum Statement {
    Let(String, Expression),
    Return(Expression),
    Comment(String),
    Expression(Expression),
}

#[derive(Debug)]
pub enum Literal {
    Integer(i64),
    String(String),
    Boolean(bool),
    List(Option<Vec<Expression>>),
}

#[derive(Debug)]
pub enum Expression {
    None,
    Identifier(Identifier),
    // TODO: Refactor Ok,Exxor, None to a `Types` enum child of Expression
    Ok(Box<Expression>),
    Error,
    Comment(String),
    Type(Vec<Identifier>),
    Literal(Literal),
    Prefix(Prefix, Box<Expression>),
    Infix(Infix, Box<Expression>, Box<Expression>),
    // for { x in [0..10] : x * x };
    // TODO: For loops
    // For()
    If {
        condition: Box<Expression>,
        consequence: Program,
        alternative: Option<Program>,
    },
    Fn {
        parameter: Vec<Identifier>,
        body: Program,
    },
    Call {
        map: Box<Expression>,
        domain: Vec<Expression>,
    },
}

pub type Program = Vec<Statement>;
