%%{
expr_end := |*
    #
    # STABBY LAMBDA
    #

#    '->'
#    => {
#      emit(:tLAMBDA, '->'.freeze, @ts, @ts + 2)
#
#      @lambda_stack.push @paren_nest
#      fnext expr_endfn; fbreak;
#    };
#
#    e_lbrace | 'do'
#    => {
#      if @lambda_stack.last == @paren_nest
#        @lambda_stack.pop
#
#        if tok == '{'.freeze
#          emit(:tLAMBEG, '{'.freeze)
#        else # 'do'
#          emit(:kDO_LAMBDA, 'do'.freeze)
#        end
#      else
#        if tok == '{'.freeze
#          emit(:tLCURLY, '{'.freeze)
#        else # 'do'
#          emit_do
#        end
#      end
#
#      fnext expr_value; fbreak;
#    };
#
#    #
#    # KEYWORDS
#    #
#
#    keyword_with_fname
#    => { emit_table(KEYWORDS)
#          fnext expr_fname; fbreak; };
#
#    'class' w_any* '<<'
#    => { emit(:kCLASS, 'class'.freeze, @ts, @ts + 5)
#          emit(:tLSHFT, '<<'.freeze,    @te - 2, @te)
#          fnext expr_value; fbreak; };
#
#    # a if b:c: Syntax error.
#    keyword_modifier
#    => { emit_table(KEYWORDS)
#          fnext expr_beg; fbreak; };
#
#    # elsif b:c: elsif b(:c)
#    keyword_with_value
#    => { emit_table(KEYWORDS)
#          fnext expr_value; fbreak; };
#
#    keyword_with_mid
#    => { emit_table(KEYWORDS)
#          fnext expr_mid; fbreak; };
#
#    keyword_with_arg
#    => {
#      emit_table(KEYWORDS)
#
#      if version?(18) && tok == 'not'.freeze
#        fnext expr_beg; fbreak;
#      else
#        fnext expr_arg; fbreak;
#      end
#    };
#
#    '__ENCODING__'
#    => {
#      if version?(18)
#        emit(:tIDENTIFIER)
#
#        unless !@static_env.nil? && @static_env.declared?(tok)
#          fnext *arg_or_cmdarg;
#        end
#      else
#        emit(:k__ENCODING__, '__ENCODING__'.freeze)
#      end
#      fbreak;
#    };
#
    keyword_with_end
    => {
        !emit_table KEYWORDS;
        fnbreak;
    };
#
#    #
#    # NUMERIC LITERALS
#    #
#
#    ( '0' [Xx] %{ @num_base = 16; @num_digits_s = p } int_hex
#    | '0' [Dd] %{ @num_base = 10; @num_digits_s = p } int_dec
#    | '0' [Oo] %{ @num_base = 8;  @num_digits_s = p } int_dec
#    | '0' [Bb] %{ @num_base = 2;  @num_digits_s = p } int_bin
#    | [1-9] digit* '_'? %{ @num_base = 10; @num_digits_s = @ts } int_dec
#    | '0'   digit* '_'? %{ @num_base = 8;  @num_digits_s = @ts } int_dec
#    ) %{ @num_suffix_s = p } int_suffix
#    => {
#      digits = tok(@num_digits_s, @num_suffix_s)
#
#      if digits.end_with? '_'.freeze
#        diagnostic :error, :trailing_in_number, { :character => '_'.freeze },
#                    range(@te - 1, @te)
#      elsif digits.empty? && @num_base == 8 && version?(18)
#        # 1.8 did not raise an error on 0o.
#        digits = '0'.freeze
#      elsif digits.empty?
#        diagnostic :error, :empty_numeric
#      elsif @num_base == 8 && (invalid_idx = digits.index(/[89]/))
#        invalid_s = @num_digits_s + invalid_idx
#        diagnostic :error, :invalid_octal, nil,
#                    range(invalid_s, invalid_s + 1)
#      end
#
#      if version?(18, 19, 20)
#        emit(:tINTEGER, digits.to_i(@num_base), @ts, @num_suffix_s)
#        p = @num_suffix_s - 1
#      else
#        @num_xfrm.call(digits.to_i(@num_base))
#      end
#      fbreak;
#    };
#
#    flo_frac flo_pow?
#    => {
#      diagnostic :error, :no_dot_digit_literal
#    };
#
#    flo_int [eE]
#    => {
#      if version?(18, 19, 20)
#        diagnostic :error,
#                    :trailing_in_number, { :character => tok(@te - 1, @te) },
#                    range(@te - 1, @te)
#      else
#        emit(:tINTEGER, tok(@ts, @te - 1).to_i, @ts, @te - 1)
#        fhold; fbreak;
#      end
#    };
#
#    flo_int flo_frac [eE]
#    => {
#      if version?(18, 19, 20)
#        diagnostic :error,
#                    :trailing_in_number, { :character => tok(@te - 1, @te) },
#                    range(@te - 1, @te)
#      else
#        emit(:tFLOAT, tok(@ts, @te - 1).to_f, @ts, @te - 1)
#        fhold; fbreak;
#      end
#    };
#
#    flo_int
#    ( flo_frac? flo_pow %{ @num_suffix_s = p } flo_pow_suffix
#    | flo_frac          %{ @num_suffix_s = p } flo_suffix
#    )
#    => {
#      digits = tok(@ts, @num_suffix_s)
#
#      if version?(18, 19, 20)
#        emit(:tFLOAT, Float(digits), @ts, @num_suffix_s)
#        p = @num_suffix_s - 1
#      else
#        @num_xfrm.call(digits)
#      end
#      fbreak;
#    };
#
#    #
#    # STRING AND XSTRING LITERALS
#    #
#
#    # `echo foo`, "bar", 'baz'
#    '`' | ['"] # '
#    => {
#      type, delimiter = tok, tok[-1].chr
#      fgoto *push_literal(type, delimiter, @ts, nil, false, false, true);
#    };
#
#    #
#    # CONSTANTS AND VARIABLES
#    #
#
#    constant
#    => { emit(:tCONSTANT)
#          fnext *arg_or_cmdarg; fbreak; };
#
#    constant ambiguous_const_suffix
#    => { emit(:tCONSTANT, tok(@ts, tm), @ts, tm)
#          p = tm - 1; fbreak; };
#
#    global_var | class_var_v | instance_var_v
#    => { p = @ts - 1; fcall expr_variable; };
#
#    #
#    # METHOD CALLS
#    #
#
#    '.' | '&.' | '::'
#    => { emit_table(PUNCTUATION)
#          fnext expr_dot; fbreak; };
#
#    call_or_var
#    => local_ident;
#
#    bareword ambiguous_fid_suffix
#    => {
#      if tm == @te
#        # Suffix was consumed, e.g. foo!
#        emit(:tFID)
#      else
#        # Suffix was not consumed, e.g. foo!=
#        emit(:tIDENTIFIER, tok(@ts, tm), @ts, tm)
#        p = tm - 1
#      end
#      fnext expr_arg; fbreak;
#    };
#
#    #
#    # OPERATORS
#    #
#
#    '*' | '=>'
#    => {
#      emit_table(PUNCTUATION)
#      fgoto expr_value;
#    };
#
#    # When '|', '~', '!', '=>' are used as operators
#    # they do not accept any symbols (or quoted labels) after.
#    # Other binary operators accept it.
#    ( operator_arithmetic | operator_rest ) - ( '|' | '~' | '!' | '*' )
#    => {
#      emit_table(PUNCTUATION);
#      fnext expr_value; fbreak;
#    };
#
#    ( e_lparen | '|' | '~' | '!' )
#    => { emit_table(PUNCTUATION)
#          fnext expr_beg; fbreak; };
#
#    e_rbrace | e_rparen | ']'
#    => {
#      emit_table(PUNCTUATION)
#
#      if @version < 24
#        @cond.lexpop
#        @cmdarg.lexpop
#      else
#        @cond.pop
#        @cmdarg.pop
#      end
#
#      if tok == '}'.freeze || tok == ']'.freeze
#        if @version >= 25
#          fnext expr_end;
#        else
#          fnext expr_endarg;
#        end
#      else # )
#        # fnext expr_endfn; ?
#      end
#
#      fbreak;
#    };
#
#    operator_arithmetic '='
#    => { emit(:tOP_ASGN, tok(@ts, @te - 1))
#          fnext expr_beg; fbreak; };
#
#    '?'
#    => { emit(:tEH, '?'.freeze)
#          fnext expr_value; fbreak; };
#
#    e_lbrack
#    => { emit(:tLBRACK2, '['.freeze)
#          fnext expr_beg; fbreak; };
#
#    punctuation_end
#    => { emit_table(PUNCTUATION)
#          fnext expr_beg; fbreak; };
#
#    #
#    # WHITESPACE
#    #
#
#    w_space_comment;
#
#    w_newline
#    => { fgoto leading_dot; };
#
#    ';'
#    => { emit(:tSEMI, ';'.freeze)
#          fnext expr_value; fbreak; };
#
#    '\\' c_line {
#      diagnostic :error, :bare_backslash, nil, range(@ts, @ts + 1)
#      fhold;
#    };
#
#    c_any
#    => {
#      diagnostic :fatal, :unexpected, { :character => tok.inspect[1..-2] }
#    };
#
#    c_eof => do_eof;
*|;
}%%
