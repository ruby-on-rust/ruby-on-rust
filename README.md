# ruby-on-rust

An implementation of ruby in pure rust.

[![Build Status](https://travis-ci.com/ruby-on-rust/ruby-on-rust.svg?branch=master)](https://travis-ci.org/ruby-on-rust/ruby-on-rust)
[![codecov](https://codecov.io/gh/ruby-on-rust/ruby-on-rust/branch/master/graph/badge.svg)](https://codecov.io/gh/ruby-on-rust/ruby-on-rust)

# about

I started this project as a way to learn rust. Eventually, I've learnt more than that, including lexer & parser, ragel, and a lot of i-did-not-know-that-in-ruby tricks.

Lots of the grammar rules and AST conventions are ripped from the `parser` gem.

I crafted a runnable lexer and parser, interpreter is still a no-go.

For lexer, I adapated some rules from `parser` and ported from `ruby-on-ragel-6` to `rust-on-ragel-7`

For parser, I use `syntax-cli` as parser generator.
