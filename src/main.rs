use std::io;
use std::io::prelude::*;
use std::fs::File;

mod lexer;
use lexer::lexer::Lexer;

fn main() -> io::Result<()> {
    let mut f = File::open("tmp/a.rb")?;
    let mut file_content = String::new();
    f.read_to_string(&mut file_content)?;

    let mut lexer = Lexer::new(file_content);

    Ok(())
}
