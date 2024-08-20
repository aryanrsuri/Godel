pub mod ast;
pub mod evaluator;
pub mod lexer;
pub mod parser;
pub mod repl;
use crate::lexer::*;
use std::env;
use std::fs::read_to_string;
use std::io;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} [filename.gdl]", args[0]);
        std::process::exit(1);
    }

    let filename = &args[1];
    if filename == "repl" {
        repl::start(false);
        return Ok(());
    } else {
        if filename == "eval" {
            repl::start(true);
            return Ok(());
        }
        let contents = read_to_string(filename)?;
        let read = contents.trim();
        let lexer = Lexer::new(read);
        let mut parser = parser::Parser::new(lexer);
        let program = parser.parse();
        for statement in program {
            println!("{:?}", statement);
        }
    }

    // repl::start();
    Ok(())
}
