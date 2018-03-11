// # Variable lexing code is accessed from both expressions and
// # string interpolation related code.

use lexer::Lexer;
use lexer::LexingState;
use lexer::action::{Action};
use lexer::matching_patterns::TMatchingPatterns;
use lexer::shared_actions::TSharedActions;

use parser::token::Token;

pub fn construct_machine_expr_variable( patterns: &TMatchingPatterns, shared_actions: &TSharedActions ) -> Vec<Box<Action>> {
    let (pattern_literals, pattern_regexs) = (*patterns).clone();

    macro_rules! action {
        ($pattern_name:expr, $procedure:expr) => {
            box Action {
                regex: pattern_regexs.get($pattern_name).expect(&format!("no matching_pattern: {:?}", $pattern_name)).clone(), // TODO clone?
                procedure: $procedure
            }
        };
    }

    macro_rules! get_shared_action {
        ( $action_name:expr ) => {
            shared_actions.get($action_name).unwrap().clone()
        };
    }

    vec![

        // TODO INCOMPLETE
        //     global_var
        //     => {
        //       if    tok =~ /^\$([1-9][0-9]*)$/
        //         emit(:tNTH_REF, tok(@ts + 1).to_i)
        //       elsif tok =~ /^\$([&`'+])$/
        //         emit(:tBACK_REF)
        //       else
        //         emit(:tGVAR)
        //       end
        //       fnext *stack_pop; fbreak;
        //     };
        // TODO INCOMPLETE
        action!("global_var", |lexer: &mut Lexer| {
            let token = Token::T_GVAR( lexer.input_stream.current_token_string() );
            lexer.emit_token(token);

            // TODO NOTE `fnext *stack_pop` seems unnecessary here?
            // since, that's what will happen, right?

            lexer.flag_breaking();
        }),

        //   class_var_v
        //   => {
        //     if tok =~ /^@@[0-9]/
        //       diagnostic :error, :cvar_name, { :name => tok }
        //     end
        // TODO INCOMPLETE
        action!("class_var_v", |lexer: &mut Lexer| {
            panic!("class_var_v UNIMPL");
        }),

        //     instance_var_v
        //     => {
        //       if tok =~ /^@[0-9]/
        //         diagnostic :error, :ivar_name, { :name => tok }
        //       end

        //       emit(:tIVAR)
        //       fnext *stack_pop; fbreak;
        //     };
        // TODO INCOMPLETE
        action!("instance_var_v", |lexer: &mut Lexer| {
            let token = Token::T_IVAR(lexer.input_stream.current_token_string());
            lexer.emit_token(token);

            // TODO NOTE `fnext *stack_pop` seems unnecessary here?
            // since, that's what will happen, right?

            lexer.flag_breaking();
        }),

    ]
}
