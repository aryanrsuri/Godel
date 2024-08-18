use crate::lexer;
pub type Identifier = lexer::Token;
#[derive(PartialEq, Clone, Debug)]
pub enum Prefix {
    Plus,
    Minus,
    Not,
}

#[derive(PartialEq, Clone, Debug)]
pub enum Infix {
    Plus,
    Minus,
    Divide,
    Multiply,
    Equal,
    NotEqual,
    GreaterThan,
    LessThan,
    Modulo,
}

#[derive(Debug)]
pub enum Statement {
    Let(String, Expression),
    Return(Expression),
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
    Ok(Box<Expression>),
    Error,
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
