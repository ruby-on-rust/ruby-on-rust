// TODO
//   regexp_modifiers := |*
//       [A-Za-z]+
//       => {
//         unknown_options = tok.scan(/[^imxouesn]/)
//         if unknown_options.any?
//           diagnostic :error, :regexp_options,
//                      { :options => unknown_options.join }
//         end
// 
//         emit(:tREGEXP_OPT)
// 
//         if @version < 24
//           fnext expr_end;
//         else
//           fnext expr_endarg;
//         end
// 
//         fbreak;
//       };
// 
//       any
//       => {
//         emit(:tREGEXP_OPT, tok(@ts, @te - 1), @ts, @te - 1)
//         fhold;
//         if @version < 24
//           fgoto expr_end;
//         else
//           fgoto expr_endarg;
//         end
//       };
//   *|;

use regex::Regex;

use lexer::Lexer;
use lexer::LexingState;
use lexer::action::{Action};
use lexer::matching_patterns::TMatchingPatterns;
use lexer::shared_actions::TSharedActions;

use parser::token::InteriorToken as Token;

pub fn construct_machine_interp_words( patterns: &TMatchingPatterns, shared_actions: &TSharedActions ) -> Vec<Box<Action>> {
    let (pattern_literals, pattern_regexs) = (*patterns).clone();

    macro_rules! action {
        ($pattern_name:expr, $procedure:expr) => {
            box Action {
                regex: pattern_regexs.get($pattern_name).expect(&format!("no matching_pattern: {:?}", $pattern_name)).clone(),
                procedure: $procedure
            }
        };
    }

    macro_rules! pattern_lit {
        ($pattern_name:expr) => {
            pattern_literals.get($pattern_name).unwrap()
        };
    }

    macro_rules! get_shared_action {
        ( $action_name:expr ) => {
            shared_actions.get($action_name).unwrap().clone()
        };
    }

//   interp_words := |*
//       interp_code => extend_interp_code;
//       interp_var  => extend_interp_var;
//       e_bs escape => extend_string_escaped;
//           TODO NOTE e_bs embedded action
//           TODO NOTE escape embedded action
//       c_space+    => extend_string_space;
//       c_eol       => extend_string_eol;
//       c_any       => extend_string;
//   *|;
    vec![
        action!("interp_code", get_shared_action!("extend_interp_code")),
        action!("interp_var", get_shared_action!("extend_interp_var")),
        // TODO
        // action_with_literal!(
        //     format!(r"{}{}",
        //         pattern_lit!("e_bs"), pattern_lit!("escape"),
        //     ), |lexer: &mut Lexer| {
        //         panic!("UNIMPL");
        //     }
        // ),
        action_with_literal!(format!(r"{}+", pattern_lit!("c_space")), get_shared_action!("extend_string_space")),
        action!("c_eol", get_shared_action!("extend_string_eol")),
        action!("c_any", get_shared_action!("extend_string")),
    ]
}

pub fn construct_machine_interp_string( patterns: &TMatchingPatterns, shared_actions: &TSharedActions ) -> Vec<Box<Action>> {
    let (pattern_literals, pattern_regexs) = (*patterns).clone();

    macro_rules! action {
        ($pattern_name:expr, $procedure:expr) => {
            box Action {
                regex: pattern_regexs.get($pattern_name).expect(&format!("no matching_pattern: {:?}", $pattern_name)).clone(),
                procedure: $procedure
            }
        };
    }

    macro_rules! pattern_lit {
        ($pattern_name:expr) => {
            pattern_literals.get($pattern_name).unwrap()
        };
    }

    macro_rules! get_shared_action {
        ( $action_name:expr ) => {
            shared_actions.get($action_name).unwrap().clone()
        };
    }

//   interp_string := |*
//       interp_code => extend_interp_code;
//       interp_var  => extend_interp_var;
//       e_bs escape => extend_string_escaped;
//           TODO NOTE e_bs embedded action
//       c_eol       => extend_string_eol;
//       c_any       => extend_string;
//   *|;
    vec![
        action!("interp_code", get_shared_action!("extend_interp_code")),
        action!("interp_var", get_shared_action!("extend_interp_var")),
        // TODO
        // action_with_literal!(
        //     format!(r"{}{}",
        //         pattern_lit!("e_bs"), pattern_lit!("escape"),
        //     ), |lexer: &mut Lexer| {
        //         panic!("UNIMPL");
        //     }
        // ),
        action!("c_eol", get_shared_action!("extend_string_eol")),
        action!("c_any", get_shared_action!("extend_string")),
    ]
}

pub fn construct_machine_plain_words( patterns: &TMatchingPatterns, shared_actions: &TSharedActions ) -> Vec<Box<Action>> {
    let (pattern_literals, pattern_regexs) = (*patterns).clone();

    macro_rules! action {
        ($pattern_name:expr, $procedure:expr) => {
            box Action {
                regex: pattern_regexs.get($pattern_name).expect(&format!("no matching_pattern: {:?}", $pattern_name)).clone(),
                procedure: $procedure
            }
        };
    }

    macro_rules! pattern_lit {
        ($pattern_name:expr) => {
            pattern_literals.get($pattern_name).unwrap()
        };
    }

    macro_rules! get_shared_action {
        ( $action_name:expr ) => {
            shared_actions.get($action_name).unwrap().clone()
        };
    }

//   plain_words := |*
//       e_bs c_any  => extend_string_escaped;
//           TODO NOTE e_bs embedded action
//       c_space+    => extend_string_space;
//       c_eol       => extend_string_eol;
//       c_any       => extend_string;
//   *|;
    vec![
        // TODO
        // action_with_literal!(
        //     format!(r"{}{}",
        //         pattern_lit!("e_bs"), pattern_lit!("c_any"),
        //     ), |lexer: &mut Lexer| {
        //         // TODO e_bs embedded action
        //         panic!("UNIMPL");
        //     }
        // ),
        action_with_literal!(format!(r"{}+", pattern_lit!("c_space")), get_shared_action!("extend_string_space")),
        action!("c_eol", get_shared_action!("extend_string_eol")),
        action!("c_any", get_shared_action!("extend_string")),
    ]
}


pub fn construct_machine_plain_string( patterns: &TMatchingPatterns, shared_actions: &TSharedActions ) -> Vec<Box<Action>> {
    let (pattern_literals, pattern_regexs) = (*patterns).clone();

    macro_rules! action {
        ($pattern_name:expr, $procedure:expr) => {
            box Action {
                regex: pattern_regexs.get($pattern_name).expect(&format!("no matching_pattern: {:?}", $pattern_name)).clone(),
                procedure: $procedure
            }
        };
    }

    macro_rules! pattern_lit {
        ($pattern_name:expr) => {
            pattern_literals.get($pattern_name).unwrap()
        };
    }

    macro_rules! get_shared_action {
        ( $action_name:expr ) => {
            shared_actions.get($action_name).unwrap().clone()
        };
    }

//   plain_string := |*
//       '\\' c_nl   => extend_string_eol;
//       e_bs c_any  => extend_string_escaped;
//           TODO NOTE e_bs embedded action
//       c_eol       => extend_string_eol;
//       c_any       => extend_string;
//   *|;
    vec![
        // TODO
        //       '\\' c_nl   => extend_string_eol;

        // TODO
        // action_with_literal!(
        //     format!(r"{}{}",
        //         pattern_lit!("e_bs"), pattern_lit!("c_any"),
        //     ), |lexer: &mut Lexer| {
        //         panic!("UNIMPL");
        //     }
        // ),

        action!("c_eol", get_shared_action!("extend_string_eol")),
        action!("c_any", get_shared_action!("extend_string")),
    ]
}


pub fn construct_machine_interp_backslash_delimited( patterns: &TMatchingPatterns, shared_actions: &TSharedActions ) -> Vec<Box<Action>> {
    let (pattern_literals, pattern_regexs) = (*patterns).clone();

    macro_rules! action {
        ($pattern_name:expr, $procedure:expr) => {
            box Action {
                regex: pattern_regexs.get($pattern_name).expect(&format!("no matching_pattern: {:?}", $pattern_name)).clone(),
                procedure: $procedure
            }
        };
    }

    macro_rules! pattern_lit {
        ($pattern_name:expr) => {
            pattern_literals.get($pattern_name).unwrap()
        };
    }

    macro_rules! get_shared_action {
        ( $action_name:expr ) => {
            shared_actions.get($action_name).unwrap().clone()
        };
    }

//   interp_backslash_delimited := |*
//       interp_code => extend_interp_code;
//       interp_var  => extend_interp_var;
//       c_eol       => extend_string_eol;
//       c_any       => extend_string;
//   *|;
    vec![
        action!("interp_code", get_shared_action!("extend_interp_code")),
        action!("interp_var", get_shared_action!("extend_interp_var")),
        action!("c_eol", get_shared_action!("extend_string_eol")),
        action!("c_any", get_shared_action!("extend_string")),
    ]
}


pub fn construct_machine_plain_backslash_delimited( patterns: &TMatchingPatterns, shared_actions: &TSharedActions ) -> Vec<Box<Action>> {
    let (pattern_literals, pattern_regexs) = (*patterns).clone();

    macro_rules! action {
        ($pattern_name:expr, $procedure:expr) => {
            box Action {
                regex: pattern_regexs.get($pattern_name).expect(&format!("no matching_pattern: {:?}", $pattern_name)).clone(),
                procedure: $procedure
            }
        };
    }

    macro_rules! pattern_lit {
        ($pattern_name:expr) => {
            pattern_literals.get($pattern_name).unwrap()
        };
    }

    macro_rules! get_shared_action {
        ( $action_name:expr ) => {
            shared_actions.get($action_name).unwrap().clone()
        };
    }

//   plain_backslash_delimited := |*
//       c_eol       => extend_string_eol;
//       c_any       => extend_string;
//   *|;
    vec![
        action!("c_eol", get_shared_action!("extend_string_eol")),
        action!("c_any", get_shared_action!("extend_string")),
    ]
}

pub fn construct_machine_interp_backslash_delimited_words( patterns: &TMatchingPatterns, shared_actions: &TSharedActions ) -> Vec<Box<Action>> {
    let (pattern_literals, pattern_regexs) = (*patterns).clone();

    macro_rules! action {
        ($pattern_name:expr, $procedure:expr) => {
            box Action {
                regex: pattern_regexs.get($pattern_name).expect(&format!("no matching_pattern: {:?}", $pattern_name)).clone(),
                procedure: $procedure
            }
        };
    }

    macro_rules! pattern_lit {
        ($pattern_name:expr) => {
            pattern_literals.get($pattern_name).unwrap()
        };
    }

    macro_rules! get_shared_action {
        ( $action_name:expr ) => {
            shared_actions.get($action_name).unwrap().clone()
        };
    }

//   interp_backslash_delimited_words := |*
//       interp_code => extend_interp_code;
//       interp_var  => extend_interp_var;
//       c_space+    => extend_string_space;
//       c_eol       => extend_string_eol;
//       c_any       => extend_string;
//   *|;
    vec![
        action!("interp_code", get_shared_action!("extend_interp_code")),
        action!("interp_var", get_shared_action!("extend_interp_var")),
        action_with_literal!( format!(r"{}+", pattern_lit!("c_space")) , get_shared_action!("extend_string_space")),
        action!("c_eol", get_shared_action!("extend_string_eol")),
        action!("c_any", get_shared_action!("extend_string")),
    ]
}


pub fn construct_machine_plain_backslash_delimited_words( patterns: &TMatchingPatterns, shared_actions: &TSharedActions ) -> Vec<Box<Action>> {
    let (pattern_literals, pattern_regexs) = (*patterns).clone();

    macro_rules! action {
        ($pattern_name:expr, $procedure:expr) => {
            box Action {
                regex: pattern_regexs.get($pattern_name).expect(&format!("no matching_pattern: {:?}", $pattern_name)).clone(),
                procedure: $procedure
            }
        };
    }

    macro_rules! pattern_lit {
        ($pattern_name:expr) => {
            pattern_literals.get($pattern_name).unwrap()
        };
    }

    macro_rules! get_shared_action {
        ( $action_name:expr ) => {
            shared_actions.get($action_name).unwrap().clone()
        };
    }

//   plain_backslash_delimited_words := |*
//       c_space+    => extend_string_space;
//       c_eol       => extend_string_eol;
//       c_any       => extend_string;
//   *|;
    vec![
        action_with_literal!( format!(r"{}+", pattern_lit!("c_space")) , get_shared_action!("extend_string_space")),
        action!("c_eol", get_shared_action!("extend_string_eol")),
        action!("c_any", get_shared_action!("extend_string")),
    ]
}
