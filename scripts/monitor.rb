#!/usr/bin/env ruby

require 'bundler/inline'
require_relative 'transform_lexer.rb'
require_relative 'build_parser.rb'

gemfile do
  source 'https://rubygems.org'
  gem 'listen', '~> 3.0'
end

# lexer
lexer_listener = Listen.to './src/lexer', only: /\.rs\.rl$/, ignore: [/tmp/], latency: 1 do |modified, added, removed|
  puts "modified absolute path: #{modified}"
  puts "added absolute path: #{added}"
  puts "removed absolute path: #{removed}"

  puts 'rebuilding lexer...'

  # transform syntax sugars
  puts 'transforming lexer...'
  transform_lexer!

  # build lexer via ragel
  puts `../ragel-dest/bin/ragel-rust ./src/lexer/tmp/lexer.rs.rl -o ./src/lexer/lexer.rs`

  # TODO HACK public lexer state values
  lexer_content = File.read './src/lexer/lexer.rs'
  File.open('./src/lexer/lexer.rs', 'w'){ |f| f.write ( lexer_content.gsub! /^static lexer_en_/, 'pub static lexer_en_' ) }

  puts 'lexer rebuilt'
end

# parser
parser_listener = Listen.to './src/parser', only: /\.g$/, latency: 1 do |modified, added, removed|
  puts 'rebuilding parser...'

  build_parser!

  puts 'parser rebuilt'
end

lexer_listener.start
parser_listener.start
sleep
