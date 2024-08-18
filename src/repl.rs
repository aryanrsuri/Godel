use super::*;
use std::io::{self, Write};
pub fn start() {
    let mut input = String::new();
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).expect("Read failed.");
        let read = input.trim();
        if read.eq_ignore_ascii_case("exit") {
            break;
        }
        let mut lexer = Lexer::new(read);
        loop {
            let token = lexer.advance();
            if token == Token::Eof {
                break;
            } else {
                println!("{:?}", token)
            };
        }
    }
}
