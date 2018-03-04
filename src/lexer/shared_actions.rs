use std::collections::HashMap;

use lexer::LexingState;
use lexer::Lexer;
use lexer::action::{ActionProc};

use parser::token::Token;

pub type TSharedActions = HashMap<&'static str, ActionProc>;

pub fn construct() -> TSharedActions {
    let mut actions: TSharedActions = HashMap::new();

    // TODO share action! macro between shared_action and transactions
    macro_rules! action {
        ($pattern_name:expr, $procedure:expr) => {
            actions.insert($pattern_name, $procedure);
        };
    }

    action!("noop", |lexer: &mut Lexer|{
        // NOTE HACKING
        // preserve current state
        let current_state = lexer.current_state.clone();
        lexer.push_next_state(current_state);
    });

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
    action!("e_rparen", |lexer: &mut Lexer| {
        lexer.paren_nest -= 1;
    });

    //   e_lbrace = '{' % {
    //     @cond.push(false); @cmdarg.push(false)

    //     current_literal = literal
    //     if current_literal
    //       current_literal.start_interp_brace
    //     end
    //   };
    action!("e_lbrace", |lexer: &mut Lexer| {
        lexer.cond.push(false);
        lexer.cmdarg.push(false);

        match lexer.literal() {
            Some(literal) => {
                literal.start_interp_brace()
            }
            None => ()
        };
    });

    //   e_rbrace = '}' % {
    //     current_literal = literal
    //     if current_literal
    //       if current_literal.end_interp_brace_and_try_closing
    //         if version?(18, 19)
    //           emit(:tRCURLY, '}'.freeze, p - 1, p)
    //         else
    //           emit(:tSTRING_DEND, '}'.freeze, p - 1, p)
    //         end

    //         if current_literal.saved_herebody_s
    //           @herebody_s = current_literal.saved_herebody_s
    //         end

    //         fhold;
    //         fnext *stack_pop;
    //         fbreak;
    //       end
    //     end
    //   };
    action!("e_rbrace", |lexer: &mut Lexer| {
        panic!("UNIMPL");
    });

    // # Ruby is context-sensitive wrt/ local identifiers.
    // action local_ident {
    //     emit(:tIDENTIFIER)

    //     if !@static_env.nil? && @static_env.declared?(tok)
    //     fnext expr_endfn; fbreak;
    //     else
    //     fnext *arg_or_cmdarg; fbreak;
    //     end
    // }
    action!("local_ident", |lexer: &mut Lexer| {
        println!("shared action local_ident invoked");

        let token = Token::T_IDENTIFIER(lexer.input_stream.current_token_string());
        lexer.emit_token(token);

        let goto_expr_endfn = match lexer.static_env {
            None => false,
            Some(ref static_env) => {
                static_env.has_declared(lexer.input_stream.current_token().unwrap())
            }
        };

        if goto_expr_endfn {
            lexer.push_next_state(state!("expr_endfn"));
            lexer.flag_breaking();
        } else {
            let next_state = lexer.arg_or_cmdarg();
            lexer.push_next_state(next_state);
            lexer.flag_breaking();
        }
    });

    actions
}
