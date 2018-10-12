%%{
# Special newline handling for "def a b:"
#
expr_labelarg := |*
  w_space_comment;

  w_newline
  => {
    if @in_kwarg
      fhold; fgoto expr_end;
    else
      fgoto line_begin;
    end
  };

  c_any
  => { fhold; fgoto expr_beg; };

  c_eof => do_eof;
*|;
}%%
