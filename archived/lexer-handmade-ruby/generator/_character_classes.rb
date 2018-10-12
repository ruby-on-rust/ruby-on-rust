# 
# ragel built in machines
# 
# any — Any character in the alphabet.
# ascii — Ascii characters. 0..127
# extend — Ascii extended characters. This is the range -128..127 for signed alphabets and the range 0..255 for unsigned alphabets.
# alpha — Alphabetic characters. [A-Za-z]
# digit — Digits. [0-9]
# alnum — Alpha numerics. [0-9A-Za-z]
# lower — Lowercase characters. [a-z]
# upper — Uppercase characters. [A-Z]
# xdigit — Hexadecimal digits. [0-9A-Fa-f]
# cntrl — Control characters. 0..31, 127
# graph — Graphical characters. [!-~]
# print — Printable characters. [ -~]
# punct — Punctuation. Graphical characters that are not alphanumerics.
# 
# [!-/:-@\[-`{-~]
# 
# space — Whitespace. [\t\v\f\n\r ]
# zlen — Zero length string. ""
# empty — Empty set. Matches nothing. ^any
# 

m! :zlen, '$' # TODO i don't think they are the same?

# 
# regex built in character classes
# 
# [[:alnum:]]    alphanumeric ([0-9A-Za-z])
# [[:alpha:]]    alphabetic ([A-Za-z])
# [[:ascii:]]    ASCII ([\x00-\x7F])
# [[:blank:]]    blank ([\t ])
# [[:cntrl:]]    control ([\x00-\x1F\x7F])
# [[:digit:]]    digits ([0-9])
# [[:graph:]]    graphical ([!-~])
# [[:lower:]]    lower case ([a-z])
# [[:print:]]    printable ([ -~])
# [[:punct:]]    punctuation ([!-/:-@\[-`{-~])
# [[:space:]]    whitespace ([\t\n\v\f\r ])
# [[:upper:]]    upper case ([A-Z])
# [[:word:]]     word characters ([0-9A-Za-z_])
# [[:xdigit:]]   hex digit ([0-9A-Fa-f])

# === CHARACTER CLASSES ===
#
# Pay close attention to the differences between c_any and any.
# c_any does not include EOF and so will cause incorrect behavior
# for machine subtraction (any-except rules) and default transitions
# for scanners.

# action do_nl {
#   # Record position of a newline for precise location reporting on tNL
#   # tokens.
#   #
#   # This action is embedded directly into c_nl, as it is idempotent and
#   # there are no cases when we need to skip it.
#   @newline_s = p
# }

# c_nl       = '\n' $ do_nl;
m! :c_nl, '\n' # TODO ACTION
# c_space    = [ \t\r\f\v];
m! :c_space, '[ \t\r\f\v]'

# c_space_nl = c_space | c_nl;
m! :c_space_nl, [ :c_space, :c_nl ]

# c_eof      = 0x04 | 0x1a | 0 | zlen; # ^D, ^Z, \0, EOF
m! :c_eof, [ '$' ] # TODO INCOMPLETE
# c_eol      = c_nl | c_eof;
m! :c_eol, %i(c_nl c_eof)
# c_any      = any - c_eof;
m! :c_any, '.' # TODO not completely same

# c_nl_zlen  = c_nl | zlen;
m! :c_nl_zlen, %i(c_nl zlen)
# c_line     = any - c_nl_zlen;

# c_unicode  = c_any - 0x00..0x7f; # TODO
# c_upper    = [A-Z];
m! :c_upper, '[[:upper:]]'
# c_lower    = [a-z_]  | c_unicode;
m! :c_lower, '[[:lower:]]' # TODO INCOMPLETE
# c_alpha    = c_lower | c_upper;
m! :c_alpha, '[[:alpha:]]'
# c_alnum    = c_alpha | [0-9];
m! :c_alnum, '[[:alnum:]]'

# action do_eof {
#   # Sit at EOF indefinitely. #advance would return $eof each time.
#   # This allows to feed the lexer more data if needed; this is only used
#   # in tests.
#   #
#   # Note that this action is not embedded into e_eof like e_heredoc_nl and e_bs
#   # below. This is due to the fact that scanner state at EOF is observed
#   # by tests, and encapsulating it in a rule would break the introspection.
#   fhold; fbreak;
# }
a! :do_eof, %q{
    // TODO
}
