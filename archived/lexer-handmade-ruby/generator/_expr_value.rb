# # Like expr_beg, but no 1.9 label or 2.2 quoted label possible.
# #
# expr_value := |*
s = Scanner.new :expr_value

#     # a:b: a(:b), a::B, A::B
#     label (any - ':')
#     => { p = @ts - 1
#           fgoto expr_end; };
# s.p p!( :label, :-, p!( :any, :-, ':' ) ), %q{
#     println!("DEBUGGING");
# }

#     # "bar", 'baz'
#     ['"] # '
#     => {
#       fgoto *push_literal(tok, tok, @ts);
#     };
s.p p!(["'", '"']), %q{
    let literal = Literal::new(some_matched_slice.clone(), some_matched_slice.clone(), matched_slice_start_pos, None, false, false, false, Rc::clone(&self.tokens));

    fgoto *self.push_literal(literal);
}

#     w_space_comment;

#     w_newline
#     => { fgoto line_begin; };

#     c_any
#     => { fhold; fgoto expr_beg; };
s.p :c_any, %q{
    fhold;
    fgoto expr_beg;
}

#     c_eof => do_eof;
# *|;
