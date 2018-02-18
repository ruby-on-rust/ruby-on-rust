use std::collections::HashMap;

use lexer::lexing_state::LexingState;
use lexer::action::{Action};
use lexer::matching_patterns;
use lexer::shared_actions;

mod line_begin; pub use self::line_begin::*;
mod expr_dot; pub use self::expr_dot::*;
mod expr_fname; pub use self::expr_fname::*;
mod expr_value; pub use self::expr_value::*;
mod expr_beg; pub use self::expr_beg::*;
// mod expr_mid; pub use self::expr_mid::*;
mod expr_arg; pub use self::expr_arg::*;
mod expr_cmdarg; pub use self::expr_cmdarg::*;
mod expr_end; pub use self::expr_end::*;
// mod expr_endarg; pub use self::expr_endarg::*;
mod expr_endfn; pub use self::expr_endfn::*;
// mod expr_labelarg; pub use self::expr_labelarg::*;

mod expr_variable; pub use self::expr_variable::*;
mod leading_dot; pub use self::leading_dot::*;

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
    // machine!("expr_mid", construct_machine_expr_mid(&patterns, &shared_actions));
    machine!("expr_arg", construct_machine_expr_arg(&patterns, &shared_actions));
    machine!("expr_cmdarg", construct_machine_expr_cmdarg(&patterns, &shared_actions));
    machine!("expr_end", construct_machine_expr_end(&patterns, &shared_actions));
    // machine!("expr_endarg", construct_machine_expr_endarg(&patterns, &shared_actions));
    machine!("expr_endfn", construct_machine_expr_endfn(&patterns, &shared_actions));
    // machine!("expr_labelarg", construct_machine_expr_labelarg(&patterns, &shared_actions));

    machine!("expr_variable", construct_machine_expr_variable(&patterns, &shared_actions));
    machine!("leading_dot", construct_machine_leading_dot(&patterns, &shared_actions));

    machines
}
