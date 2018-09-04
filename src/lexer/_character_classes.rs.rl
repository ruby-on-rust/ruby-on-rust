%%{
# === CHARACTER CLASSES ===
#
# Pay close attention to the differences between c_any and any.
# c_any does not include EOF and so will cause incorrect behavior
# for machine subtraction (any-except rules) and default transitions
# for scanners.

action do_nl {
  // TODO
  // # Record position of a newline for precise location reporting on tNL
  // # tokens.
  // #
  // # This action is embedded directly into c_nl, as it is idempotent and
  // # there are no cases when we need to skip it.
  // @newline_s = p
}

c_nl       = '\n' $ do_nl;
c_space    = [ \t\r\f\v];
c_space_nl = c_space | c_nl;

c_eof      = 0x04 | 0x1a | 0 | zlen; # ^D, ^Z, \0, EOF
c_eol      = c_nl | c_eof;
c_any      = any - c_eof;

c_nl_zlen  = c_nl | zlen;
c_line     = any - c_nl_zlen;

c_unicode  = c_any - 0x00..0x7f;
c_upper    = [A-Z];
c_lower    = [a-z_]  | c_unicode;
c_alpha    = c_lower | c_upper;
c_alnum    = c_alpha | [0-9];

action do_eof {
  // # Sit at EOF indefinitely. #advance would return $eof each time.
  // # This allows to feed the lexer more data if needed; this is only used
  // # in tests.
  // #
  // # Note that this action is not embedded into e_eof like e_heredoc_nl and e_bs
  // # below. This is due to the fact that scanner state at EOF is observed
  // # by tests, and encapsulating it in a rule would break the introspection.
  // TODO
  // fhold; fbreak;
}
}%%
