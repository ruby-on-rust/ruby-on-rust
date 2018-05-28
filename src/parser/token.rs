#[derive(Debug, Clone, Copy)]
pub struct Token {
    pub kind: i32,
    pub value: &'static str,

    pub start_offset: i32,
    pub end_offset: i32,
    pub start_line: i32,
    pub end_line: i32,
    pub start_column: i32,
    pub end_column: i32,
}
