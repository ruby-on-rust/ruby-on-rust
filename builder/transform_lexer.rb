#!/usr/bin/env ruby

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
  # emit T_IDENTIFIER, token_start_offset, token_end_offset;
  # 
  #     for T_IDENTIFIER(ts+start_offset, te+start_offset)
  # 
  # emit T_IDENTIFIER; => emit T_IDENTIFIER, 0, 0;
  # 
  # TODO HACKING
  # for Token variant without value, like Token::T_LBRACK, use a trailing _
  # 
  #     emit T_LBRACK_
  # 
  lexer_rs_rl_content.gsub!(/!emit (\w+);/) do |match| "!emit #$1, 0, 0;" end
  lexer_rs_rl_content.gsub!(/!emit (\w+), (\d+), (\d+);/) do |match|
    variant = $1
    start_offset = $2
    end_offset = $3

    token = if variant.end_with? '_'
              "Token::#{variant.delete_suffix '_'}"
            else
              "Token::#{variant}(slice)"
            end

    """
    {
        let slice = self.input_slice(ts + #{start_offset}, te + #{end_offset});
    
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
        let slice = self.input_slice(ts, te);
        let token = self.current_slice_as_token_from_table(\"#{table}\", slice);
        self.emit(token);
    }
    """
  end


  # 
  # !fnext_stack_pop;
  # 
  #     !f_next_stack_pop;
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
  File.open(target_file_path, 'w') { |f| f.write lexer_rs_rl_content }
end
