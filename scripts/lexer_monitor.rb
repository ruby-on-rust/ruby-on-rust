#!/usr/bin/env ruby

require 'bundler/inline'
require_relative 'transform_lexer.rb'

gemfile do
  source 'https://rubygems.org'
  gem 'listen', '~> 3.0'
  gem 'byebug'
end

# lexer
listener = Listen.to './src/lexer', only: /\.rs\.rl$/, ignore: [/tmp/], latency: 5 do |modified, added, removed|
  puts "modified absolute path: #{modified}"
  puts "added absolute path: #{added}"
  puts "removed absolute path: #{removed}"

  puts 'rebuilding lexer...'

  # transform syntax sugars
  puts 'transforming lexer...'
  transform!

  # build lexer via ragel
  puts `../ragel-dest/bin/ragel-rust ./src/lexer/tmp/lexer.rs.rl -o ./src/lexer/lexer.rs`

  # TODO HACK public lexer state values
  lexer_content = File.read './src/lexer/lexer.rs'
  File.open('./src/lexer/lexer.rs', 'w'){ |f| f.write ( lexer_content.gsub! /^static lexer_en_/, 'pub static lexer_en_' ) }

  puts 'lexer rebuilt'
end

listener.start
sleep
