pub mod object;
use super::ast::*;
use object::*;

pub struct Evaluator {}

impl Evaluator {
    pub fn new() -> Evaluator {
        Evaluator {}
    }

    pub fn eval(&mut self, program: &Program) -> Option<Object> {
        let mut result: Option<Object> = None;
        for statement in program {
            match self.eval_statement(statement) {
                obj => result = obj,
                // _ => unreachable!("Only integer wors"),
            }
        }
        result
    }

    pub fn eval_statement(&mut self, statement: &Statement) -> Option<Object> {
        match statement {
            Statement::Expression(expression) => self.eval_expression(expression),
            _ => None,
        }
    }

    pub fn eval_expression(&mut self, expression: &Expression) -> Option<Object> {
        match expression {
            Expression::Literal(literal) => Some(self.eval_literal(literal)),
            Expression::Prefix(prefix, expression) => self
                .eval_expression(&expression)
                .map(|right| self.eval_prefix(prefix, right)),
            _ => unreachable!("Only Literal(int,bool,string) works"),
        }
    }

    pub fn eval_prefix(&mut self, prefix: &Prefix, object: Object) -> Object {
        match prefix {
            Prefix::Not => self.eval_not_prefix(object),
            Prefix::Plus => self.eval_plus_prefix(object),
            Prefix::Minus => self.eval_minus_prefix(object),
            Prefix::Cardinal => self.eval_cardinal_prefix(object),
        }
    }

    pub fn eval_not_prefix(&mut self, object: Object) -> Object {
        match object {
            Object::Boolean(true) => Object::Boolean(false),
            Object::Boolean(false) => Object::Boolean(true),
            Object::Null => Object::Boolean(true),
            _ => Object::Boolean(false),
        }
    }

    pub fn eval_plus_prefix(&mut self, object: Object) -> Object {
        match object {
            Object::Integer(value) => Object::Integer(value),
            _ => Object::Null,
        }
    }

    pub fn eval_minus_prefix(&mut self, object: Object) -> Object {
        match object {
            Object::Integer(value) => Object::Integer(-1 * value),
            _ => Object::Null,
        }
    }

    pub fn eval_cardinal_prefix(&mut self, object: Object) -> Object {
        match object {
            Object::List(value) => Object::Integer(value.len() as i64),
            _ => Object::Integer(1),
        }
    }

    pub fn eval_literal(&mut self, literal: &Literal) -> Object {
        match literal {
            Literal::Integer(value) => Object::Integer(*value),
            Literal::Boolean(value) => Object::Boolean(*value),
            Literal::String(value) => Object::String(value.clone()),
            _ => unreachable!("List/Hash doesn't work."),
        }
    }
}
