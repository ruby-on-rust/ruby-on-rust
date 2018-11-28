%%{
# The rationale for this state is pretty complex. Normally, if an argument
# is passed to a command and then there is a block (tLCURLY...tRCURLY),
# the block is attached to the innermost argument (`f` in `m f {}`), or it
# is a parse error (`m 1 {}`). But there is a special case for passing a single
# primary expression grouped with parentheses: if you write `m (1) {}` or
# (2.0 only) `m () {}`, then the block is attached to `m`.
#
# Thus, we recognize the opening `(` of a command (remember, a command is
# a method call without parens) as a tLPAREN_ARG; then, in parser, we recognize
# `tLPAREN_ARG expr rparen` as a `primary_expr` and before rparen, set the
# lexer's state to `expr_endarg`, which makes it emit the possibly following
# `{` as `tLBRACE_ARG`.
#
# The default post-`expr_endarg` state is `expr_end`, so this state also handles
# `do` (as `kDO_BLOCK` in `expr_beg`).
expr_endarg := |*
    e_lbrace
    => {
        wip!();
        // if @lambda_stack.last == @paren_nest
        //   @lambda_stack.pop
        //   emit(:tLAMBEG, '{'.freeze)
        // else
        //   emit(:tLBRACE_ARG, '{'.freeze)
        // end
        // fnext expr_value; fbreak;
    };

    'do'
    => {
        wip!();
        // TODO
        // emit_do(true)
        // fnext expr_value; fbreak;
    };

    w_space_comment;

    c_any
    => { fhold; fgoto expr_end; };

    c_eof => do_eof;
*|;
}%%
