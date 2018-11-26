%%{
# Literal function name in method call (e.g. `a.class`).
#
# Transitions to `expr_arg` afterwards.
#
expr_dot := |*
    constant
    => {
        !emit T_CONSTANT;
        fnext *self.arg_or_cmdarg(); fnbreak;
    };

    call_or_var
    => {
        !emit T_IDENTIFIER;
        fnext *self.arg_or_cmdarg(); fnbreak;
    };

    bareword ambiguous_fid_suffix
    => {
        !emit T_FID, ts, tm;
        fnext *self.arg_or_cmdarg(); p = tm - 1; fnbreak;
    };

    # See the comment in `expr_fname`.
    operator_fname      |
    operator_arithmetic |
    operator_rest
    => {
        !emit_table PUNCTUATION;
        fnext expr_arg; fnbreak;
    };

    w_any;

    c_any
    => { fhold; fgoto expr_end; };

    c_eof => do_eof;
*|;
}%%
