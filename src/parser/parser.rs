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
    _1(Nodes),
    _2(Node)
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
static PRODUCTIONS : [[i32; 2]; 66] = [
    [-1, 1],
    [0, 1],
    [1, 2],
    [2, 0],
    [2, 1],
    [2, 3],
    [3, 1],
    [4, 1],
    [5, 1],
    [6, 1],
    [7, 3],
    [7, 1],
    [8, 1],
    [9, 0],
    [9, 2],
    [10, 1],
    [11, 1],
    [11, 3],
    [12, 1],
    [12, 1],
    [12, 1],
    [12, 1],
    [12, 1],
    [12, 2],
    [12, 3],
    [13, 1],
    [13, 1],
    [13, 1],
    [14, 1],
    [15, 1],
    [16, 3],
    [16, 1],
    [17, 3],
    [18, 0],
    [18, 3],
    [19, 1],
    [19, 2],
    [20, 3],
    [21, 0],
    [21, 3],
    [22, 0],
    [22, 2],
    [23, 0],
    [23, 2],
    [24, 1],
    [25, 1],
    [26, 3],
    [27, 1],
    [28, 1],
    [29, 1],
    [29, 1],
    [30, 1],
    [30, 1],
    [30, 1],
    [30, 1],
    [31, 1],
    [31, 1],
    [32, 0],
    [32, 1],
    [33, 0],
    [33, 1],
    [33, 1],
    [34, 1],
    [34, 1],
    [35, 1],
    [35, 2]
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
    static ref TOKENS_MAP: HashMap<&'static str, i32> = hashmap! { "tEQL" => 36, "tCOMMA" => 37, "tCOLON3" => 38, "tCONSTANT" => 39, "tLBRACK" => 40, "tRBRACK" => 41, "tSTRING_BEG" => 42, "tSTRING_END" => 43, "tSTRING" => 44, "tWORDS_BEG" => 45, "tSPACE" => 46, "tQWORDS_BEG" => 47, "tSTRING_CONTENT" => 48, "tSYMBOL" => 49, "tSYMBEG" => 50, "tINTEGER" => 51, "tIDENTIFIER" => 52, "tIVAR" => 53, "kNIL" => 54, "kSELF" => 55, "kTRUE" => 56, "kFALSE" => 57, "tNL" => 58, "tSEMI" => 59, "$" => 60 };

    /**
     * Parsing table.
     *
     * Vector index is the state number, value is a map
     * from an encoded symbol to table entry (TE).
     */
    static ref TABLE: Vec<HashMap<i32, TE>>= vec![
    hashmap! { 0 => TE::Transit(1), 1 => TE::Transit(2), 2 => TE::Transit(3), 3 => TE::Transit(4), 4 => TE::Transit(5), 5 => TE::Transit(6), 6 => TE::Transit(8), 7 => TE::Transit(7), 12 => TE::Transit(9), 13 => TE::Transit(13), 14 => TE::Transit(14), 15 => TE::Transit(27), 16 => TE::Transit(28), 17 => TE::Transit(15), 20 => TE::Transit(16), 25 => TE::Transit(21), 26 => TE::Transit(22), 27 => TE::Transit(20), 28 => TE::Transit(23), 29 => TE::Transit(10), 30 => TE::Transit(33), 31 => TE::Transit(17), 38 => TE::Shift(18), 40 => TE::Shift(19), 42 => TE::Shift(29), 44 => TE::Shift(30), 45 => TE::Shift(31), 47 => TE::Shift(32), 49 => TE::Shift(25), 50 => TE::Shift(26), 51 => TE::Shift(24), 52 => TE::Shift(11), 53 => TE::Shift(12), 54 => TE::Shift(34), 55 => TE::Shift(35), 56 => TE::Shift(36), 57 => TE::Shift(37), 58 => TE::Reduce(3), 59 => TE::Reduce(3), 60 => TE::Reduce(3) },
    hashmap! { 60 => TE::Accept },
    hashmap! { 60 => TE::Reduce(1) },
    hashmap! { 32 => TE::Transit(38), 34 => TE::Transit(40), 35 => TE::Transit(39), 58 => TE::Shift(42), 59 => TE::Shift(41), 60 => TE::Reduce(57) },
    hashmap! { 58 => TE::Reduce(4), 59 => TE::Reduce(4), 60 => TE::Reduce(4) },
    hashmap! { 58 => TE::Reduce(6), 59 => TE::Reduce(6), 60 => TE::Reduce(6) },
    hashmap! { 58 => TE::Reduce(7), 59 => TE::Reduce(7), 60 => TE::Reduce(7) },
    hashmap! { 58 => TE::Reduce(8), 59 => TE::Reduce(8), 60 => TE::Reduce(8) },
    hashmap! { 36 => TE::Shift(45) },
    hashmap! { 37 => TE::Reduce(11), 41 => TE::Reduce(11), 58 => TE::Reduce(11), 59 => TE::Reduce(11), 60 => TE::Reduce(11) },
    hashmap! { 36 => TE::Reduce(9), 37 => TE::Reduce(55), 41 => TE::Reduce(55), 58 => TE::Reduce(55), 59 => TE::Reduce(55), 60 => TE::Reduce(55) },
    hashmap! { 36 => TE::Reduce(49), 37 => TE::Reduce(49), 41 => TE::Reduce(49), 58 => TE::Reduce(49), 59 => TE::Reduce(49), 60 => TE::Reduce(49) },
    hashmap! { 36 => TE::Reduce(50), 37 => TE::Reduce(50), 41 => TE::Reduce(50), 58 => TE::Reduce(50), 59 => TE::Reduce(50), 60 => TE::Reduce(50) },
    hashmap! { 37 => TE::Reduce(18), 41 => TE::Reduce(18), 58 => TE::Reduce(18), 59 => TE::Reduce(18), 60 => TE::Reduce(18) },
    hashmap! { 37 => TE::Reduce(19), 41 => TE::Reduce(19), 58 => TE::Reduce(19), 59 => TE::Reduce(19), 60 => TE::Reduce(19) },
    hashmap! { 37 => TE::Reduce(20), 41 => TE::Reduce(20), 58 => TE::Reduce(20), 59 => TE::Reduce(20), 60 => TE::Reduce(20) },
    hashmap! { 37 => TE::Reduce(21), 41 => TE::Reduce(21), 58 => TE::Reduce(21), 59 => TE::Reduce(21), 60 => TE::Reduce(21) },
    hashmap! { 37 => TE::Reduce(22), 41 => TE::Reduce(22), 58 => TE::Reduce(22), 59 => TE::Reduce(22), 60 => TE::Reduce(22) },
    hashmap! { 39 => TE::Shift(48) },
    hashmap! { 6 => TE::Transit(8), 7 => TE::Transit(52), 8 => TE::Transit(51), 9 => TE::Transit(49), 11 => TE::Transit(50), 12 => TE::Transit(9), 13 => TE::Transit(13), 14 => TE::Transit(14), 15 => TE::Transit(27), 16 => TE::Transit(28), 17 => TE::Transit(15), 20 => TE::Transit(16), 25 => TE::Transit(21), 26 => TE::Transit(22), 27 => TE::Transit(20), 28 => TE::Transit(23), 29 => TE::Transit(10), 30 => TE::Transit(33), 31 => TE::Transit(17), 38 => TE::Shift(18), 40 => TE::Shift(19), 41 => TE::Reduce(13), 42 => TE::Shift(29), 44 => TE::Shift(30), 45 => TE::Shift(31), 47 => TE::Shift(32), 49 => TE::Shift(25), 50 => TE::Shift(26), 51 => TE::Shift(24), 52 => TE::Shift(11), 53 => TE::Shift(12), 54 => TE::Shift(34), 55 => TE::Shift(35), 56 => TE::Shift(36), 57 => TE::Shift(37) },
    hashmap! { 37 => TE::Reduce(25), 41 => TE::Reduce(25), 58 => TE::Reduce(25), 59 => TE::Reduce(25), 60 => TE::Reduce(25) },
    hashmap! { 37 => TE::Reduce(26), 41 => TE::Reduce(26), 58 => TE::Reduce(26), 59 => TE::Reduce(26), 60 => TE::Reduce(26) },
    hashmap! { 37 => TE::Reduce(27), 41 => TE::Reduce(27), 58 => TE::Reduce(27), 59 => TE::Reduce(27), 60 => TE::Reduce(27) },
    hashmap! { 37 => TE::Reduce(47), 41 => TE::Reduce(47), 58 => TE::Reduce(47), 59 => TE::Reduce(47), 60 => TE::Reduce(47) },
    hashmap! { 37 => TE::Reduce(48), 41 => TE::Reduce(48), 58 => TE::Reduce(48), 59 => TE::Reduce(48), 60 => TE::Reduce(48) },
    hashmap! { 37 => TE::Reduce(45), 41 => TE::Reduce(45), 58 => TE::Reduce(45), 59 => TE::Reduce(45), 60 => TE::Reduce(45) },
    hashmap! { 23 => TE::Transit(58), 43 => TE::Reduce(42), 48 => TE::Reduce(42) },
    hashmap! { 37 => TE::Reduce(28), 41 => TE::Reduce(28), 58 => TE::Reduce(28), 59 => TE::Reduce(28), 60 => TE::Reduce(28) },
    hashmap! { 37 => TE::Reduce(29), 41 => TE::Reduce(29), 58 => TE::Reduce(29), 59 => TE::Reduce(29), 60 => TE::Reduce(29) },
    hashmap! { 22 => TE::Transit(62), 43 => TE::Reduce(40), 48 => TE::Reduce(40) },
    hashmap! { 37 => TE::Reduce(31), 41 => TE::Reduce(31), 58 => TE::Reduce(31), 59 => TE::Reduce(31), 60 => TE::Reduce(31) },
    hashmap! { 18 => TE::Transit(65), 43 => TE::Reduce(33), 48 => TE::Reduce(33) },
    hashmap! { 21 => TE::Transit(71), 43 => TE::Reduce(38), 48 => TE::Reduce(38) },
    hashmap! { 37 => TE::Reduce(56), 41 => TE::Reduce(56), 58 => TE::Reduce(56), 59 => TE::Reduce(56), 60 => TE::Reduce(56) },
    hashmap! { 37 => TE::Reduce(51), 41 => TE::Reduce(51), 58 => TE::Reduce(51), 59 => TE::Reduce(51), 60 => TE::Reduce(51) },
    hashmap! { 37 => TE::Reduce(52), 41 => TE::Reduce(52), 58 => TE::Reduce(52), 59 => TE::Reduce(52), 60 => TE::Reduce(52) },
    hashmap! { 37 => TE::Reduce(53), 41 => TE::Reduce(53), 58 => TE::Reduce(53), 59 => TE::Reduce(53), 60 => TE::Reduce(53) },
    hashmap! { 37 => TE::Reduce(54), 41 => TE::Reduce(54), 58 => TE::Reduce(54), 59 => TE::Reduce(54), 60 => TE::Reduce(54) },
    hashmap! { 60 => TE::Reduce(2) },
    hashmap! { 3 => TE::Transit(43), 4 => TE::Transit(5), 5 => TE::Transit(6), 6 => TE::Transit(8), 7 => TE::Transit(7), 12 => TE::Transit(9), 13 => TE::Transit(13), 14 => TE::Transit(14), 15 => TE::Transit(27), 16 => TE::Transit(28), 17 => TE::Transit(15), 20 => TE::Transit(16), 25 => TE::Transit(21), 26 => TE::Transit(22), 27 => TE::Transit(20), 28 => TE::Transit(23), 29 => TE::Transit(10), 30 => TE::Transit(33), 31 => TE::Transit(17), 38 => TE::Shift(18), 40 => TE::Shift(19), 42 => TE::Shift(29), 44 => TE::Shift(30), 45 => TE::Shift(31), 47 => TE::Shift(32), 49 => TE::Shift(25), 50 => TE::Shift(26), 51 => TE::Shift(24), 52 => TE::Shift(11), 53 => TE::Shift(12), 54 => TE::Shift(34), 55 => TE::Shift(35), 56 => TE::Shift(36), 57 => TE::Shift(37), 59 => TE::Shift(44), 60 => TE::Reduce(58) },
    hashmap! { 38 => TE::Reduce(64), 40 => TE::Reduce(64), 42 => TE::Reduce(64), 44 => TE::Reduce(64), 45 => TE::Reduce(64), 47 => TE::Reduce(64), 49 => TE::Reduce(64), 50 => TE::Reduce(64), 51 => TE::Reduce(64), 52 => TE::Reduce(64), 53 => TE::Reduce(64), 54 => TE::Reduce(64), 55 => TE::Reduce(64), 56 => TE::Reduce(64), 57 => TE::Reduce(64), 59 => TE::Reduce(64), 60 => TE::Reduce(64) },
    hashmap! { 38 => TE::Reduce(62), 40 => TE::Reduce(62), 42 => TE::Reduce(62), 44 => TE::Reduce(62), 45 => TE::Reduce(62), 47 => TE::Reduce(62), 49 => TE::Reduce(62), 50 => TE::Reduce(62), 51 => TE::Reduce(62), 52 => TE::Reduce(62), 53 => TE::Reduce(62), 54 => TE::Reduce(62), 55 => TE::Reduce(62), 56 => TE::Reduce(62), 57 => TE::Reduce(62), 59 => TE::Reduce(62), 60 => TE::Reduce(62) },
    hashmap! { 38 => TE::Reduce(63), 40 => TE::Reduce(63), 42 => TE::Reduce(63), 44 => TE::Reduce(63), 45 => TE::Reduce(63), 47 => TE::Reduce(63), 49 => TE::Reduce(63), 50 => TE::Reduce(63), 51 => TE::Reduce(63), 52 => TE::Reduce(63), 53 => TE::Reduce(63), 54 => TE::Reduce(63), 55 => TE::Reduce(63), 56 => TE::Reduce(63), 57 => TE::Reduce(63), 59 => TE::Reduce(63), 60 => TE::Reduce(63) },
    hashmap! { 58 => TE::Reduce(5), 59 => TE::Reduce(5), 60 => TE::Reduce(5) },
    hashmap! { 38 => TE::Reduce(65), 40 => TE::Reduce(65), 42 => TE::Reduce(65), 44 => TE::Reduce(65), 45 => TE::Reduce(65), 47 => TE::Reduce(65), 49 => TE::Reduce(65), 50 => TE::Reduce(65), 51 => TE::Reduce(65), 52 => TE::Reduce(65), 53 => TE::Reduce(65), 54 => TE::Reduce(65), 55 => TE::Reduce(65), 56 => TE::Reduce(65), 57 => TE::Reduce(65), 59 => TE::Reduce(65), 60 => TE::Reduce(65) },
    hashmap! { 6 => TE::Transit(8), 7 => TE::Transit(47), 10 => TE::Transit(46), 12 => TE::Transit(9), 13 => TE::Transit(13), 14 => TE::Transit(14), 15 => TE::Transit(27), 16 => TE::Transit(28), 17 => TE::Transit(15), 20 => TE::Transit(16), 25 => TE::Transit(21), 26 => TE::Transit(22), 27 => TE::Transit(20), 28 => TE::Transit(23), 29 => TE::Transit(10), 30 => TE::Transit(33), 31 => TE::Transit(17), 38 => TE::Shift(18), 40 => TE::Shift(19), 42 => TE::Shift(29), 44 => TE::Shift(30), 45 => TE::Shift(31), 47 => TE::Shift(32), 49 => TE::Shift(25), 50 => TE::Shift(26), 51 => TE::Shift(24), 52 => TE::Shift(11), 53 => TE::Shift(12), 54 => TE::Shift(34), 55 => TE::Shift(35), 56 => TE::Shift(36), 57 => TE::Shift(37) },
    hashmap! { 37 => TE::Reduce(10), 41 => TE::Reduce(10), 58 => TE::Reduce(10), 59 => TE::Reduce(10), 60 => TE::Reduce(10) },
    hashmap! { 37 => TE::Reduce(15), 41 => TE::Reduce(15), 58 => TE::Reduce(15), 59 => TE::Reduce(15), 60 => TE::Reduce(15) },
    hashmap! { 37 => TE::Reduce(23), 41 => TE::Reduce(23), 58 => TE::Reduce(23), 59 => TE::Reduce(23), 60 => TE::Reduce(23) },
    hashmap! { 41 => TE::Shift(53) },
    hashmap! { 33 => TE::Transit(54), 37 => TE::Shift(55), 41 => TE::Reduce(59), 58 => TE::Shift(56) },
    hashmap! { 37 => TE::Reduce(16), 41 => TE::Reduce(16), 58 => TE::Reduce(16) },
    hashmap! { 37 => TE::Reduce(12), 41 => TE::Reduce(12), 58 => TE::Reduce(12) },
    hashmap! { 37 => TE::Reduce(24), 41 => TE::Reduce(24), 58 => TE::Reduce(24), 59 => TE::Reduce(24), 60 => TE::Reduce(24) },
    hashmap! { 41 => TE::Reduce(14) },
    hashmap! { 6 => TE::Transit(8), 7 => TE::Transit(52), 8 => TE::Transit(57), 12 => TE::Transit(9), 13 => TE::Transit(13), 14 => TE::Transit(14), 15 => TE::Transit(27), 16 => TE::Transit(28), 17 => TE::Transit(15), 20 => TE::Transit(16), 25 => TE::Transit(21), 26 => TE::Transit(22), 27 => TE::Transit(20), 28 => TE::Transit(23), 29 => TE::Transit(10), 30 => TE::Transit(33), 31 => TE::Transit(17), 38 => TE::Shift(18), 40 => TE::Shift(19), 41 => TE::Reduce(61), 42 => TE::Shift(29), 44 => TE::Shift(30), 45 => TE::Shift(31), 47 => TE::Shift(32), 49 => TE::Shift(25), 50 => TE::Shift(26), 51 => TE::Shift(24), 52 => TE::Shift(11), 53 => TE::Shift(12), 54 => TE::Shift(34), 55 => TE::Shift(35), 56 => TE::Shift(36), 57 => TE::Shift(37) },
    hashmap! { 41 => TE::Reduce(60) },
    hashmap! { 37 => TE::Reduce(17), 41 => TE::Reduce(17), 58 => TE::Reduce(17) },
    hashmap! { 24 => TE::Transit(60), 43 => TE::Shift(59), 48 => TE::Shift(61) },
    hashmap! { 37 => TE::Reduce(46), 41 => TE::Reduce(46), 58 => TE::Reduce(46), 59 => TE::Reduce(46), 60 => TE::Reduce(46) },
    hashmap! { 43 => TE::Reduce(43), 48 => TE::Reduce(43) },
    hashmap! { 43 => TE::Reduce(44), 46 => TE::Reduce(44), 48 => TE::Reduce(44) },
    hashmap! { 24 => TE::Transit(64), 43 => TE::Shift(63), 48 => TE::Shift(61) },
    hashmap! { 37 => TE::Reduce(30), 41 => TE::Reduce(30), 58 => TE::Reduce(30), 59 => TE::Reduce(30), 60 => TE::Reduce(30) },
    hashmap! { 43 => TE::Reduce(41), 48 => TE::Reduce(41) },
    hashmap! { 19 => TE::Transit(67), 24 => TE::Transit(68), 43 => TE::Shift(66), 48 => TE::Shift(61) },
    hashmap! { 37 => TE::Reduce(32), 41 => TE::Reduce(32), 58 => TE::Reduce(32), 59 => TE::Reduce(32), 60 => TE::Reduce(32) },
    hashmap! { 24 => TE::Transit(70), 46 => TE::Shift(69), 48 => TE::Shift(61) },
    hashmap! { 46 => TE::Reduce(35), 48 => TE::Reduce(35) },
    hashmap! { 43 => TE::Reduce(34), 48 => TE::Reduce(34) },
    hashmap! { 46 => TE::Reduce(36), 48 => TE::Reduce(36) },
    hashmap! { 43 => TE::Shift(72), 48 => TE::Shift(73) },
    hashmap! { 37 => TE::Reduce(37), 41 => TE::Reduce(37), 58 => TE::Reduce(37), 59 => TE::Reduce(37), 60 => TE::Reduce(37) },
    hashmap! { 46 => TE::Shift(74) },
    hashmap! { 43 => TE::Reduce(39), 48 => TE::Reduce(39) }
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
use ast::node::{ Node, Nodes };

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
    handlers: [fn(&mut Parser) -> SV; 66],
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
    Parser::_handler38,
    Parser::_handler39,
    Parser::_handler40,
    Parser::_handler41,
    Parser::_handler42,
    Parser::_handler43,
    Parser::_handler44,
    Parser::_handler45,
    Parser::_handler46,
    Parser::_handler47,
    Parser::_handler48,
    Parser::_handler49,
    Parser::_handler50,
    Parser::_handler51,
    Parser::_handler52,
    Parser::_handler53,
    Parser::_handler54,
    Parser::_handler55,
    Parser::_handler56,
    Parser::_handler57,
    Parser::_handler58,
    Parser::_handler59,
    Parser::_handler60,
    Parser::_handler61,
    Parser::_handler62,
    Parser::_handler63,
    Parser::_handler64,
    Parser::_handler65
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

                    let result = get_result!(parsed, _2);
                    
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

let __ = node::compstmt(_1);
SV::_2(__)
}


fn _handler3(&mut self) -> SV {

    println!("   *** PARSER: _handler3");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.


let __ = vec![];
SV::_1(__)
}


fn _handler4(&mut self) -> SV {

    println!("   *** PARSER: _handler4");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.
let mut _1 = pop!(self.values_stack, _2);

let __ = vec![_1];
SV::_1(__)
}


fn _handler5(&mut self) -> SV {

    println!("   *** PARSER: _handler5");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.
let mut _3 = pop!(self.values_stack, _2);
self.values_stack.pop();
let mut _1 = pop!(self.values_stack, _1);

let mut nodes = _1;
        nodes.push(_3);
        let __ = nodes;
SV::_1(__)
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
let mut _1 = pop!(self.values_stack, _2);

let __ = node::assignable(_1);
SV::_2(__)
}


fn _handler10(&mut self) -> SV {

    println!("   *** PARSER: _handler10");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.
let mut _3 = pop!(self.values_stack, _2);
let mut _2 = pop!(self.values_stack, _0);
let mut _1 = pop!(self.values_stack, _2);

let __ = node::assign(_1, *_2.interior_token, _3);
SV::_2(__)
}


fn _handler11(&mut self) -> SV {

    println!("   *** PARSER: _handler11");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.
let mut _1 = self.values_stack.pop().unwrap();

let __ = _1;
__
}


fn _handler12(&mut self) -> SV {

    println!("   *** PARSER: _handler12");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.
let mut _1 = self.values_stack.pop().unwrap();

let __ = _1;
__
}


fn _handler13(&mut self) -> SV {

    println!("   *** PARSER: _handler13");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.


// TODO shared macro
        let __ = Node::Nodes(vec![]);
SV::_2(__)
}


fn _handler14(&mut self) -> SV {

    println!("   *** PARSER: _handler14");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.
self.values_stack.pop();
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
let mut _1 = pop!(self.values_stack, _2);

let __ = Node::Nodes(vec![_1]);
SV::_2(__)
}


fn _handler17(&mut self) -> SV {

    println!("   *** PARSER: _handler17");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.
let mut _3 = pop!(self.values_stack, _2);
let mut _2 = pop!(self.values_stack, _0);
let mut _1 = pop!(self.values_stack, _2);

// Node::Nodes, , Node

        let __;
        if let Node::Nodes(mut nodes) = _1 {
            nodes.push(_3);
            __ = Node::Nodes(nodes);
        } else {unreachable!();};
SV::_2(__)
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
let mut _1 = self.values_stack.pop().unwrap();

let __ = _1;
__
}


fn _handler20(&mut self) -> SV {

    println!("   *** PARSER: _handler20");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.
let mut _1 = self.values_stack.pop().unwrap();

let __ = _1;
__
}


fn _handler21(&mut self) -> SV {

    println!("   *** PARSER: _handler21");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.
let mut _1 = self.values_stack.pop().unwrap();

let __ = _1;
__
}


fn _handler22(&mut self) -> SV {

    println!("   *** PARSER: _handler22");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.
let mut _1 = self.values_stack.pop().unwrap();

let __ = _1;
__
}


fn _handler23(&mut self) -> SV {

    println!("   *** PARSER: _handler23");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.
let mut _2 = pop!(self.values_stack, _0);
let mut _1 = pop!(self.values_stack, _0);

let __ = node::const_global(*_1.interior_token, *_2.interior_token);
SV::_2(__)
}


fn _handler24(&mut self) -> SV {

    println!("   *** PARSER: _handler24");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.
let mut _3 = pop!(self.values_stack, _0);
let mut _2 = pop!(self.values_stack, _2);
let mut _1 = pop!(self.values_stack, _0);

let __ = node::array(_2);
SV::_2(__)
}


fn _handler25(&mut self) -> SV {

    println!("   *** PARSER: _handler25");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.
let mut _1 = self.values_stack.pop().unwrap();

let __ = _1;
__
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

let __ = _1;
__
}


fn _handler28(&mut self) -> SV {

    println!("   *** PARSER: _handler28");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.
let mut _1 = pop!(self.values_stack, _2);

let __ = node::string_compose(_1);
SV::_2(__)
}


fn _handler29(&mut self) -> SV {

    println!("   *** PARSER: _handler29");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.
let mut _1 = pop!(self.values_stack, _2);

let __ = Node::Nodes(vec![_1]);
SV::_2(__)
}


fn _handler30(&mut self) -> SV {

    println!("   *** PARSER: _handler30");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.
let mut _3 = pop!(self.values_stack, _0);
let mut _2 = pop!(self.values_stack, _2);
let mut _1 = pop!(self.values_stack, _0);

let __ = node::string_compose(_2);
        // TODO dedent_string;
SV::_2(__)
}


fn _handler31(&mut self) -> SV {

    println!("   *** PARSER: _handler31");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.
let mut _1 = pop!(self.values_stack, _0);

let __;
        if let box InteriorToken::T_STRING(string_value) = _1.interior_token {
            __ = Node::Str(string_value);
        } else { unreachable!(); }
        // TODO builder.dedent_string;
SV::_2(__)
}


fn _handler32(&mut self) -> SV {

    println!("   *** PARSER: _handler32");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.
self.values_stack.pop();
let mut _2 = pop!(self.values_stack, _2);
self.values_stack.pop();

let __ = node::words_compose(_2);
SV::_2(__)
}


fn _handler33(&mut self) -> SV {

    println!("   *** PARSER: _handler33");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.


let __ = Node::Nodes(vec![]);
SV::_2(__)
}


fn _handler34(&mut self) -> SV {

    println!("   *** PARSER: _handler34");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.
let mut _3 = pop!(self.values_stack, _0);
let mut _2 = pop!(self.values_stack, _2);
let mut _1 = pop!(self.values_stack, _2);

let __;
        if let Node::Nodes(mut nodes) = _1 {
            nodes.push(_2);
            __ = Node::Nodes(nodes);
        } else {unreachable!();};
SV::_2(__)
}


fn _handler35(&mut self) -> SV {

    println!("   *** PARSER: _handler35");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.
let mut _1 = pop!(self.values_stack, _2);

let __ = Node::Nodes(vec![_1]);
SV::_2(__)
}


fn _handler36(&mut self) -> SV {

    println!("   *** PARSER: _handler36");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.
let mut _2 = pop!(self.values_stack, _2);
let mut _1 = pop!(self.values_stack, _2);

let __;
        if let Node::Nodes(mut nodes) = _1 {
            nodes.push(_2);
            __ = Node::Nodes(nodes);
        } else { unreachable!(); };
SV::_2(__)
}


fn _handler37(&mut self) -> SV {

    println!("   *** PARSER: _handler37");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.
self.values_stack.pop();
let mut _2 = pop!(self.values_stack, _2);
self.values_stack.pop();

let __ = node::words_compose(_2);
SV::_2(__)
}


fn _handler38(&mut self) -> SV {

    println!("   *** PARSER: _handler38");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.


let __ = Node::Nodes(vec![]);
SV::_2(__)
}


fn _handler39(&mut self) -> SV {

    println!("   *** PARSER: _handler39");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.
let mut _3 = pop!(self.values_stack, _0);
let mut _2 = pop!(self.values_stack, _0);
let mut _1 = pop!(self.values_stack, _2);

let __;
        if let Node::Nodes(mut nodes) = _1 {
            nodes.push(node::string_internal(*_2.interior_token));
            __ = Node::Nodes(nodes);
        } else {unreachable!();};
SV::_2(__)
}


fn _handler40(&mut self) -> SV {

    println!("   *** PARSER: _handler40");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.


let __ = Node::Nodes(vec![]);
SV::_2(__)
}


fn _handler41(&mut self) -> SV {

    println!("   *** PARSER: _handler41");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.
let mut _2 = pop!(self.values_stack, _2);
let mut _1 = pop!(self.values_stack, _2);

// string_contents: Node::Nodes
        // string_content: Node::Str

        let __;
        if let Node::Nodes(mut n_strs) = _1 {
            n_strs.push(_2);
            __ = Node::Nodes(n_strs);
        } else { unreachable!(); };
SV::_2(__)
}


fn _handler42(&mut self) -> SV {

    println!("   *** PARSER: _handler42");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.


let __ = Node::Nodes(vec![]);
SV::_2(__)
}


fn _handler43(&mut self) -> SV {

    println!("   *** PARSER: _handler43");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.
let mut _2 = pop!(self.values_stack, _2);
let mut _1 = pop!(self.values_stack, _2);

let __;
        if let Node::Nodes(mut nodes) = _1 {
            nodes.push(_2);
            __ = Node::Nodes(nodes);
        } else { unreachable!(); };
SV::_2(__)
}


fn _handler44(&mut self) -> SV {

    println!("   *** PARSER: _handler44");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.
let mut _1 = pop!(self.values_stack, _0);

let __;
        if let box InteriorToken::T_STRING_CONTENT(string_value) = _1.interior_token {
            __ = Node::Str(string_value);
        } else { unreachable!(); };
SV::_2(__)
}


fn _handler45(&mut self) -> SV {

    println!("   *** PARSER: _handler45");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.
let mut _1 = pop!(self.values_stack, _0);

// TODO lexer.state
        let __ = node::symbol(*_1.interior_token);
SV::_2(__)
}


fn _handler46(&mut self) -> SV {

    println!("   *** PARSER: _handler46");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.
let mut _3 = pop!(self.values_stack, _0);
let mut _2 = pop!(self.values_stack, _2);
let mut _1 = pop!(self.values_stack, _0);

// TODO lexer.state
        let __ = node::symbol_compose(_2);
SV::_2(__)
}


fn _handler47(&mut self) -> SV {

    println!("   *** PARSER: _handler47");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.
let mut _1 = self.values_stack.pop().unwrap();

let __ = _1;
__
}


fn _handler48(&mut self) -> SV {

    println!("   *** PARSER: _handler48");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.
let mut _1 = self.values_stack.pop().unwrap();

let __;
        if let SV::_0(token) = _1 {
            if let box InteriorToken::T_INTEGER(value) = token.interior_token {
                __ = Node::Int(value);
            } else { unreachable!(); }
        } else { unreachable!(); };
SV::_2(__)
}


fn _handler49(&mut self) -> SV {

    println!("   *** PARSER: _handler49");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.
let mut _1 = pop!(self.values_stack, _0);

let __ = node::ident(*_1.interior_token);
SV::_2(__)
}


fn _handler50(&mut self) -> SV {

    println!("   *** PARSER: _handler50");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.
let mut _1 = pop!(self.values_stack, _0);

let __ = node::ivar(*_1.interior_token);
SV::_2(__)
}


fn _handler51(&mut self) -> SV {

    println!("   *** PARSER: _handler51");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.
self.values_stack.pop();

let __ = Node::Nil;
SV::_2(__)
}


fn _handler52(&mut self) -> SV {

    println!("   *** PARSER: _handler52");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.
self.values_stack.pop();

let __ = Node::NSelf;
SV::_2(__)
}


fn _handler53(&mut self) -> SV {

    println!("   *** PARSER: _handler53");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.
self.values_stack.pop();

let __ = Node::True;
SV::_2(__)
}


fn _handler54(&mut self) -> SV {

    println!("   *** PARSER: _handler54");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.
self.values_stack.pop();

let __ = Node::False;
SV::_2(__)
}


fn _handler55(&mut self) -> SV {

    println!("   *** PARSER: _handler55");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.
let mut _1 = pop!(self.values_stack, _2);

let __ = node::accessible(_1);
SV::_2(__)
}


fn _handler56(&mut self) -> SV {

    println!("   *** PARSER: _handler56");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.
let mut _1 = pop!(self.values_stack, _2);

let __ = node::accessible(_1);
SV::_2(__)
}


fn _handler57(&mut self) -> SV {

    println!("   *** PARSER: _handler57");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.


let __ = SV::Undefined;
__
}


fn _handler58(&mut self) -> SV {

    println!("   *** PARSER: _handler58");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.
let mut _1 = self.values_stack.pop().unwrap();

let __ = _1;
__
}


fn _handler59(&mut self) -> SV {

    println!("   *** PARSER: _handler59");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.


let __ = SV::Undefined;
__
}


fn _handler60(&mut self) -> SV {

    println!("   *** PARSER: _handler60");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.
let mut _1 = self.values_stack.pop().unwrap();

let __ = _1;
__
}


fn _handler61(&mut self) -> SV {

    println!("   *** PARSER: _handler61");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.
let mut _1 = self.values_stack.pop().unwrap();

let __ = _1;
__
}


fn _handler62(&mut self) -> SV {

    println!("   *** PARSER: _handler62");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.
let mut _1 = self.values_stack.pop().unwrap();

let __ = _1;
__
}


fn _handler63(&mut self) -> SV {

    println!("   *** PARSER: _handler63");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.
let mut _1 = self.values_stack.pop().unwrap();

let __ = _1;
__
}


fn _handler64(&mut self) -> SV {

    println!("   *** PARSER: _handler64");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.
let mut _1 = self.values_stack.pop().unwrap();

let __ = _1;
__
}


fn _handler65(&mut self) -> SV {

    println!("   *** PARSER: _handler65");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.
self.values_stack.pop();
self.values_stack.pop();

let __ = SV::Undefined;
__
}
}
