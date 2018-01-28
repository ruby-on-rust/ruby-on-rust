use regex::Regex;

use lexer::Lexer;

pub type ActionProc = fn(&mut Lexer) -> ();

#[derive(Clone)]
pub struct Action {
    pub regex: Regex,
    pub procedure: ActionProc
}
