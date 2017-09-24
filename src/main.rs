pub mod ruby_on_rust;
pub mod ast;

fn main() {
    println!("{:?}", ruby_on_rust::parse_Expr("(1 + 2 * 3)"));
}
