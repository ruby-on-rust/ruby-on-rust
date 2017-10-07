use std::io::prelude::*;
use std::io;

mod parser;
mod interpreter;
use interpreter::interpreter::Interpreter; // TODO got to be kidding...

fn main() {
    let mut interpreter = Interpreter { ..Default::default() };
    let mut input_line = String::new();

    loop {
        print!("> ");
        let _ = io::stdout().flush();

        io::stdin().read_line(&mut input_line).ok().expect("input failed");

        match input_line.trim() {
            "exit" => break,
            line => {
                let line = line.to_string();

                let stmt_ast = parser::parser::parse_Stmt(&line).expect("Parsing line");

                interpreter.eval_line(*stmt_ast);
            }
        }

        input_line.clear();
    }
}
