#![feature(io)]
#![feature(box_syntax, box_patterns)]

extern crate regex;

use std::io::prelude::*;
use std::fs::File;

mod lexer;
mod parser;
use parser::parser::{Parser, Token};

fn main() {

    let mut f = File::open("tmp/a.rb").expect("cant read file");
    let mut file_content = String::new();
    f.read_to_string(&mut file_content).expect("cant read file");

    let mut lexer = lexer::Lexer::new(file_content);

    lexer.lex();

    let tokens = lexer.tokens;

    let mut parser = Parser::new(None);
    println!("\n\n---\n\nLexer emitted tokens:");
    for t in tokens {
        println!("TOKEN: {:?}", t);

        parser.parse(t);
    }

    parser.parse(Token::EOI);

    println!("{:?}", parser.extra());
}
