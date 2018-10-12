%%{
#
# === EMBEDDED DOCUMENT (aka BLOCK COMMENT) PARSING ===
#

line_comment := |*
    '=end' c_line* c_nl_zlen
    => {
      emit_comment(@eq_begin_s, @te)
      fgoto *@cs_before_block_comment;
    };

    c_line* c_nl;

    c_line* zlen
    => {
      diagnostic :fatal, :embedded_document, nil,
                  range(@eq_begin_s, @eq_begin_s + '=begin'.length)
    };
*|;
}%%
