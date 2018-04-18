#![feature(io)]
#![feature(box_syntax, box_patterns)]
#![feature(type_ascription)]
#![feature(use_extern_macros)]

extern crate regex;

extern crate plex;

mod shared;
pub mod ast;
pub mod lexer;
pub mod parser;
// pub use parser::parser::{Parser, Token};
