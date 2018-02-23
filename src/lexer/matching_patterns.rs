use std::collections::HashMap;

use regex::Regex;

// TODO should these be 'static ?
type TMatchingPatternLiterals = HashMap<&'static str, &'static str>;
type TMatchingPatternRegexs = HashMap<&'static str, Regex>;
pub type TMatchingPatterns = ( TMatchingPatternLiterals, TMatchingPatternRegexs );

pub fn construct() -> TMatchingPatterns {
    let mut pattern_literals: TMatchingPatternLiterals = HashMap::new();
    let mut patterns: TMatchingPatternRegexs = HashMap::new();

    macro_rules! pattern {
        ($name:expr, $regex:expr) => {
            pattern_literals.insert($name, $regex);
            patterns.insert($name, Regex::new( &format!(r"^{}", $regex) ).unwrap());
        };
    }

    // 
    // NATIVE
    // 

    patterns.insert("any", Regex::new(r"(?s)^.").unwrap()); // TODO NOT SURE
    patterns.insert("zlen", Regex::new(r"^$").unwrap()); // TODO REALLY?

    // 
    // CHARACTER CLASSES
    // 

    //   c_nl       = '\n' $ do_nl;
    pattern!("c_nl", r"\n"); // WITH EMBEDDED ACTION
    //   c_space    = [ \t\r\f\v];
    pattern!("c_space", r"[ \t\r\f\v]");
    //   c_space_nl = c_space | c_nl;
    pattern!("c_space_nl", r"[ \n\t\r\f\v]"); // TODO NOT CORRESPONDING

    //   c_eof      = 0x04 | 0x1a | 0 | zlen; # ^D, ^Z, \0, EOF
    pattern!("c_eof", r"\z"); // TODO NOT CORRESPONDING
    //   c_eol      = c_nl | c_eof;
    pattern!("c_eol", r"(\n|\z)"); // TODO NOT CORRESPONDING
    //   c_any      = any - c_eof;
    pattern!("c_any", r"^."); // TODO NOT CORRESPONDING

    //   c_nl_zlen  = c_nl | zlen;
    pattern!("c_nl_zlen", r"\n"); // TODO NOT CORRESPONDING

    //   c_line     = any - c_nl_zlen;
    pattern!("c_line", r"[^\n]"); // TODO NOT CORRESPONDING

    // TODO
    //   c_unicode  = c_any - 0x00..0x7f;
    //   c_upper    = [A-Z];
    //   c_lower    = [a-z_]  | c_unicode;
    //   c_alpha    = c_lower | c_upper;
    //   c_alnum    = c_alpha | [0-9];

    // 
    // TOKEN DEFINITIONS
    // 

    // # All operators are punctuation. There is more to punctuation
    // # than just operators. Operators can be overridden by user;
    // # punctuation can not.

    // # A list of operators which are valid in the function name context, but
    // # have different semantics in others.
    // operator_fname      = '[]' | '[]=' | '`'  | '-@' | '+@' | '~@'  | '!@' ;
    pattern!("operator_fname", r"(\[\])|(\[\]=)|`|(-@)|(\+@)|(~@)|(!@)");

    // # A list of operators which can occur within an assignment shortcut (+ → +=).
    // operator_arithmetic = '&'  | '|'   | '&&' | '||' | '^'  | '+'   | '-'  |
    //                       '*'  | '/'   | '**' | '~'  | '<<' | '>>'  | '%'  ;
    pattern!("operator_arithmetic", r"(&)|(\|)|(&&)|(\|\|)|(\^)|(\+)|(-)|(\*)|(/)|(\*\*)|(~)|(<<)|(>>)|(%)");

    // # A list of all user-definable operators not covered by groups above.
    // operator_rest       = '=~' | '!~' | '==' | '!=' | '!'   | '===' |
    //                       '<'  | '<=' | '>'  | '>=' | '<=>' | '=>'  ;
    pattern!("operator_rest", "(=~)|(!~)|(==)|(!=)|(!)|(===)|(<)|(<=)|(>)|(>=)|(<=>)|(=>)");

    //   # Note that `{` and `}` need to be referred to as e_lbrace and e_rbrace,
    //   # as they are ambiguous with interpolation `#{}` and should be counted.
    //   # These braces are not present in punctuation lists.

    //   # A list of punctuation which has different meaning when used at the
    //   # beginning of expression.
    //   punctuation_begin   = '-'  | '+'  | '::' | '('  | '['  |
    //                         '*'  | '**' | '&'  ;
    pattern!("punctuation_begin", r"(-)|(\+)|(::)|(\()|(\[)|(\*)|(\*\*)|(&)");

    //   # A list of all punctuation except punctuation_begin.
    //   punctuation_end     = ','  | '='  | '->' | '('  | '['  | ']'   |
    //                         '::' | '?'  | ':'  | '.'  | '..' | '...' ;
    pattern!("punctuation_end", r"(,)|(=)|(->)|(\()|(\[)|(\])|(::)|(\?)|(:)|(\.)|(\.\.)|(\.\.\.)");

    // # A list of keywords which have different meaning at the beginning of expression.
    // keyword_modifier    = 'if'     | 'unless' | 'while'  | 'until' | 'rescue' ;
    pattern!("keyword_modifier", "(if)|(unless)|(while)|(until)|(rescue)");

    // # A list of keywords which accept an argument-like expression, i.e. have the
    // # same post-processing as method calls or commands. Example: `yield 1`,
    // # `yield (1)`, `yield(1)`, are interpreted as if `yield` was a function.
    // keyword_with_arg    = 'yield'  | 'super'  | 'not'    | 'defined?' ;
    pattern!("keyword_with_arg", "(yield)|(super)|(not)|(defined?)");

    // # A list of keywords which accept a literal function name as an argument.
    // keyword_with_fname  = 'def'    | 'undef'  | 'alias'  ;
    pattern!("keyword_with_fname", "(def)|(undef)|(alias)");

    // # A list of keywords which accept an expression after them.
    // keyword_with_value  = 'else'   | 'case'   | 'ensure' | 'module' | 'elsif' | 'then'  |
    //                       'for'    | 'in'     | 'do'     | 'when'   | 'begin' | 'class' |
    //                       'and'    | 'or'     ;
    pattern!("keyword_with_value", "(else)|(case)|(ensure)|(module)|(elsif)|(then)|(for)|(in)|(do)|(when)|(begin)|(class)|(and)|(or)");

    // # A list of keywords which accept a value, and treat the keywords from
    // # `keyword_modifier` list as modifiers.
    // keyword_with_mid    = 'rescue' | 'return' | 'break'  | 'next'   ;
    pattern!("keyword_with_mid", "(rescue)|(return)|(break)|(next)");

    // # A list of keywords which do not accept an expression after them.
    // keyword_with_end    = 'end'    | 'self'   | 'true'   | 'false'  | 'retry'    |
    //                       'redo'   | 'nil'    | 'BEGIN'  | 'END'    | '__FILE__' |
    //                       '__LINE__' | '__ENCODING__';
    pattern!("keyword_with_end", "(end)|(self)|(true)|(false)|(retry)|(redo)|(nil)|(BEGIN)|(END)|(__FILE__)|(__LINE__)|(__ENCODING__)");

    // # All keywords.
    // keyword             = keyword_with_value | keyword_with_mid |
    //                       keyword_with_end   | keyword_with_arg |
    //                       keyword_with_fname | keyword_modifier ;
    pattern!("keyword", r"(\[\])|(\[\]=)|`|(-@)|(\+@)|(~@)|(!@)|(&)|(\|)|(&&)|(\|\|)|(\^)|(\+)|(-)|(\*)|(/)|(\*\*)|(~)|(<<)|(>>)|(%)|(if)|(unless)|(while)|(until)|(rescue)|(end)|(self)|(true)|(false)|(retry)|(redo)|(nil)|(BEGIN)|(END)|(__FILE__)|(__LINE__)|(__ENCODING__)|(if)|(unless)|(while)|(until)|(rescue)|(end)|(self)|(true)|(false)|(retry)|(redo)|(nil)|(BEGIN)|(END)|(__FILE__)|(__LINE__)|(__ENCODING__)");

    //   constant       = c_upper c_alnum*;
    pattern!("constant", "[[:upper:]][[:alnum:]]*");
    //   bareword       = c_alpha c_alnum*;
    pattern!("bareword", "[[:alpha:]][[:alnum:]]*");

    //   call_or_var    = c_lower c_alnum*;
    pattern!("call_or_var", "[[:lower:]][[:alnum:]]*");
    //   class_var      = '@@' bareword;
    pattern!("class_var", "@@[[:alpha:]][[:alnum:]]*");
    //   instance_var   = '@' bareword;
    pattern!("instance_var", "@[[:alpha:]][[:alnum:]]*");
    //   global_var     = '$'
    //       ( bareword | digit+
    //       | [`'+~*$&?!@/\\;,.=:<>"] # `
    //       | '-' c_alnum
    //       )
    //   ;
    // TODO use indoc!
    pattern!("global_var", "\\$(
([[:alpha:]][[:alnum:]]*)|
([[:digit:]]+)|
([`'\\+~\\*\\$&?!@/;,\\.=:<>\"])|
(-[[:alnum:]])
)");

    //   # Ruby accepts (and fails on) variables with leading digit
    //   # in literal context, but not in unquoted symbol body.
    //   class_var_v    = '@@' c_alnum+;
    pattern!("class_var_v", "@@[:alnum:]+");
    //   instance_var_v = '@' c_alnum+;
    pattern!("instance_var_v", "@[:alnum:]+");

    //   label          = bareword [?!]? ':';
    pattern!("label", r"[[:alpha:]][[:alnum:]]*[\?!]?:");

    // 
    // NUMERIC PARSING
    // 

    // TODO INCOMPLETE
    pattern!("int_dec", "[1-9][[:digit:]]*_?([[:digit:]]_)*[[:digit:]]*_?");


    // #
    // # === STRING AND HEREDOC PARSING ===
    // #

    // # Heredoc parsing is quite a complex topic. First, consider that heredocs
    // # can be arbitrarily nested. For example:
    // #
    // #     puts <<CODE
    // #     the result is: #{<<RESULT.inspect
    // #       i am a heredoc
    // #     RESULT
    // #     }
    // #     CODE
    // #
    // # which, incidentally, evaluates to:
    // #
    // #     the result is: "  i am a heredoc\n"
    // #
    // # To parse them, lexer refers to two kinds (remember, nested heredocs)
    // # of positions in the input stream, namely heredoc_e
    // # (HEREDOC declaration End) and @herebody_s (HEREdoc BODY line Start).
    // #
    // # heredoc_e is simply contained inside the corresponding Literal, and
    // # when the heredoc is closed, the lexing is restarted from that position.
    // #
    // # @herebody_s is quite more complex. First, @herebody_s changes after each
    // # heredoc line is lexed. This way, at '\n' tok(@herebody_s, @te) always
    // # contains the current line, and also when a heredoc is started, @herebody_s
    // # contains the position from which the heredoc will be lexed.
    // #
    // # Second, as (insanity) there are nested heredocs, we need to maintain a
    // # stack of these positions. Each time #push_literal is called, it saves current
    // # @heredoc_s to literal.saved_herebody_s, and after an interpolation (possibly
    // # containing another heredocs) is closed, the previous value is restored.

    // e_heredoc_nl = c_nl % {
    // # After every heredoc was parsed, @herebody_s contains the
    // # position of next token after all heredocs.
    // if @herebody_s
    //     p = @herebody_s
    //     @herebody_s = nil
    // end
    // };
    // TODO INCOMPLETE
    // c_heredoc_nl embedded proc
    pattern!("e_heredoc_nl", r"\n");


    //   #
    //   # === INTERPOLATION PARSING ===
    //   #

    //   # Interpolations with immediate variable names simply call into
    //   # the corresponding machine.

    //   interp_var = '#' ( global_var | class_var_v | instance_var_v );

    //   action extend_interp_var {
    //     current_literal = literal
    //     current_literal.flush_string
    //     current_literal.extend_content

    //     emit(:tSTRING_DVAR, nil, @ts, @ts + 1)

    //     p = @ts
    //     fcall expr_variable;
    //   }

    //   # Interpolations with code blocks must match nested curly braces, as
    //   # interpolation ending is ambiguous with a block ending. So, every
    //   # opening and closing brace should be matched with e_[lr]brace rules,
    //   # which automatically perform the counting.
    //   #
    //   # Note that interpolations can themselves be nested, so brace balance
    //   # is tied to the innermost literal.
    //   #
    //   # Also note that literals themselves should not use e_[lr]brace rules
    //   # when matching their opening and closing delimiters, as the amount of
    //   # braces inside the characters of a string literal is independent.

    //   interp_code = '#{';

    //   e_lbrace = '{' % {
    //     @cond.push(false); @cmdarg.push(false)

    //     current_literal = literal
    //     if current_literal
    //       current_literal.start_interp_brace
    //     end
    //   };
    // NOTE moved to shared_actions
    pattern!("e_lbrace", r"\{");

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

    //   action extend_interp_code {
    //     current_literal = literal
    //     current_literal.flush_string
    //     current_literal.extend_content

    //     emit(:tSTRING_DBEG, '#{'.freeze)

    //     if current_literal.heredoc?
    //       current_literal.saved_herebody_s = @herebody_s
    //       @herebody_s = nil
    //     end

    //     current_literal.start_interp_brace
    //     fcall expr_value;
    //   }

    //   # Actual string parsers are simply combined from the primitives defined
    //   # above.

    //   interp_words := |*
    //       interp_code => extend_interp_code;
    //       interp_var  => extend_interp_var;
    //       e_bs escape => extend_string_escaped;
    //       c_space+    => extend_string_space;
    //       c_eol       => extend_string_eol;
    //       c_any       => extend_string;
    //   *|;

    //   interp_string := |*
    //       interp_code => extend_interp_code;
    //       interp_var  => extend_interp_var;
    //       e_bs escape => extend_string_escaped;
    //       c_eol       => extend_string_eol;
    //       c_any       => extend_string;
    //   *|;

    //   plain_words := |*
    //       e_bs c_any  => extend_string_escaped;
    //       c_space+    => extend_string_space;
    //       c_eol       => extend_string_eol;
    //       c_any       => extend_string;
    //   *|;

    //   plain_string := |*
    //       '\\' c_nl   => extend_string_eol;
    //       e_bs c_any  => extend_string_escaped;
    //       c_eol       => extend_string_eol;
    //       c_any       => extend_string;
    //   *|;

    //   interp_backslash_delimited := |*
    //       interp_code => extend_interp_code;
    //       interp_var  => extend_interp_var;
    //       c_eol       => extend_string_eol;
    //       c_any       => extend_string;
    //   *|;

    //   plain_backslash_delimited := |*
    //       c_eol       => extend_string_eol;
    //       c_any       => extend_string;
    //   *|;

    //   interp_backslash_delimited_words := |*
    //       interp_code => extend_interp_code;
    //       interp_var  => extend_interp_var;
    //       c_space+    => extend_string_space;
    //       c_eol       => extend_string_eol;
    //       c_any       => extend_string;
    //   *|;

    //   plain_backslash_delimited_words := |*
    //       c_space+    => extend_string_space;
    //       c_eol       => extend_string_eol;
    //       c_any       => extend_string;
    //   *|;

    //   regexp_modifiers := |*
    //       [A-Za-z]+
    //       => {
    //         unknown_options = tok.scan(/[^imxouesn]/)
    //         if unknown_options.any?
    //           diagnostic :error, :regexp_options,
    //                      { :options => unknown_options.join }
    //         end

    //         emit(:tREGEXP_OPT)

    //         if @version < 24
    //           fnext expr_end;
    //         else
    //           fnext expr_endarg;
    //         end

    //         fbreak;
    //       };

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

    // #
    // # === WHITESPACE HANDLING ===
    // #

    // # Various contexts in Ruby allow various kinds of whitespace
    // # to be used. They are grouped to clarify the lexing machines
    // # and ease collection of comments.

    // # A line of code with inline #comment at end is always equivalent
    // # to a line of code ending with just a newline, so an inline
    // # comment is deemed equivalent to non-newline whitespace
    // # (c_space character class).

    // w_space =
    //     c_space+
    //     | '\\' e_heredoc_nl
    //     ;
    pattern!("w_space", r"([ \t\r\f\v]+)"); // TODO INCOMPLETE

    // w_comment =
    //     '#'     %{ @sharp_s = p - 1 }
    //     # The (p == pe) condition compensates for added "\0" and
    //     # the way Ragel handles EOF.
    //     c_line* %{ emit_comment(@sharp_s, p == pe ? p - 2 : p) }
    //     ;

    // w_space_comment =
    //     w_space
    //     | w_comment
    //     ;

    pattern!("w_space_comment", r"[ \t\r\f\v]+"); // TODO IMCOMPLETE

    // # A newline in non-literal context always interoperates with
    // # here document logic and can always be escaped by a backslash,
    // # still interoperating with here document logic in the same way,
    // # yet being invisible to anything else.
    // #
    // # To demonstrate:
    // #
    // #     foo = <<FOO \
    // #     bar
    // #     FOO
    // #      + 2
    // #
    // # is equivalent to `foo = "bar\n" + 2`.

    // w_newline =
    //     e_heredoc_nl;
    pattern!("w_newline", r"\n"); // TODO NOT CORRESPONDING

    // w_any =
    //     w_space
    //     | w_comment
    //     | w_newline
    //     ;
    pattern!("w_any", r"[ \t\r\f\v]+"); // TODO INCOMPLETE

    //   #
    //   # === EXPRESSION PARSING ===
    //   #

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


    // # Resolving kDO/kDO_COND/kDO_BLOCK ambiguity requires embedding
    // # @cond/@cmdarg-related code to e_lbrack, e_lparen and e_lbrace.

    // e_lbrack = '[' % {
    //     @cond.push(false); @cmdarg.push(false)
    // };

    // # Ruby 1.9 lambdas require parentheses counting in order to
    // # emit correct opening kDO/tLBRACE.

    // e_lparen = '(' % {
    //     @cond.push(false); @cmdarg.push(false)

    //     @paren_nest += 1
    // };
    pattern!("e_lparen", r"\(");

    // e_rparen = ')' % {
    //     @paren_nest -= 1
    // };

    // # Ruby is context-sensitive wrt/ local identifiers.
    // action local_ident {
    //     emit(:tIDENTIFIER)

    //     if !@static_env.nil? && @static_env.declared?(tok)
    //     fnext expr_endfn; fbreak;
    //     else
    //     fnext *arg_or_cmdarg; fbreak;
    //     end
    // }

    (pattern_literals, patterns)
}
