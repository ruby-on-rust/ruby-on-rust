use std::env;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

mod parser;
mod interpreter;
use interpreter::interpreter::Interpreter; // TODO got to be kidding...

fn main() {
    let mut interpreter = Interpreter::new();

    // TODO proper cli
    if let Some(file_path) = env::args().nth(1) {
        let file = File::open(file_path).expect("Opening file");

        // TODO REVISIT implement comment via parser
        // https://github.com/nikomatsakis/lalrpop/issues/10

        // APPROACH 1
        // let mut content = String::new();
        // let _ = file.read_to_string(&mut content);

        // APPROACH 2
        let buf = BufReader::new(file);
        let content: String = buf.lines()
            .map(|l| l.expect("Parsing line"))
            .filter(|l| !l.starts_with('#'))
            .map(move |mut l| { l.push('\n'); l })
            .collect();

        let program_ast = parser::parser::parse_Program(&content).expect("Parsing file");

        interpreter.eval_program(program_ast);
    } else {
        unimplemented!();
        // let mut input_line = String::new();
        // loop {
        //     print!("> ");
        //     let _ = io::stdout().flush();

        //     io::stdin().read_line(&mut input_line).ok().expect("input failed");

        //     match input_line.trim() {
        //         "exit" => break,
        //         line => {
        //             let line = line.to_string();

        //             let stmt_ast = parser::parser::parse_Stmt(&line).expect("Parsing line");

        //             interpreter.context.eval_stmt(*stmt_ast);
        //         }
        //     }

        //     input_line.clear();
        // }
    }
}
