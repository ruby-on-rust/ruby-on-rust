#![macro_use]

use std::str::FromStr;

// ORIGINAL
// 
// :line_begin    => lex_en_line_begin,
// :expr_dot      => lex_en_expr_dot,
// :expr_fname    => lex_en_expr_fname,
// :expr_value    => lex_en_expr_value,
// :expr_beg      => lex_en_expr_beg,
// :expr_mid      => lex_en_expr_mid,
// :expr_arg      => lex_en_expr_arg,
// :expr_cmdarg   => lex_en_expr_cmdarg,
// :expr_end      => lex_en_expr_end,
// :expr_endarg   => lex_en_expr_endarg,
// :expr_endfn    => lex_en_expr_endfn,
// :expr_labelarg => lex_en_expr_labelarg,

// :interp_string => lex_en_interp_string,
// :interp_words  => lex_en_interp_words,
// :plain_string  => lex_en_plain_string,
// :plain_words   => lex_en_plain_string,

macro_rules! get_state {
    ($e:expr) => (
        $e.parse::<LexingState>().unwrap()
    );
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum LexingState {
    LineBegin,
    ExprValue,
    ExprBegin,
    ExprEnd,
}

impl FromStr for LexingState {
    type Err = ();

    fn from_str(s: &str) -> Result<LexingState, ()> {
        match s {
            "line_begin" => Ok(LexingState::LineBegin),
            "expr_value" => Ok(LexingState::ExprValue),
            "expr_begin" => Ok(LexingState::ExprBegin),
            "expr_end" => Ok(LexingState::ExprEnd),
            _ => Err(()),
        }
    }
}

pub fn get_lexing_state_by_name(name: &str) -> LexingState {
    name.parse::<LexingState>().unwrap()
}
