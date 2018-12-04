#[macro_use] extern crate lazy_static;
#[macro_use] extern crate maplit;
#[macro_use] extern crate strum_macros;

use std::fs;

mod explainer;
mod token;
mod ast;
mod lexer;
mod parser;
mod interpreter;

use crate::{
    parser::parser::Parser,
    interpreter::interpreter::Interpreter,
};

fn main() {
    let code = fs::read_to_string("./tmp/a.rb").expect("Unable to read file");

    let mut parser = Parser::new();
    let node = parser.parse(&code);

    println!("parsed: {:?}", node);

    let mut interpreter = Interpreter::new();

    let result = interpreter.eval(node);

    println!("evaluated result: {:?}", result);
}
