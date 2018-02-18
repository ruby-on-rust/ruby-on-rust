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
        // println!("action invoked for c_eof");
        lexer.flag_breaking();
    });

    // # Resolving kDO/kDO_COND/kDO_BLOCK ambiguity requires embedding
    // # @cond/@cmdarg-related code to e_lbrack, e_lparen and e_lbrace.

    // e_lbrack = '[' % {
    //   @cond.push(false); @cmdarg.push(false)
    // };
    action!("e_lbrack", |lexer: &mut Lexer| {
        lexer.cond.push(false);
        lexer.cmdarg.push(false);
    });

    // # Ruby 1.9 lambdas require parentheses counting in order to
    // # emit correct opening kDO/tLBRACE.

    // e_lparen = '(' % {
    //   @cond.push(false); @cmdarg.push(false)
    //   @paren_nest += 1
    // };

    action!("e_lparen", |lexer: &mut Lexer| {
        lexer.cond.push(false);
        lexer.cmdarg.push(false);
        lexer.paren_nest += 1;
    });

    // e_rparen = ')' % {
    //   @paren_nest -= 1
    // };

    actions
}
