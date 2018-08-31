fn main() {
    let mut f = File::open("tmp/a.rb").expect("cant open file");
    let mut file_content = String::new();
    f.read_to_string(&mut file_content).expect("cant read file");

    let mut parser = parser::parser::Parser::new(file_content);

    let node = parser.parse();
    println!("====== parser parsed node:\n{:?}", node );
}
