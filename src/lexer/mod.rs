#[derive(PartialEq, Debug, Clone)]
pub enum Token {
    Illegal,
    Eof,
    Identifier(String),
    String(String),
    Integer(String),
    Comment(String),
    Assign,
    Cons,
    Plus,
    Ampersand,
    // And,
    // LeftShift,
    // RightShift,
    Modulo,
    Minus,
    Asterisk,
    Exponent,
    Comma,
    Colon,
    Semicolon,
    LeftBracket,
    RightBracket,
    Period,
    Range,
    LeftParen,
    LeftBrace,
    RightParen,
    RightBrace,
    Rarrow,
    Lt,
    Gt,
    Fslash,
    Equal,
    Notequal,
    Bang,
    Else,
    Return,
    In,
    If,
    For,
    Type,
    False,
    True,
    Vbar,
    Pipe,
    Ok,
    None,
    Unit,
    Error,
    Fn,
    Let,
    Cardinal,
}

#[derive(Debug)]
pub struct Lexer {
    input: Vec<char>,
    cur: usize,
    next_cur: usize,
    ch: char,
}

fn is_whitespace(c: char) -> bool {
    c == ' ' || c == '\t' || c == '\n' || c == '\r'
}

fn is_numeric(c: char) -> bool {
    ('0'..='9').contains(&c)
}

fn is_alphanumeric(c: char) -> bool {
    ('a'..='z').contains(&c) || ('A'..='Z').contains(&c) || is_numeric(c) || c == '_'
}

impl Lexer {
    pub fn new(input: &str) -> Lexer {
        let mut lexer = Lexer {
            input: input.chars().collect(),
            cur: 0,
            next_cur: 0,
            ch: '\0',
        };
        lexer.read();
        lexer
    }

    pub fn read(&mut self) {
        if self.next_cur >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input[self.next_cur];
        }
        self.cur = self.next_cur;
        self.next_cur += 1;
    }

    pub fn peek(&mut self) -> char {
        self.input[self.next_cur]
    }

    pub fn read_string(&mut self) -> Token {
        self.read();
        // on ident essentially
        let current = self.cur;
        loop {
            if is_alphanumeric(self.ch) || is_whitespace(self.ch) {
                self.read();
            } else {
                break;
            }
        }

        // TODO: Force end delimter to be doublequote or error
        self.read();
        Token::String(self.input[current..self.cur - 1].iter().collect::<String>())
    }
    pub fn read_number(&mut self) -> Token {
        let current = self.cur;
        loop {
            if is_numeric(self.ch) {
                self.read();
            } else {
                break;
            }
        }
        return Token::Integer(self.input[current..self.cur].iter().collect::<String>());
    }

    pub fn read_comment(&mut self) -> Token {
        let current = self.cur;
        loop {
            if is_alphanumeric(self.ch) || self.ch == ' ' || self.ch == '\t' {
                self.read();
            } else {
                break;
            }
        }
        Token::Comment(self.input[current..self.cur].iter().collect::<String>())
    }

    pub fn read_identifier(&mut self) -> Token {
        let current = self.cur;
        loop {
            if is_alphanumeric(self.ch) {
                self.read();
            } else {
                break;
            }
        }
        let literal = self.input[current..self.cur].iter().collect::<String>();
        return match literal.as_str() {
            "fn" => Token::Fn,
            "let" => Token::Let,
            "return" => Token::Return,
            "else" => Token::Else,
            "if" => Token::If,
            "for" => Token::For,
            "type" => Token::Type,
            "true" => Token::True,
            "false" => Token::False,
            "Ok" => Token::Ok,
            "None" => Token::None,
            "Error" => Token::Error,
            _ => Token::Identifier(literal),
        };
    }

    pub fn advance(&mut self) -> Token {
        loop {
            if is_whitespace(self.ch) {
                self.read()
            } else {
                break;
            }
        }
        let token: Token = match self.ch {
            '=' => {
                if self.peek() == '=' {
                    self.read();
                    Token::Equal
                } else {
                    Token::Assign
                }
            }
            ';' => Token::Semicolon,
            '(' => {
                if self.peek() == ')' {
                    self.read();
                    Token::Unit
                } else {
                    Token::LeftParen
                }
            }
            ')' => Token::RightParen,
            ',' => Token::Comma,
            '+' => Token::Plus,
            '-' => {
                if self.peek() == '>' {
                    self.read();
                    Token::Rarrow
                } else {
                    Token::Minus
                }
            }
            '!' => {
                if self.peek() == '=' {
                    self.read();
                    Token::Notequal
                } else {
                    Token::Bang
                }
            }
            ':' => {
                if self.peek() == ':' {
                    self.read();
                    Token::Cons
                } else {
                    Token::Colon
                }
            }
            '.' => {
                if self.peek() == '.' {
                    self.read();
                    Token::Range
                } else {
                    Token::Period
                }
            }
            '|' => {
                if self.peek() == '>' {
                    self.read();
                    Token::Pipe
                } else {
                    Token::Vbar
                }
            }
            '<' => {
                if self.peek() == '-' {
                    self.read();
                    Token::In
                } else {
                    Token::Lt
                }
            }
            '/' => {
                if self.peek() == '/' {
                    self.read();
                    self.read();
                    let comment = self.read_comment();
                    return comment;
                } else {
                    Token::Fslash
                }
            }
            '[' => Token::LeftBracket,
            ']' => Token::RightBracket,
            '*' => {
                if self.peek() == '*' {
                    self.read();
                    Token::Exponent
                } else {
                    Token::Asterisk
                }
            }
            '&' => Token::Ampersand,
            '#' => Token::Cardinal,
            '%' => Token::Modulo,
            '>' => Token::Gt,
            '{' => Token::LeftBrace,
            '}' => Token::RightBrace,
            '"' => return self.read_string(),
            '0'..='9' => return self.read_number(),
            'a'..='z' | 'A'..='Z' => return self.read_identifier(),
            '\0' => Token::Eof,
            _ => Token::Illegal,
        };

        self.read();
        token
    }
}
