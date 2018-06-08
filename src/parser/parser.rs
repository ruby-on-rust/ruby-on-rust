#![allow(dead_code)]
#![allow(unused_mut)]

use regex::Regex;
use std::collections::HashMap;

/**
 * Stack value.
 */

#[derive(Debug)]
enum SV {

    Undefined,
    _0(Token),
    _1(Node)
}

/**
 * Lex rules.
 */
static LEX_RULES: [&'static str; 0] = [
    
];

/**
 * EOF value.
 */
static EOF: &'static str = "$";

/**
 * A macro for map literals.
 *
 * hashmap!{ 1 => "one", 2 => "two" };
 */
macro_rules! hashmap(
    { $($key:expr => $value:expr),+ } => {
        {
            let mut m = ::std::collections::HashMap::new();
            $(
                m.insert($key, $value);
            )+
            m
        }
     };
);

/**
 * Unwraps a SV for the result. The result type is known from the grammar.
 */
macro_rules! get_result {
    ($r:expr, $ty:ident) => (match $r { SV::$ty(v) => v, _ => unreachable!() });
}

/**
 * Pops a SV with needed enum value.
 */
macro_rules! pop {
    ($s:expr, $ty:ident) => (get_result!($s.pop().unwrap(), $ty));
}

/**
 * Productions data.
 *
 * 0 - encoded non-terminal, 1 - length of RHS to pop from the stack
 */
static PRODUCTIONS : [[i32; 2]; 39] = [
    [-1, 1],
    [0, 1],
    [1, 2],
    [2, 0],
    [2, 1],
    [3, 1],
    [4, 1],
    [5, 1],
    [6, 1],
    [7, 1],
    [8, 0],
    [8, 2],
    [9, 1],
    [9, 3],
    [10, 1],
    [10, 1],
    [10, 1],
    [10, 3],
    [11, 1],
    [12, 1],
    [13, 1],
    [14, 3],
    [14, 1],
    [15, 0],
    [15, 2],
    [16, 1],
    [17, 1],
    [18, 1],
    [19, 1],
    [19, 1],
    [19, 1],
    [20, 1],
    [21, 0],
    [21, 1],
    [22, 0],
    [22, 1],
    [22, 1],
    [23, 1],
    [24, 1]
];

/**
 * Table entry.
 */
enum TE {
    Accept,

    // Shift, and transit to the state.
    Shift(usize),

    // Reduce by a production number.
    Reduce(usize),

    // Simple state transition.
    Transit(usize),
}

lazy_static! {
    /**
     * Lexical rules grouped by lexer state (by start condition).
     */
    static ref LEX_RULES_BY_START_CONDITIONS: HashMap<&'static str, Vec<i32>> = hashmap! { "INITIAL" => vec! [  ] };

    /**
     * Maps a string name of a token type to its encoded number (the first
     * token number starts after all numbers for non-terminal).
     */
    static ref TOKENS_MAP: HashMap<&'static str, i32> = hashmap! { "tCOMMA" => 25, "tLBRACK" => 26, "tRBRACK" => 27, "tSTRING_BEG" => 28, "tSTRING_END" => 29, "tSTRING" => 30, "tSTRING_CONTENT" => 31, "tINTEGER" => 32, "kNIL" => 33, "kTRUE" => 34, "kFALSE" => 35, "tNL" => 36, "$" => 37 };

    /**
     * Parsing table.
     *
     * Vector index is the state number, value is a map
     * from an encoded symbol to table entry (TE).
     */
    static ref TABLE: Vec<HashMap<i32, TE>>= vec![
    hashmap! { 0 => TE::Transit(1), 1 => TE::Transit(2), 2 => TE::Transit(3), 3 => TE::Transit(4), 4 => TE::Transit(5), 5 => TE::Transit(6), 6 => TE::Transit(7), 10 => TE::Transit(8), 11 => TE::Transit(9), 12 => TE::Transit(10), 13 => TE::Transit(16), 14 => TE::Transit(17), 17 => TE::Transit(13), 18 => TE::Transit(14), 19 => TE::Transit(20), 20 => TE::Transit(11), 26 => TE::Shift(12), 28 => TE::Shift(18), 30 => TE::Shift(19), 32 => TE::Shift(15), 33 => TE::Shift(21), 34 => TE::Shift(22), 35 => TE::Shift(23), 36 => TE::Reduce(3), 37 => TE::Reduce(3) },
    hashmap! { 37 => TE::Accept },
    hashmap! { 37 => TE::Reduce(1) },
    hashmap! { 21 => TE::Transit(24), 23 => TE::Transit(26), 24 => TE::Transit(25), 36 => TE::Shift(27), 37 => TE::Reduce(32) },
    hashmap! { 36 => TE::Reduce(4), 37 => TE::Reduce(4) },
    hashmap! { 36 => TE::Reduce(5), 37 => TE::Reduce(5) },
    hashmap! { 36 => TE::Reduce(6), 37 => TE::Reduce(6) },
    hashmap! { 36 => TE::Reduce(7), 37 => TE::Reduce(7) },
    hashmap! { 25 => TE::Reduce(8), 27 => TE::Reduce(8), 36 => TE::Reduce(8), 37 => TE::Reduce(8) },
    hashmap! { 25 => TE::Reduce(14), 27 => TE::Reduce(14), 36 => TE::Reduce(14), 37 => TE::Reduce(14) },
    hashmap! { 25 => TE::Reduce(15), 27 => TE::Reduce(15), 36 => TE::Reduce(15), 37 => TE::Reduce(15) },
    hashmap! { 25 => TE::Reduce(16), 27 => TE::Reduce(16), 36 => TE::Reduce(16), 37 => TE::Reduce(16) },
    hashmap! { 6 => TE::Transit(31), 7 => TE::Transit(30), 8 => TE::Transit(28), 9 => TE::Transit(29), 10 => TE::Transit(8), 11 => TE::Transit(9), 12 => TE::Transit(10), 13 => TE::Transit(16), 14 => TE::Transit(17), 17 => TE::Transit(13), 18 => TE::Transit(14), 19 => TE::Transit(20), 20 => TE::Transit(11), 26 => TE::Shift(12), 27 => TE::Reduce(10), 28 => TE::Shift(18), 30 => TE::Shift(19), 32 => TE::Shift(15), 33 => TE::Shift(21), 34 => TE::Shift(22), 35 => TE::Shift(23) },
    hashmap! { 25 => TE::Reduce(18), 27 => TE::Reduce(18), 36 => TE::Reduce(18), 37 => TE::Reduce(18) },
    hashmap! { 25 => TE::Reduce(26), 27 => TE::Reduce(26), 36 => TE::Reduce(26), 37 => TE::Reduce(26) },
    hashmap! { 25 => TE::Reduce(27), 27 => TE::Reduce(27), 36 => TE::Reduce(27), 37 => TE::Reduce(27) },
    hashmap! { 25 => TE::Reduce(19), 27 => TE::Reduce(19), 36 => TE::Reduce(19), 37 => TE::Reduce(19) },
    hashmap! { 25 => TE::Reduce(20), 27 => TE::Reduce(20), 36 => TE::Reduce(20), 37 => TE::Reduce(20) },
    hashmap! { 15 => TE::Transit(37), 29 => TE::Reduce(23), 31 => TE::Reduce(23) },
    hashmap! { 25 => TE::Reduce(22), 27 => TE::Reduce(22), 36 => TE::Reduce(22), 37 => TE::Reduce(22) },
    hashmap! { 25 => TE::Reduce(31), 27 => TE::Reduce(31), 36 => TE::Reduce(31), 37 => TE::Reduce(31) },
    hashmap! { 25 => TE::Reduce(28), 27 => TE::Reduce(28), 36 => TE::Reduce(28), 37 => TE::Reduce(28) },
    hashmap! { 25 => TE::Reduce(29), 27 => TE::Reduce(29), 36 => TE::Reduce(29), 37 => TE::Reduce(29) },
    hashmap! { 25 => TE::Reduce(30), 27 => TE::Reduce(30), 36 => TE::Reduce(30), 37 => TE::Reduce(30) },
    hashmap! { 37 => TE::Reduce(2) },
    hashmap! { 37 => TE::Reduce(33) },
    hashmap! { 37 => TE::Reduce(38) },
    hashmap! { 37 => TE::Reduce(37) },
    hashmap! { 27 => TE::Shift(32) },
    hashmap! { 22 => TE::Transit(33), 25 => TE::Shift(34), 27 => TE::Reduce(34), 36 => TE::Shift(35) },
    hashmap! { 25 => TE::Reduce(12), 27 => TE::Reduce(12), 36 => TE::Reduce(12) },
    hashmap! { 25 => TE::Reduce(9), 27 => TE::Reduce(9), 36 => TE::Reduce(9) },
    hashmap! { 25 => TE::Reduce(17), 27 => TE::Reduce(17), 36 => TE::Reduce(17), 37 => TE::Reduce(17) },
    hashmap! { 27 => TE::Reduce(11) },
    hashmap! { 6 => TE::Transit(31), 7 => TE::Transit(36), 10 => TE::Transit(8), 11 => TE::Transit(9), 12 => TE::Transit(10), 13 => TE::Transit(16), 14 => TE::Transit(17), 17 => TE::Transit(13), 18 => TE::Transit(14), 19 => TE::Transit(20), 20 => TE::Transit(11), 26 => TE::Shift(12), 27 => TE::Reduce(36), 28 => TE::Shift(18), 30 => TE::Shift(19), 32 => TE::Shift(15), 33 => TE::Shift(21), 34 => TE::Shift(22), 35 => TE::Shift(23) },
    hashmap! { 27 => TE::Reduce(35) },
    hashmap! { 25 => TE::Reduce(13), 27 => TE::Reduce(13), 36 => TE::Reduce(13) },
    hashmap! { 16 => TE::Transit(39), 29 => TE::Shift(38), 31 => TE::Shift(40) },
    hashmap! { 25 => TE::Reduce(21), 27 => TE::Reduce(21), 36 => TE::Reduce(21), 37 => TE::Reduce(21) },
    hashmap! { 29 => TE::Reduce(24), 31 => TE::Reduce(24) },
    hashmap! { 29 => TE::Reduce(25), 31 => TE::Reduce(25) }
];
}

// ------------------------------------
// Module include prologue.
//
// Should include at least result type:
//
// type TResult = <...>;
//
// Can also include parsing hooks:
//
//   fn on_parse_begin(parser: &mut Parser, string: &'static str) {
//     ...
//   }
//
//   fn on_parse_begin(parser: &mut Parser, string: &'static str) {
//     ...
//   }
//

use parser::token::{ InteriorToken, Token };
use parser::tokenizer::Tokenizer;
use ast::node;
use ast::node::Node;

pub type TResult = Node;

// ---  end of Module include ---------



/**
 * Parser.
 */
pub struct Parser {
    /**
     * Parsing stack: semantic values.
     */
    values_stack: Vec<SV>,

    /**
     * Parsing stack: state numbers.
     */
    states_stack: Vec<usize>,

    /**
     * Tokenizer instance.
     */
    tokenizer: Tokenizer,

    /**
     * Semantic action handlers.
     */
    handlers: [fn(&mut Parser) -> SV; 39],
}

impl Parser {
    /**
     * Creates a new Parser instance.
     */
    pub fn new() -> Parser {
        Parser {
            // Stacks.
            values_stack: Vec::new(),
            states_stack: Vec::new(),

            tokenizer: Tokenizer::new(),

            handlers: [
    Parser::_handler0,
    Parser::_handler1,
    Parser::_handler2,
    Parser::_handler3,
    Parser::_handler4,
    Parser::_handler5,
    Parser::_handler6,
    Parser::_handler7,
    Parser::_handler8,
    Parser::_handler9,
    Parser::_handler10,
    Parser::_handler11,
    Parser::_handler12,
    Parser::_handler13,
    Parser::_handler14,
    Parser::_handler15,
    Parser::_handler16,
    Parser::_handler17,
    Parser::_handler18,
    Parser::_handler19,
    Parser::_handler20,
    Parser::_handler21,
    Parser::_handler22,
    Parser::_handler23,
    Parser::_handler24,
    Parser::_handler25,
    Parser::_handler26,
    Parser::_handler27,
    Parser::_handler28,
    Parser::_handler29,
    Parser::_handler30,
    Parser::_handler31,
    Parser::_handler32,
    Parser::_handler33,
    Parser::_handler34,
    Parser::_handler35,
    Parser::_handler36,
    Parser::_handler37,
    Parser::_handler38
],
        }
    }

    /**
     * Parses a string.
     */
    pub fn parse(&mut self, string: &'static str) -> TResult {
        

        // Initialize the tokenizer and the string.
        self.tokenizer.init_string(string);

        // Initialize the stacks.
        self.values_stack.clear();

        // Initial 0 state.
        self.states_stack.clear();
        self.states_stack.push(0);

        let mut token = self.tokenizer.get_next_token();
        let mut shifted_token = token.clone();

        loop {
            let state = *self.states_stack.last().unwrap();
            let column = token.kind;

            if !TABLE[state].contains_key(&column) {
                self.unexpected_token(&token);
                break;
            }

            let entry = &TABLE[state][&column];

            match entry {

                
                // Shift a token, go to state.

                // Shift a token, go to state.
                &TE::Shift(next_state) => {
                    println!("");
                    println!("*** PARSER: SHIFT!");
                
                    // Push token.
                    self.values_stack.push(SV::_0(token.clone()));
                
                    // Push next state number: "s5" -> 5
                    self.states_stack.push(next_state as usize);
                
                    shifted_token = token;
                    token = self.tokenizer.get_next_token();
                
                    println!("*** PARSER: shifted_token: {:?}", shifted_token);
                    println!("*** PARSER: next token: {:?}", token.value);
                    println!("*** PARSER: values_stack: {:?}", self.values_stack);
                },
                
                
                // Reduce by production.

                &TE::Reduce(production_number) => {
                    println!("");
                    println!("*** PARSER: REDUCE!");
    
                    let production = PRODUCTIONS[production_number];
    
                    // println!("production: {:?}", production);
    
                    self.tokenizer.yytext = shifted_token.value;
                    self.tokenizer.yyleng = shifted_token.value.len();
    
                    let mut rhs_length = production[1];
                    while rhs_length > 0 {
                        self.states_stack.pop();
                        rhs_length = rhs_length - 1;
                    }
    
                    // Call the handler, push result onto the stack.
                    let result_value = self.handlers[production_number](self);

                    println!("*** PARSER: handler: {:?}", production_number );
                    println!("*** PARSER: result_value: {:?}", result_value);
    
                    let previous_state = *self.states_stack.last().unwrap();
                    let symbol_to_reduce_with = production[0];
    
                    // Then push LHS onto the stack.
                    self.values_stack.push(result_value);
    
                    let next_state = match &TABLE[previous_state][&symbol_to_reduce_with] {
                        &TE::Transit(next_state) => next_state,
                        _ => unreachable!(),
                    };
    
                    self.states_stack.push(next_state);

                    println!("*** PARSER: values_stack: {:?}", self.values_stack);
                },

                // Accept the string.

                &TE::Accept => {
                    // Pop state number.
                    self.states_stack.pop();

                    // Pop the parsed value.
                    let parsed = self.values_stack.pop().unwrap();

                    if self.states_stack.len() != 1 ||
                        self.states_stack.pop().unwrap() != 0 ||
                        self.tokenizer.has_more_tokens() {
                        self.unexpected_token(&token);
                    }

                    let result = get_result!(parsed, _1);
                    
                    return result;
                },

                _ => unreachable!(),
            }
        }

        unreachable!();
    }

    fn unexpected_token(&mut self, token: &Token) {
        if token.value == EOF && !self.tokenizer.has_more_tokens() {
            self.unexpected_end_of_input();
        }

        self.tokenizer.panic_unexpected_token(
            token.value,
            token.start_line,
            token.start_column
        );
    }

    fn unexpected_end_of_input(&mut self) {
        panic!("\n\nUnexpected end of input.\n\n");
    }

    fn _handler0(&mut self) -> SV {
// Semantic values prologue.
let mut _1 = self.values_stack.pop().unwrap();

let __ = _1;
__
}


fn _handler1(&mut self) -> SV {

    println!("   *** PARSER: _handler1");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.
let mut _1 = self.values_stack.pop().unwrap();

let __ = _1;
__
}


fn _handler2(&mut self) -> SV {

    println!("   *** PARSER: _handler2");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.
self.values_stack.pop();
let mut _1 = pop!(self.values_stack, _1);

// TODO builder.compstmt
        let __ = _1;
SV::_1(__)
}


fn _handler3(&mut self) -> SV {

    println!("   *** PARSER: _handler3");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.
// self.values_stack.pop();

// TODO result = []
        let __ = Node::Dummy;
SV::_1(__)
}


fn _handler4(&mut self) -> SV {

    println!("   *** PARSER: _handler4");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.
let mut _1 = pop!(self.values_stack, _1);

// TODO [ val[0] ]
        let __ = _1;
SV::_1(__)
}


fn _handler5(&mut self) -> SV {

    println!("   *** PARSER: _handler5");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.
let mut _1 = self.values_stack.pop().unwrap();

let __ = _1;
__
}


fn _handler6(&mut self) -> SV {

    println!("   *** PARSER: _handler6");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.
let mut _1 = self.values_stack.pop().unwrap();

let __ = _1;
__
}


fn _handler7(&mut self) -> SV {

    println!("   *** PARSER: _handler7");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.
let mut _1 = self.values_stack.pop().unwrap();

let __ = _1;
__
}


fn _handler8(&mut self) -> SV {

    println!("   *** PARSER: _handler8");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.
let mut _1 = self.values_stack.pop().unwrap();

let __ = _1;
__
}


fn _handler9(&mut self) -> SV {

    println!("   *** PARSER: _handler9");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.
let mut _1 = self.values_stack.pop().unwrap();

let __ = _1;
__
}


fn _handler10(&mut self) -> SV {

    println!("   *** PARSER: _handler10");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.
// //self.values_stack.pop();


        // TODO shared macro
        let __ = Node::Nodes(vec![]);
SV::_1(__)
}


fn _handler11(&mut self) -> SV {

    println!("   *** PARSER: _handler11");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.
self.values_stack.pop();
let mut _1 = self.values_stack.pop().unwrap();

let __ = _1;
__
}


fn _handler12(&mut self) -> SV {

    println!("   *** PARSER: _handler12");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.
let mut _1 = pop!(self.values_stack, _1);

let __ = Node::Nodes(vec![_1]);
SV::_1(__)
}


fn _handler13(&mut self) -> SV {

    println!("   *** PARSER: _handler13");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.
let mut _3 = pop!(self.values_stack, _1);
let mut _2 = pop!(self.values_stack, _0);
let mut _1 = pop!(self.values_stack, _1);

// Node::Nodes, , Node

        let __;
        if let Node::Nodes(mut nodes) = _1 {
            nodes.push(_3);
            __ = Node::Nodes(nodes);
        } else {unreachable!();};
SV::_1(__)
}


fn _handler14(&mut self) -> SV {

    println!("   *** PARSER: _handler14");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.
let mut _1 = self.values_stack.pop().unwrap();

let __ = _1;
__
}


fn _handler15(&mut self) -> SV {

    println!("   *** PARSER: _handler15");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.
let mut _1 = self.values_stack.pop().unwrap();

let __ = _1;
__
}


fn _handler16(&mut self) -> SV {

    println!("   *** PARSER: _handler16");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.
let mut _1 = self.values_stack.pop().unwrap();

let __ = _1;
__
}


fn _handler17(&mut self) -> SV {

    println!("   *** PARSER: _handler17");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.
let mut _3 = pop!(self.values_stack, _0);
let mut _2 = pop!(self.values_stack, _1);
let mut _1 = pop!(self.values_stack, _0);

let __ = node::array(_2);
SV::_1(__)
}


fn _handler18(&mut self) -> SV {

    println!("   *** PARSER: _handler18");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.
let mut _1 = self.values_stack.pop().unwrap();

let __ = _1;
__
}


fn _handler19(&mut self) -> SV {

    println!("   *** PARSER: _handler19");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.
let mut _1 = pop!(self.values_stack, _1);

let __ = node::string_compose(_1);
SV::_1(__)
}


fn _handler20(&mut self) -> SV {

    println!("   *** PARSER: _handler20");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.
let mut _1 = pop!(self.values_stack, _1);

let __ = Node::Nodes(vec![_1]);
SV::_1(__)
}


fn _handler21(&mut self) -> SV {

    println!("   *** PARSER: _handler21");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.
let mut _3 = pop!(self.values_stack, _0);
let mut _2 = pop!(self.values_stack, _1);
let mut _1 = pop!(self.values_stack, _0);

let __ = node::string_compose(_2);
        // TODO dedent_string

        // let __;
        // if let Node::Nodes(n_strs) = _2 {
        //     __ = node::string_compose
        // } else { unreachable!(); };
SV::_1(__)
}


fn _handler22(&mut self) -> SV {

    println!("   *** PARSER: _handler22");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.
let mut _1 = pop!(self.values_stack, _0);

let __;
        if let box InteriorToken::T_STRING(string_value) = _1.interior_token {
            __ = Node::Str(string_value);
        } else { unreachable!(); }
        // TODO builder.dedent_string;
SV::_1(__)
}


fn _handler23(&mut self) -> SV {

    println!("   *** PARSER: _handler23");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.
// //self.values_stack.pop();


        let __ = Node::Nodes(vec![]);
SV::_1(__)
}


fn _handler24(&mut self) -> SV {

    println!("   *** PARSER: _handler24");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.
let mut _2 = pop!(self.values_stack, _1);
let mut _1 = pop!(self.values_stack, _1);

// string_contents: Node::Nodes
        // string_content: Node::Str

        let __;
        if let Node::Nodes(mut n_strs) = _1 {
            n_strs.push(_2);
            __ = Node::Nodes(n_strs);
        } else { unreachable!(); };
SV::_1(__)
}


fn _handler25(&mut self) -> SV {

    println!("   *** PARSER: _handler25");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.
let mut _1 = pop!(self.values_stack, _0);

let __;
        if let box InteriorToken::T_STRING_CONTENT(string_value) = _1.interior_token {
            __ = Node::Str(string_value);
        } else { unreachable!(); };
SV::_1(__)
}


fn _handler26(&mut self) -> SV {

    println!("   *** PARSER: _handler26");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.
let mut _1 = self.values_stack.pop().unwrap();

let __ = _1;
__
}


fn _handler27(&mut self) -> SV {

    println!("   *** PARSER: _handler27");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.
let mut _1 = self.values_stack.pop().unwrap();

let __;
        if let SV::_0(token) = _1 {
            if let box InteriorToken::T_INTEGER(value) = token.interior_token {
                __ = Node::Int(value);
            } else { unreachable!(); }
        } else { unreachable!(); };
SV::_1(__)
}


fn _handler28(&mut self) -> SV {

    println!("   *** PARSER: _handler28");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.
// self.values_stack.pop();

let __ = Node::Nil;
SV::_1(__)
}


fn _handler29(&mut self) -> SV {

    println!("   *** PARSER: _handler29");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.
// self.values_stack.pop();

let __ = Node::True;
SV::_1(__)
}


fn _handler30(&mut self) -> SV {

    println!("   *** PARSER: _handler30");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.
// self.values_stack.pop();

let __ = Node::False;
SV::_1(__)
}


fn _handler31(&mut self) -> SV {

    println!("   *** PARSER: _handler31");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.
let mut _1 = pop!(self.values_stack, _1);

// TODO so not proper, open an issue
        // or make a macro
        // TODO NOTE
        // this is a different from the other `so not proper` stuff
        // note about how to extract values
        // 
        // |_1:Node| means a `pop` and `unwrap`, so `_1` is already Node
        // 
        let __ = node::accessible(_1);
        // let __;
        // if let SV::_1(node) = _1 {
        //     __ = node::accessible(node);
        // } else { unreachable!(); };
SV::_1(__)
}


fn _handler32(&mut self) -> SV {

    println!("   *** PARSER: _handler32");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.
// self.values_stack.pop();

let __ = SV::Undefined;
__
}


fn _handler33(&mut self) -> SV {

    println!("   *** PARSER: _handler33");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.
let mut _1 = self.values_stack.pop().unwrap();

let __ = _1;
__
}


fn _handler34(&mut self) -> SV {

    println!("   *** PARSER: _handler34");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.
// self.values_stack.pop();

let __ = SV::Undefined;
__
}


fn _handler35(&mut self) -> SV {

    println!("   *** PARSER: _handler35");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.
let mut _1 = self.values_stack.pop().unwrap();

let __ = _1;
__
}


fn _handler36(&mut self) -> SV {

    println!("   *** PARSER: _handler36");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.
let mut _1 = self.values_stack.pop().unwrap();

let __ = _1;
__
}


fn _handler37(&mut self) -> SV {

    println!("   *** PARSER: _handler37");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.
let mut _1 = self.values_stack.pop().unwrap();

let __ = _1;
__
}


fn _handler38(&mut self) -> SV {

    println!("   *** PARSER: _handler38");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.
let mut _1 = self.values_stack.pop().unwrap();

let __ = _1;
__
}
}
