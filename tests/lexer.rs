// based on 
// https://github.com/whitequark/parser/blob/2a73841d6da04a5ab9bd270561165fd766722d43/test/test_lexer.rb

// TODO assert macro helper

extern crate ruby_on_rust;

use ruby_on_rust::lexer::Lexer;
use ruby_on_rust::parser::token::{Token, TokenString};

#[test]
fn test_identifier() {
    let content = String::from("identifier");

    let mut lexer = Lexer::new(content);

    let token = lexer.advance().unwrap();
    assert_eq!(token, Token::T_IDENTIFIER(TokenString::from("identifier")));
}


//   def test_string_double_interp
//     assert_scanned("\"blah #x a \#@a b \#$b c \#{3} # \"",
//                    :tSTRING_BEG,     "\"",         [0, 1],
//                    :tSTRING_CONTENT, "blah #x a ", [1, 11],
//                    :tSTRING_DVAR,    nil,          [11, 12],
//                    :tIVAR,           "@a",         [12, 14],
//                    :tSTRING_CONTENT, " b ",        [14, 17],
//                    :tSTRING_DVAR,    nil,          [17, 18],
//                    :tGVAR,           "$b",         [18, 20],
//                    :tSTRING_CONTENT, " c ",        [20, 23],
//                    :tSTRING_DBEG,    '#{',         [23, 25],
//                    :tINTEGER,        3,            [25, 26],
//                    :tRCURLY,         "}",          [26, 27],
//                    :tSTRING_CONTENT, " # ",        [27, 30],
//                    :tSTRING_END,     "\"",         [30, 31])
//   end
#[test]
fn string_double_interp() {
    let content = String::from("\"blah #x a #@a b #$b c #{3} # \"");

    let mut lexer = Lexer::new(content);

    assert_eq!(lexer.advance().unwrap(), Token::T_STRING_BEG);
    assert_eq!(lexer.advance().unwrap(), Token::T_STRING_CONTENT(TokenString::from("blah #x a ")));
}
