#!/usr/bin/env ruby

require 'bundler/inline'

gemfile do
  source 'https://rubygems.org'
  gem 'listen', '~> 3.0'
end

# lexer
listener = Listen.to './src/lexer', ignore: [/lexer\.rs$/], latency: 2 do |modified, added, removed|
  # puts "modified absolute path: #{modified}"
  # puts "added absolute path: #{added}"
  # puts "removed absolute path: #{removed}"

  puts 'rebuilding lexer...'

  # build lexer via remote ragel
  puts `scp ./src/lexer/lexer.rs.rl ragel-builder:/root/ragel/lexer/`
  puts `scp ./src/lexer/_*.rs.rl ragel-builder:/root/ragel/lexer/`
  puts `ssh ragel-builder "/root/ragel/dist/ragel-7.0.0.11/bin/ragel-rust /root/ragel/lexer/lexer.rs.rl -o /root/ragel/lexer/lexer.rs"`
  puts `scp ragel-builder:/root/ragel/lexer/lexer.rs ./src/lexer/lexer.rs`

  # transform syntax sugars
  puts 'transforming lexer...'
  require './builder/transform_lexer.rb'

  puts 'lexer rebuilt'
end

listener.start
sleep
