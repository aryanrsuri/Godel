pub mod object;
use super::ast::*;
use object::*;

pub struct Evaluator {}

impl Evaluator {
    pub fn new() -> Evaluator {
        Evaluator {}
    }

    fn is_truthy(object: Object) -> bool {
        match object {
            Object::Null => false,
            Object::Boolean(true) => true,
            Object::Boolean(false) => false,
            _ => true,
        }
    }

    pub fn eval(&mut self, program: &Program) -> Option<Object> {
        let mut result: Option<Object> = None;
        for statement in program {
            match self.eval_statement(statement) {
                Some(Object::Return(value)) => return Some(*value),
                obj => result = obj,
                // _ => unreachable!("Only integer wors"),
            }
        }
        result
    }

    pub fn eval_statement(&mut self, statement: &Statement) -> Option<Object> {
        match statement {
            Statement::Expression(expression) => self.eval_expression(expression),
            Statement::Return(expression) => match self.eval_expression(expression) {
                Some(value) => Some(Object::Return(Box::new(value))),
                None => return None,
            },
            _ => None,
        }
    }

    pub fn eval_expression(&mut self, expression: &Expression) -> Option<Object> {
        match expression {
            Expression::Literal(literal) => Some(self.eval_literal(literal)),
            Expression::Prefix(prefix, expression) => self
                .eval_expression(&expression)
                .map(|right| self.eval_prefix(prefix, right)),
            Expression::Infix(infix, left_expression, right_expression) => {
                let left = self.eval_expression(left_expression);
                let right = self.eval_expression(right_expression);
                if left.is_some() && right.is_some() {
                    Some(self.eval_infix(infix, left.unwrap(), right.unwrap()))
                } else {
                    None
                }
            }
            Expression::If {
                condition,
                consequence,
                alternative,
            } => self.eval_if(condition, consequence, alternative),
            _ => unreachable!("Only Literal(int,bool,string) works"),
        }
    }
    pub fn eval_if(
        &mut self,
        condition: &Expression,
        consequence: &Program,
        alternative: &Option<Program>,
    ) -> Option<Object> {
        let cond = match self.eval_expression(condition) {
            Some(cond) => cond,
            None => return None,
        };

        if Self::is_truthy(cond) {
            return self.eval_block(consequence);
        } else if let Some(alt) = alternative {
            self.eval_block(alt)
        } else {
            None
        }
    }

    pub fn eval_block(&mut self, program: &Program) -> Option<Object> {
        let mut result = None;
        for statement in program {
            match self.eval_statement(statement) {
                Some(Object::Return(value)) => return Some(Object::Return(value)),
                obj => result = obj,
            }
        }
        result
    }

    pub fn eval_infix(&mut self, infix: &Infix, left: Object, right: Object) -> Object {
        match left {
            Object::Integer(left_int) => {
                if let Object::Integer(right_int) = right {
                    self.eval_infix_int(infix, left_int, right_int)
                } else {
                    Object::Null
                }
            }
            Object::Boolean(left_bool) => {
                if let Object::Boolean(right_bool) = right {
                    match infix {
                        Infix::Equal => Object::Boolean(left_bool == right_bool),
                        Infix::NotEqual => Object::Boolean(left_bool != right_bool),
                        _ => Object::Null,
                    }
                } else {
                    Object::Null
                }
            }
            _ => Object::Null,
        }
    }

    pub fn eval_infix_int(&mut self, infix: &Infix, left_int: i64, right_int: i64) -> Object {
        match infix {
            Infix::Plus => Object::Integer(left_int + right_int),
            Infix::Minus => Object::Integer(left_int - right_int),
            Infix::Multiply => Object::Integer(left_int * right_int),
            Infix::Divide => Object::Integer(left_int / right_int),
            Infix::Modulo => Object::Integer(left_int % right_int),
            Infix::Exponent => Object::Integer(left_int.pow(right_int as u32)),
            Infix::LessThan => Object::Boolean(left_int < right_int),
            Infix::GreaterThan => Object::Boolean(left_int > right_int),
            Infix::Equal => Object::Boolean(left_int == right_int),
            Infix::NotEqual => Object::Boolean(left_int != right_int),
            Infix::Cons => Object::Null,
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
