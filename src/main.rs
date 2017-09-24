pub mod parser;

fn main() {
    println!("{:?}", parser::parser::parse_Expr("(1 + (2) * 3)"));
}
