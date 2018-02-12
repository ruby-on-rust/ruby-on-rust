use regex::Regex;

use lexer::Lexer;

pub type ActionProc = fn(&mut Lexer) -> ();

#[derive(Clone)]
pub struct Action {
    // TODO RENAMING regex -> pattern ?
    pub regex: Regex,
    pub procedure: ActionProc
}
