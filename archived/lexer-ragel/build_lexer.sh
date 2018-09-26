#!/bin/bash

# 

scp ./src/lexer/lexer.rs.rl remote-machine:/root/ragel/lexer
# scp ./src/lexer/_*.rs.rl remote-machine:/root/ragel/lexer

# 

ssh remote-machine "/root/ragel/compiled-ragel/bin/ragel-rust /root/ragel/lexer/lexer.rs.rl -o /root/ragel/lexer/lexer.rs"

# 

scp remote-machine:/root/ragel/lexer/lexer.rs ./src/lexer/lexer.rs
