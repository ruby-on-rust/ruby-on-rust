%%{
# The rationale for this state is that several keywords accept value
# (i.e. should transition to `expr_beg`), do not accept it like a command
# (i.e. not an `expr_arg`), and must behave like a statement, that is,
# accept a modifier if/while/etc.
#
expr_mid := |*
    keyword_modifier
    => { emit_table(KEYWORDS)
          fnext expr_beg; fbreak; };

    bareword
    => { p = @ts - 1; fgoto expr_beg; };

    w_space_comment;

    w_newline
    => { fhold; fgoto expr_end; };

    c_any
    => { fhold; fgoto expr_beg; };

    c_eof => do_eof;
*|;
}%%
