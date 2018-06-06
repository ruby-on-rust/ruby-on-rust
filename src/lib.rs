// #![feature(io)]
#![feature(box_syntax, box_patterns)]
// #![feature(type_ascription)]

extern crate regex;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate maplit;
#[macro_use] extern crate strum_macros;

mod shared;
// pub mod ast;
pub mod lexer;
pub mod parser;
// // pub use parser::parser::{Parser, Token};
