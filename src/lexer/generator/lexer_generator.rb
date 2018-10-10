# based on https://github.com/whitequark/parser/blob/b3a0cd6be2f2d498c36cd5ae0dd39d9d25497c53/lib/parser/lexer.rl

# require 'byebug'

$actions = {}
$machines = {}
$scanners = {}

require_relative 'action'
require_relative 'pattern'
require_relative 'scanner'

def p! *p
  Pattern.new *p
end

# 
# define a machine
# 
# machine is basically a pattern with a name
# 
# m name, *pattern_rules
# m :c_nl, '\n'
# m :c_nl, '[ \t]'
# m :name, :other_machine
# 
# m name, rule_1, rule_2 => rule_1 concat rule_2
# m name, rule_1, :-, rule_2 => rule_1 - rule_2
# m name, [rule_1, rule_2...] => rule_1 or rule_2 or ...
# 
def m! name, *p
  $machines[name] = p!(*p)
end

# 
# action
# 
def a! name, code
  Action.new code, name
end

# pre-defined actions
a! :nil, ''

require_relative '_character_classes'
require_relative '_token_definitions'
require_relative '_numeric'
require_relative '_escape_sequence'
require_relative '_string_and_heredoc'
require_relative '_interpolation'
require_relative '_whitespace'
require_relative '_expression'
require_relative '_expr_variable.rb'
# require_relative '_expr_fname.rb'
# require_relative '_expr_endfn.rb'
# require_relative '_expr_dot.rb'
# require_relative '_expr_arg.rb'
# require_relative '_expr_cmdarg.rb'
# require_relative '_expr_endarg.rb'
# require_relative '_expr_mid.rb'
require_relative '_expr_beg.rb'
# require_relative '_expr_labelarg.rb'
require_relative '_expr_value.rb'
require_relative '_expr_end.rb'
# require_relative '_leading_dot.rb'
# require_relative '_line_comment.rb'
require_relative '_line_begin.rb'

# puts info
puts 'machines:'
pp $machines

puts 'scanners:'
pp $scanners

# 
# write rust code for lexer#advance
# 

lexer_rs_rl_content = File.read './src/lexer/lexer.rl.rs'

lexer_rs_rl_content.gsub! "// %% write each scanners branch\n", $scanners.map{|name, scanner| scanner.code }.join

lexer_rs_rl_content.gsub! "// %% write matching action\n", """
              #{$actions.map{ |id, action|
                  """
              #{id} => #{action.code},
                  """
              }.join}
"""

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

lexer_rs_rl_content.gsub! "// %% write token tables matching\n", """
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

File.open './src/lexer/lexer.rs', 'w' do |f| f.write lexer_rs_rl_content end

# puts lexer
