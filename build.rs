use std::process::Command;

fn main() {
    Command::new("./vendor/lemon/lemon_rust").arg("-Tvendor/lemon/lempar.rs ./src/parser.y").status().unwrap();
}
