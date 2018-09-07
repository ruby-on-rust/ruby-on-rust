# #
# # === WHITESPACE HANDLING ===
# #

# # Various contexts in Ruby allow various kinds of whitespace
# # to be used. They are grouped to clarify the lexing machines
# # and ease collection of comments.

# # A line of code with inline #comment at end is always equivalent
# # to a line of code ending with just a newline, so an inline
# # comment is deemed equivalent to non-newline whitespace
# # (c_space character class).

# w_space =
#     c_space+
#   | '\\' e_heredoc_nl
#   ;

# w_comment =
#     '#'     %{ @sharp_s = p - 1 }
#     # The (p == pe) condition compensates for added "\0" and
#     # the way Ragel handles EOF.
#     c_line* %{ emit_comment(@sharp_s, p == pe ? p - 2 : p) }
#   ;

# w_space_comment =
#     w_space
#   | w_comment
#   ;

# # A newline in non-literal context always interoperates with
# # here document logic and can always be escaped by a backslash,
# # still interoperating with here document logic in the same way,
# # yet being invisible to anything else.
# #
# # To demonstrate:
# #
# #     foo = <<FOO \
# #     bar
# #     FOO
# #      + 2
# #
# # is equivalent to `foo = "bar\n" + 2`.

# w_newline =
#     e_heredoc_nl;

# w_any =
#     w_space
#   | w_comment
#   | w_newline
#   ;

m! :w_any, ' ' # TODO
