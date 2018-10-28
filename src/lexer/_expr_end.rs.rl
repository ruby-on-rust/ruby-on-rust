%%{
expr_end := |*

#
# STABBY LAMBDA
#

# TODO
#    '->'
#    => {
#      emit(:tLAMBDA, '->'.freeze, @ts, @ts + 2)
#
#      @lambda_stack.push @paren_nest
#      fnext expr_endfn; fbreak;
#    };

# TODO
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
    # KEYWORDS
    #

    keyword_with_fname
    => {
        !emit_table KEYWORDS;
        fnext expr_fname; fnbreak;
    };

# TODO
#    'class' w_any* '<<'
#    => { emit(:kCLASS, 'class'.freeze, @ts, @ts + 5)
#          emit(:tLSHFT, '<<'.freeze,    @te - 2, @te)
#          fnext expr_value; fbreak; };
#
    # a if b:c: Syntax error.
    keyword_modifier
    => {
        !emit_table KEYWORDS;
        fnext expr_beg; fnbreak;
    };

    # elsif b:c: elsif b(:c)
    keyword_with_value
    => {
        !emit_table KEYWORDS;
        fnext expr_value; fnbreak;
    };

    keyword_with_mid
    => {
        !emit_table KEYWORDS;
        fnext expr_mid; fnbreak;
    };

    keyword_with_arg
    => {
        !emit_table KEYWORDS;

        //   if version?(18) && tok == 'not'.freeze
        //     fnext expr_beg; fbreak;
        //   else
        //     fnext expr_arg; fbreak;
        //   end
        // NOTE ignored ruby18
        fnext expr_arg; fnbreak;
    };

# TODO
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

    keyword_with_end
    => {
        !emit_table KEYWORDS;
        fnbreak;
    };

    #
    # NUMERIC LITERALS
    #

    ( '0' [Xx] %{ self.num_base = 16; self.num_digits_s = p } int_hex
    | '0' [Dd] %{ self.num_base = 10; self.num_digits_s = p } int_dec
    | '0' [Oo] %{ self.num_base = 8;  self.num_digits_s = p } int_dec
    | '0' [Bb] %{ self.num_base = 2;  self.num_digits_s = p } int_bin
    | [1-9] digit* '_'? %{ self.num_base = 10; self.num_digits_s = ts } int_dec
    | '0'   digit* '_'? %{ self.num_base = 8;  self.num_digits_s = ts } int_dec
    ) %{ self.num_suffix_s = p } int_suffix
    => {
        // TODO WIP

        // digits = tok(@num_digits_s, @num_suffix_s)
        let digits = self.current_slice(self.num_digits_s, self.num_suffix_s);

        // if digits.end_with? '_'.freeze
        //   diagnostic :error, :trailing_in_number, { :character => '_'.freeze },
        //               range(@te - 1, @te)
        // elsif digits.empty? && @num_base == 8 && version?(18)
        //   # 1.8 did not raise an error on 0o.
        //   digits = '0'.freeze
        // elsif digits.empty?
        //   diagnostic :error, :empty_numeric
        // elsif @num_base == 8 && (invalid_idx = digits.index(/[89]/))
        //   invalid_s = @num_digits_s + invalid_idx
        //   diagnostic :error, :invalid_octal, nil,
        //               range(invalid_s, invalid_s + 1)
        // end
        // 
        // if version?(18, 19, 20)
        //   emit(:tINTEGER, digits.to_i(@num_base), @ts, @num_suffix_s)
        //   p = @num_suffix_s - 1
        // else
        //   @num_xfrm.call(digits.to_i(@num_base))
        // end
        // fbreak;

        let token = Token::T_INTEGER(digits.parse::<isize>().unwrap());
        self.emit(token);

        fnbreak;
    };

# TODO
#    flo_frac flo_pow?
#    => {
#      diagnostic :error, :no_dot_digit_literal
#    };

# TODO
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

# TODO
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

# TODO
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
    # STRING AND XSTRING LITERALS
    #

    # `echo foo`, "bar", 'baz'
    '`' | ['"] # '
    => {
        // type, delimiter = tok, tok[-1].chr
        // fgoto *push_literal(type, delimiter, @ts, nil, false, false, true);
        let literal_type = self.current_slice(ts, te);
        let literal_delimiter = self.current_slice(te - 1, te);
        let literal = Literal::new(literal_type, literal_delimiter, ts, None, false, false, false, Rc::clone(&self.tokens));
        fgoto *self.push_literal(literal);
    };

    #
    # CONSTANTS AND VARIABLES
    #

    constant
    => {
        // TODO
        // emit(:tCONSTANT)
        // fnext *arg_or_cmdarg; fbreak;
    };

    constant ambiguous_const_suffix
    => {
        !emit T_CONSTANT, ts, tm;
        p = tm - 1;
        fnbreak;
    };

    global_var | class_var_v | instance_var_v
    => { p = ts - 1; fncall expr_variable; };

    #
    # METHOD CALLS
    #

    '.' | '&.' | '::'
    => {
        !emit_table PUNCTUATION;
        fnext expr_dot; fnbreak;
    };

    call_or_var
    => local_ident;

# TODO
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
    # OPERATORS
    #

    '*' | '=>'
    => {
      !emit_table PUNCTUATION;
      fgoto expr_value;
    };

    # When '|', '~', '!', '=>' are used as operators
    # they do not accept any symbols (or quoted labels) after.
    # Other binary operators accept it.
    ( operator_arithmetic | operator_rest ) - ( '|' | '~' | '!' | '*' )
    => {
      !emit_table PUNCTUATION;
      fnext expr_value; fnbreak;
    };

    ( e_lparen | '|' | '~' | '!' )
    => {
        !emit_table PUNCTUATION;
        fnext expr_beg; fnbreak;
    };

    e_rbrace | e_rparen | ']'
    => {
        !emit_table PUNCTUATION;

        // NOTE ignored ruby24
        // if @version < 24
        //   @cond.lexpop
        //   @cmdarg.lexpop
        // else
        //   @cond.pop
        //   @cmdarg.pop
        // end
        self.cond.pop();
        self.cmdarg.pop();

        // TODO WIP
        // if tok == '}'.freeze || tok == ']'.freeze
        //   if @version >= 25
        //     fnext expr_end;
        //   else
        //     fnext expr_endarg;
        //   end
        // else # )
        //   # fnext expr_endfn; ?
        // end
        fnext expr_end;

        fnbreak;
    };

    operator_arithmetic '='
    => {
        // emit(:tOP_ASGN, tok(@ts, @te - 1))
        !emit T_OP_ASGN, ts, te - 1;
        fnext expr_beg; fnbreak;
    };

    '?'
    => {
        !emit T_EH_;
        fnext expr_value; fnbreak;
    };

    e_lbrack
    => {
        !emit T_LBRACK2_;
        fnext expr_beg; fnbreak;
    };

    punctuation_end
    => {
        !emit_table PUNCTUATION;
        fnext expr_beg; fnbreak;
    };

    #
    # WHITESPACE
    #

    w_space_comment;

    w_newline
    => { fgoto leading_dot; };

    ';'
    => {
        // emit(:tSEMI, ';'.freeze)
        !emit T_SEMI_;
        fnext expr_value; fnbreak;
    };

# TODO
#    '\\' c_line {
#      diagnostic :error, :bare_backslash, nil, range(@ts, @ts + 1)
#      fhold;
#    };

    c_any
    => {
        // diagnostic :fatal, :unexpected, { :character => tok.inspect[1..-2] }
        panic!("lexer diagnostic: unexpected char: {}", self.current_slice(ts, te)); // TODO char position
    };

    c_eof => do_eof;
*|;
}%%
