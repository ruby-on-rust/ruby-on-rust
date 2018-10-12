%%{
# Literal function name in method call (e.g. `a.class`).
#
# Transitions to `expr_arg` afterwards.
#
expr_dot := |*
    constant
    => { emit(:tCONSTANT)
          fnext *arg_or_cmdarg; fbreak; };

    call_or_var
    => { emit(:tIDENTIFIER)
          fnext *arg_or_cmdarg; fbreak; };

    bareword ambiguous_fid_suffix
    => { emit(:tFID, tok(@ts, tm), @ts, tm)
          fnext *arg_or_cmdarg; p = tm - 1; fbreak; };

    # See the comment in `expr_fname`.
    operator_fname      |
    operator_arithmetic |
    operator_rest
    => { emit_table(PUNCTUATION)
          fnext expr_arg; fbreak; };

    w_any;

    c_any
    => { fhold; fgoto expr_end; };

    c_eof => do_eof;
*|;
}%%
