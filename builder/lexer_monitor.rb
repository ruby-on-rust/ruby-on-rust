#!/usr/bin/env ruby

require 'bundler/inline'

gemfile do
  source 'https://rubygems.org'
  gem 'listen', '~> 3.0'
  # gem 'byebug'
end

# lexer
listener = Listen.to './src/lexer', only: /\.rs\.rl$/, latency: 5 do |modified, added, removed|
  # puts "modified absolute path: #{modified}"
  # puts "added absolute path: #{added}"
  # puts "removed absolute path: #{removed}"

  puts 'rebuilding lexer...'

  # transform syntax sugars
  puts 'transforming lexer...'
  require './builder/transform_lexer.rb'

  # build lexer via remote ragel
  puts `scp ./src/lexer/tmp/*.rs.rl ragel-builder:/root/ragel/lexer/`
  puts `ssh ragel-builder "/root/ragel/dist/ragel-7.0.0.11/bin/ragel-rust /root/ragel/lexer/lexer.rs.rl -o /root/ragel/lexer/lexer.rs"`
  puts `scp ragel-builder:/root/ragel/lexer/lexer.rs ./src/lexer/lexer.rs`

  # TODO HACK public lexer state values
  lexer_content = File.read './src/lexer/lexer.rs'
  File.open('./src/lexer/lexer.rs', 'w'){ |f| f.write ( lexer_content.gsub! /^static lexer_en_/, 'pub static lexer_en_' ) }

  puts 'lexer rebuilt'
end

listener.start
sleep
