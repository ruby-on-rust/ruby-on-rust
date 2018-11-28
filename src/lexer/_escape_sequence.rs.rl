// TODO

#
# === ESCAPE SEQUENCE PARSING ===
#

# Escape parsing code is a Ragel pattern, not a scanner, and therefore
# it shouldn't directly raise errors or perform other actions with side effects.
# In reality this would probably just mess up error reporting in pathological
# cases, through.

# The amount of code required to parse \M\C stuff correctly is ridiculous.

%%{

escaped_nl = "\\" c_nl;

action unicode_points {
  // @escape = ""

  // codepoints  = tok(@escape_s + 2, p - 1)
  // codepoint_s = @escape_s + 2

  // if @version < 24
  //   if codepoints.start_with?(" ") || codepoints.start_with?("\t")
  //     diagnostic :fatal, :invalid_unicode_escape, nil,
  //       range(@escape_s + 2, @escape_s + 3)
  //   end

  //   if spaces_p = codepoints.index(/[ \t]{2}/)
  //     diagnostic :fatal, :invalid_unicode_escape, nil,
  //       range(codepoint_s + spaces_p + 1, codepoint_s + spaces_p + 2)
  //   end

  //   if codepoints.end_with?(" ") || codepoints.end_with?("\t")
  //     diagnostic :fatal, :invalid_unicode_escape, nil, range(p - 1, p)
  //   end
  // end

  // codepoints.scan(/([0-9a-fA-F]+)|([ \t]+)/).each do |(codepoint_str, spaces)|
  //   if spaces
  //     codepoint_s += spaces.length
  //   else
  //     codepoint = codepoint_str.to_i(16)

  //     if codepoint >= 0x110000
  //       diagnostic :error, :unicode_point_too_large, nil,
  //                   range(codepoint_s, codepoint_s + codepoint_str.length)
  //       break
  //     end

  //     @escape     += codepoint.chr(Encoding::UTF_8)
  //     codepoint_s += codepoint_str.length
  //   end
  // end
}

action unescape_char {
  // codepoint = @source_pts[p - 1]
  // if (@escape = ESCAPES[codepoint]).nil?
  //   @escape = encode_escape(@source_buffer.slice(p - 1))
  // end
}

action invalid_complex_escape {
  // diagnostic :fatal, :invalid_escape
}

action slash_c_char {
  // @escape = encode_escape(@escape[0].ord & 0x9f)
}

action slash_m_char {
  // @escape = encode_escape(@escape[0].ord | 0x80)
}

maybe_escaped_char = (
      '\\' c_any      %unescape_char
  | ( c_any - [\\] )  % {
      // @escape = @source_buffer.slice(p - 1).chr
  }
);

maybe_escaped_ctrl_char = ( # why?!
      '\\' c_any      %unescape_char %slash_c_char
  |   '?'             % {
      // @escape = "\x7f"
  }
  | ( c_any - [\\?] ) % {
      // @escape = @source_buffer.slice(p - 1).chr
  } %slash_c_char
);

escape = (
    # \377
    [0-7]{1,3}
    % {
      // @escape = encode_escape(tok(@escape_s, p).to_i(8) % 0x100)
    }

    # \xff
  | 'x' xdigit{1,2}
      % {
        // @escape = encode_escape(tok(@escape_s + 1, p).to_i(16))
      }

    # %q[\x]
  | 'x' ( c_any - xdigit )
    % {
      // diagnostic :fatal, :invalid_hex_escape, nil, range(@escape_s - 1, p + 2)
    }

    # \u263a
  | 'u' xdigit{4}
    % {
      // @escape = tok(@escape_s + 1, p).to_i(16).chr(Encoding::UTF_8)
    }

    # \u123
  | 'u' xdigit{0,3}
    % {
      // diagnostic :fatal, :invalid_unicode_escape, nil, range(@escape_s - 1, p)
    }

    # u{not hex} or u{}
  | 'u{' ( c_any - xdigit - [ \t}] )* '}'
    % {
      // diagnostic :fatal, :invalid_unicode_escape, nil, range(@escape_s - 1, p)
    }

    # \u{  \t  123  \t 456   \t\t }
  | 'u{' [ \t]* ( xdigit{1,6} [ \t]+ )*
    (
      ( xdigit{1,6} [ \t]* '}'
        %unicode_points
      )
      |
      ( xdigit* ( c_any - xdigit - [ \t}] )+ '}'
        | ( c_any - [ \t}] )* c_eof
        | xdigit{7,}
      ) % {
        // diagnostic :fatal, :unterminated_unicode, nil, range(p - 1, p)
      }
    )

    # \C-\a \cx
  | ( 'C-' | 'c' ) escaped_nl?
    maybe_escaped_ctrl_char

    # \M-a
  | 'M-' escaped_nl?
    maybe_escaped_char
    %slash_m_char

    # \C-\M-f \M-\cf \c\M-f
  | ( ( 'C-'   | 'c' ) escaped_nl?   '\\M-'
    |   'M-\\'         escaped_nl? ( 'C-'   | 'c' ) ) escaped_nl?
    maybe_escaped_ctrl_char
    %slash_m_char

  | 'C' c_any %invalid_complex_escape
  | 'M' c_any %invalid_complex_escape
  | ( 'M-\\C' | 'C-\\M' ) c_any %invalid_complex_escape

  | ( c_any - [0-7xuCMc] ) %unescape_char

  | c_eof % {
    // diagnostic :fatal, :escape_eof, nil, range(p - 1, p)
  }
);

# Use rules in form of `e_bs escape' when you need to parse a sequence.
e_bs = '\\' % {
  // @escape_s = p
  // @escape   = nil
};

}%%
