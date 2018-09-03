%%{
leading_dot := |*
    # Insane leading dots:
    # a #comment
    #  .b: a.b
    c_space* %{ tm = p } ('.' | '&.')
    => { p = tm - 1; fgoto expr_end; };

    any
    => { emit(:tNL, nil, @newline_s, @newline_s + 1)
          fhold; fnext line_begin; fbreak; };
*|;
}%%
