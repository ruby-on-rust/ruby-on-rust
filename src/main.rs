#![feature(io)]
#![feature(box_syntax, box_patterns)]

use std::io;
use std::io::prelude::*;
use std::fs::File;

mod lexer;
use lexer::Lexer;

mod parser;

fn main() {
    let f = File::open("tmp/a.rb").unwrap();

    let mut lexer = lexer::Lexer::new(f.chars());

    lexer.lex();

    let tokens = lexer.tokens;

    let mut parser = parser::Parser::new(None);

    println!("TOKENs:");
    for t in &tokens {
        println!("TOKEN: {:?}", t);
    }

    for t in tokens {
        parser.parse(t);
    }

    parser.parse(parser::Token::EOI);

    println!("{:?}", parser.extra());
}
