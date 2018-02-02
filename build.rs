use std::process::Command;

fn main() {
    Command::new("./vendor/lemon/lemon_rust").arg("src/parser/parser.y -Tvendor/lemon/lempar.rs").status().unwrap();
}
