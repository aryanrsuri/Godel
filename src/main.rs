mod ast;
mod lexer;
mod parser;
mod repl;
use crate::lexer::*;
fn main() {
    repl::start();
    //     let fn_ternary = "let timegt = fn () -> {
    // if time > 13244 { time } else { 13244 }
    // };";
    //     let result_type = "
    // let real = fn x -> if x > 0 { Ok x } else { None };
    // ";
    //     let ok = "
    // Ok x;
    // ";
    //
    //     let tests = vec![
    //         fn_ternary,
    //         result_type,
    //         ok,
    //         "Error;",
    //         "let real = fn x -> if x > 0 {Ok x} else {Error};",
    //         "add(10,2)",
    //         "return real(10);", // "12
    //                             // |> truncate
    //                             // |> real",
    //     ];
    //
    //     for test in tests {
    //         let lexer = Lexer::new(
    //             // "let x = 5;\nreturn 5;\nadd;120;!5;\n5 - 5;\n5 + !1;\nfalse;\n!true;\n(5 + 5) * 10;\nif x > 10 { x };\nif x > 10 { x } else { 10 };",
    //             // "fn x,y -> {x} ;\nfn z -> z + 10;\nfn () -> 200;\nfn () -> {!true};",
    //             test,
    //         );
    //
    //         let mut parser = parser::Parser::new(lexer);
    //
    //         let program = parser.parse();
    //         println!("--------------");
    //         for st in program {
    //             println!("{:?}\n", st);
    //         }
    //     }
}
