%%{
# The previous token was an identifier which was seen while in the
# command mode (that is, the state at the beginning of #advance was
# expr_value). This state is very similar to expr_arg, but disambiguates
# two very rare and specific condition:
#   * In 1.8 mode, "foo (lambda do end)".
#   * In 1.9+ mode, "f x: -> do foo do end end".
expr_cmdarg := |*
    w_space+ e_lparen
    => {
      emit(:tLPAREN_ARG, '('.freeze, @te - 1, @te)
      if version?(18)
        fnext expr_value; fbreak;
      else
        fnext expr_beg; fbreak;
      end
    };

    w_space* 'do'
    => {
      if @cond.active?
        emit(:kDO_COND, 'do'.freeze, @te - 2, @te)
      else
        emit(:kDO, 'do'.freeze, @te - 2, @te)
      end
      fnext expr_value; fbreak;
    };

    c_any             |
    # Disambiguate with the `do' rule above.
    w_space* bareword |
    w_space* label
    => { p = @ts - 1
          fgoto expr_arg; };

    c_eof => do_eof;
*|;
}%%
