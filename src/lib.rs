#![feature(io)]
#![feature(box_syntax, box_patterns)]
#![feature(concat_idents)]

extern crate regex;

mod shared;

#[macro_use]
pub mod ast;
pub mod lexer;
pub mod parser;
