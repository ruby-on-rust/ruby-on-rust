use std::collections::HashMap;

use lexer::lexing_state::LexingState;
use lexer::action::{Action};
use lexer::matching_patterns;
use lexer::shared_actions;

mod line_begin; pub use self::line_begin::*;
mod expr_variable; pub use self::expr_variable::*;
mod expr_value; pub use self::expr_value::*;
mod expr_begin; pub use self::expr_begin::*;
mod expr_end; pub use self::expr_end::*;
mod leading_dot; pub use self::leading_dot::*;

pub fn construct() -> HashMap<LexingState, Vec<Box<Action>>> {
    let patterns = matching_patterns::construct();
    let shared_actions = shared_actions::construct();

    let mut transactions = HashMap::new();

    macro_rules! transaction {
        ( $state:expr, $actions:expr ) => {
            let state = $state.parse::<LexingState>().expect("can't parse LexingState");
            transactions.insert(state, $actions);
        };
    }

    transaction!("expr_variable", construct_machine_expr_variable(&patterns, &shared_actions));
    transaction!("expr_value", construct_machine_expr_value(&patterns, &shared_actions));
    transaction!("expr_begin", construct_machine_expr_begin(&patterns, &shared_actions));
    transaction!("expr_end", construct_machine_expr_end(&patterns, &shared_actions));
    transaction!("line_begin", construct_machine_line_begin(&patterns, &shared_actions));
    transaction!("leading_dot", construct_machine_leading_dot(&patterns, &shared_actions));

    transactions
}
