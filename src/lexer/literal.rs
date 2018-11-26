// TODO INCOMPLETE
// TODO NOTE DOCS

// TODO handle binary encoding issues

use std::rc::Rc;
use std::cell::RefCell;
use crate::lexer::lexer::*;
use crate::token::token::Token;

// TODO impl Debug manually
#[derive(Clone)]
pub struct Literal {
    nesting: i32,

    start_tok: Token,
    interpolate: bool,

    start_delim: Option<String>,
    end_delim: Option<String>,

    heredoc_e: Option<i32>,
    indent: bool,
    label_allowed: bool,

    dedent_body: bool,

    space_emitted: bool,

    monolithic: bool,

    // # String type. For :'foo', it is :'
    str_type: String,

    // # Start of the string type specifier.
    // starting point of the literal, normarlly related to @ts
    str_s: i32,

    interp_braces: i32,

    buffer: String,
    buffer_s: Option<i32>,
    buffer_e: Option<i32>,

    is_words: bool,

    lexer_tokens: Rc<RefCell<Vec<Token>>>,
}

impl Literal {
    //     def initialize(lexer, str_type, delimiter, str_s, heredoc_e = nil,
    //                    indent = false, dedent_body = false, label_allowed = false)
    pub fn new(
        str_type: String,
        delimiter: String,
        str_s: i32,
        heredoc_e: Option<i32>, // TODO
        indent: bool,
        dedent_body: bool, // TODO
        label_allowed: bool,
        lexer_tokens: Rc<RefCell<Vec<Token>>>,
    ) -> Literal {
        println!("creating new literal with: str_type: {:?}", str_type);

        // TODO
        //       # DELIMITERS and TYPES are hashes with keys encoded in binary.
        //       # Coerce incoming data to the same encoding.
        //       str_type     = coerce_encoding(str_type)
        //       delimiter    = coerce_encoding(delimiter)

        // TODO
        //       unless TYPES.include?(str_type)
        //         lexer.send(:diagnostic, :error, :unexpected_percent_str,
        //                    { :type => str_type }, @lexer.send(:range, str_s, str_s + 2))
        //       end

        // TODO
        //       @start_tok, @interpolate = TYPES[str_type]
        //       @start_delim = DELIMITERS.include?(delimiter) ? delimiter : nil
        //       @end_delim   = DELIMITERS.fetch(delimiter, delimiter)
        // 
        //       @heredoc_e     = heredoc_e
        //       @indent        = indent
        //       @label_allowed = label_allowed
        // 
        //       @dedent_body   = dedent_body
        //       @dedent_level  = nil
        // 
        //       @space_emitted = true
        // 
        //       # Monolithic strings are glued into a single token, e.g.
        //       # tSTRING_BEG tSTRING_CONTENT tSTRING_END -> tSTRING.
        //       @monolithic  = (@start_tok == :tSTRING_BEG  &&
        //                       %w(' ").include?(str_type) &&
        //                       !heredoc?)
        // 
        //       # Capture opening delimiter in percent-literals.
        //       @str_type += delimiter if @str_type.start_with?('%'.freeze)
        // 

        let (start_tok, interpolate) = match str_type.as_ref() {
            "'"   => ( Token::T_STRING_BEG,   false ),
            "<<'" => ( Token::T_STRING_BEG,   false ),
            "%q"  => ( Token::T_STRING_BEG,   false ),
            "\""   => ( Token::T_STRING_BEG,   true  ),
            "<<\"" => ( Token::T_STRING_BEG,   true  ),
            "%"   => ( Token::T_STRING_BEG,   true  ),
            "%Q"  => ( Token::T_STRING_BEG,   true  ),

            "%w"  => ( Token::T_QWORDS_BEG,   false ),
            "%W"  => ( Token::T_WORDS_BEG,    true  ),

            "%i"  => ( Token::T_QSYMBOLS_BEG, false ),
            "%I"  => ( Token::T_SYMBOLS_BEG,  true  ),

            // TODO
            // these seems not elegant
            // maybe we should consider make all Token variants contain a value
            ":'"  => ( Token::T_SYMBEG(String::from(":'")),       false ),
            "%s"  => ( Token::T_SYMBEG(String::from("%s")),       false ),
            ":\""  => ( Token::T_SYMBEG(String::from(":\"")),       true  ),

            "/"   => ( Token::T_REGEXP_BEG,   true  ),
            "%r"  => ( Token::T_REGEXP_BEG,   true  ),

            "%x"  => ( Token::T_XSTRING_BEG,  true  ),
            "`"   => ( Token::T_XSTRING_BEG,  true  ),
            "<<`" => ( Token::T_XSTRING_BEG,  true  ),

            _ => { panic!("unknown str_type"); }
        };

        //       # Monolithic strings are glued into a single token, e.g.
        //       # tSTRING_BEG tSTRING_CONTENT tSTRING_END -> tSTRING.
        //       @monolithic  = (@start_tok == :tSTRING_BEG  &&
        //                       %w(' ").include?(str_type) &&
        //                       !heredoc?)
        // TODO handle heredoc
        let monolithic = ( start_tok.clone() == Token::T_STRING_BEG && ( &str_type == "'" || &str_type == "\"" ) );

        let mut literal = Literal {
            nesting: 1,

            //       @start_tok, @interpolate = TYPES[str_type]
            start_tok: start_tok.clone(),
            interpolate,

            //       @start_delim = DELIMITERS.include?(delimiter) ? delimiter : nil
            start_delim: match delimiter.as_ref() {
                "(" | "[" | "{" | "<" => Some(delimiter.clone()),
                _ => None
            },
            //       @end_delim   = DELIMITERS.fetch(delimiter, delimiter)
            end_delim: match delimiter.as_ref() {
                "(" => Some(String::from(")")),
                "[" => Some(String::from("]")),
                "{" => Some(String::from("}}")),
                "<" => Some(String::from(">")),
                _ => Some(delimiter)
            },

            heredoc_e,
            indent,
            label_allowed,

            dedent_body,

            space_emitted: true,
            monolithic,

            str_type,
            str_s,

            interp_braces: 0,

            buffer: String::from(""),
            buffer_s: None,
            buffer_e: None,

            is_words:   start_tok.clone() == Token::T_WORDS_BEG ||
                        start_tok.clone() == Token::T_QWORDS_BEG ||
                        start_tok.clone() == Token::T_SYMBOLS_BEG ||
                        start_tok.clone() == Token::T_QSYMBOLS_BEG,

            lexer_tokens,
        };

        // println!("creating new literal: {:?}", literal.clone());

        // emit_start_tok unless @monolithic
        if !monolithic { literal.emit_start_tok(); }

        literal
    }

    //     def interpolate?
    //       @interpolate
    //     end
    // NOTE use self.interpolate instead

    //     def words?
    //       type == :tWORDS_BEG || type == :tQWORDS_BEG ||
    //         type == :tSYMBOLS_BEG || type == :tQSYMBOLS_BEG
    //     end
    // NOTE use self.is_words instead

    //     def regexp?
    //       type == :tREGEXP_BEG
    //     end

    //     def heredoc?
    //       !!@heredoc_e
    //     end
    pub fn is_heredoc(&self) -> bool { self.heredoc_e.is_some() }

    //     def backslash_delimited?
    //       @end_delim == '\\'.freeze
    //     end
    fn is_backslash_delimited(&self) -> bool { self.end_delim.clone().unwrap() == String::from(r"\") }

    //     def type
    //       @start_tok
    //     end

    //     def munge_escape?(character)
    //       character = coerce_encoding(character)

    //       if words? && character =~ /[ \t\v\r\f\n]/
    //         true
    //       else
    //         ['\\'.freeze, @start_delim, @end_delim].include?(character)
    //       end
    //     end

    //     def nest_and_try_closing(delimiter, ts, te, lookahead=nil)
    //       delimiter = coerce_encoding(delimiter)
    // 
    //       if @start_delim && @start_delim == delimiter
    //         @nesting += 1
    //       elsif delimiter?(delimiter)
    //         @nesting -= 1
    //       end
    // 
    //       # Finalize if last matching delimiter is closed.
    //       if @nesting == 0
    //         if words?
    //           extend_space(ts, ts)
    //         end
    // 
    //         if lookahead && @label_allowed && lookahead[0] == ?: &&
    //            lookahead[1] != ?: && @start_tok == :tSTRING_BEG
    //           # This is a quoted label.
    //           flush_string
    //           emit(:tLABEL_END, @end_delim, ts, te + 1)
    //         elsif @monolithic
    //           # Emit the string as a single token.
    //           emit(:tSTRING, @buffer, @str_s, te)
    //         else
    //           # If this is a heredoc, @buffer contains the sentinel now.
    //           # Just throw it out. Lexer flushes the heredoc after each
    //           # non-heredoc-terminating \n anyway, so no data will be lost.
    //           flush_string unless heredoc?
    // 
    //           emit(:tSTRING_END, @end_delim, ts, te)
    //         end
    //       end
    //     end
    // 
    // TODO INCOMPLETE
    // 
    // NOTE return the last emitted token
    pub fn nest_and_try_closing(&mut self, delimiter: &String, ts: i32, te: i32, lookahead: Option<String>) -> Option<Token> {
        // TODO is this still necessary?
        // Some("") -> None
        let lookahead = if (lookahead.is_some() && !lookahead.clone().unwrap().is_empty()) { lookahead } else { None };

        println!("### literal:nest_and_try_closing: invoking, delimiter: {:?}", delimiter);
        println!("### literal:nest_and_try_closing: lookahead: {:?}", lookahead);

        if self.start_delim.is_some() && self.start_delim.clone().unwrap() == *delimiter {
            self.nesting += 1;
        } else {
            if self.is_delimiter(&delimiter) {
                self.nesting -= 1;
            }
        }

        println!("### literal:nest_and_try_closing: self.nesting: {}", self.nesting);

        if self.nesting == 0 {
            if self.is_words {
                self.extend_space(ts, ts);
            }

            // if lookahead && @label_allowed && lookahead[0] == ?: &&
            //    lookahead[1] != ?: && @start_tok == :tSTRING_BEG
            //   # This is a quoted label.
            //   flush_string
            //   emit(:tLABEL_END, @end_delim, ts, te + 1)
            // elsif @monolithic
            //   # Emit the string as a single token.
            //   emit(:tSTRING, @buffer, @str_s, te)
            // else
            //   # If this is a heredoc, @buffer contains the sentinel now.
            //   # Just throw it out. Lexer flushes the heredoc after each
            //   # non-heredoc-terminating \n anyway, so no data will be lost.
            //   flush_string unless heredoc?
            //
            //   emit(:tSTRING_END, @end_delim, ts, te)
            // end
            if  lookahead.is_some() &&
                self.label_allowed &&
                ( lookahead.clone().unwrap().chars().nth(0).unwrap() == ':' ) && // TODO there must be a better way
                ( lookahead.clone().unwrap().chars().nth(1).unwrap() != ':' ) {
                    //   # This is a quoted label.
                    self.flush_string();

                    return Some(self.emit(Token::T_LABEL_END));
            } else {
                if self.monolithic {
                    //   # Emit the string as a single token.
                    let token = Token::T_STRING(self.buffer.clone());
                    return Some(self.emit(token));
                } else {
                    //   # If this is a heredoc, @buffer contains the sentinel now.
                    //   # Just throw it out. Lexer flushes the heredoc after each
                    //   # non-heredoc-terminating \n anyway, so no data will be lost.
                    if !self.is_heredoc() {
                        self.flush_string();
                    }

                    return Some(self.emit(Token::T_STRING_END));
                }
            }
        }

        None
    }

    //     def infer_indent_level(line)
    //       return if !@dedent_body
    // 
    //       indent_level = 0
    //       line.each_char do |char|
    //         case char
    //         when ?\s
    //           indent_level += 1
    //         when ?\t
    //           indent_level += (8 - indent_level % 8)
    //         else
    //           if @dedent_level.nil? || @dedent_level > indent_level
    //             @dedent_level = indent_level
    //           end
    //           break
    //         end
    //       end
    //     end

    //     def start_interp_brace
    //       @interp_braces += 1
    //     end
    pub fn start_interp_brace(&mut self) {
        println!("literal.start_interp_brace invoked");

        self.interp_braces += 1;
    }

    //     def end_interp_brace_and_try_closing
    //       @interp_braces -= 1
    // 
    //       (@interp_braces == 0)
    //     end
    pub fn end_interp_brace_and_try_closing(&mut self) -> bool {
        println!("literal:end_interp_brace_and_try_closing. self.interp_braces: {}", self.interp_braces);

        self.interp_braces -= 1;
        self.interp_braces == 0
    }

    //     def extend_string(string, ts, te)
    //       @buffer_s ||= ts
    //       @buffer_e = te
    // 
    //       @buffer << string
    //     end
    pub fn extend_string(&mut self, string: &String, ts: i32, te: i32) {
        println!("### literal: invoking literal.extend_string, string: {:?}, buffer: {:?}", string, self.buffer);

        if self.buffer_s.is_none() { self.buffer_s = Some(ts); }
        self.buffer_e = Some(te);

        self.buffer += string;

        println!("### literal: invoked literal.extend_string, now buffer: {:?}", self.buffer);
    }

    //     def flush_string
    //       if @monolithic
    //         emit_start_tok
    //         @monolithic = false
    //       end
    // 
    //       unless @buffer.empty?
    //         emit(:tSTRING_CONTENT, @buffer, @buffer_s, @buffer_e)
    // 
    //         clear_buffer
    //         extend_content
    //       end
    //     end
    pub fn flush_string(&mut self) {
        println!("literal.flush_string invoking...");

        if self.monolithic {
            self.emit_start_tok();
            self.monolithic = false;
        }

        if !self.buffer.is_empty() {
            let token = Token::T_STRING_CONTENT(self.buffer.clone());
            self.emit(token);

            self.clear_buffer();
            self.extend_content();
        }
    }

    //     def extend_content
    //       @space_emitted = false
    //     end
    pub fn extend_content(&mut self) {
        self.space_emitted = false;
    }

    // def extend_space(ts, te)
    //   flush_string
    // 
    //   unless @space_emitted
    //     emit(:tSPACE, nil, ts, te)
    // 
    //     @space_emitted = true
    //   end
    // end
    // 
    // NOTE
    // this fund includes tokens emitting (flush_string)
    // have to make sure emits those tokens after lexer called this function
    // 
    // TODO NOTE FUNCTION
    pub fn extend_space(&mut self, ts: i32, te: i32) {
        self.flush_string();
        if !self.space_emitted {
            self.emit(Token::T_SPACE);
            self.space_emitted = true;
        }
    }

    //     protected

    //     def delimiter?(delimiter)
    //       if @indent
    //         @end_delim == delimiter.lstrip
    //       else
    //         @end_delim == delimiter
    //       end
    //     end
    // TODO NOTE FUNCTION
    fn is_delimiter(&self, delimiter: &String) -> bool {
        if self.indent {
            return self.end_delim.is_some() && ( self.end_delim.clone().unwrap() == delimiter.clone().trim_left() );
        } else {
            return self.end_delim.is_some() && ( self.end_delim.clone().unwrap() == delimiter.clone() );
        }
    }

    //     def coerce_encoding(string)
    //       string.dup.force_encoding(Encoding::BINARY)
    //     end

    //     def clear_buffer
    //       @buffer = ''
    // 
    //       # Prime the buffer with lexer encoding; otherwise,
    //       # concatenation will produce varying results.
    //       @buffer.force_encoding(@lexer.source_buffer.source.encoding)
    // 
    //       @buffer_s = nil
    //       @buffer_e = nil
    //     end
    fn clear_buffer(&mut self) {
        println!("literal.clear_buffer invoking...");

        self.buffer = String::from("");

        self.buffer_s = None;
        self.buffer_e = None;
    }

    //     def emit_start_tok
    //       str_e = @heredoc_e || @str_s + @str_type.length
    //       emit(@start_tok, @str_type, @str_s, str_e)
    //     end
    fn emit_start_tok(&mut self) {
        // TODO DUMMY haven't handle heredoc_e

        // let str_e = self.str_s + self.str_type.len();
        let token = self.start_tok.clone();
    
        self.emit(token);
    }

    //     def emit(token, type, s, e)
    //       @lexer.send(:emit, token, type, s, e)
    //     end
    fn emit(&mut self, token: Token) -> Token {
        self.lexer_tokens.borrow_mut().push(token.clone());
        token
    }
}

impl Lexer {
    //   def push_literal(*args)
    //     new_literal = Literal.new(self, *args)
    //     @literal_stack.push(new_literal)
    //     next_state_for_literal(new_literal)
    //   end
    // NOTE returns next_state_for_literal
    pub fn push_literal(&mut self, literal: Literal) -> i32 {
        // println!("### literal: push_literal: invoked. literal: {:?}", literal);
        // TODO DEBUG INFO
        print!("push_literal invoking");

        let next_state = self.next_state_for_literal(&literal);
        print!("push_literal: next_state: {}", next_state);

        self.literal_stack.push(RefCell::new(literal));

        return next_state;
    }

    //   def next_state_for_literal(literal)
    //     if literal.words? && literal.backslash_delimited?
    //       if literal.interpolate?
    //         self.class.lex_en_interp_backslash_delimited_words
    //       else
    //         self.class.lex_en_plain_backslash_delimited_words
    //       end
    //     elsif literal.words? && !literal.backslash_delimited?
    //       if literal.interpolate?
    //         self.class.lex_en_interp_words
    //       else
    //         self.class.lex_en_plain_words
    //       end
    //     elsif !literal.words? && literal.backslash_delimited?
    //       if literal.interpolate?
    //         self.class.lex_en_interp_backslash_delimited
    //       else
    //         self.class.lex_en_plain_backslash_delimited
    //       end
    //     else
    //       if literal.interpolate?
    //         self.class.lex_en_interp_string
    //       else
    //         self.class.lex_en_plain_string
    //       end
    //     end
    //   end
    pub fn next_state_for_literal(&self, literal: &Literal) -> i32 {
        let mut next_state: i32;

        if literal.is_words {
            if literal.is_backslash_delimited() {
                if literal.interpolate {
                    next_state = lexer_en_interp_backslash_delimited_words;
                } else {
                    next_state = lexer_en_plain_backslash_delimited_words;
                }
            } else {
                if literal.interpolate {
                    next_state = lexer_en_interp_words;
                } else {
                    next_state = lexer_en_plain_words;
                }
            }
        } else {
            if literal.is_backslash_delimited() {
                if literal.interpolate {
                    next_state = lexer_en_interp_backslash_delimited;
                } else {
                    next_state = lexer_en_plain_backslash_delimited;
                }
            } else {
                if literal.interpolate {
                    next_state = lexer_en_interp_string;
                } else {
                    next_state = lexer_en_plain_string;
                }
            }
        }

        return next_state;
    }

    //   def literal
    //     @literal_stack.last
    //   end
    // pub fn literal(&mut self) -> Literal {
    //     *self.literal_stack.last().expect("no current literal").borrow_mut()
    // }

    //   def pop_literal
    //     old_literal = @literal_stack.pop
    // 
    //     @dedent_level = old_literal.dedent_level
    // 
    //     if old_literal.type == :tREGEXP_BEG
    //       # Fetch modifiers.
    //       self.class.lex_en_regexp_modifiers
    //     elsif @version < 24
    //       self.class.lex_en_expr_end
    //     else
    //       self.class.lex_en_expr_endarg
    //     end
    //   end
    // TODO DUMMY
    pub fn pop_literal(&mut self) -> i32 {
        println!("### literal: pop_literal: invoked");

        // let old_literal = self.literal_stack.pop().unwrap();
        // self.dedent_level = old_literal.dedent_level;

        // println!("### literal: literal_stack: {:?}", self.literal_stack);

        lexer_en_expr_endarg
    }
}
