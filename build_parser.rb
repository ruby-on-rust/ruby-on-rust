#!/usr/bin/env ruby

# TODO refine this file

puts "invoking generator..."

# puts "validating grammar..."
# puts `syntax-cli -g src/parser/parser.g -m LALR1 --validate`

# puts table = `syntax-cli -g src/parser/parser.g -m lalr1 -t -s first`
# File.write './src/parser/table', table

syntax_cli_path = 'node ../syntax/dist/bin/syntax.js'

puts `#{syntax_cli_path} -g src/parser/parser.g -m LALR1 -o src/parser/parser.rs`

puts "cleaning..."

parser_file = 'src/parser/parser.rs'
content = File.read parser_file

# content.gsub! """
# extern crate regex;
# 
# #[macro_use]
# extern crate lazy_static;
# """, ''

content.gsub! /(^\/\*\*$\n^ \* Generic tokenizer used by the parser in the Syntax tool)(.*)(^\/\/ Parser\.)/m, ''

#
# unwrap interior token::Token from parser::Token
# 
# NOTE assume _0 being Token, this may change
#
content.gsub! 'pop!(self.values_stack, _0)', 'interior_token!(pop!(self.values_stack, _0))'

#
# since we removed Copy trait from the original Token
#
content.gsub! 'let mut shifted_token = token;', 'let mut shifted_token = token.clone();'
content.gsub! 'self.values_stack.push(SV::_0(token));', 'self.values_stack.push(SV::_0(token.clone()));'

# 
# parser: &'static str -> &str
# 
content.gsub! 'pub fn parse(&mut self, string: &\'static str) -> TResult {', 'pub fn parse(&mut self, string: &str) -> TResult {'

File.open parser_file, "w" do |file| file.puts content end

#
# tokens map
#
# NOTE
# { "tINTEGER" => 14, "tNL" => 15, "$" => 16 }
# to
# { "T_INTEGER" => 14, ... }

puts '---'
puts "Handling tokens map..."
original_map_str = File.read('src/parser/parser.rs').scan(/hashmap!\ \{.*\};/).last.delete_prefix('hashmap! ').delete_suffix(';')
original_map = eval original_map_str
pp original_map
tokens_map = original_map.transform_keys do |k|
  case
  when k.start_with?('t')
    'T_' + k.delete_prefix('t')
  when k.start_with?('k')
    'K_' + k.delete_prefix('k')
  when k == '$'
    k
  else
    raise "unreachable! don't know how to transform key `#{k}`"
  end
end

token_file = 'src/parser/token.rs'
content = File.read token_file
content.gsub! /(\/\/\ STARTING\ OF\ TOKENS_MAP)(.*?)(\/\/\ END\ OF\ TOKENS_MAP\n)/m, "// STARTING OF TOKENS_MAP\n" + "let tokens_map: HashMap<&str, isize> = hashmap! #{tokens_map.to_s};" + "\n// END OF TOKENS_MAP\n"
File.open token_file, "w" do |file| file.puts content end
