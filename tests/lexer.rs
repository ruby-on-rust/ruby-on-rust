// based on 
// https://github.com/whitequark/parser/blob/2a73841d6da04a5ab9bd270561165fd766722d43/test/test_lexer.rb

// TODO assert macro helper

extern crate ruby_on_rust;

use ruby_on_rust::lexer::Lexer;
use ruby_on_rust::parser::token::{Token, TokenString};

#[test]
fn identifier() {
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
// 
// NOTE
// so the expected result in the test case is not true for ruby25 :facepalm:
// 
// the result in parser::currentruby(ruby25) for the last token tRCURLY is actually:
//   [:tSTRING_DEND, ["}", #<Parser::Source::Range (string) 26...27>]]
// 
// TODO
// NOTE
// emmmm, seems like this case needs a parser to work, (to set the lexer's state)
// see the note before machine:expr_endarg
// 
#[test]
fn string_double_interp() {
    let content = String::from("\"blah #x a #@a b #$b c #{3} # \"");

    let mut lexer = Lexer::new(content);

    assert_eq!(lexer.advance().unwrap(), Token::T_STRING_BEG);
    assert_eq!(lexer.advance().unwrap(), Token::T_STRING_CONTENT(TokenString::from("blah #x a ")));
    assert_eq!(lexer.advance().unwrap(), Token::T_STRING_DVAR);
    assert_eq!(lexer.advance().unwrap(), Token::T_IVAR(TokenString::from("@a")));
    assert_eq!(lexer.advance().unwrap(), Token::T_STRING_CONTENT(TokenString::from(" b ")));
    assert_eq!(lexer.advance().unwrap(), Token::T_STRING_DVAR);
    assert_eq!(lexer.advance().unwrap(), Token::T_GVAR(TokenString::from("$b")));
    assert_eq!(lexer.advance().unwrap(), Token::T_STRING_CONTENT(TokenString::from(" c ")));
    assert_eq!(lexer.advance().unwrap(), Token::T_STRING_DBEG);
    assert_eq!(lexer.advance().unwrap(), Token::T_INTEGER(3));
    // // assert_eq!(lexer.advance().unwrap(), Token::T_RCURLY); // -> T_STRING_DEND
    // assert_eq!(lexer.advance().unwrap(), Token::T_STRING_DEND);
    // assert_eq!(lexer.advance().unwrap(), Token::T_STRING_CONTENT(TokenString::from(" # ")));
    // assert_eq!(lexer.advance().unwrap(), Token::T_STRING_END);
    // // TODO assert must be empty, impl in helper macro
}
