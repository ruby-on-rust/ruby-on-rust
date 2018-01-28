#![feature(io)]
#![feature(box_syntax, box_patterns)]

extern crate regex;

use std::io;
use std::io::prelude::*;
use std::fs::File;

mod lexer;
use lexer::Lexer;

fn main() {
    let mut f = File::open("tmp/a.rb").expect("cant read file");
    let mut file_content = String::new();
    f.read_to_string(&mut file_content).expect("cant read file");

    let mut lexer = lexer::Lexer::new(file_content);

    lexer.lex();

    // let tokens = lexer.tokens;

    // let mut parser = parser::Parser::new(None);

    // println!("TOKENs:");
    // for t in &tokens {
    //     println!("TOKEN: {:?}", t);
    // }

    // for t in tokens {
    //     parser.parse(t);
    // }

    // parser.parse(parser::Token::EOI);

    // println!("{:?}", parser.extra());
}
