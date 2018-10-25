# ripped from parser
# Mapping of strings to parser tokens.
TOKEN_TABLES = {
  PUNCTUATION: {
    '='   => :tEQL,     '&'   => :tAMPER2,  '|'   => :tPIPE,
    '!'   => :tBANG,    '^'   => :tCARET,   '+'   => :tPLUS,
    '-'   => :tMINUS,   '*'   => :tSTAR2,   '/'   => :tDIVIDE,
    '%'   => :tPERCENT, '~'   => :tTILDE,   ','   => :tCOMMA,
    ';'   => :tSEMI,    '.'   => :tDOT,     '..'  => :tDOT2,
    '...' => :tDOT3,    '['   => :tLBRACK2, ']'   => :tRBRACK,
    '('   => :tLPAREN2, ')'   => :tRPAREN,  '?'   => :tEH,
    ':'   => :tCOLON,   '&&'  => :tANDOP,   '||'  => :tOROP,
    '-@'  => :tUMINUS,  '+@'  => :tUPLUS,   '~@'  => :tTILDE,
    '**'  => :tPOW,     '->'  => :tLAMBDA,  '=~'  => :tMATCH,
    '!~'  => :tNMATCH,  '=='  => :tEQ,      '!='  => :tNEQ,
    '>'   => :tGT,      '>>'  => :tRSHFT,   '>='  => :tGEQ,
    '<'   => :tLT,      '<<'  => :tLSHFT,   '<='  => :tLEQ,
    '=>'  => :tASSOC,   '::'  => :tCOLON2,  '===' => :tEQQ,
    '<=>' => :tCMP,     '[]'  => :tAREF,    '[]=' => :tASET,
    '{'   => :tLCURLY,  '}'   => :tRCURLY,  '`'   => :tBACK_REF2,
    '!@'  => :tBANG,    '&.'  => :tANDDOT,
  },

  PUNCTUATION_BEGIN: {
    '&'   => :tAMPER,   '*'   => :tSTAR,    '**'  => :tDSTAR,
    '+'   => :tUPLUS,   '-'   => :tUMINUS,  '::'  => :tCOLON3,
    '('   => :tLPAREN,  '{'   => :tLBRACE,  '['   => :tLBRACK,
  },

  KEYWORDS: {
    'if'     => :kIF_MOD,      'unless'   => :kUNLESS_MOD,
    'while'  => :kWHILE_MOD,   'until'    => :kUNTIL_MOD,
    'rescue' => :kRESCUE_MOD,  'defined?' => :kDEFINED,
    'BEGIN'  => :klBEGIN,      'END'      => :klEND,
  },

  KEYWORDS_BEGIN: {
    'if'     => :kIF,          'unless'   => :kUNLESS,
    'while'  => :kWHILE,       'until'    => :kUNTIL,
    'rescue' => :kRESCUE,      'defined?' => :kDEFINED,
    'BEGIN'  => :klBEGIN,      'END'      => :klEND,
  }
}

%w(class module def undef begin end then elsif else ensure case when
    for break next redo retry in do return yield super self nil true
    false and or not alias __FILE__ __LINE__ __ENCODING__).each do |keyword|
  TOKEN_TABLES[:KEYWORDS_BEGIN][keyword] = TOKEN_TABLES[:KEYWORDS][keyword] = :"k#{keyword.upcase}"
end

def transform!
  # Dir['./src/lexer/tmp/*.rs.rl'].each do |f| File.delete f end
  Dir['./src/lexer/*.rs.rl'].each do |f|

    lexer_rs_rl_content = File.read f

    # 
    # token tables
    # 

    lexer_rs_rl_content.gsub! "!write token tables matching\n", """
      #{TOKEN_TABLES.map{|table_name, table_hash|
        """
      \"#{table_name}\" => {
        match current_slice.as_ref() {
          #{table_hash.map{|key, value|
            # :kIF => "K_If"
            variant = value.to_s.upcase
            variant = if variant.start_with? 'K_' # __FILE__, etc.
                        variant
                      else
                        variant[0] + '_' + variant.slice(1..-1)
                      end

            """
            \"#{key}\" => { return Token::#{variant}; },
            """
          }.join}
          _ => { panic!(\"unreachable! no tokens in table #{table_name}\"); }
        }
      },
        """
      }.join}
    """

    # emit
    # 
    #     !emit T_IDENTIFIER, ts, te;
    #     !emit T_IDENTIFIER; => emit T_IDENTIFIER, ts, te;
    # 
    # TODO HACKING
    # for Token variant without value, like Token::T_LBRACK, use a trailing _
    # 
    #     !emit T_LBRACK_;
    # 
    lexer_rs_rl_content.gsub!(/!emit ([^,;]+);$/m) do |match| "!emit #$1, ts, te;" end
    lexer_rs_rl_content.gsub!(/!emit ([^,;]+), ([^,;]+), ([^,;]+);$/m) do |match|
      variant = $1
      start_p = $2
      end_p = $3

      token = if variant.end_with? '_'
                "Token::#{variant.delete_suffix '_'}"
              else
                "Token::#{variant}(slice)"
              end

      """
      {
          let slice = self.current_slice(#{start_p}, te + #{end_p});
      
          let token = #{token};
          self.emit(token);
      }
      """
    end

    # emit_table KEYWORD
    lexer_rs_rl_content.gsub!(/!emit_table (\w+);/) do |match|
      table = $1

      """
      {
          let slice = self.current_slice(ts, te);
          let token = self.current_slice_as_token_from_table(\"#{table}\", slice);
          self.emit(token);
      }
      """
    end


    # 
    # !fnext_stack_pop;
    # 
    #     !fnext_stack_pop;
    # 
    #     {top -= 1;
    #     let _poped_next_state = stack[top as usize];
    #     fnext _poped_next_state;}
    # 
    # NOTE we're using this transformer instead of native rust fn,
    # since we can't update both self.top and local var top
    # 

    lexer_rs_rl_content.gsub!(/!fnext_stack_pop;/) do |match|

      """
      {
          top -= 1;
          let _poped_next_state = stack[top as usize];
          fnext *_poped_next_state;
      }
      """
    end

    target_file_path = f.gsub 'src/lexer/', 'src/lexer/tmp/'
    puts "writing #{target_file_path}..."
    File.open(target_file_path, 'w') { |f| f.write lexer_rs_rl_content }
  end
end
