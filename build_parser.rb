#!/usr/bin/env ruby

# TODO refine this file

puts "invoking generator..."

# puts "validating grammar..."
# puts `syntax-cli -g src/parser/parser.g -m LALR1 --validate`

# puts table = `syntax-cli -g src/parser/parser.g -m lalr1 -t -s first`
# File.write './src/parser/table', table

puts `syntax-cli -g src/parser/parser.g -m LALR1 -o src/parser/parser.rs`

puts "cleaning..."

parser_file = 'src/parser/parser.rs'
content = File.read parser_file

content.gsub! """
extern crate regex;

#[macro_use]
extern crate lazy_static;
""", ''

content.gsub! /(^\/\*\*$\n^ \* Generic tokenizer used by the parser in the Syntax tool)(.*)(^\/\/ Parser\.)/m, ''

#
# `let $$ =` -> `$$ =`
#
content.gsub! '<REMOVE THIS LET>let ', ''

#
# Debug info in parser
#

content.gsub! "enum SV {", """
#[derive(Debug)]
enum SV {
"""

content.gsub! /(\/\/ Shift a token, go to state\.)(.*)(\/\/ Reduce by production\.)/m, %q[
                // Shift a token, go to state.

                // Shift a token, go to state.
                &TE::Shift(next_state) => {
                    println!("");
                    println!("*** PARSER: SHIFT!");
                
                    // Push token.
                    self.values_stack.push(SV::_0(token));
                
                    // Push next state number: "s5" -> 5
                    self.states_stack.push(next_state as usize);
                
                    shifted_token = token;
                    token = self.tokenizer.get_next_token();
                
                    println!("*** PARSER: shifted_token: {:?}", shifted_token);
                    println!("*** PARSER: next token: {:?}", token.value);
                    println!("*** PARSER: values_stack: {:?}", self.values_stack);
                },
                
                // Reduce by production.
]

content.gsub! /(\/\/ Reduce by production\.)(.*)(\/\/ Accept the string\.)/m, %q[
                // Reduce by production.

                &TE::Reduce(production_number) => {
                    println!("");
                    println!("*** PARSER: REDUCE!");
    
                    let production = PRODUCTIONS[production_number];
    
                    // println!("production: {:?}", production);
    
                    self.tokenizer.yytext = shifted_token.value;
                    self.tokenizer.yyleng = shifted_token.value.len();
    
                    let mut rhs_length = production[1];
                    while rhs_length > 0 {
                        self.states_stack.pop();
                        rhs_length = rhs_length - 1;
                    }
    
                    // Call the handler, push result onto the stack.
                    let result_value = self.handlers[production_number](self);

                    println!("*** PARSER: handler: {:?}", production_number );
                    println!("*** PARSER: result_value: {:?}", result_value);
    
                    let previous_state = *self.states_stack.last().unwrap();
                    let symbol_to_reduce_with = production[0];
    
                    // Then push LHS onto the stack.
                    self.values_stack.push(result_value);
    
                    let next_state = match &TABLE[previous_state][&symbol_to_reduce_with] {
                        &TE::Transit(next_state) => next_state,
                        _ => unreachable!(),
                    };
    
                    self.states_stack.push(next_state);

                    println!("*** PARSER: values_stack: {:?}", self.values_stack);
                },

                // Accept the string.
]

#
# since we removed Copy trait from the original Token
#
content.gsub! 'let mut shifted_token = token;', 'let mut shifted_token = token.clone();'
content.gsub! 'self.values_stack.push(SV::_0(token));', 'self.values_stack.push(SV::_0(token.clone()));'

#
# debug info in handlers
#
(1..99).each do |i|
  content.gsub! "fn _handler#{i}(&mut self) -> SV {\n", %Q[
fn _handler#{i}(&mut self) -> SV {\n
    println!("   *** PARSER: _handler#{i}");
    println!("   values_stack: {:?}", self.values_stack);
  ]
end

# 
# parser: &'static str -> &str
# 
content.gsub! 'pub fn parse(&mut self, string: &\'static str) -> TResult {', 'pub fn parse(&mut self, string: &str) -> TResult {'

# 
# parser: return value for methods migrating WIP
# TODO CLEANUP
# 
content.gsub! "wip!();\n__\n", "wip!();\nSV::Undefined\n"

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
