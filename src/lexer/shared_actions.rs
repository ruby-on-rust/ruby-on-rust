use std::collections::HashMap;

use regex::Regex;

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


    // //   #
    // //   # === ESCAPE SEQUENCE PARSING ===
    // //   #

    // //   # Escape parsing code is a Ragel pattern, not a scanner, and therefore
    // //   # it shouldn't directly raise errors or perform other actions with side effects.
    // //   # In reality this would probably just mess up error reporting in pathological
    // //   # cases, through.

    // //   # The amount of code required to parse \M\C stuff correctly is ridiculous.

    // //   escaped_nl = "\\" c_nl;

    // //   action unicode_points {
    // //     @escape = ""

    // //     codepoints  = tok(@escape_s + 2, p - 1)
    // //     codepoint_s = @escape_s + 2

    // //     if @version < 24
    // //       if codepoints.start_with?(" ") || codepoints.start_with?("\t")
    // //         diagnostic :fatal, :invalid_unicode_escape, nil,
    // //           range(@escape_s + 2, @escape_s + 3)
    // //       end

    // //       if spaces_p = codepoints.index(/[ \t]{2}/)
    // //         diagnostic :fatal, :invalid_unicode_escape, nil,
    // //           range(codepoint_s + spaces_p + 1, codepoint_s + spaces_p + 2)
    // //       end

    // //       if codepoints.end_with?(" ") || codepoints.end_with?("\t")
    // //         diagnostic :fatal, :invalid_unicode_escape, nil, range(p - 1, p)
    // //       end
    // //     end

    // //     codepoints.scan(/([0-9a-fA-F]+)|([ \t]+)/).each do |(codepoint_str, spaces)|
    // //       if spaces
    // //         codepoint_s += spaces.length
    // //       else
    // //         codepoint = codepoint_str.to_i(16)

    // //         if codepoint >= 0x110000
    // //           diagnostic :error, :unicode_point_too_large, nil,
    // //                      range(codepoint_s, codepoint_s + codepoint_str.length)
    // //           break
    // //         end

    // //         @escape     += codepoint.chr(Encoding::UTF_8)
    // //         codepoint_s += codepoint_str.length
    // //       end
    // //     end
    // //   }

    // //   action unescape_char {
    // //     codepoint = @source_pts[p - 1]
    // //     if (@escape = ESCAPES[codepoint]).nil?
    // //       @escape = encode_escape(@source_buffer.slice(p - 1))
    // //     end
    // //   }

    // //   action invalid_complex_escape {
    // //     diagnostic :fatal, :invalid_escape
    // //   }

    // //   action slash_c_char {
    // //     @escape = encode_escape(@escape[0].ord & 0x9f)
    // //   }

    // //   action slash_m_char {
    // //     @escape = encode_escape(@escape[0].ord | 0x80)
    // //   }

    // //   maybe_escaped_char = (
    // //         '\\' c_any      %unescape_char
    // //     | ( c_any - [\\] )  % { @escape = @source_buffer.slice(p - 1).chr }
    // //   );

    // //   maybe_escaped_ctrl_char = ( # why?!
    // //         '\\' c_any      %unescape_char %slash_c_char
    // //     |   '?'             % { @escape = "\x7f" }
    // //     | ( c_any - [\\?] ) % { @escape = @source_buffer.slice(p - 1).chr } %slash_c_char
    // //   );

    // //   escape = (
    // //       # \377
    // //       [0-7]{1,3}
    // //       % { @escape = encode_escape(tok(@escape_s, p).to_i(8) % 0x100) }

    // //       # \xff
    // //     | 'x' xdigit{1,2}
    // //         % { @escape = encode_escape(tok(@escape_s + 1, p).to_i(16)) }

    // //       # %q[\x]
    // //     | 'x' ( c_any - xdigit )
    // //       % {
    // //         diagnostic :fatal, :invalid_hex_escape, nil, range(@escape_s - 1, p + 2)
    // //       }

    // //       # \u263a
    // //     | 'u' xdigit{4}
    // //       % { @escape = tok(@escape_s + 1, p).to_i(16).chr(Encoding::UTF_8) }

    // //       # \u123
    // //     | 'u' xdigit{0,3}
    // //       % {
    // //         diagnostic :fatal, :invalid_unicode_escape, nil, range(@escape_s - 1, p)
    // //       }

    // //       # u{not hex} or u{}
    // //     | 'u{' ( c_any - xdigit - [ \t}] )* '}'
    // //       % {
    // //         diagnostic :fatal, :invalid_unicode_escape, nil, range(@escape_s - 1, p)
    // //       }

    // //       # \u{  \t  123  \t 456   \t\t }
    // //     | 'u{' [ \t]* ( xdigit{1,6} [ \t]+ )*
    // //       (
    // //         ( xdigit{1,6} [ \t]* '}'
    // //           %unicode_points
    // //         )
    // //         |
    // //         ( xdigit* ( c_any - xdigit - [ \t}] )+ '}'
    // //           | ( c_any - [ \t}] )* c_eof
    // //           | xdigit{7,}
    // //         ) % {
    // //           diagnostic :fatal, :unterminated_unicode, nil, range(p - 1, p)
    // //         }
    // //       )

    // //       # \C-\a \cx
    // //     | ( 'C-' | 'c' ) escaped_nl?
    // //       maybe_escaped_ctrl_char

    // //       # \M-a
    // //     | 'M-' escaped_nl?
    // //       maybe_escaped_char
    // //       %slash_m_char

    // //       # \C-\M-f \M-\cf \c\M-f
    // //     | ( ( 'C-'   | 'c' ) escaped_nl?   '\\M-'
    // //       |   'M-\\'         escaped_nl? ( 'C-'   | 'c' ) ) escaped_nl?
    // //       maybe_escaped_ctrl_char
    // //       %slash_m_char

    // //     | 'C' c_any %invalid_complex_escape
    // //     | 'M' c_any %invalid_complex_escape
    // //     | ( 'M-\\C' | 'C-\\M' ) c_any %invalid_complex_escape

    // //     | ( c_any - [0-7xuCMc] ) %unescape_char

    // //     | c_eof % {
    // //       diagnostic :fatal, :escape_eof, nil, range(p - 1, p)
    // //     }
    // //   );

    //   # Use rules in form of `e_bs escape' when you need to parse a sequence.
    //   e_bs = '\\' % {
    //     @escape_s = p
    //     @escape   = nil
    //   };
    action!("e_bs", |lexer: &mut Lexer| {
        panic!("UNIMPL");
    });

    // // #
    // // # === STRING AND HEREDOC PARSING ===
    // // #

    // // # Heredoc parsing is quite a complex topic. First, consider that heredocs
    // // # can be arbitrarily nested. For example:
    // // #
    // // #     puts <<CODE
    // // #     the result is: #{<<RESULT.inspect
    // // #       i am a heredoc
    // // #     RESULT
    // // #     }
    // // #     CODE
    // // #
    // // # which, incidentally, evaluates to:
    // // #
    // // #     the result is: "  i am a heredoc\n"
    // // #
    // // # To parse them, lexer refers to two kinds (remember, nested heredocs)
    // // # of positions in the input stream, namely heredoc_e
    // // # (HEREDOC declaration End) and @herebody_s (HEREdoc BODY line Start).
    // // #
    // // # heredoc_e is simply contained inside the corresponding Literal, and
    // // # when the heredoc is closed, the lexing is restarted from that position.
    // // #
    // // # @herebody_s is quite more complex. First, @herebody_s changes after each
    // // # heredoc line is lexed. This way, at '\n' tok(@herebody_s, @te) always
    // // # contains the current line, and also when a heredoc is started, @herebody_s
    // // # contains the position from which the heredoc will be lexed.
    // // #
    // // # Second, as (insanity) there are nested heredocs, we need to maintain a
    // // # stack of these positions. Each time #push_literal is called, it saves current
    // // # @heredoc_s to literal.saved_herebody_s, and after an interpolation (possibly
    // // # containing another heredocs) is closed, the previous value is restored.

    // // e_heredoc_nl = c_nl % {
    // // # After every heredoc was parsed, @herebody_s contains the
    // // # position of next token after all heredocs.
    // // if @herebody_s
    // //     p = @herebody_s
    // //     @herebody_s = nil
    // // end
    // // };
    // // TODO INCOMPLETE
    // // e_heredoc_nl embedded proc
    // pattern!("e_heredoc_nl", r"\n");

    //   action extend_string {
    //     string = tok
    // 
    //     # tLABEL_END is only possible in non-cond context on >= 2.2
    //     if @version >= 22 && !@cond.active?
    //       lookahead = @source_buffer.slice(@te...@te+2)
    //     end
    // 
    //     current_literal = literal
    //     if !current_literal.heredoc? &&
    //           (token = current_literal.nest_and_try_closing(string, @ts, @te, lookahead))
    //       if token[0] == :tLABEL_END
    //         p += 1
    //         pop_literal
    //         fnext expr_labelarg;
    //       else
    //         fnext *pop_literal;
    //       end
    //       fbreak;
    //     else
    //       current_literal.extend_string(string, @ts, @te)
    //     end
    //   }
    action!("extend_string", |lexer: &mut Lexer| {
        panic!("UNIMPL");
    });

    //   action extend_string_escaped {
    //     current_literal = literal
    //     # Get the first character after the backslash.
    //     escaped_char = @source_buffer.slice(@escape_s).chr
    // 
    //     if current_literal.munge_escape? escaped_char
    //       # If this particular literal uses this character as an opening
    //       # or closing delimiter, it is an escape sequence for that
    //       # particular character. Write it without the backslash.
    // 
    //       if current_literal.regexp? && REGEXP_META_CHARACTERS.match(escaped_char)
    //         # Regular expressions should include escaped delimiters in their
    //         # escaped form, except when the escaped character is
    //         # a closing delimiter but not a regexp metacharacter.
    //         #
    //         # The backslash itself cannot be used as a closing delimiter
    //         # at the same time as an escape symbol, but it is always munged,
    //         # so this branch also executes for the non-closing-delimiter case
    //         # for the backslash.
    //         current_literal.extend_string(tok, @ts, @te)
    //       else
    //         current_literal.extend_string(escaped_char, @ts, @te)
    //       end
    //     else
    //       # It does not. So this is an actual escape sequence, yay!
    //       if current_literal.regexp?
    //         # Regular expressions should include escape sequences in their
    //         # escaped form. On the other hand, escaped newlines are removed.
    //         current_literal.extend_string(tok.gsub("\\\n".freeze, ''.freeze), @ts, @te)
    //       else
    //         current_literal.extend_string(@escape || tok, @ts, @te)
    //       end
    //     end
    //   }
    action!("extend_string_escaped", |lexer: &mut Lexer| {
        panic!("UNIMPL");
    });

    //   # Extend a string with a newline or a EOF character.
    //   # As heredoc closing line can immediately precede EOF, this action
    //   # has to handle such case specially.
    //   action extend_string_eol {
    //     current_literal = literal
    //     if @te == pe
    //       diagnostic :fatal, :string_eof, nil,
    //                  range(current_literal.str_s, current_literal.str_s + 1)
    //     end
    // 
    //     if current_literal.heredoc?
    //       line = tok(@herebody_s, @ts).gsub(/\r+$/, ''.freeze)
    // 
    //       if version?(18, 19, 20)
    //         # See ruby:c48b4209c
    //         line = line.gsub(/\r.*$/, ''.freeze)
    //       end
    // 
    //       # Try ending the heredoc with the complete most recently
    //       # scanned line. @herebody_s always refers to the start of such line.
    //       if current_literal.nest_and_try_closing(line, @herebody_s, @ts)
    //         # Adjust @herebody_s to point to the next line.
    //         @herebody_s = @te
    // 
    //         # Continue regular lexing after the heredoc reference (<<END).
    //         p = current_literal.heredoc_e - 1
    //         fnext *pop_literal; fbreak;
    //       else
    //         # Calculate indentation level for <<~HEREDOCs.
    //         current_literal.infer_indent_level(line)
    // 
    //         # Ditto.
    //         @herebody_s = @te
    //       end
    //     else
    //       # Try ending the literal with a newline.
    //       if current_literal.nest_and_try_closing(tok, @ts, @te)
    //         fnext *pop_literal; fbreak;
    //       end
    // 
    //       if @herebody_s
    //         # This is a regular literal intertwined with a heredoc. Like:
    //         #
    //         #     p <<-foo+"1
    //         #     bar
    //         #     foo
    //         #     2"
    //         #
    //         # which, incidentally, evaluates to "bar\n1\n2".
    //         p = @herebody_s - 1
    //         @herebody_s = nil
    //       end
    //     end
    // 
    //     if current_literal.words? && !eof_codepoint?(@source_pts[p])
    //       current_literal.extend_space @ts, @te
    //     else
    //       # A literal newline is appended if the heredoc was _not_ closed
    //       # this time (see fbreak above). See also Literal#nest_and_try_closing
    //       # for rationale of calling #flush_string here.
    //       current_literal.extend_string tok, @ts, @te
    //       current_literal.flush_string
    //     end
    //   }
    action!("extend_string_eol", |lexer: &mut Lexer| {
        panic!("UNIMPL");
    });

    //   action extend_string_space {
    //     literal.extend_space @ts, @te
    //   }
    action!("extend_string_space", |lexer: &mut Lexer| {
        panic!("UNIMPL");
    });


    //   #
    //   # === INTERPOLATION PARSING ===
    //   #

    //   action extend_interp_var {
    //     current_literal = literal
    //     current_literal.flush_string
    //     current_literal.extend_content
    // 
    //     emit(:tSTRING_DVAR, nil, @ts, @ts + 1)
    // 
    //     p = @ts
    //     fcall expr_variable;
    //   }
    action!("extend_interp_var", |lexer: &mut Lexer| {
        panic!("UNIMPL");
    });

    //   action extend_interp_code {
    //     current_literal = literal
    //     current_literal.flush_string
    //     current_literal.extend_content
    // 
    //     emit(:tSTRING_DBEG, '#{'.freeze)
    // 
    //     if current_literal.heredoc?
    //       current_literal.saved_herebody_s = @herebody_s
    //       @herebody_s = nil
    //     end
    // 
    //     current_literal.start_interp_brace
    //     fcall expr_value;
    //   }
    action!("extend_interp_code", |lexer: &mut Lexer| {
        panic!("UNIMPL");
    });


    //   # These rules implement a form of manually defined lookahead.
    //   # The default longest-match scanning does not work here due
    //   # to sheer ambiguity.

    //   ambiguous_fid_suffix =         # actual    parsed
    //       [?!]    %{ tm = p }      | # a?        a?
    //       [?!]'=' %{ tm = p - 2 }    # a!=b      a != b
    //   ;

    //   ambiguous_ident_suffix =       # actual    parsed
    //       ambiguous_fid_suffix     |
    //       '='     %{ tm = p }      | # a=        a=
    //       '=='    %{ tm = p - 2 }  | # a==b      a == b
    //       '=~'    %{ tm = p - 2 }  | # a=~b      a =~ b
    //       '=>'    %{ tm = p - 2 }  | # a=>b      a => b
    //       '==='   %{ tm = p - 3 }    # a===b     a === b
    //   ;

    //   ambiguous_symbol_suffix =      # actual    parsed
    //       ambiguous_ident_suffix |
    //       '==>'   %{ tm = p - 2 }    # :a==>b    :a= => b
    //   ;

    //   # Ambiguous with 1.9 hash labels.
    //   ambiguous_const_suffix =       # actual    parsed
    //       '::'    %{ tm = p - 2 }    # A::B      A :: B
    //   ;

    // NOTE shared action for `ambiguous_fid_suffix` `ambiguous_ident_suffix` `ambiguous_symbol_suffix` `ambiguous_const_suffix`
    action!("ambiguous_suffix", |lexer: &mut Lexer| {
        let current_slice = lexer.input_stream.current_token().unwrap();

        if let Some(capture) = Regex::new(r"^===").unwrap().captures(&current_slice) { lexer.input_stream.tm = lexer.input_stream.p - 3; return; }
        if let Some(capture) = Regex::new(r"^==>").unwrap().captures(&current_slice) { lexer.input_stream.tm = lexer.input_stream.p - 2; return; }
        if let Some(capture) = Regex::new(r"^[?!]=").unwrap().captures(&current_slice) { lexer.input_stream.tm = lexer.input_stream.p - 2; return; }
        if let Some(capture) = Regex::new(r"^==").unwrap().captures(&current_slice) { lexer.input_stream.tm = lexer.input_stream.p - 2; return; }
        if let Some(capture) = Regex::new(r"^=~").unwrap().captures(&current_slice) { lexer.input_stream.tm = lexer.input_stream.p - 2; return; }
        if let Some(capture) = Regex::new(r"^=>").unwrap().captures(&current_slice) { lexer.input_stream.tm = lexer.input_stream.p - 2; return; }
        if let Some(capture) = Regex::new(r"^::").unwrap().captures(&current_slice) { lexer.input_stream.tm = lexer.input_stream.p - 2; return; }
        if let Some(capture) = Regex::new(r"^[?!]").unwrap().captures(&current_slice) { lexer.input_stream.tm = lexer.input_stream.p; return; }
        if let Some(capture) = Regex::new(r"^=").unwrap().captures(&current_slice) { lexer.input_stream.tm = lexer.input_stream.p; return; }
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
