use parser::token::Token;

#[derive(Debug, PartialEq)]
pub enum Node {
    Int(isize),
    Ident(Token),
    Assign(Box<Node>, Token, Box<Node>),
    Assignable,
}
