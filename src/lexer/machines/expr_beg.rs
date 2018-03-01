//   # Beginning of an expression.
//   #
//   # Don't fallthrough to this state from `c_any`; make sure to handle
//   # `c_space* c_nl` and let `expr_end` handle the newline.
//   # Otherwise code like `f\ndef x` gets glued together and the parser
//   # explodes.
//   #

use regex::Regex;

use lexer::Lexer;
use lexer::LexingState;
use lexer::action::{Action};
use lexer::matching_patterns::TMatchingPatterns;
use lexer::shared_actions::TSharedActions;
use lexer::literal::Literal;

use parser::token::Token;

pub fn construct_machine_expr_beg( patterns: &TMatchingPatterns, shared_actions: &TSharedActions ) -> Vec<Box<Action>> {
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
        //       # +5, -5, -  5
        //       [+\-] w_any* [0-9]
        //       => {
        //         emit(:tUNARY_NUM, tok(@ts, @ts + 1), @ts, @ts + 1)
        //         fhold; fnext expr_end; fbreak;
        //       };
        action_with_literal!(
            format!(r"{}{}{}",
                r"[+-]",
                pattern_lit!("w_any"),
                r"[[:digit:]]"
            ), |lexer: &mut Lexer| {
                let token = Token::T_UNARY_NUM(lexer.input_stream.token_string_from_range(lexer.input_stream.ts.unwrap(), lexer.input_stream.ts.unwrap() + 1));
                lexer.emit_token(token);
                lexer.input_stream.hold_current_char();
                lexer.push_next_state(state!("expr_end"));
                lexer.flag_breaking();
            }),

        //       # splat *a
        //       '*'
        //       => { emit(:tSTAR, '*'.freeze)
        //            fbreak; };
        action_with_literal!(r"\*", |lexer: &mut Lexer| {
            lexer.emit_token(Token::T_STAR); // TODO original :tSTAR has a value '*'
            lexer.flag_breaking();
        }),

        //       #
        //       # STRING AND REGEXP LITERALS
        //       #

        //       # /regexp/oui
        //       # /=/ (disambiguation with /=)
        //       '/' c_any
        //       => {
        //         type = delimiter = tok[0].chr
        //         fhold; fgoto *push_literal(type, delimiter, @ts);
        //       };
        // 
        action_with_literal!(
            format!(r"/{}",
                pattern_lit!("c_any"),
            ), |lexer: &mut Lexer| {
                let lit_type: String = lexer.input_stream.current_token().unwrap().chars().take(1).collect(); // TODO really...
                let lit_delimiter = lit_type.clone();
                let literal = Literal::new( lit_type, lit_delimiter, lexer.input_stream.ts.unwrap() );

                lexer.input_stream.hold_current_char();

                // TODO can we write sth like this when NLL is available?
                // lexer.push_next_state(lexer.push_literal(literal));

                let next_state = lexer.push_literal(literal);
                lexer.push_next_state(next_state);
            }),

        //       # %<string>
        //       '%' ( any - [A-Za-z] )
        //       => {
        //         type, delimiter = @source_buffer.slice(@ts).chr, tok[-1].chr
        //         fgoto *push_literal(type, delimiter, @ts);
        //       };
        // 
        // NOTE tok is `%(`, `%<`, etc.
        // 
        action_with_literal!(r"%[^[:alpha:]]", |lexer: &mut Lexer| {
            let lit_type = String::from("%");
            let lit_delimiter = lexer.input_stream.current_token().unwrap().chars().last().unwrap().to_string();

            let literal = Literal::new( lit_type, lit_delimiter, lexer.input_stream.ts.unwrap() );

            let next_state = lexer.push_literal(literal);
            lexer.push_next_state(next_state);
        }),

        //       # %w(we are the people)
        //       '%' [A-Za-z]+ c_any
        //       => {
        //         type, delimiter = tok[0..-2], tok[-1].chr
        //         fgoto *push_literal(type, delimiter, @ts);
        //       };
        action_with_literal!(
            format!(r"%[[:alpha:]]+{}", pattern_lit!("c_any")),
            |lexer: &mut Lexer| {
                let current_slice = lexer.input_stream.current_token().unwrap();
                let lit_type = String::from(&current_slice[ 0..( current_slice.len() - 1 ) ]);
                let lit_delimiter = current_slice.chars().last().unwrap().to_string();
                let literal = Literal::new( lit_type, lit_delimiter, lexer.input_stream.ts.unwrap() );
                let next_state = lexer.push_literal(literal);
                lexer.push_next_state(next_state);
            }),

        //       '%' c_eof
        //       => {
        //         diagnostic :fatal, :string_eof, nil, range(@ts, @ts + 1)
        //       };
        // TODO INCOMPLETE
        action_with_literal!(
            format!(r"%{}", pattern_lit!("c_eof")),
            |lexer: &mut Lexer| {
                // TODO
            }
        ),

        //       # Heredoc start.
        //       # <<END  | <<'END'  | <<"END"  | <<`END`  |
        //       # <<-END | <<-'END' | <<-"END" | <<-`END` |
        //       # <<~END | <<~'END' | <<~"END" | <<~`END`
        //       '<<' [~\-]?
        //         ( '"' ( any - '"' )* '"'
        //         | "'" ( any - "'" )* "'"
        //         | "`" ( any - "`" )* "`"
        //         | bareword ) % { heredoc_e      = p }
        //         c_line* c_nl % { new_herebody_s = p }
        //       => {
        //         tok(@ts, heredoc_e) =~ /^<<(-?)(~?)(["'`]?)(.*)\3$/m

        //         indent      = !$1.empty? || !$2.empty?
        //         dedent_body = !$2.empty?
        //         type        =  $3.empty? ? '<<"'.freeze : ('<<'.freeze + $3)
        //         delimiter   =  $4

        //         if @version >= 24
        //           if delimiter.count("\n") > 0
        //             if delimiter.end_with?("\n")
        //               diagnostic :warning, :heredoc_id_ends_with_nl, nil, range(@ts, @ts + 1)
        //               delimiter = delimiter.rstrip
        //             else
        //               diagnostic :fatal, :heredoc_id_has_newline, nil, range(@ts, @ts + 1)
        //             end
        //           end
        //         end

        //         if dedent_body && version?(18, 19, 20, 21, 22)
        //           emit(:tLSHFT, '<<'.freeze, @ts, @ts + 2)
        //           p = @ts + 1
        //           fnext expr_beg; fbreak;
        //         else
        //           fnext *push_literal(type, delimiter, @ts, heredoc_e, indent, dedent_body);

        //           @herebody_s ||= new_herebody_s
        //           p = @herebody_s - 1
        //         end
        //       };
        // TODO UNIMPL

        //       #
        //       # SYMBOL LITERALS
        //       #

        //       # :"bar", :'baz'
        //       ':' ['"] # '
        //       => {
        //         type, delimiter = tok, tok[-1].chr
        //         fgoto *push_literal(type, delimiter, @ts);
        //       };
        // NOTE tok is like `:'`, `:"`
        action_with_literal!(
            ":['\"]",
            |lexer: &mut Lexer| {
                let current_slice = lexer.input_stream.current_token().unwrap();
                let lit_type = current_slice.clone();
                let lit_delimiter = current_slice.chars().last().unwrap().to_string();
                let literal = Literal::new( lit_type, lit_delimiter, lexer.input_stream.ts.unwrap() );
                let next_state = lexer.push_literal(literal);
                lexer.push_next_state(next_state);
            }
        ),

        //       ':' bareword ambiguous_symbol_suffix
        //       => {
        //         emit(:tSYMBOL, tok(@ts + 1, tm), @ts, tm)
        //         p = tm - 1
        //         fnext expr_end; fbreak;
        //       };
        // 
        // NOTE embedded action
        //          ambiguous_symbol_suffix <- ambiguous_ident_suffix <- ambiguous_fid_suffix
        // TODO UNIMPL
        // 
        action_with_literal!(
            // TODO DUMMY
            // format!(":{}{}", pattern_lit!("bareword"), pattern_lit!("ambiguous_symbol_suffix")),
            format!(":{}", pattern_lit!("bareword")),
            |lexer: &mut Lexer| {
                panic!("TODO");
            }
        ),

        //       ':' ( bareword | global_var | class_var | instance_var |
        //             operator_fname | operator_arithmetic | operator_rest )
        //       => {
        //         emit(:tSYMBOL, tok(@ts + 1), @ts)
        //         fnext expr_end; fbreak;
        //       };
        action_with_literal!(
            format!(":({}|{}|{}|{}|{}|{}|{})",
                pattern_lit!("bareword"), pattern_lit!("global_var"),
                pattern_lit!("class_var"), pattern_lit!("instance_var"),
                pattern_lit!("operator_fname"), pattern_lit!("operator_arithmetic"), pattern_lit!("operator_rest")
            ),
            |lexer: &mut Lexer| {
                let token = Token::T_SYMBOL( lexer.input_stream.token_string_from_range(lexer.input_stream.ts.unwrap()+1, lexer.input_stream.te.unwrap()) );
                lexer.emit_token(token);
                lexer.push_next_state(state!("expr_end"));
                lexer.flag_breaking();
            }
        ),

        //       #
        //       # AMBIGUOUS TERNARY OPERATOR
        //       #

        //       # Character constant, like ?a, ?\n, ?\u1000, and so on
        //       # Don't accept \u escape with multiple codepoints, like \u{1 2 3}
        //       '?' ( e_bs ( escape - ( '\u{' (xdigit+ [ \t]+)+ xdigit+ '}' ))
        //           | (c_any - c_space_nl - e_bs) % { @escape = nil }
        //           )
        //       => {
        //         value = @escape || tok(@ts + 1)

        //         if version?(18)
        //           emit(:tINTEGER, value.dup.force_encoding(Encoding::BINARY)[0].ord)
        //         else
        //           emit(:tCHARACTER, value)
        //         end

        //         fnext expr_end; fbreak;
        //       };
        // TODO UNIMPL

        //       '?' c_space_nl
        //       => {
        //         escape = { " "  => '\s', "\r" => '\r', "\n" => '\n', "\t" => '\t',
        //                    "\v" => '\v', "\f" => '\f' }[@source_buffer.slice(@ts + 1)]
        //         diagnostic :warning, :invalid_escape_use, { :escape => escape }, range

        //         p = @ts - 1
        //         fgoto expr_end;
        //       };
        action_with_literal!(
            format!(r"\?{}", pattern_lit!("c_space_nl")),
            |lexer: &mut Lexer| {
                panic!("UNIMPL");
            }
        ),

        //       '?' c_eof
        //       => {
        //         diagnostic :fatal, :incomplete_escape, nil, range(@ts, @ts + 1)
        //       };
        action_with_literal!(
            format!(r"\?{}", pattern_lit!("c_eof")),
            |lexer: &mut Lexer| {
                panic!("UNIMPL");
            }
        ),

        //       # f ?aa : b: Disambiguate with a character literal.
        //       '?' [A-Za-z_] bareword
        //       => {
        //         p = @ts - 1
        //         fgoto expr_end;
        //       };
        action_with_literal!(
            format!(r"\?[A-Za-z_]{}", pattern_lit!("bareword")),
            |lexer: &mut Lexer| {
                lexer.input_stream.hold_current_token();
                lexer.push_next_state(state!("expr_end"));
            }
        ),

        //       #
        //       # KEYWORDS AND PUNCTUATION
        //       #

        //       # a({b=>c})
        //       e_lbrace
        //       => {
        //         if @lambda_stack.last == @paren_nest
        //           @lambda_stack.pop
        //           emit(:tLAMBEG, '{'.freeze)
        //         else
        //           emit(:tLBRACE, '{'.freeze)
        //         end
        //         fbreak;
        //       };
        action!("e_lbrace", |lexer: &mut Lexer| {
            lexer.invoke_proc("e_lbrace");

            if lexer.lambda_stack.last() != None && lexer.lambda_stack.last().unwrap() == &lexer.paren_nest {
                lexer.lambda_stack.pop();
                lexer.emit_token(Token::T_LAMBEG);
            } else {
                lexer.emit_token(Token::T_LBRACE);
            }

            lexer.flag_breaking();
        }),

        //       # a([1, 2])
        //       e_lbrack
        //       => { emit(:tLBRACK, '['.freeze)
        //            fbreak; };
        action!("e_lbrack", |lexer: &mut Lexer| {
            lexer.invoke_proc("e_lbrack");
            lexer.emit_token(Token::T_LBRACK);
            lexer.flag_breaking();
        }),

        //       # a()
        //       e_lparen
        //       => { emit(:tLPAREN, '('.freeze)
        //            fbreak; };
        action!("e_lparen", |lexer: &mut Lexer| {
            lexer.invoke_proc("e_lparen");
            lexer.emit_token(Token::T_LPAREN);
            lexer.flag_breaking();
        }),

        //       # a(+b)
        //       punctuation_begin
        //       => { emit_table(PUNCTUATION_BEGIN)
        //            fbreak; };
        action!("punctuation_begin", |lexer: &mut Lexer| {
            lexer.emit_token_from_table("puncuation_begin");
            lexer.flag_breaking();
        }),

        //       # rescue Exception => e: Block rescue.
        //       # Special because it should transition to expr_mid.
        //       'rescue' %{ tm = p } '=>'?
        //       => { emit(:kRESCUE, 'rescue'.freeze, @ts, tm)
        //            p = tm - 1
        //            fnext expr_mid; fbreak; };
        // TODO UNIMPL

        //       # if a: Statement if.
        //       keyword_modifier
        //       => { emit_table(KEYWORDS_BEGIN)
        //            fnext expr_value; fbreak; };
        action!("keyword_modifier", |lexer: &mut Lexer| {
            lexer.emit_token_from_table("keywords_begin");
            lexer.push_next_state(state!("expr_value"));
            lexer.flag_breaking();
        }),

        //       #
        //       # RUBY 1.9 HASH LABELS
        //       #

        //       label ( any - ':' )
        //       => {
        //         fhold;

        //         if version?(18)
        //           ident = tok(@ts, @te - 2)

        //           emit((@source_buffer.slice(@ts) =~ /[A-Z]/) ? :tCONSTANT : :tIDENTIFIER,
        //                ident, @ts, @te - 2)
        //           fhold; # continue as a symbol

        //           if !@static_env.nil? && @static_env.declared?(ident)
        //             fnext expr_end;
        //           else
        //             fnext *arg_or_cmdarg;
        //           end
        //         else
        //           emit(:tLABEL, tok(@ts, @te - 2), @ts, @te - 1)
        //           fnext expr_labelarg;
        //         end

        //         fbreak;
        //       };
        action_with_literal!(
            format!("{}[^:]", pattern_lit!("label")),
            |lexer: &mut Lexer| {
                panic!("UNIMPL");
            }
        ),

        //       #
        //       # CONTEXT-DEPENDENT VARIABLE LOOKUP OR COMMAND INVOCATION
        //       #

        //       # foo= bar:  Disambiguate with bareword rule below.
        //       bareword ambiguous_ident_suffix |
        //       # def foo:   Disambiguate with bareword rule below.
        //       keyword
        //       => { p = @ts - 1
        //            fgoto expr_end; };
        // TODO UNIMPL
        // TODO WITH EMBEDDED ACTION ambiguous_ident_suffix <- ambiguous_fid_suffix


        //       # a = 42;     a [42]: Indexing.
        //       # def a; end; a [42]: Array argument.
        //       call_or_var
        //       => local_ident;
        action!("call_or_var", get_shared_action!("local_ident")),

        //       (call_or_var - keyword)
        //         % { ident_tok = tok; ident_ts = @ts; ident_te = @te; }
        //       w_space+ '('
        //       => {
        //         emit(:tIDENTIFIER, ident_tok, ident_ts, ident_te)
        //         p = ident_te - 1

        //         if !@static_env.nil? && @static_env.declared?(ident_tok) && @version < 25
        //           fnext expr_endfn;
        //         else
        //           fnext expr_cmdarg;
        //         end
        //         fbreak;
        //       };
        // TODO UNIMPL


        //       #
        //       # WHITESPACE
        //       #

        //       w_any;
        action!("w_any", get_shared_action!("noop")),

        //       e_heredoc_nl '=begin' ( c_space | c_nl_zlen )
        //       => { p = @ts - 1
        //            fgoto line_begin; };
        // TODO UNIMPL

        //       #
        //       # DEFAULT TRANSITION
        //       #

        //       # The following rules match most binary and all unary operators.
        //       # Rules for binary operators provide better error reporting.
        //       operator_arithmetic '='    |
        //       operator_rest              |
        //       punctuation_end            |
        //       c_any
        //       => { p = @ts - 1; fgoto expr_end; };
        // 
        action_with_literal!(
            format!(r"({}=)|({})|({})|({})",
                pattern_lit!("operator_arithmetic"),
                pattern_lit!("operator_rest"),
                pattern_lit!("punctuation_end"),
                pattern_lit!("c_any"),
            ),
            |lexer: &mut Lexer| {
                lexer.input_stream.hold_current_token();
                lexer.push_next_state(state!("expr_end"));
            }
        ),

        //       c_eof => do_eof;
        action!("c_eof", get_shared_action!("do_eof")),
    ]
}
