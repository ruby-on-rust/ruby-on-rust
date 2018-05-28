#!/usr/bin/env ruby

`syntax-cli -g src/parser/parser.g -m LALR1 -o src/parser/parser.rs`

parser_file = 'src/parser/parser.rs'
content = File.read parser_file

content.gsub! """
extern crate regex;

#[macro_use]
extern crate lazy_static;
""", ''

content.gsub! /(^\/\*\*$\n^ \* Generic tokenizer used by the parser in the Syntax tool)(.*)(^\/\/ Parser\.)/m, ''

File.open parser_file, "w" do |file|
  file.puts content
end
