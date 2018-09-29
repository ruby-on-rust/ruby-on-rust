// based on https://github.com/whitequark/parser/blob/b3a0cd6be2f2d498c36cd5ae0dd39d9d25497c53/test/test_lexer.rb

// TODO macro!

extern crate ruby_on_rust;
use ruby_on_rust::{
  lexer::lexer::Lexer,
  token::token::Token,
};

#[test]
fn identifier() {
    let content = String::from("identifier");

    let mut lexer = Lexer::new(content);

    let token = lexer.advance().unwrap();
    assert_eq!(token, Token::T_IDENTIFIER(String::from("identifier")));
}
