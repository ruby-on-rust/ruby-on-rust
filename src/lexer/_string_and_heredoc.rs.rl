#
# === STRING AND HEREDOC PARSING ===
#

# Heredoc parsing is quite a complex topic. First, consider that heredocs
# can be arbitrarily nested. For example:
#
#     puts <<CODE
#     the result is: #{<<RESULT.inspect
#       i am a heredoc
#     RESULT
#     }
#     CODE
#
# which, incidentally, evaluates to:
#
#     the result is: "  i am a heredoc\n"
#
# To parse them, lexer refers to two kinds (remember, nested heredocs)
# of positions in the input stream, namely heredoc_e
# (HEREDOC declaration End) and @herebody_s (HEREdoc BODY line Start).
#
# heredoc_e is simply contained inside the corresponding Literal, and
# when the heredoc is closed, the lexing is restarted from that position.
#
# @herebody_s is quite more complex. First, @herebody_s changes after each
# heredoc line is lexed. This way, at '\n' tok(@herebody_s, @te) always
# contains the current line, and also when a heredoc is started, @herebody_s
# contains the position from which the heredoc will be lexed.
#
# Second, as (insanity) there are nested heredocs, we need to maintain a
# stack of these positions. Each time #push_literal is called, it saves current
# @heredoc_s to literal.saved_herebody_s, and after an interpolation (possibly
# containing another heredocs) is closed, the previous value is restored.

%%{
e_heredoc_nl = c_nl % {
  // TODO
  // # After every heredoc was parsed, @herebody_s contains the
  // # position of next token after all heredocs.
  // if @herebody_s
  //   p = @herebody_s
  //   @herebody_s = nil
  // end
};

# action extend_string {
#   string = tok
#
#   # tLABEL_END is only possible in non-cond context on >= 2.2
#   if @version >= 22 && !@cond.active?
#     lookahead = @source_buffer.slice(@te...@te+2)
#   end
#
#   current_literal = literal
#   if !current_literal.heredoc? &&
#         (token = current_literal.nest_and_try_closing(string, @ts, @te, lookahead))
#     if token[0] == :tLABEL_END
#       p += 1
#       pop_literal
#       fnext expr_labelarg;
#     else
#       fnext *pop_literal;
#     end
#     fbreak;
#   else
#     current_literal.extend_string(string, @ts, @te)
#   end
# }
action extend_string {
    println!("action:extend_string invoking");

    let temp_string = self.input_slice(ts, te);
    // NOTE ignored ruby22-and-below cases
    // TODO INCOMPLETE handle @cond.active
    let lookahead = self.input_slice(te, te + 2);

    let mut current_literal = self.literal().expect("literal_stack is empty").clone();
    if !current_literal.is_heredoc() {
        if let Some(token) = current_literal.nest_and_try_closing(&temp_string, ts, te, Some(lookahead)) {
            if let Token::T_LABEL_END = token {
                p += 1;
                self.pop_literal();
                fnext expr_labelarg;
            } else {
                fnext *self.pop_literal();
            }

            fnbreak;
        }
    }

    current_literal.extend_string(&temp_string, ts, te);

    // NOTE
    // due to limitations of borrowing in rust, we have to
    // 1 clone current_literal
    // 2 modify it 
    // 3 re-save it to the stack
    // TODO leverage RefCell
    self.literal_stack.pop();
    self.literal_stack.push(current_literal);
}

action extend_string_escaped {
  // TODO
  // current_literal = literal
  // # Get the first character after the backslash.
  // escaped_char = @source_buffer.slice(@escape_s).chr

  // if current_literal.munge_escape? escaped_char
  //   # If this particular literal uses this character as an opening
  //   # or closing delimiter, it is an escape sequence for that
  //   # particular character. Write it without the backslash.

  //   if current_literal.regexp? && REGEXP_META_CHARACTERS.match(escaped_char)
  //     # Regular expressions should include escaped delimiters in their
  //     # escaped form, except when the escaped character is
  //     # a closing delimiter but not a regexp metacharacter.
  //     #
  //     # The backslash itself cannot be used as a closing delimiter
  //     # at the same time as an escape symbol, but it is always munged,
  //     # so this branch also executes for the non-closing-delimiter case
  //     # for the backslash.
  //     current_literal.extend_string(tok, @ts, @te)
  //   else
  //     current_literal.extend_string(escaped_char, @ts, @te)
  //   end
  // else
  //   # It does not. So this is an actual escape sequence, yay!
  //   if current_literal.regexp?
  //     # Regular expressions should include escape sequences in their
  //     # escaped form. On the other hand, escaped newlines are removed.
  //     current_literal.extend_string(tok.gsub("\\\n".freeze, ''.freeze), @ts, @te)
  //   elsif current_literal.heredoc? && escaped_char == "\n".freeze
  //     if current_literal.squiggly_heredoc?
  //       # Squiggly heredocs like
  //       #   <<~-HERE
  //       #     1\
  //       #     2
  //       #   HERE
  //       # treat '\' as a line continuation, but still dedent the body, so the heredoc above becomes "12\n".
  //       # This information is emitted as is, without escaping,
  //       # later this escape sequence (\\n) gets handled manually in the Lexer::Dedenter
  //       current_literal.extend_string(tok, @ts, @te)
  //     else
  //       # Plain heredocs also parse \\n as a line continuation,
  //       # but they don't need to know that there was originally a newline in the
  //       # code, so we escape it and emit as "  1  2\n"
  //       current_literal.extend_string(tok.gsub("\\\n".freeze, ''.freeze), @ts, @te)
  //     end
  //   else
  //     current_literal.extend_string(@escape || tok, @ts, @te)
  //   end
  // end
}

# Extend a string with a newline or a EOF character.
# As heredoc closing line can immediately precede EOF, this action
# has to handle such case specially.
action extend_string_eol {
  // TODO
  // current_literal = literal
  // if @te == pe
  //   diagnostic :fatal, :string_eof, nil,
  //               range(current_literal.str_s, current_literal.str_s + 1)
  // end

  // if current_literal.heredoc?
  //   line = tok(@herebody_s, @ts).gsub(/\r+$/, ''.freeze)

  //   if version?(18, 19, 20)
  //     # See ruby:c48b4209c
  //     line = line.gsub(/\r.*$/, ''.freeze)
  //   end

  //   # Try ending the heredoc with the complete most recently
  //   # scanned line. @herebody_s always refers to the start of such line.
  //   if current_literal.nest_and_try_closing(line, @herebody_s, @ts)
  //     # Adjust @herebody_s to point to the next line.
  //     @herebody_s = @te

  //     # Continue regular lexing after the heredoc reference (<<END).
  //     p = current_literal.heredoc_e - 1
  //     fnext *pop_literal; fbreak;
  //   else
  //     # Calculate indentation level for <<~HEREDOCs.
  //     current_literal.infer_indent_level(line)

  //     # Ditto.
  //     @herebody_s = @te
  //   end
  // else
  //   # Try ending the literal with a newline.
  //   if current_literal.nest_and_try_closing(tok, @ts, @te)
  //     fnext *pop_literal; fbreak;
  //   end

  //   if @herebody_s
  //     # This is a regular literal intertwined with a heredoc. Like:
  //     #
  //     #     p <<-foo+"1
  //     #     bar
  //     #     foo
  //     #     2"
  //     #
  //     # which, incidentally, evaluates to "bar\n1\n2".
  //     p = @herebody_s - 1
  //     @herebody_s = nil
  //   end
  // end

  // if current_literal.words? && !eof_codepoint?(@source_pts[p])
  //   current_literal.extend_space @ts, @te
  // else
  //   # A literal newline is appended if the heredoc was _not_ closed
  //   # this time (see fbreak above). See also Literal#nest_and_try_closing
  //   # for rationale of calling #flush_string here.
  //   current_literal.extend_string tok, @ts, @te
  //   current_literal.flush_string
  // end
}

action extend_string_space {
  // TODO
  // literal.extend_space @ts, @te
}
}%%
