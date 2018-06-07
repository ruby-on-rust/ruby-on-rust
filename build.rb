#!/usr/bin/env ruby

puts "invoking generator..."

puts "validating grammar..."
puts `syntax-cli -g src/parser/parser.g -m LALR1 --validate`

puts `syntax-cli -g src/parser/parser.g -m lalr1 -t -s first`

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
# since we removed Copy trait from the original Token
#
content.gsub! 'let mut shifted_token = token;', 'let mut shifted_token = token.clone();'
content.gsub! 'self.values_stack.push(SV::_0(token));', 'self.values_stack.push(SV::_0(token.clone()));'

File.open parser_file, "w" do |file|
  file.puts content
end


#
# tokens map
#
# NOTE
# { "tINTEGER" => 14, "tNL" => 15, "$" => 16 }
# to
# { "T_INTEGER" => 14, ... }

original_map_str = File.read('src/parser/parser.rs').scan(/hashmap!\ \{.*\};/).last.delete_prefix('hashmap! ').delete_suffix(';')
original_map = eval original_map_str
tokens_map = original_map.transform_keys do |k|
  case
  when k.start_with?('t')
    'T_' + k.delete_prefix('t')
  when k.start_with?('k')
    'K_' + k.delete_prefix('k')
  when k == '$'
    k
  end
end

token_file = 'src/parser/token.rs'
content = File.read token_file
content.gsub! /(\/\/\ STARTING\ OF\ TOKENS_MAP)(.*?)(\/\/\ END\ OF\ TOKENS_MAP\n)/m, "// STARTING OF TOKENS_MAP\n" + "let tokens_map: HashMap<&str, isize> = hashmap! #{tokens_map.to_s};" + "\n// END OF TOKENS_MAP\n"
File.open token_file, "w" do |file| file.puts content end

# TODO

#[derive(Debug)]
# enum SV

# macro_rules! get_result {
#   ($r:expr, $ty:ident) => ({
#       println!("get_result!");
#       println!("$r: {:?}", stringify!($r) );
#       println!("$ty: {:?}", stringify!($ty) );
# 
#       // match $r {
#       //     SV::_1(v) => { panic!("matched _1"); },
#       //     _ => { println!("matched _"); panic!("a"); }
#       // }
# 
#       // let mut tester: () = $r;
#       let rr = $r;
#       match rr {
#           SV::$ty(v) => { v },
#           _ => {
#               println!("###$r {:?}", rr );
#               println!("###$ty: {:?}", stringify!($ty) );
#               unreachable!()
#           }
#       }
#   });
# }


# // Shift a token, go to state.
# &TE::Shift(next_state) => {
#     println!("*** SHIFT!");

#     // Push token.
#     self.values_stack.push(SV::_0(token));

#     // Push next state number: "s5" -> 5
#     self.states_stack.push(next_state as usize);

#     shifted_token = token;
#     token = self.tokenizer.get_next_token();

#     println!("shifted_token: {:?}", shifted_token.value);
#     println!("next token: {:?}", token.value);
# },

# // Reduce by production.
# &TE::Reduce(production_number) => {
#     println!("*** REDUCE!");

#     let production = PRODUCTIONS[production_number];

#     // println!("production: {:?}", production);

#     self.tokenizer.yytext = shifted_token.value;
#     self.tokenizer.yyleng = shifted_token.value.len();

#     // println!("shifted_token: {:?}", shifted_token.value);

#     let mut rhs_length = production[1];
#     while rhs_length > 0 {
#         self.states_stack.pop();
#         rhs_length = rhs_length - 1;
#     }

#     // Call the handler, push result onto the stack.
#     let result_value = self.handlers[production_number](self);
#     println!("handler: {:?}", production_number );
#     println!("result_value: {:?}", result_value);

#     let previous_state = *self.states_stack.last().unwrap();
#     let symbol_to_reduce_with = production[0];

#     // Then push LHS onto the stack.
#     self.values_stack.push(result_value);

#     let next_state = match &TABLE[previous_state][&symbol_to_reduce_with] {
#         &TE::Transit(next_state) => next_state,
#         _ => unreachable!(),
#     };

#     self.states_stack.push(next_state);
# },
