%%{
# After literal function name in definition. Behaves like `expr_end`,
# but allows a tLABEL.
#
# Transitions to `expr_end` afterwards.
#
expr_endfn := |*
    label ( any - ':' )
    => {
        // emit(:tLABEL, tok(@ts, @te - 2), @ts, @te - 1)
        !emit T_LABEL, ts, te - 2;
        fhold; fnext expr_labelarg; fnbreak;
    };

    w_space_comment;

    c_any
    => {
        fhold; fgoto expr_end;
    };

    c_eof => do_eof;
*|;
}%%
