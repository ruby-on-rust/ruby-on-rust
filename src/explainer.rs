pub fn explain(field: &str, message: String) {
    match field {
        "parser" => {
            println!("{}", message);
        },
        "lexer" => {
            println!("{:<48} {}", "", message);
        },
        _ => { panic!("unknown field"); }
    };
}
