extern crate regex;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate maplit;
#[macro_use] extern crate strum_macros;

use std::io;
use std::io::prelude::*;
use std::fs::File;

mod token;
mod ast;
mod lexer;
mod parser;

fn main() -> io::Result<()> {
    let mut f = File::open("tmp/a.rb")?;
    let mut file_content = String::new();
    f.read_to_string(&mut file_content)?;

    let mut parser = parser::parser::Parser::new();
    parser.parse(&file_content);

    Ok(())
}
