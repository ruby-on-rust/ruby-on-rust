use std::collections::HashMap;

use lexer::Lexer;
use lexer::action::{ActionProc};

pub type TSharedActions = HashMap<&'static str, ActionProc>;

pub fn construct() -> TSharedActions {
    let mut actions: TSharedActions = HashMap::new();

    // TODO share action! macro between shared_action and transactions
    macro_rules! action {
        ($pattern_name:expr, $procedure:expr) => {
            actions.insert($pattern_name, $procedure);
        };
    }

    action!("noop", |lexer: &mut Lexer|{});

    // original do_eof
    action!("do_eof", |lexer: &mut Lexer| {
        println!("action invoked for c_eof");
        lexer.flag_breaking();
    });

    actions
}
