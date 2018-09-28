#   line_begin := |*
s = Scanner.new :line_begin

#       w_any;
s.p :w_any, nil

#       '=begin' ( c_space | c_nl_zlen )
#       => { @eq_begin_s = @ts
#            fgoto line_comment; };
s.p p!( '=begin', p!( %i(c_space c_nl_zlen) ) ), %q{
    //TODO
}

#       '__END__' ( c_eol - zlen )
#       => { p = pe - 3 };
s.p p!( '__END__', p!( :c_eol, :-, :zlen ) ), %q{
    //TODO
}

#       c_any
#       => { fhold; fgoto expr_value; };
s.p :c_any, %q{
    fhold;
    //fgoto expr_value
}

#       c_eof => do_eof;
s.p :c_eof, :do_eof

#   *|;
