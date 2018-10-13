# Like expr_beg, but no 1.9 label or 2.2 quoted label possible.

%%{
expr_value := |*
    # a:b: a(:b), a::B, A::B
    label (any - ':') => {
        p = ts - 1;
        fgoto expr_end;
    };

    # "bar", 'baz'
    ['"] # '
    => {
        let literal = Literal::new(self.input_slice(ts,te), self.input_slice(ts,te), ts, None, false, false, false, Rc::clone(&self.tokens));

        fgoto *self.push_literal(literal);
    };

    w_space_comment;

    w_newline => {
        fgoto line_begin;
    };

    c_any
    => {
        fhold; fgoto expr_beg;
    };

    c_eof => do_eof;
*|;

}%%
