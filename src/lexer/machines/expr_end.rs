// TRACKER
//   SOME UNIMPL

//   expr_end := |*


//   *|;

use regex::Regex;

use lexer::Lexer;
use lexer::LexingState;
use lexer::action::{Action};
use lexer::matching_patterns::TMatchingPatterns;
use lexer::shared_actions::TSharedActions;

use parser::parser::Token;

pub fn construct_machine_expr_end( patterns: &TMatchingPatterns, shared_actions: &TSharedActions ) -> Vec<Box<Action>> {
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

    vec![

        //       #
        //       # STABBY LAMBDA
        //       #

        //       '->'
        //       => {
        //         emit(:tLAMBDA, '->'.freeze, @ts, @ts + 2)
        // 
        //         @lambda_stack.push @paren_nest
        //         fnext expr_endfn; fbreak;
        //       };

        action_with_literal!("->", |lexer: &mut Lexer| {
            lexer.emit_token(Token::T_LAMBDA);

            lexer.lambda_stack.push(lexer.paren_nest);
            lexer.push_next_state(state!("expr_endfn"));
            lexer.flag_breaking();
        }),

        //       e_lbrace | 'do'
        //       => {
        //         if @lambda_stack.last == @paren_nest
        //           @lambda_stack.pop

        //           if tok == '{'.freeze
        //             emit(:tLAMBEG, '{'.freeze)
        //           else # 'do'
        //             emit(:kDO_LAMBDA, 'do'.freeze)
        //           end
        //         else
        //           if tok == '{'.freeze
        //             emit(:tLCURLY, '{'.freeze)
        //           else # 'do'
        //             emit_do
        //           end
        //         end

        //         fnext expr_value; fbreak;
        //       };
        action_with_literal!(
            format!("({})|(do)", pattern_lit!("e_lbrace")),
            |lexer: &mut Lexer| {

                if lexer.input_stream.current_token().unwrap() == String::from("{") {
                    lexer.invoke_proc("e_lbrace");
                }

                if lexer.lambda_stack.last() != None && lexer.lambda_stack.last().unwrap() == &lexer.paren_nest {
                    lexer.lambda_stack.pop();

                    if lexer.input_stream.current_token().unwrap() == String::from("{") {
                        lexer.emit_token(Token::T_LAMBEG);
                    } else {
                        lexer.emit_token(Token::K_DO_LAMBDA);
                    }
                } else {
                    if lexer.input_stream.current_token().unwrap() == String::from("{") {
                        lexer.emit_token(Token::T_LCURLY);
                    } else {
                        panic!("UNIMPL emit_do");
                    }
                }
                lexer.push_next_state(state!("expr_value"));
                lexer.flag_breaking();
            }
        ),

        //       #
        //       # KEYWORDS
        //       #

        //       keyword_with_fname
        //       => { emit_table(KEYWORDS)
        //            fnext expr_fname; fbreak; };
        action!("keyword_with_fname", |lexer: &mut Lexer| {
            lexer.emit_token_from_table("keywords");
            lexer.push_next_state(state!("expr_fname"));
            lexer.flag_breaking();
        }),

        //       'class' w_any* '<<'
        //       => { emit(:kCLASS, 'class'.freeze, @ts, @ts + 5)
        //            emit(:tLSHFT, '<<'.freeze,    @te - 2, @te)
        //            fnext expr_value; fbreak; };
        action_with_literal!(
            format!(r"class{}<<", pattern_lit!("w_any_*")),
            |lexer: &mut Lexer| {
                lexer.emit_token(Token::K_CLASS);
                lexer.emit_token(Token::T_LSHFT); // TODO originally has value `<<`
                lexer.push_next_state(state!("expr_value"));
                lexer.flag_breaking();
            }
        ),

        //       # a if b:c: Syntax error.
        //       keyword_modifier
        //       => { emit_table(KEYWORDS)
        //            fnext expr_beg; fbreak; };
        action!("keyword_modifier", |lexer: &mut Lexer| {
            lexer.emit_token_from_table("keywords");
            lexer.push_next_state(state!("expr_beg"));
            lexer.flag_breaking();
        }),

        //       # elsif b:c: elsif b(:c)
        //       keyword_with_value
        //       => { emit_table(KEYWORDS)
        //            fnext expr_value; fbreak; };
        action!("keyword_with_value", |lexer: &mut Lexer| {
            lexer.emit_token_from_table("keywords");
            lexer.push_next_state(state!("expr_value"));
            lexer.flag_breaking();
        }),

        //       keyword_with_mid
        //       => { emit_table(KEYWORDS)
        //            fnext expr_mid; fbreak; };
        action!("keyword_with_mid", |lexer: &mut Lexer| {
            lexer.emit_token_from_table("keywords");
            lexer.push_next_state(state!("expr_mid"));
            lexer.flag_breaking();
        }),

        //       keyword_with_arg
        //       => {
        //         emit_table(KEYWORDS)

        //         if version?(18) && tok == 'not'.freeze
        //           fnext expr_beg; fbreak;
        //         else
        //           fnext expr_arg; fbreak;
        //         end
        //       };
        action!("keyword_with_arg", |lexer: &mut Lexer| {
            lexer.emit_token_from_table("keywords");
            // NOTE IGNORED version18
            lexer.push_next_state(state!("expr_arg"));
            lexer.flag_breaking();
        }),

        //       '__ENCODING__'
        //       => {
        //         if version?(18)
        //           emit(:tIDENTIFIER)

        //           unless !@static_env.nil? && @static_env.declared?(tok)
        //             fnext *arg_or_cmdarg;
        //           end
        //         else
        //           emit(:k__ENCODING__, '__ENCODING__'.freeze)
        //         end
        //         fbreak;
        //       };
        action_with_literal!("__ENCODING__", |lexer: &mut Lexer| {
            // NOTE ignored version18
            lexer.emit_token(Token::K__ENCODING__);
            lexer.flag_breaking();
        }),

        //       keyword_with_end
        //       => { emit_table(KEYWORDS)
        //            fbreak; };
        action!("keyword_with_end", |lexer: &mut Lexer| {
            lexer.emit_token_from_table("keywords");
            lexer.flag_breaking();
        }),

        //       #
        //       # NUMERIC LITERALS
        //       #

        //       ( '0' [Xx] %{ @num_base = 16; @num_digits_s = p } int_hex
        //       | '0' [Dd] %{ @num_base = 10; @num_digits_s = p } int_dec
        //       | '0' [Oo] %{ @num_base = 8;  @num_digits_s = p } int_dec
        //       | '0' [Bb] %{ @num_base = 2;  @num_digits_s = p } int_bin
        //       | [1-9] digit* '_'? %{ @num_base = 10; @num_digits_s = @ts } int_dec
        //       | '0'   digit* '_'? %{ @num_base = 8;  @num_digits_s = @ts } int_dec
        //       ) %{ @num_suffix_s = p } int_suffix
        //       => {
        //         digits = tok(@num_digits_s, @num_suffix_s)

        //         if digits.end_with? '_'.freeze
        //           diagnostic :error, :trailing_in_number, { :character => '_'.freeze },
        //                      range(@te - 1, @te)
        //         elsif digits.empty? && @num_base == 8 && version?(18)
        //           # 1.8 did not raise an error on 0o.
        //           digits = '0'.freeze
        //         elsif digits.empty?
        //           diagnostic :error, :empty_numeric
        //         elsif @num_base == 8 && (invalid_idx = digits.index(/[89]/))
        //           invalid_s = @num_digits_s + invalid_idx
        //           diagnostic :error, :invalid_octal, nil,
        //                      range(invalid_s, invalid_s + 1)
        //         end

        //         if version?(18, 19, 20)
        //           emit(:tINTEGER, digits.to_i(@num_base), @ts, @num_suffix_s)
        //           p = @num_suffix_s - 1
        //         else
        //           @num_xfrm.call(digits.to_i(@num_base))
        //         end
        //         fbreak;
        //       };
        // 
        // TODO UNIMPL
        // 

        //       flo_frac flo_pow?
        //       => {
        //         diagnostic :error, :no_dot_digit_literal
        //       };
        // 
        // TODO UNIMPL
        // 

        //       flo_int [eE]
        //       => {
        //         if version?(18, 19, 20)
        //           diagnostic :error,
        //                      :trailing_in_number, { :character => tok(@te - 1, @te) },
        //                      range(@te - 1, @te)
        //         else
        //           emit(:tINTEGER, tok(@ts, @te - 1).to_i, @ts, @te - 1)
        //           fhold; fbreak;
        //         end
        //       };
        // 
        // TODO UNIMPL
        // 

        //       flo_int flo_frac [eE]
        //       => {
        //         if version?(18, 19, 20)
        //           diagnostic :error,
        //                      :trailing_in_number, { :character => tok(@te - 1, @te) },
        //                      range(@te - 1, @te)
        //         else
        //           emit(:tFLOAT, tok(@ts, @te - 1).to_f, @ts, @te - 1)
        //           fhold; fbreak;
        //         end
        //       };
        // 
        // TODO UNIMPL
        // 

        //       flo_int
        //       ( flo_frac? flo_pow %{ @num_suffix_s = p } flo_pow_suffix
        //       | flo_frac          %{ @num_suffix_s = p } flo_suffix
        //       )
        //       => {
        //         digits = tok(@ts, @num_suffix_s)
        // 
        //         if version?(18, 19, 20)
        //           emit(:tFLOAT, Float(digits), @ts, @num_suffix_s)
        //           p = @num_suffix_s - 1
        //         else
        //           @num_xfrm.call(digits)
        //         end
        //         fbreak;
        //       };
        // 
        // TODO UNIMPL
        // 

        //       #
        //       # STRING AND XSTRING LITERALS
        //       #

        //       # `echo foo`, "bar", 'baz'
        //       '`' | ['"] # '
        //       => {
        //         type, delimiter = tok, tok[-1].chr
        //         fgoto *push_literal(type, delimiter, @ts, nil, false, false, true);
        //       };
        // 
        // TODO UNIMPL
        // 

        //       #
        //       # CONSTANTS AND VARIABLES
        //       #

        //       constant
        //       => { emit(:tCONSTANT)
        //            fnext *arg_or_cmdarg; fbreak; };
        action!("constant", |lexer: &mut Lexer| {
            let next_state = lexer.arg_or_cmdarg();
            lexer.push_next_state(next_state);
            lexer.flag_breaking();
        }),

        //       constant ambiguous_const_suffix
        //       => { emit(:tCONSTANT, tok(@ts, tm), @ts, tm)
        //            p = tm - 1; fbreak; };
        // 
        // TODO UNIMPL 
        // TODO EMBEDDED ACTIONS
        // 

        //       global_var | class_var_v | instance_var_v
        //       => { p = @ts - 1; fcall expr_variable; };
        action_with_literal!(
            format!("({})|({})|({})", pattern_lit!("global_var"), pattern_lit!("class_var_v"), pattern_lit!("instance_var_v")),
            |lexer: &mut Lexer| {
                lexer.input_stream.hold_current_token();
                // TODO is this enough to simulate `fcall`?
                lexer.push_next_state(state!("expr_variable"));
            }
        ),

        //       #
        //       # METHOD CALLS
        //       #

        //       '.' | '&.' | '::'
        //       => { emit_table(PUNCTUATION)
        //            fnext expr_dot; fbreak; };
        action_with_literal!( r"(\.)|(&\.)|(::)", |lexer: &mut Lexer| {
            lexer.emit_token_from_table("punctuation");
            lexer.push_next_state(state!("expr_dot"));
            lexer.flag_breaking();
        }),

        //       call_or_var
        //       => local_ident;
        action!("call_or_var", get_shared_action!("local_ident")),

        //       bareword ambiguous_fid_suffix
        //       => {
        //         if tm == @te
        //           # Suffix was consumed, e.g. foo!
        //           emit(:tFID)
        //         else
        //           # Suffix was not consumed, e.g. foo!=
        //           emit(:tIDENTIFIER, tok(@ts, tm), @ts, tm)
        //           p = tm - 1
        //         end
        //         fnext expr_arg; fbreak;
        //       };
        // 
        // TODO UNIMPL
        // TODO EMBEDDED ACTIONS
        // 

        //       #
        //       # OPERATORS
        //       #

        //       ( e_lparen
        //       | operator_arithmetic
        //       | operator_rest
        //       )
        //       => { emit_table(PUNCTUATION)
        //            fnext expr_beg; fbreak; };
        // 
        // NOTE SEPARATED INTO 2 ACTIONS
        action_with_literal!(
            format!(r"{}", pattern_lit!("e_lparen")),
            |lexer: &mut Lexer| {
                lexer.invoke_proc("e_lparen");
                lexer.emit_token_from_table("punctuation");
                lexer.push_next_state(state!("expr_beg"));
                lexer.flag_breaking();
            }
        ),
        action_with_literal!(
            format!(r"({})|({})", pattern_lit!("operator_arithmetic"), pattern_lit!("operator_rest")),
            |lexer: &mut Lexer| {
                lexer.emit_token_from_table("punctuation");
                lexer.push_next_state(state!("expr_beg"));
                lexer.flag_breaking();
            }
        ),

        //       e_rbrace | e_rparen | ']'
        //       => {
        //         emit_table(PUNCTUATION)
        //         @cond.lexpop; @cmdarg.lexpop

        //         if RBRACE_OR_RBRACK.include?(tok)
        //           fnext expr_endarg;
        //         else # )
        //           # fnext expr_endfn; ?
        //         end

        //         fbreak;
        //       };
        action_with_literal!(
            format!(r"({})|({})|(\])", pattern_lit!("e_rbrace"), pattern_lit!("e_rparen")),
            |lexer: &mut Lexer| {

                match lexer.input_stream.current_token().unwrap().as_ref() {
                    "]" => { lexer.invoke_proc("e_rbrace"); },
                    ")" => { lexer.invoke_proc("e_rparen"); },
                    _ => ()
                };

                lexer.emit_token_from_table("punctuation");
                lexer.cond.lexpop();
                lexer.cmdarg.lexpop();

                // RBRACE_OR_RBRACK = %w"} ]".freeze
                // `}` `]`
                match lexer.input_stream.current_token().unwrap().as_ref() {
                    "}" | "]" => {
                        lexer.push_next_state(state!("expr_endarg"));
                    },
                    _ => ()
                };

                lexer.flag_breaking();
            }
        ),

        //       operator_arithmetic '='
        //       => { emit(:tOP_ASGN, tok(@ts, @te - 1))
        //            fnext expr_beg; fbreak; };
        action_with_literal!( format!(r"{}=", pattern_lit!("operator_arithmetic")), |lexer: &mut Lexer| {
            lexer.emit_token(Token::T_OP_ASGN);
            lexer.push_next_state(state!("expr_beg"));
            lexer.flag_breaking();
        }),


        //       '?'
        //       => { emit(:tEH, '?'.freeze)
        //            fnext expr_value; fbreak; };
        action_with_literal!(r"?", |lexer: &mut Lexer| {
            lexer.emit_token(Token::T_EH);
            lexer.push_next_state(state!("expr_value"));
            lexer.flag_breaking();
        }),

        //       e_lbrack
        //       => { emit(:tLBRACK2, '['.freeze)
        //            fnext expr_beg; fbreak; };
        action_with_literal!(r"\[", |lexer: &mut Lexer| {
            lexer.invoke_proc("e_lbrack");
            lexer.emit_token(Token::T_LBRACK2);
            lexer.push_next_state(state!("expr_beg"));
            lexer.flag_breaking();
        }),

        //       punctuation_end
        //       => { emit_table(PUNCTUATION)
        //            fnext expr_beg; fbreak; };
        action!("punctuation_end", |lexer: &mut Lexer| {
            lexer.emit_token_from_table("punctuation");
            lexer.push_next_state(state!("expr_beg"));
            lexer.flag_breaking();
        }),

        //       #
        //       # WHITESPACE
        //       #

        //       w_space_comment;
        action!("w_space_comment", get_shared_action!("noop")),

        //       w_newline
        //       => { fgoto leading_dot; };
        action!("w_newline", |lexer: &mut Lexer| {
            lexer.push_next_state(state!("leading_dot"));
        }),

        //       ';'
        //       => { emit(:tSEMI, ';'.freeze)
        //            fnext expr_value; fbreak; };
        action_with_literal!(";", |lexer: &mut Lexer| {
            lexer.emit_token(Token::T_SEMI);
            lexer.push_next_state(state!("expr_value"));
            lexer.flag_breaking();
        }),

        //       '\\' c_line {
        //         diagnostic :error, :bare_backslash, nil, range(@ts, @ts + 1)
        //         fhold;
        //       };
        // 
        // TODO UNIMPL
        // 

        //       c_any
        //       => {
        //         diagnostic :fatal, :unexpected, { :character => tok.inspect[1..-2] }
        //       };
        action!("c_any", |lexer: &mut Lexer| {panic!("UNIMPL");}),

        //       c_eof => do_eof;
        action!("c_eof", get_shared_action!("do_eof")),

    ]
}
