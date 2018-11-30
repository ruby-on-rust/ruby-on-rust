use prettytable::{Table, Row, Cell};

pub fn explain(field: &str, message: String) {
    let mut table = Table::new();

    let row_content = match field {
        "parser" => vec![ Cell::new(&message), Cell::new(""), Cell::new("") ],
        _ => { panic!("unknown field"); }
    };

    table.add_row(Row::new(row_content));

    table.printstd();
}
