use std::collections::HashMap;

use lexer::lexing_state::LexingState;
use lexer::action::{Action};
use lexer::matching_patterns;
use lexer::shared_actions;

mod expr_variable; pub use self::expr_variable::*;
mod expr_fname; pub use self::expr_fname::*;
mod expr_endfn; pub use self::expr_endfn::*;
mod line_begin; pub use self::line_begin::*;
mod expr_value; pub use self::expr_value::*;
mod expr_begin; pub use self::expr_begin::*;
mod expr_end; pub use self::expr_end::*;
mod leading_dot; pub use self::leading_dot::*;

pub fn construct() -> HashMap<LexingState, Vec<Box<Action>>> {
    let patterns = matching_patterns::construct();
    let shared_actions = shared_actions::construct();

    let mut machines = HashMap::new();

    macro_rules! machine {
        ( $state:expr, $actions:expr ) => {
            let state = $state.parse::<LexingState>().expect("can't parse LexingState");
            machines.insert(state, $actions);
        };
    }

    machine!("expr_variable", construct_machine_expr_variable(&patterns, &shared_actions));
    machine!("expr_fname", construct_machine_expr_fname(&patterns, &shared_actions));
    machine!("expr_endfn", construct_machine_expr_endfn(&patterns, &shared_actions));
    machine!("expr_value", construct_machine_expr_value(&patterns, &shared_actions));
    machine!("expr_begin", construct_machine_expr_begin(&patterns, &shared_actions));
    machine!("expr_end", construct_machine_expr_end(&patterns, &shared_actions));
    machine!("line_begin", construct_machine_line_begin(&patterns, &shared_actions));
    machine!("leading_dot", construct_machine_leading_dot(&patterns, &shared_actions));

    machines
}
