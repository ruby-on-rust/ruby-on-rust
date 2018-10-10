# #
# # === INTERPOLATION PARSING ===
# #

# # Interpolations with immediate variable names simply call into
# # the corresponding machine.

# interp_var = '#' ( global_var | class_var_v | instance_var_v );

# action extend_interp_var {
#   current_literal = literal
#   current_literal.flush_string
#   current_literal.extend_content
# 
#   emit(:tSTRING_DVAR, nil, @ts, @ts + 1)
# 
#   p = @ts
#   fcall expr_variable;
# }

# # Interpolations with code blocks must match nested curly braces, as
# # interpolation ending is ambiguous with a block ending. So, every
# # opening and closing brace should be matched with e_[lr]brace rules,
# # which automatically perform the counting.
# #
# # Note that interpolations can themselves be nested, so brace balance
# # is tied to the innermost literal.
# #
# # Also note that literals themselves should not use e_[lr]brace rules
# # when matching their opening and closing delimiters, as the amount of
# # braces inside the characters of a string literal is independent.

# interp_code = '#{';

# e_lbrace = '{' % {
#   @cond.push(false); @cmdarg.push(false)

#   current_literal = literal
#   if current_literal
#     current_literal.start_interp_brace
#   end
# };

# e_rbrace = '}' % {
#   current_literal = literal
#   if current_literal
#     if current_literal.end_interp_brace_and_try_closing
#       if version?(18, 19)
#         emit(:tRCURLY, '}'.freeze, p - 1, p)
#         if @version < 24
#           @cond.lexpop
#           @cmdarg.lexpop
#         else
#           @cond.pop
#           @cmdarg.pop
#         end
#       else
#         emit(:tSTRING_DEND, '}'.freeze, p - 1, p)
#       end

#       if current_literal.saved_herebody_s
#         @herebody_s = current_literal.saved_herebody_s
#       end


#       fhold;
#       fnext *next_state_for_literal(current_literal);
#       fbreak;
#     end
#   end
# };

# action extend_interp_code {
#   current_literal = literal
#   current_literal.flush_string
#   current_literal.extend_content

#   emit(:tSTRING_DBEG, '#{'.freeze)

#   if current_literal.heredoc?
#     current_literal.saved_herebody_s = @herebody_s
#     @herebody_s = nil
#   end

#   current_literal.start_interp_brace
#   fnext expr_value;
#   fbreak;
# }

# # Actual string parsers are simply combined from the primitives defined
# # above.

# interp_words := |*
#     interp_code => extend_interp_code;
#     interp_var  => extend_interp_var;
#     e_bs escape => extend_string_escaped;
#     c_space+    => extend_string_space;
#     c_eol       => extend_string_eol;
#     c_any       => extend_string;
# *|;

# interp_string := |*
#     interp_code => extend_interp_code;
#     interp_var  => extend_interp_var;
#     e_bs escape => extend_string_escaped;
#     c_eol       => extend_string_eol;
#     c_any       => extend_string;
# *|;

# plain_words := |*
#     e_bs c_any  => extend_string_escaped;
#     c_space+    => extend_string_space;
#     c_eol       => extend_string_eol;
#     c_any       => extend_string;
# *|;

# plain_string := |*
#     '\\' c_nl   => extend_string_eol;
#     e_bs c_any  => extend_string_escaped;
#     c_eol       => extend_string_eol;
#     c_any       => extend_string;
# *|;
# TODO INCOMPLETE
s = Scanner.new :plain_string
s.p :c_any, :extend_string

# interp_backslash_delimited := |*
#     interp_code => extend_interp_code;
#     interp_var  => extend_interp_var;
#     c_eol       => extend_string_eol;
#     c_any       => extend_string;
# *|;

# plain_backslash_delimited := |*
#     c_eol       => extend_string_eol;
#     c_any       => extend_string;
# *|;

# interp_backslash_delimited_words := |*
#     interp_code => extend_interp_code;
#     interp_var  => extend_interp_var;
#     c_space+    => extend_string_space;
#     c_eol       => extend_string_eol;
#     c_any       => extend_string;
# *|;

# plain_backslash_delimited_words := |*
#     c_space+    => extend_string_space;
#     c_eol       => extend_string_eol;
#     c_any       => extend_string;
# *|;

# regexp_modifiers := |*
#     [A-Za-z]+
#     => {
#       unknown_options = tok.scan(/[^imxouesn]/)
#       if unknown_options.any?
#         diagnostic :error, :regexp_options,
#                     { :options => unknown_options.join }
#       end

#       emit(:tREGEXP_OPT)
#       fnext expr_end;
#       fbreak;
#     };

#     any
#     => {
#       emit(:tREGEXP_OPT, tok(@ts, @te - 1), @ts, @te - 1)
#       fhold;
#       fgoto expr_end;
#     };
# *|;
