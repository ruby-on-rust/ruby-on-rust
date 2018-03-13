// TODO INCOMPLETE
// TODO NOTE DOCS

// TODO handle binary encoding issues

// module Parser

//   class Lexer::Literal
//     attr_accessor :saved_herebody_s

// end

use lexer::Lexer;
use lexer::LexingState;
use parser::token::Token;

#[derive(Debug, Clone)]
pub struct Literal {
    nesting: usize,

    start_tok: Token,
    interpolate: bool,

    start_delim: Option<String>,
    end_delim: Option<String>,

    heredoc_e: Option<usize>,
    indent: bool,
    label_allowed: bool,

    dedent_body: bool,

    space_emitted: bool,

    monolithic: bool,

    // # String type. For :'foo', it is :'
    str_type: String,

    // # Start of the string type specifier.
    // starting point of the literal, normarlly related to @ts
    str_s: usize,

    interp_braces: usize,

    buffer: String,
    buffer_s: Option<usize>,
    buffer_e: Option<usize>,

    // TODO NOTE
    tokens_to_emit: Vec<Token>,

    is_words: bool,
}

impl Literal {
    //     def initialize(lexer, str_type, delimiter, str_s, heredoc_e = nil,
    //                    indent = false, dedent_body = false, label_allowed = false)
    // TODO NOTE
    // this fund includes tokens emitting (flush_string)
    // have to make sure emits those tokens after lexer called this function
    pub fn new(
        str_type: String,
        delimiter: String,
        str_s: usize,
        heredoc_e: Option<usize>, // TODO
        indent: bool,
        dedent_body: bool, // TODO
        label_allowed: bool
    ) -> Literal {
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

            ":'"  => ( Token::T_SYMBEG,       false ),
            "%s"  => ( Token::T_SYMBEG,       false ),
            ":\""  => ( Token::T_SYMBEG,       true  ),

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

            tokens_to_emit: vec![],

            is_words:   start_tok.clone() == Token::T_WORDS_BEG ||
                        start_tok.clone() == Token::T_QWORDS_BEG ||
                        start_tok.clone() == Token::T_SYMBOLS_BEG ||
                        start_tok.clone() == Token::T_QSYMBOLS_BEG,
        };

        println!("creating new literal: {:?}", literal.clone());

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
    fn is_heredoc(&self) -> bool { self.heredoc_e.is_some() }

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
    // NOTE
    // original method includes emitting token (and return the token),
    // now we will emit the returned token after invoking
    // and since we dont want to save lexer in Literal,
    // we have to maintain a `tokens_to_emit : Vec<Token>`,
    // 
    // this function return the final_token only
    // and after every time lexer called `nest_and_try_closing`, lexer have to
    // 1. call `literal.consume_tokens_to_emit` and emit every tokens in manually, and
    // 2. use final_token_to_emit just like the origin return value
    // 
    pub fn nest_and_try_closing(&mut self, delimiter: String, ts: usize, te: usize, lookahead: Option<String>) -> Option<Token> {
        // Some("") -> None
        let lookahead = if (lookahead.is_some() && !lookahead.clone().unwrap().is_empty()) { lookahead } else { None };

        println!("### invoking `nest_and_try_closing`, delimiter: {:?}", delimiter);
        println!("### lookahead: {:?}", lookahead);

        if self.start_delim.is_some() && self.start_delim.clone().unwrap() == delimiter {
            self.nesting += 1;
        } else {
            if self.is_delimiter(&delimiter) {
                self.nesting -= 1;
            }
        }

        println!("### self.nesting: {}", self.nesting);

        if self.nesting == 0 {
            // if words?
            //   extend_space(ts, ts)
            // end
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
                ( lookahead.clone().unwrap().chars().nth(0).unwrap() == ':' ) &&
                ( lookahead.clone().unwrap().chars().nth(1).unwrap() != ':' ) {
                    //   # This is a quoted label.
                    self.flush_string();

                    return Some(Token::T_LABEL_END);
            } else {
                if self.monolithic {
                    //   # Emit the string as a single token.
                    // let token = );
                    return Some(Token::T_STRING(self.buffer.clone()));
                } else {
                    //   # If this is a heredoc, @buffer contains the sentinel now.
                    //   # Just throw it out. Lexer flushes the heredoc after each
                    //   # non-heredoc-terminating \n anyway, so no data will be lost.
                    if !self.is_heredoc() {
                        self.flush_string();
                    }

                    return(Some(Token::T_STRING_END));
                }
            }
        }

        None
    }

    pub fn consume_tokens_to_emit(&mut self) -> Vec<Token> {
        let mut tokens_to_emit = vec![];

        loop {
            if self.tokens_to_emit.len() == 0 { break; }
            tokens_to_emit.push(self.tokens_to_emit.remove(0));
        }

        tokens_to_emit
    }

    //     def infer_indent_level(line)
    //       return if !@dedent_body

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
        self.interp_braces += 1;
    }

    //     def end_interp_brace_and_try_closing
    //       @interp_braces -= 1
    // 
    //       (@interp_braces == 0)
    //     end
    pub fn end_interp_brace_and_try_closing(&mut self) -> bool {
        self.interp_braces -= 1;
        self.interp_braces == 0
    }

    //     def extend_string(string, ts, te)
    //       @buffer_s ||= ts
    //       @buffer_e = te
    // 
    //       @buffer << string
    //     end
    pub fn extend_string(&mut self, string: String, ts: usize, te: usize) {
        if self.buffer_s.is_none() { self.buffer_s = Some(ts); }
        self.buffer_e = Some(te);

        self.buffer += &string;

        println!("invoked `extend_string`, now buffer: {:?}", self.buffer);
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
    // TODO NOTE FUNCTION
    // TODO NOTE
    // this fund includes tokens emitting (flush_string)
    // have to make sure emits those tokens after lexer called this function
    pub fn flush_string(&mut self) {
        if self.monolithic {
            self.emit_start_tok();
        }

        if !self.buffer.is_empty() {
            self.tokens_to_emit.push(Token::T_STRING_CONTENT(self.buffer.clone()));

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
    pub fn extend_space(&mut self, ts: usize, te: usize) {
        self.flush_string();
        if !self.space_emitted {
            self.tokens_to_emit.push(Token::T_SPACE);
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
    
        self.tokens_to_emit.push(token);
    }

    //     def emit(token, type, s, e)
    //       @lexer.send(:emit, token, type, s, e)
    //     end
    //   end
}

impl Lexer {
    //   def push_literal(*args)
    //     new_literal = Literal.new(self, *args)
    //     @literal_stack.push(new_literal)
    // 
    //     if new_literal.words? && new_literal.backslash_delimited?
    //       if new_literal.interpolate?
    //         self.class.lex_en_interp_backslash_delimited_words
    //       else
    //         self.class.lex_en_plain_backslash_delimited_words
    //       end
    //     elsif new_literal.words? && !new_literal.backslash_delimited?
    //       if new_literal.interpolate?
    //         self.class.lex_en_interp_words
    //       else
    //         self.class.lex_en_plain_words
    //       end
    //     elsif !new_literal.words? && new_literal.backslash_delimited?
    //       if new_literal.interpolate?
    //         self.class.lex_en_interp_backslash_delimited
    //       else
    //         self.class.lex_en_plain_backslash_delimited
    //       end
    //     else
    //       if new_literal.interpolate?
    //         self.class.lex_en_interp_string
    //       else
    //         self.class.lex_en_plain_string
    //       end
    //     end
    //   end
    // 
    // TODO INCOMPLETE
    // 
    // usually being called as `fgoto *push_literal`, implying it returns a state
    pub fn push_literal(&mut self, literal: Literal) -> LexingState {
        self.literal_stack.push(literal.clone());

        let literal = literal.clone();

        if literal.is_words && literal.is_backslash_delimited() {
            if literal.interpolate {
                return state!("interp_backslash_delimited_words");
            } else {
                return state!("plain_backslash_delimited_words");
            }
        }

        if literal.is_words && !literal.is_backslash_delimited() {
            if literal.interpolate {
                return state!("interp_words");
            } else {
                return state!("plain_words");
            }
        }

        if !literal.is_words && literal.is_backslash_delimited() {
            if literal.interpolate {
                return state!("interp_backslash_delimited");
            } else {
                return state!("plain_backslash_delimited");
            }
        }

        if literal.interpolate {
            return state!("interp_string");
        } else {
            return state!("plain_string");
        }
    }

    //   def literal
    //     @literal_stack.last
    //   end
    pub fn literal(&mut self) -> Option<&mut Literal> {
        self.literal_stack.last_mut()
    }

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
    pub fn pop_literal(&mut self) -> LexingState {
        let old_literal = self.literal_stack.pop().unwrap();
        state!("expr_endarg")
    }
}
