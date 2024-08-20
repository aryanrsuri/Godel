use super::*;
use std::io::{self, Write};

pub fn start(eval: bool) {
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
        let program = parser.parse();
        if eval {
            let mut ev = evaluator::Evaluator::new();
            let evaluated = ev.eval(&program);
            match evaluated {
                Some(object) => object.inspect(),
                None => (),
            }
        } else {
            println!("{:#?}", program);
        }
    }
}
