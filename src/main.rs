use std::env;
use std::io;
use std::io::prelude::*;
use std::fs::File;

mod parser;
mod interpreter;
use interpreter::interpreter::Interpreter; // TODO got to be kidding...

fn main() {
    let mut interpreter = Interpreter::new();

    // TODO proper cli
    if let Some(file_path) = env::args().nth(1) {
        let mut file = File::open(file_path).unwrap();
        let mut content = String::new();
        let _ = file.read_to_string(&mut content);

        let stmts_ast = parser::parser::parse_Stmts(&content).expect("Parsing file");

        interpreter.eval_stmts(stmts_ast);
    } else {
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

                    interpreter.context.eval_stmt(*stmt_ast);
                }
            }

            input_line.clear();
        }
    }
}
