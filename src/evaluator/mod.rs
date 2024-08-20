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
            _ => unreachable!("Only Integer works "),
        }
    }

    pub fn eval_literal(&mut self, literal: &Literal) -> Object {
        match literal {
            Literal::Integer(value) => Object::Integer(*value),
            _ => unreachable!("Only Ineger works "),
        }
    }
}
