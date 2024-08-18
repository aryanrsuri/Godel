use super::*;
use std::io::{self, Write};
pub fn start() {
    loop {
        print!("$ ");
        let mut input = String::new();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).expect("Read failed.");
        let read = input.trim();
        if read.eq_ignore_ascii_case("exit") {
            break;
        }
        let lexer = Lexer::new(read);
        let mut parser = parser::Parser::new(lexer);
        println!("{:#?}", parser.parse());
    }
}
