# TODO

%%{

line_begin := |*
    w_any;

    # '=begin' ( c_space | c_nl_zlen )
    # => {
    #     @eq_begin_s = @ts
    #     fgoto line_comment;
    # };

    # '__END__' ( c_eol - zlen )
    # => { p = pe - 3 };

    c_any
    => { fhold; fgoto expr_value; };

    # c_eof => do_eof;
*|;

}%%
