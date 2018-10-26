%%{
# Beginning of an expression.
#
# Don't fallthrough to this state from `c_any`; make sure to handle
# `c_space* c_nl` and let `expr_end` handle the newline.
# Otherwise code like `f\ndef x` gets glued together and the parser
# explodes.
#
expr_beg := |*
    # +5, -5, -  5
    [+\-] w_any* [0-9]
    => {
      // emit(:tUNARY_NUM, tok(@ts, @ts + 1), @ts, @ts + 1)
      !emit T_UNARY_NUM, ts, ts + 1;
      fhold; fnext expr_end; fnbreak;
    };

    # splat *a
    '*'
    => {
        !emit T_STAR_;
        fnbreak;
    };

    #
    # STRING AND REGEXP LITERALS
    #

    # # /regexp/oui
    # # /=/ (disambiguation with /=)
    # '/' c_any
    # => {
    #   // type = delimiter = tok[0].chr
    #   // fhold; fgoto *push_literal(type, delimiter, @ts);
    # };

    # # %<string>
    # '%' ( any - [A-Za-z] )
    # => {
    #   // type, delimiter = @source_buffer.slice(@ts).chr, tok[-1].chr
    #   // fgoto *push_literal(type, delimiter, @ts);
    # };

    # %w(we are the people)
    '%' [A-Za-z]+ c_any
    => {
        // type, delimiter = tok[0..-2], tok[-1].chr
        // fgoto *push_literal(type, delimiter, @ts);

        panic!("WIP");
        // let literal_type = self.current_slice(ts, te).clone();
        // let literal_delimiter = literal_type.chars().last().unwrap().to_string();
        // let literal = Literal::new(literal_type, literal_delimiter, ts, None, false, false, false, Rc::clone(&self.tokens));
        // fgoto *self.push_literal(literal);
    };

    '%' c_eof
    => {
        // diagnostic :fatal, :string_eof, nil, range(@ts, @ts + 1)
        panic!("lexer diagnostic");
    };

    # # Heredoc start.
    # # <<END  | <<'END'  | <<"END"  | <<`END`  |
    # # <<-END | <<-'END' | <<-"END" | <<-`END` |
    # # <<~END | <<~'END' | <<~"END" | <<~`END`
    # '<<' [~\-]?
    #   ( '"' ( any - '"' )* '"'
    #   | "'" ( any - "'" )* "'"
    #   | "`" ( any - "`" )* "`"
    #   | bareword ) % { heredoc_e      = p }
    #   c_line* c_nl % { new_herebody_s = p }
    # => {
    #   tok(@ts, heredoc_e) =~ /^<<(-?)(~?)(["'`]?)(.*)\3$/m
    #
    #   indent      = !$1.empty? || !$2.empty?
    #   dedent_body = !$2.empty?
    #   type        =  $3.empty? ? '<<"'.freeze : ('<<'.freeze + $3)
    #   delimiter   =  $4
    #
    #   if @version >= 24
    #     if delimiter.count("\n") > 0
    #       if delimiter.end_with?("\n")
    #         diagnostic :warning, :heredoc_id_ends_with_nl, nil, range(@ts, @ts + 1)
    #         delimiter = delimiter.rstrip
    #       else
    #         diagnostic :fatal, :heredoc_id_has_newline, nil, range(@ts, @ts + 1)
    #       end
    #     end
    #   end
    #
    #   if dedent_body && version?(18, 19, 20, 21, 22)
    #     emit(:tLSHFT, '<<'.freeze, @ts, @ts + 2)
    #     p = @ts + 1
    #     fnext expr_beg; fbreak;
    #   else
    #     fnext *push_literal(type, delimiter, @ts, heredoc_e, indent, dedent_body);
    #
    #     @herebody_s ||= new_herebody_s
    #     p = @herebody_s - 1
    #   end
    # };

    #
    # SYMBOL LITERALS
    #

    # :&&, :||
    ':' ('&&' | '||') => {
        fhold; fhold;
        !emit T_SYMBEG, ts, ts + 1;
        fgoto expr_fname;
    };

    # :"bar", :'baz'
    ':' ['"] # '
    => {
        // type, delimiter = tok, tok[-1].chr
        // fgoto *push_literal(type, delimiter, @ts);
        let literal_type = self.current_slice(ts, te).clone();
        let literal_delimiter = literal_type.chars().last().unwrap().to_string();
        let literal = Literal::new(literal_type, literal_delimiter, ts, None, false, false, false, Rc::clone(&self.tokens));
        fgoto *self.push_literal(literal);
    };

    # :!@ is :!
    # :~@ is :~
    ':' [!~] '@'
    => {
        !emit T_SYMBOL, ts + 1, ts + 2;
        fnext expr_end; fnbreak;
    };

    ':' bareword ambiguous_symbol_suffix
    => {
        !emit T_SYMBOL, ts + 1, tm;
        p = tm - 1;
        fnext expr_end; fnbreak;
    };

    ':' ( bareword | global_var | class_var | instance_var |
          operator_fname | operator_arithmetic | operator_rest )
    => {
      // emit(:tSYMBOL, tok(@ts + 1), @ts)
      !emit T_SYMBOL, ts + 1, te;
      fnext expr_end; fnbreak;
    };

    #
    # AMBIGUOUS TERNARY OPERATOR
    #

    # # Character constant, like ?a, ?\n, ?\u1000, and so on
    # # Don't accept \u escape with multiple codepoints, like \u{1 2 3}
    # '?' ( e_bs ( escape - ( '\u{' (xdigit+ [ \t]+)+ xdigit+ '}' ))
    #     | (c_any - c_space_nl - e_bs) % { @escape = nil }
    #     )
    # => {
    #   // value = @escape || tok(@ts + 1)
    #
    #   // if version?(18)
    #   //   emit(:tINTEGER, value.getbyte(0))
    #   // else
    #   //   emit(:tCHARACTER, value)
    #   // end
    # 
    #   // fnext expr_end; fbreak;
    # };

    # '?' c_space_nl
    # => {
    #   // escape = { " "  => '\s', "\r" => '\r', "\n" => '\n', "\t" => '\t',
    #   //             "\v" => '\v', "\f" => '\f' }[@source_buffer.slice(@ts + 1)]
    #   // diagnostic :warning, :invalid_escape_use, { :escape => escape }, range
    # 
    #   // p = @ts - 1
    #   // fgoto expr_end;
    # };

    '?' c_eof
    => {
        // diagnostic :fatal, :incomplete_escape, nil, range(@ts, @ts + 1)
        panic!("lexer diagnostic");
    };

    # f ?aa : b: Disambiguate with a character literal.
    '?' [A-Za-z_] bareword
    => {
      p = ts - 1;
      fgoto expr_end;
    };

    #
    # KEYWORDS AND PUNCTUATION
    #

    # a({b=>c})
    e_lbrace
    => {
        panic!("WIP");
        // if @lambda_stack.last == @paren_nest
        //   @lambda_stack.pop
        //   emit(:tLAMBEG, '{'.freeze)
        // else
        //   emit(:tLBRACE, '{'.freeze)
        // end
        // fbreak;
    };

    # a([1, 2])
    e_lbrack
    => {
        !emit T_LBRACK_;
        fnbreak;
    };

    # a()
    e_lparen
    => {
        !emit T_LPAREN_;
        fnbreak;
    };

    # a(+b)
    punctuation_begin
    => {
        !emit_table PUNCTUATION_BEGIN;
        fnbreak;
    };

    # # rescue Exception => e: Block rescue.
    # # Special because it should transition to expr_mid.
    # 'rescue' %{ tm = p } '=>'?
    # => {
    #     // emit(:kRESCUE, 'rescue'.freeze, @ts, tm)
    #     // p = tm - 1
    #     // fnext expr_mid; fbreak;
    # };

    # # if a: Statement if.
    # keyword_modifier
    # => {
    #     // emit_table(KEYWORDS_BEGIN)
    #     // fnext expr_value; fbreak;
    # };

    #
    # RUBY 1.9 HASH LABELS
    #

    # label ( any - ':' )
    # => {
    #   // fhold;
    #
    #   // if version?(18)
    #   //   ident = tok(@ts, @te - 2)
    #
    #   //   emit((@source_buffer.slice(@ts) =~ /[A-Z]/) ? :tCONSTANT : :tIDENTIFIER,
    #   //         ident, @ts, @te - 2)
    #   //   fhold; # continue as a symbol
    #
    #   //   if !@static_env.nil? && @static_env.declared?(ident)
    #   //     fnext expr_end;
    #   //   else
    #   //     fnext *arg_or_cmdarg;
    #   //   end
    #   // else
    #   //   emit(:tLABEL, tok(@ts, @te - 2), @ts, @te - 1)
    #   //   fnext expr_labelarg;
    #   // end
    #
    #   // fbreak;
    # };

    #
    # CONTEXT-DEPENDENT VARIABLE LOOKUP OR COMMAND INVOCATION
    #

    # foo= bar:  Disambiguate with bareword rule below.
    bareword ambiguous_ident_suffix |
    # def foo:   Disambiguate with bareword rule below.
    keyword
    => {
        p = ts - 1;
        fgoto expr_end;
    };

    # a = 42;     a [42]: Indexing.
    # def a; end; a [42]: Array argument.
    call_or_var
    => local_ident;

    # (call_or_var - keyword)
    #   % { ident_tok = tok; ident_ts = @ts; ident_te = @te; }
    # w_space+ '('
    # => {
    #   // emit(:tIDENTIFIER, ident_tok, ident_ts, ident_te)
    #   // p = ident_te - 1
    #
    #   // if !@static_env.nil? && @static_env.declared?(ident_tok) && @version < 25
    #   //   fnext expr_endfn;
    #   // else
    #   //   fnext expr_cmdarg;
    #   // end
    #   // fbreak;
    # };

    #
    # WHITESPACE
    #

    w_any;

    # e_heredoc_nl '=begin' ( c_space | c_nl_zlen )
    # => {
    #   // p = @ts - 1
    #   // @cs_before_block_comment = @cs
    #   // fgoto line_begin;
    # };

    #
    # DEFAULT TRANSITION
    #

    # The following rules match most binary and all unary operators.
    # Rules for binary operators provide better error reporting.
    operator_arithmetic '='    |
    operator_rest              |
    punctuation_end            |
    c_any
    => {
        p = ts - 1; fgoto expr_end;
    };

    c_eof => do_eof;
*|;
}%%
