use std::collections::HashMap;

use regex::Regex;

pub type TMatchingPatterns = HashMap<&'static str, Regex>;

pub fn construct() -> TMatchingPatterns {
    let mut patterns: TMatchingPatterns = HashMap::new();

    macro_rules! pattern {
        ($name:expr, $regex:expr) => {
            patterns.insert($name, Regex::new( &format!(r"^{}", $regex) ).unwrap());
        };
    }

    // TODO maybe impl a macro patterns!



    // 
    // NATIVE
    // 

    patterns.insert("any", Regex::new(r"(?s)^.").unwrap()); // TODO NOT SURE
    patterns.insert("zlen", Regex::new(r"^$").unwrap()); // TODO REALLY?

    // 
    // CHARACTER CLASSES
    // 

    //   c_nl       = '\n' $ do_nl;
    pattern!("c_nl", "\\n");     // TODO NOT CORRESPONDING
    //   c_space    = [ \t\r\f\v];
    pattern!("c_space", "[ \\t\\r\\f\\v]");
    //   c_space_nl = c_space | c_nl;
    pattern!("c_space_nl", "[ \\n\\t\\r\\f\\v]");

    //   c_eof      = 0x04 | 0x1a | 0 | zlen; # ^D, ^Z, \0, EOF
    pattern!("c_eof", "\\z"); // TODO NOT CORRESPONDING
    //   c_eol      = c_nl | c_eof;
    pattern!("c_eol", "\\n|\\z"); // TODO NOT CORRESPONDING
    //   c_any      = any - c_eof;
    patterns.insert("c_any", Regex::new(r"(?s)^.").unwrap()); // TODO NOT CORRESPONDING

    //   c_nl_zlen  = c_nl | zlen;
    pattern!("c_nl_zlen", "\\n"); // TODO NOT CORRESPONDING

    //   c_line     = any - c_nl_zlen;
    pattern!("c_line", "[^\\n]"); // TODO NOT CORRESPONDING

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
    pattern!("operator_fname", "(\\[\\])|(\\[\\]=)|`|(-@)|(\\+@)|(~@)|(!@)");

    // # A list of operators which can occur within an assignment shortcut (+ → +=).
    // operator_arithmetic = '&'  | '|'   | '&&' | '||' | '^'  | '+'   | '-'  |
    //                       '*'  | '/'   | '**' | '~'  | '<<' | '>>'  | '%'  ;
    pattern!("operator_arithmetic", "(&)|(\\|)|(&&)|(\\|\\|)|(\\^)|(\\+)|(-)|(\\*)|(/)|(\\*\\*)|(~)|(<<)|(>>)|(%)");

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
    pattern!("punctuation_begin", "(-)|(\\+)|(::)|(\\()|(\\[)|(\\*)|(\\*\\*)|(&)");

    //   # A list of all punctuation except punctuation_begin.
    //   punctuation_end     = ','  | '='  | '->' | '('  | '['  | ']'   |
    //                         '::' | '?'  | ':'  | '.'  | '..' | '...' ;
    pattern!("punctuation_end", "(,)|(=)|(->)|(\\()|(\\[)|(\\])|(::)|(\\?)|(:)|(\\.)|(\\.\\.)|(\\.\\..)");

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
    pattern!("keyword", "(\\[\\])|(\\[\\]=)|`|(-@)|(\\+@)|(~@)|(!@)|(&)|(\\|)|(&&)|(\\|\\|)|(\\^)|(\\+)|(-)|(\\*)|(/)|(\\*\\*)|(~)|(<<)|(>>)|(%)|(if)|(unless)|(while)|(until)|(rescue)|(end)|(self)|(true)|(false)|(retry)|(redo)|(nil)|(BEGIN)|(END)|(__FILE__)|(__LINE__)|(__ENCODING__)|(if)|(unless)|(while)|(until)|(rescue)|(end)|(self)|(true)|(false)|(retry)|(redo)|(nil)|(BEGIN)|(END)|(__FILE__)|(__LINE__)|(__ENCODING__)");

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
    //   instance_var_v = '@' c_alnum+;

    //   label          = bareword [?!]? ':';

    // 
    // NUMERIC PARSING
    // 

    // TODO ORIGINAL
    pattern!("int_dec", "[1-9][[:digit:]]*_?([[:digit:]]_)*[[:digit:]]*_?");


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
    pattern!("w_space", "([ \\t\\r\\f\\v]+)"); // TODO INCOMPLETE

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

    pattern!("w_space_comment", "[ \\t\\r\\f\\v]+"); // TODO IMCOMPLETE

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

    // w_any =
    //     w_space
    //     | w_comment
    //     | w_newline
    //     ;
    pattern!("w_any", "[ \\t\\r\\f\\v]+"); // TODO INCOMPLETE

    patterns
}
