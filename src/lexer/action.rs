// use std::fmt;
use regex::Regex;

use lexer::Lexer;

pub type ActionProc = fn(&mut Lexer) -> ();

#[derive(Clone)]
pub struct Action {
    // TODO RENAMING regex -> pattern ?
    pub regex: Regex,
    pub procedure: ActionProc
}

// impl fmt::Debug for Action {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "Action with regex: {:?}\n", self.regex)
//     }
// }

macro_rules! action_with_literal {
    ($pattern_literal:expr, $procedure:expr) => {
        box Action {
            regex: Regex::new( &format!(r"^{}", $pattern_literal) ).unwrap(),
            procedure: $procedure
        }
    };
}
