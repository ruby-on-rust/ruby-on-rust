// based on 
// https://github.com/whitequark/parser/blob/2a73841d6da04a5ab9bd270561165fd766722d43/test/test_lexer.rb

extern crate ruby_on_rust;

use ruby_on_rust::lexer::Lexer;
use ruby_on_rust::parser::parser::{Token, TokenString};

#[test]
fn test_identifier() {
    let content = String::from("identifier");

    let mut lexer = Lexer::new(content);

    lexer.lex();

    let tokens = lexer.tokens;

    println!("tokens {:?}", tokens);

    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens.get(0).unwrap(), &Token::T_IDENTIFIER(TokenString::from("identifier")));
}
