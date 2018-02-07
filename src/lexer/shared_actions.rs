use std::collections::HashMap;

use lexer::Lexer;
use lexer::action::{ActionProc};

pub fn construct() -> HashMap<&'static str, ActionProc> {
    let mut actions: HashMap<&'static str, ActionProc> = HashMap::new();

    // TODO share action! macro between shared_action and transactions
    macro_rules! action {
        ($pattern_name:expr, $procedure:expr) => {
            actions.insert($pattern_name, $procedure);
        };
    }

    // original do_eof
    action!("do_eof", |lexer: &mut Lexer| {
        println!("action invoked for c_eof");
        lexer.flag_breaking();
    });

    actions
}
