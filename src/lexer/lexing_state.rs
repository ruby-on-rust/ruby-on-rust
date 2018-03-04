use std::str::FromStr;

// ORIGINAL
// 
// :line_begin    => lex_en_line_begin,
// :expr_dot      => lex_en_expr_dot,
// :expr_fname    => lex_en_expr_fname,
// :expr_value    => lex_en_expr_value,
// :expr_beg      => lex_egn_expr_beg,
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

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum LexingState {
    LineBegin,
    ExprDot,
    ExprFname,
    ExprValue,
    ExprBeg,
    ExprMid,
    ExprArg,
    ExprCmdarg,
    ExprEnd,
    ExprEndarg,
    ExprEndfn,
    ExprLabelarg,

    // TODO
    // not original states in lexer.rl,
    // just machine
    ExprVariable,
    LeadingDot,
}

impl FromStr for LexingState {
    type Err = ();

    fn from_str(s: &str) -> Result<LexingState, ()> {
        match s {
            "line_begin" => Ok(LexingState::LineBegin),
            "expr_dot" => Ok(LexingState::ExprDot),
            "expr_fname" => Ok(LexingState::ExprFname),
            "expr_value" => Ok(LexingState::ExprValue),
            "expr_beg" => Ok(LexingState::ExprBeg),
            "expr_mid" => Ok(LexingState::ExprMid),
            "expr_arg" => Ok(LexingState::ExprArg),
            "expr_cmdarg" => Ok(LexingState::ExprCmdarg),
            "expr_end" => Ok(LexingState::ExprEnd),
            "expr_endarg" => Ok(LexingState::ExprEndarg),
            "expr_endfn" => Ok(LexingState::ExprEndfn),
            "expr_labelarg" => Ok(LexingState::ExprLabelarg),

            "expr_variable" => Ok(LexingState::ExprVariable),
            "leading_dot" => Ok(LexingState::LeadingDot),

            _ => Err(()),
        }
    }
}

macro_rules! state { ($state_name:expr) => { $state_name.parse::<LexingState>().unwrap() }; }
