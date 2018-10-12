%%{
# Literal function name in definition (e.g. `def class`).
# Keywords are returned as their respective tokens; this is used
# to support singleton def `def self.foo`. Global variables are
# returned as `tGVAR`; this is used in global variable alias
# statements `alias $a $b`. Symbols are returned verbatim; this
# is used in `alias :a :"b#{foo}"` and `undef :a`.
#
# Transitions to `expr_endfn` afterwards.
#
expr_fname := |*
    keyword
    => { emit_table(KEYWORDS_BEGIN);
          fnext expr_endfn; fbreak; };

    constant
    => { emit(:tCONSTANT)
          fnext expr_endfn; fbreak; };

    bareword [?=!]?
    => { emit(:tIDENTIFIER)
          fnext expr_endfn; fbreak; };

    global_var
    => { p = @ts - 1
          fnext expr_end; fcall expr_variable; };

    # If the handling was to be delegated to expr_end,
    # these cases would transition to something else than
    # expr_endfn, which is incorrect.
    operator_fname      |
    operator_arithmetic |
    operator_rest
    => { emit_table(PUNCTUATION)
          fnext expr_endfn; fbreak; };

    '::'
    => { fhold; fhold; fgoto expr_end; };

    ':'
    => { fhold; fgoto expr_beg; };

    '%s' c_any
    => {
      if version?(23)
        type, delimiter = tok[0..-2], tok[-1].chr
        fgoto *push_literal(type, delimiter, @ts);
      else
        p = @ts - 1
        fgoto expr_end;
      end
    };

    w_any;

    c_any
    => { fhold; fgoto expr_end; };

    c_eof => do_eof;
*|;
}%%
