extern crate lalrpop;

fn main() {
    lalrpop::Configuration::new()
        .log_verbose()
        .process_file("./src/parser/parser.lalrpop")
        .unwrap();
}
