use std::collections::HashMap;

use lexer::lexing_state::LexingState;
use lexer::action::{Action};
use lexer::matching_patterns;
use lexer::shared_actions;

mod line_begin; use self::line_begin::*;
mod expr_dot; use self::expr_dot::*;
mod expr_fname; use self::expr_fname::*;
mod expr_value; use self::expr_value::*;
mod expr_beg; use self::expr_beg::*;
mod expr_mid; use self::expr_mid::*;
mod expr_arg; use self::expr_arg::*;
mod expr_cmdarg; use self::expr_cmdarg::*;
mod expr_end; use self::expr_end::*;
mod expr_endarg; use self::expr_endarg::*;
mod expr_endfn; use self::expr_endfn::*;
mod expr_labelarg; use self::expr_labelarg::*;

mod interpolation; use self::interpolation::*;

mod expr_variable; use self::expr_variable::*;
mod leading_dot; use self::leading_dot::*;

pub fn construct(shared_actions: &shared_actions::TSharedActions) -> HashMap<LexingState, Vec<Box<Action>>> {
    let patterns = matching_patterns::construct();

    let mut machines = HashMap::new();

    macro_rules! machine {
        ( $state:expr, $actions:expr ) => {
            let state = $state.parse::<LexingState>().expect("can't parse LexingState");
            machines.insert(state, $actions);
        };
    }

    machine!("line_begin", construct_machine_line_begin(&patterns, &shared_actions));
    machine!("expr_dot", construct_machine_expr_dot(&patterns, &shared_actions));
    machine!("expr_fname", construct_machine_expr_fname(&patterns, &shared_actions));
    machine!("expr_value", construct_machine_expr_value(&patterns, &shared_actions));
    machine!("expr_beg", construct_machine_expr_beg(&patterns, &shared_actions));
    machine!("expr_mid", construct_machine_expr_mid(&patterns, &shared_actions));
    machine!("expr_arg", construct_machine_expr_arg(&patterns, &shared_actions));
    machine!("expr_cmdarg", construct_machine_expr_cmdarg(&patterns, &shared_actions));
    machine!("expr_end", construct_machine_expr_end(&patterns, &shared_actions));
    machine!("expr_endarg", construct_machine_expr_endarg(&patterns, &shared_actions));
    machine!("expr_endfn", construct_machine_expr_endfn(&patterns, &shared_actions));
    machine!("expr_labelarg", construct_machine_expr_labelarg(&patterns, &shared_actions));

    machine!("interp_words", construct_machine_interp_words(&patterns, &shared_actions));
    machine!("interp_string", construct_machine_interp_string(&patterns, &shared_actions));
    machine!("plain_words", construct_machine_plain_words(&patterns, &shared_actions));
    machine!("plain_string", construct_machine_plain_string(&patterns, &shared_actions));
    machine!("interp_backslash_delimited", construct_machine_interp_backslash_delimited(&patterns, &shared_actions));
    machine!("plain_backslash_delimited", construct_machine_plain_backslash_delimited(&patterns, &shared_actions));
    machine!("interp_backslash_delimited_words", construct_machine_interp_backslash_delimited_words(&patterns, &shared_actions));
    machine!("plain_backslash_delimited_words", construct_machine_plain_backslash_delimited_words(&patterns, &shared_actions));

    machine!("expr_variable", construct_machine_expr_variable(&patterns, &shared_actions));
    machine!("leading_dot", construct_machine_leading_dot(&patterns, &shared_actions));

    machines
}
