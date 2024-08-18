use std::borrow::Borrow;

use crate::{ast::*, lexer::*};
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Precendence {
    Lowest,
    Equals,
    Comparison,
    Sum,
    Product,
    Prefix,
    Call,
}

pub fn token_to_precedence(token: &Token) -> Precendence {
    match token {
        Token::Equal | Token::Notequal => Precendence::Equals,
        Token::Lt | Token::Gt => Precendence::Comparison,
        Token::Plus | Token::Minus => Precendence::Sum,
        Token::Fslash | Token::Asterisk | Token::Modulo => Precendence::Product,
        _ => Precendence::Lowest,
    }
}

#[derive(Debug)]
pub struct Parser {
    lexer: Lexer,
    current: Token,
    peek: Token,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Parser {
        let mut parser = Parser {
            lexer,
            current: Token::Eof,
            peek: Token::Eof,
        };

        parser.advance();
        parser.advance();
        parser
    }

    pub fn advance(&mut self) {
        // FIXME: Do without clone?
        self.current = self.peek.clone();
        self.peek = self.lexer.advance();
    }

    /// given a Parser object, iterate over the string
    /// and return a valid Program, i.e a collection of
    /// Statements
    pub fn parse(&mut self) -> Program {
        let mut program: Program = vec![];
        while !self.current_token_is(Token::Eof) {
            match self.parse_statement() {
                Some(statement) => program.push(statement),
                None => {}
            };
            self.advance();
        }
        program
    }

    pub fn peek_precendence(&mut self) -> Precendence {
        token_to_precedence(&self.peek)
    }
    pub fn curr_precendence(&mut self) -> Precendence {
        token_to_precedence(&self.current)
    }

    pub fn current_token_is(&mut self, token: Token) -> bool {
        self.current == token
    }

    pub fn peek_token_is(&mut self, token: Token) -> bool {
        self.peek == token
    }

    pub fn if_peek_advance(&mut self, token: Token) -> bool {
        if self.peek_token_is(token.clone()) {
            self.advance();
            return true;
        } else {
            return false;
        }
    }

    pub fn parse_statement(&mut self) -> Option<Statement> {
        println!("Current is {:#?}", self.current);
        return match self.current {
            Token::Let => self.parse_let_statement(),
            Token::Return => self.parse_return_statement(),
            _ => self.parse_expression_statement(),
        };
    }

    pub fn parse_expression_statement(&mut self) -> Option<Statement> {
        match self.parse_expression(Precendence::Lowest) {
            Some(expr) => {
                if self.peek_token_is(Token::Semicolon) {
                    self.advance();
                }
                Some(Statement::Expression(expr))
            }
            None => None,
        }
    }

    pub fn parse_expression(&mut self, precendence: Precendence) -> Option<Expression> {
        // Prefix
        let mut left = match self.current {
            Token::Identifier(_) => self.parse_identifier_expression(),
            Token::Integer(_) => self.parse_integer_expression(),
            Token::True | Token::False => self.parse_boolean_expression(),
            Token::Bang | Token::Minus => self.parse_prefix_expression(),
            Token::If => self.parse_if_expression(),
            Token::Fn => self.parse_function_expression(),
            Token::LeftParen => self.parse_grouped_expression(),
            _ => return None,
        };
        _ = precendence;
        while !self.peek_token_is(Token::Semicolon) && precendence < self.peek_precendence() {
            match self.peek {
                Token::Plus
                | Token::Minus
                | Token::Fslash
                | Token::Asterisk
                | Token::Equal
                | Token::Notequal
                | Token::Lt
                | Token::Gt => {
                    self.advance();
                    left = self.parse_infix_expression(left.unwrap());
                }
                _ => return left,
            }
        }
        left
    }

    pub fn parse_function_expression(&mut self) -> Option<Expression> {
        // FIXME: Need to advance to have current be the first identifier?
        self.advance();
        let parameters = match self.parse_fn_parameters() {
            Some(params) => params,
            None => return None,
        };

        // FIXME: Should be at the last identifier peek is Lbrace
        println!("current {:#?} {:#?}", self.current, self.peek);
        if !self.if_peek_advance(Token::Rarrow) {
            return None;
        }

        if !self.if_peek_advance(Token::LeftBrace) {
            return None;
        }

        Some(Expression::Fn {
            parameter: parameters,
            body: self.parse_block_fn_statement(),
        })
    }

    pub fn parse_fn_parameters(&mut self) -> Option<Vec<Identifier>> {
        let mut params: Vec<Identifier> = vec![];
        // FIXME: Might need an advance here?
        match self.parse_identifier() {
            Some(ident) => params.push(ident),
            None => return None,
        };

        while self.peek_token_is(Token::Comma) {
            self.advance();
            self.advance();

            match self.parse_identifier() {
                Some(ident) => params.push(ident),
                None => return None,
            };
        }

        Some(params)
    }
    pub fn parse_block_fn_statement(&mut self) -> Program {
        self.advance();
        let mut block = vec![];
        while !self.current_token_is(Token::RightBrace) {
            if self.current_token_is(Token::Eof) {
                return block;
            }
            match self.parse_statement() {
                Some(statement) => block.push(statement),
                None => {}
            }
            self.advance();
        }

        block
    }

    pub fn parse_block_statement(&mut self) -> Program {
        self.advance();
        let mut block = vec![];
        while !self.current_token_is(Token::RightBrace) {
            if self.current_token_is(Token::Eof) {
                return block;
            }
            match self.parse_statement() {
                Some(statement) => block.push(statement),
                None => {}
            }
            self.advance();
        }

        block
    }

    pub fn parse_if_expression(&mut self) -> Option<Expression> {
        // if x > 10 { x } else { 10 } ;
        self.advance();
        let condition = match self.parse_expression(Precendence::Lowest) {
            Some(expression) => expression,
            None => return None,
        };
        self.advance();
        println!("Should be at the leftbrace for current {:#?}", self.current);
        let consuequence = self.parse_block_statement();
        println!(
            "Should be at the rightbrace for current {:#?} peek {:#?}",
            self.current, self.peek
        );
        let mut alternative: Option<Program> = None;
        if self.peek_token_is(Token::Else) {
            self.advance();
            if !self.if_peek_advance(Token::LeftBrace) {
                return None;
            }

            alternative = Some(self.parse_block_statement());
        }
        return Some(Expression::If {
            condition: Box::new(condition),
            consequence: consuequence,
            alternative: alternative,
        });
    }

    pub fn parse_grouped_expression(&mut self) -> Option<Expression> {
        self.advance();
        let expression = self.parse_expression(Precendence::Lowest);
        if !self.if_peek_advance(Token::RightParen) {
            None
        } else {
            expression
        }
    }
    pub fn parse_boolean_expression(&mut self) -> Option<Expression> {
        Some(Expression::Literal(Literal::Boolean(
            self.current_token_is(Token::True),
        )))
    }

    pub fn parse_infix_expression(&mut self, left: Expression) -> Option<Expression> {
        let infix = match self.current {
            Token::Plus => Infix::Plus,
            Token::Minus => Infix::Minus,
            Token::Fslash => Infix::Divide,
            Token::Asterisk => Infix::Multiply,
            Token::Equal => Infix::Equal,
            Token::Notequal => Infix::NotEqual,
            Token::Lt => Infix::LessThan,
            Token::Gt => Infix::GreaterThan,
            _ => return None,
        };

        let precedence = self.curr_precendence();

        self.advance();
        self.parse_expression(precedence)
            .map(|expr| Expression::Infix(infix, Box::new(left), Box::new(expr)))
    }

    pub fn parse_prefix_expression(&mut self) -> Option<Expression> {
        let prefix = match self.current {
            Token::Bang => Prefix::Not,
            Token::Minus => Prefix::Minus,
            Token::Plus => Prefix::Plus,
            _ => return None,
        };

        self.advance();

        self.parse_expression(Precendence::Prefix)
            .map(|expr| Expression::Prefix(prefix, Box::new(expr)))
    }

    pub fn parse_integer_expression(&mut self) -> Option<Expression> {
        let integer = match &self.current {
            Token::Integer(v) => v.parse::<i64>().unwrap(),
            _ => return None,
        };
        return Some(Expression::Literal(Literal::Integer(integer)));
    }

    pub fn parse_identifier(&mut self) -> Option<Identifier> {
        return Some(self.current.clone());
    }

    pub fn parse_identifier_expression(&mut self) -> Option<Expression> {
        return Some(Expression::Identifier(self.current.clone()));
    }

    pub fn parse_return_statement(&mut self) -> Option<Statement> {
        if !self.current_token_is(Token::Return) {
            return None;
        }

        self.advance();
        while !self.current_token_is(Token::Semicolon) {
            self.advance();
        }

        return Some(Statement::Return(Expression::None));
    }

    pub fn parse_let_statement(&mut self) -> Option<Statement> {
        // FIXME: Use wildcard if_peek_advance()
        // if !self.if_peek_advance(Token::Identifier(_)) {}

        let identifier = match &self.peek {
            Token::Identifier(s) => s.clone(),
            _ => return None,
        };
        self.advance();

        // FIXME: Name could just be ident string not the whole token
        if !self.if_peek_advance(Token::Assign) {
            return None;
        }

        // FIXME: Skipping expressions for now
        while !self.current_token_is(Token::Semicolon) {
            self.advance();
        }

        return Some(Statement::Let(identifier, Expression::None));
    }
}
