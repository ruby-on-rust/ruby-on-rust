pub mod parser;

fn main() {
    println!("{:?}", parser::parser::parse_Expr("a = (1 + (2) * 3)"));
}
