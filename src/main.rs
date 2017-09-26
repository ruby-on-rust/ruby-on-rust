use std::io::prelude::*;
use std::io;

mod parser;
mod interpreter;
use interpreter::interpreter::Interpreter; // TODO got to be kidding...

fn main() {
    let mut interpreter = Interpreter {};
    let mut input_line = String::new();

    loop {
        print!("> ");
        io::stdout().flush();

        io::stdin().read_line(&mut input_line).ok().expect("input failed");

        match input_line.trim() {
            "exit" => break,
            line => {
                let line = line.to_string();

                let expr_ast = parser::parser::parse_Expr(&line).expect("parsing failed");

                interpreter.eval_line(*expr_ast);
            }
        }

        input_line.clear();
    }
}
