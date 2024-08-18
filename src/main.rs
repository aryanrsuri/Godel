mod ast;
mod lexer;
mod parser;
mod repl;
use crate::lexer::*;
fn main() {
    // repl::start();
    let lexer = Lexer::new(
        // "let x = 5;\nreturn 5;\nadd;120;!5;\n5 - 5;\n5 + !1;\nfalse;\n!true;\n(5 + 5) * 10;\nif x > 10 { x };\nif x > 10 { x } else { 10 };",
        "fn x,y -> {x} ;\nfn z -> z + 10;\nfn () -> 200;\nfn () -> {!true};",
    );
    let mut parser = parser::Parser::new(lexer);

    let program = parser.parse();
    println!("{:#?}", program);
}
