%%{
# The previous token emitted was a `tIDENTIFIER` or `tFID`; no space
# is consumed; the current expression is a command or method call.
#
expr_arg := |*
    #
    # COMMAND MODE SPECIFIC TOKENS
    #

    # cmd (1 + 2)
    # See below the rationale about expr_endarg.
    w_space+ e_lparen
    => {
      // if version?(18)
      //   emit(:tLPAREN2, '('.freeze, @te - 1, @te)
      //   fnext expr_value; fbreak;
      // else
      //   emit(:tLPAREN_ARG, '('.freeze, @te - 1, @te)
      //   fnext expr_beg; fbreak;
      // end
      // NOTE ignored ruby18
      !emit T_LPAREN_ARG_;
      fnext expr_beg; fnbreak;
    };

    # meth(1 + 2)
    # Regular method call.
    e_lparen
    => {
        // emit(:tLPAREN2, '('.freeze)
        !emit T_LPAREN2_;
        fnext expr_beg; fnbreak;
    };

    # meth [...]
    # Array argument. Compare with indexing `meth[...]`.
    w_space+ e_lbrack
    => {
        // emit(:tLBRACK, '['.freeze, @te - 1, @te)
        !emit T_LBRACK_;
        fnext expr_beg; fnbreak;
    };

    # cmd {}
    # Command: method call without parentheses.
    w_space* e_lbrace
    => {
        // if @lambda_stack.last == @paren_nest
        //   @lambda_stack.pop
        //   emit(:tLAMBEG, '{'.freeze, @te - 1, @te)
        // else
        //   emit(:tLCURLY, '{'.freeze, @te - 1, @te)
        // end
        // fnext expr_value; fbreak;
        wip!();
    };

    #
    # AMBIGUOUS TOKENS RESOLVED VIA EXPR_BEG
    #

    # a??
    # Ternary operator
    '?' c_space_nl
    => {
        //   # Unlike expr_beg as invoked in the next rule, do not warn
        //   p = @ts - 1
        //   fgoto expr_end;
        wip!();
    };

    # a ?b, a? ?
    # Character literal or ternary operator
    w_space* '?'
    => { fhold; fgoto expr_beg; };

    # a %{1}, a %[1] (but not "a %=1=" or "a % foo")
    # a /foo/ (but not "a / foo" or "a /=foo")
    # a <<HEREDOC
    w_space+ %{ tm = p }
    ( [%/] ( c_any - c_space_nl - '=' ) # /
    | '<<'
    )
    => {
        //    if tok(tm, tm + 1) == '/'.freeze
        //      # Ambiguous regexp literal.
        //      diagnostic :warning, :ambiguous_literal, nil, range(tm, tm + 1)
        //    end
        // 
        //    p = tm - 1
        //    fgoto expr_beg;
        wip!();
    };

    # x *1
    # Ambiguous splat, kwsplat or block-pass.
    w_space+ %{ tm = p } ( '+' | '-' | '*' | '&' | '**' )
    => {
        //   diagnostic :warning, :ambiguous_prefix, { :prefix => tok(tm, @te) },
        //               range(tm, @te)
        // 
        //   p = tm - 1
        //   fgoto expr_beg;
        wip!();
    };

    # x ::Foo
    # Ambiguous toplevel constant access.
    w_space+ '::'
    => { fhold; fhold; fgoto expr_beg; };

    # x:b
    # Symbol.
    w_space* ':'
    => { fhold; fgoto expr_beg; };

    w_space+ label
    => { p = ts - 1; fgoto expr_beg; };

    #
    # AMBIGUOUS TOKENS RESOLVED VIA EXPR_END
    #

    # a ? b
    # Ternary operator.
    w_space+ %{ tm = p } '?' c_space_nl
    => { p = tm - 1; fgoto expr_end; };

    # x + 1: Binary operator or operator-assignment.
    w_space* operator_arithmetic
                ( '=' | c_space_nl )?    |
    # x rescue y: Modifier keyword.
    w_space* keyword_modifier            |
    # a &. b: Safe navigation operator.
    w_space* '&.'                        |
    # Miscellanea.
    w_space* punctuation_end
    => {
        p = ts - 1;
        fgoto expr_end;
    };

    w_space;

    w_comment
    => { fgoto expr_end; };

    w_newline
    => { fhold; fgoto expr_end; };

    c_any
    => { fhold; fgoto expr_beg; };

    c_eof => do_eof;
*|;
}%%
