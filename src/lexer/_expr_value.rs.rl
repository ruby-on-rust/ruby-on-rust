# Like expr_beg, but no 1.9 label or 2.2 quoted label possible.

# TODO

%%{

expr_value := |*
    # a:b: a(:b), a::B, A::B
    # label (any - ':')
    # => { p = @ts - 1
    #       fgoto expr_end; };

    # "bar", 'baz'
    # ['"] # '
    # => {
    #   fgoto *push_literal(tok, tok, @ts);
    # };

    # w_space_comment;

    # w_newline
    # => { fgoto line_begin; };

    c_any
    => {
        // TODO
        // fhold; fgoto expr_beg;
    };

    # c_eof => do_eof;
*|;

}%%
