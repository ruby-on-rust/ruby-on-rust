# based on https://github.com/whitequark/parser/blob/b3a0cd6be2f2d498c36cd5ae0dd39d9d25497c53/lib/parser/lexer.rl

require 'byebug'

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
require_relative '_tokens'
require_relative '_numeric'
require_relative '_escape_sequence'
require_relative '_string_and_heredoc'
require_relative '_interpolation'
require_relative '_whitespace'
require_relative '_expression'
# require_relative '_expr_variable.rb'
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

File.open './src/lexer/lexer.rs', 'w' do |f| f.write lexer_rs_rl_content end

# puts lexer
