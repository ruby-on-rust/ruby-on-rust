extern crate ruby_on_rust;

use ruby_on_rust::lexer::Lexer;
use ruby_on_rust::parser::parser::Parser;
use ruby_on_rust::ast::node::Node;

// TODO INCOMPLETE
#[test]
fn test_int() {
    let content = String::from("42");
    let mut parser = Parser::new(content);
    let node = parser.parse();
    assert_eq!(node, Node { ntype: String::from("int"), value: 42 });
}
