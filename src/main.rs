mod parser;

fn main() {
    let mut parser = parser::Parser::new(None);

    let tokens = vec![
        parser::Token::VALUE(1),
        parser::Token::OP_PLUS,
        parser::Token::VALUE(2),
    ];

    for t in tokens {
        parser.parse(t);
    }
    parser.parse(parser::Token::EOI);

    println!("{:?}", parser.extra());
}
