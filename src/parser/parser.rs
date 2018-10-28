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
static PRODUCTIONS : [[i32; 2]; 78] = [
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
    [12, 3],
    [12, 2],
    [12, 3],
    [12, 3],
    [13, 1],
    [14, 1],
    [14, 1],
    [14, 1],
    [15, 1],
    [16, 1],
    [17, 3],
    [17, 1],
    [18, 3],
    [19, 0],
    [19, 3],
    [20, 1],
    [20, 2],
    [21, 3],
    [22, 0],
    [22, 3],
    [23, 0],
    [23, 2],
    [24, 0],
    [24, 2],
    [25, 1],
    [26, 1],
    [27, 3],
    [28, 1],
    [29, 1],
    [30, 1],
    [30, 1],
    [30, 1],
    [30, 1],
    [30, 1],
    [31, 1],
    [31, 1],
    [31, 1],
    [31, 1],
    [32, 1],
    [32, 1],
    [33, 0],
    [33, 2],
    [34, 1],
    [34, 3],
    [35, 3],
    [35, 2],
    [36, 0],
    [36, 1],
    [37, 0],
    [37, 1],
    [37, 1],
    [38, 1],
    [38, 1],
    [39, 1],
    [39, 2]
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
    static ref TOKENS_MAP: HashMap<&'static str, i32> = hashmap! { "tEQL" => 40, "tCOMMA" => 41, "tCOLON2" => 42, "tCONSTANT" => 43, "tCOLON3" => 44, "tLBRACK" => 45, "tRBRACK" => 46, "tLBRACE" => 47, "tRCURLY" => 48, "tSTRING_BEG" => 49, "tSTRING_END" => 50, "tSTRING" => 51, "tWORDS_BEG" => 52, "tSPACE" => 53, "tQWORDS_BEG" => 54, "tSTRING_CONTENT" => 55, "tSYMBOL" => 56, "tSYMBEG" => 57, "tINTEGER" => 58, "tIDENTIFIER" => 59, "tIVAR" => 60, "tGVAR" => 61, "tCVAR" => 62, "kNIL" => 63, "kSELF" => 64, "kTRUE" => 65, "kFALSE" => 66, "tASSOC" => 67, "tLABEL" => 68, "tNL" => 69, "tSEMI" => 70, "$" => 71 };

    /**
     * Parsing table.
     *
     * Vector index is the state number, value is a map
     * from an encoded symbol to table entry (TE).
     */
    static ref TABLE: Vec<HashMap<i32, TE>>= vec![
    hashmap! { 0 => TE::Transit(1), 1 => TE::Transit(2), 2 => TE::Transit(3), 3 => TE::Transit(4), 4 => TE::Transit(5), 5 => TE::Transit(6), 6 => TE::Transit(8), 7 => TE::Transit(7), 12 => TE::Transit(9), 13 => TE::Transit(21), 14 => TE::Transit(16), 15 => TE::Transit(17), 16 => TE::Transit(32), 17 => TE::Transit(33), 18 => TE::Transit(18), 21 => TE::Transit(19), 26 => TE::Transit(26), 27 => TE::Transit(27), 28 => TE::Transit(25), 29 => TE::Transit(28), 30 => TE::Transit(10), 31 => TE::Transit(38), 32 => TE::Transit(20), 43 => TE::Shift(14), 44 => TE::Shift(22), 45 => TE::Shift(23), 47 => TE::Shift(24), 49 => TE::Shift(34), 51 => TE::Shift(35), 52 => TE::Shift(36), 54 => TE::Shift(37), 56 => TE::Shift(30), 57 => TE::Shift(31), 58 => TE::Shift(29), 59 => TE::Shift(11), 60 => TE::Shift(12), 61 => TE::Shift(13), 62 => TE::Shift(15), 63 => TE::Shift(39), 64 => TE::Shift(40), 65 => TE::Shift(41), 66 => TE::Shift(42), 69 => TE::Reduce(3), 70 => TE::Reduce(3), 71 => TE::Reduce(3) },
    hashmap! { 71 => TE::Accept },
    hashmap! { 71 => TE::Reduce(1) },
    hashmap! { 36 => TE::Transit(43), 38 => TE::Transit(45), 39 => TE::Transit(44), 69 => TE::Shift(47), 70 => TE::Shift(46), 71 => TE::Reduce(69) },
    hashmap! { 69 => TE::Reduce(4), 70 => TE::Reduce(4), 71 => TE::Reduce(4) },
    hashmap! { 69 => TE::Reduce(6), 70 => TE::Reduce(6), 71 => TE::Reduce(6) },
    hashmap! { 69 => TE::Reduce(7), 70 => TE::Reduce(7), 71 => TE::Reduce(7) },
    hashmap! { 69 => TE::Reduce(8), 70 => TE::Reduce(8), 71 => TE::Reduce(8) },
    hashmap! { 40 => TE::Shift(50) },
    hashmap! { 41 => TE::Reduce(11), 42 => TE::Reduce(27), 46 => TE::Reduce(11), 48 => TE::Reduce(11), 67 => TE::Reduce(11), 69 => TE::Reduce(11), 70 => TE::Reduce(11), 71 => TE::Reduce(11) },
    hashmap! { 40 => TE::Reduce(9), 41 => TE::Reduce(61), 42 => TE::Reduce(61), 46 => TE::Reduce(61), 48 => TE::Reduce(61), 67 => TE::Reduce(61), 69 => TE::Reduce(61), 70 => TE::Reduce(61), 71 => TE::Reduce(61) },
    hashmap! { 40 => TE::Reduce(52), 41 => TE::Reduce(52), 42 => TE::Reduce(52), 46 => TE::Reduce(52), 48 => TE::Reduce(52), 67 => TE::Reduce(52), 69 => TE::Reduce(52), 70 => TE::Reduce(52), 71 => TE::Reduce(52) },
    hashmap! { 40 => TE::Reduce(53), 41 => TE::Reduce(53), 42 => TE::Reduce(53), 46 => TE::Reduce(53), 48 => TE::Reduce(53), 67 => TE::Reduce(53), 69 => TE::Reduce(53), 70 => TE::Reduce(53), 71 => TE::Reduce(53) },
    hashmap! { 40 => TE::Reduce(54), 41 => TE::Reduce(54), 42 => TE::Reduce(54), 46 => TE::Reduce(54), 48 => TE::Reduce(54), 67 => TE::Reduce(54), 69 => TE::Reduce(54), 70 => TE::Reduce(54), 71 => TE::Reduce(54) },
    hashmap! { 40 => TE::Reduce(55), 41 => TE::Reduce(55), 42 => TE::Reduce(55), 46 => TE::Reduce(55), 48 => TE::Reduce(55), 67 => TE::Reduce(55), 69 => TE::Reduce(55), 70 => TE::Reduce(55), 71 => TE::Reduce(55) },
    hashmap! { 40 => TE::Reduce(56), 41 => TE::Reduce(56), 42 => TE::Reduce(56), 46 => TE::Reduce(56), 48 => TE::Reduce(56), 67 => TE::Reduce(56), 69 => TE::Reduce(56), 70 => TE::Reduce(56), 71 => TE::Reduce(56) },
    hashmap! { 41 => TE::Reduce(18), 42 => TE::Reduce(18), 46 => TE::Reduce(18), 48 => TE::Reduce(18), 67 => TE::Reduce(18), 69 => TE::Reduce(18), 70 => TE::Reduce(18), 71 => TE::Reduce(18) },
    hashmap! { 41 => TE::Reduce(19), 42 => TE::Reduce(19), 46 => TE::Reduce(19), 48 => TE::Reduce(19), 67 => TE::Reduce(19), 69 => TE::Reduce(19), 70 => TE::Reduce(19), 71 => TE::Reduce(19) },
    hashmap! { 41 => TE::Reduce(20), 42 => TE::Reduce(20), 46 => TE::Reduce(20), 48 => TE::Reduce(20), 67 => TE::Reduce(20), 69 => TE::Reduce(20), 70 => TE::Reduce(20), 71 => TE::Reduce(20) },
    hashmap! { 41 => TE::Reduce(21), 42 => TE::Reduce(21), 46 => TE::Reduce(21), 48 => TE::Reduce(21), 67 => TE::Reduce(21), 69 => TE::Reduce(21), 70 => TE::Reduce(21), 71 => TE::Reduce(21) },
    hashmap! { 41 => TE::Reduce(22), 42 => TE::Reduce(22), 46 => TE::Reduce(22), 48 => TE::Reduce(22), 67 => TE::Reduce(22), 69 => TE::Reduce(22), 70 => TE::Reduce(22), 71 => TE::Reduce(22) },
    hashmap! { 42 => TE::Shift(53) },
    hashmap! { 43 => TE::Shift(55) },
    hashmap! { 6 => TE::Transit(8), 7 => TE::Transit(59), 8 => TE::Transit(58), 9 => TE::Transit(56), 11 => TE::Transit(57), 12 => TE::Transit(9), 13 => TE::Transit(21), 14 => TE::Transit(16), 15 => TE::Transit(17), 16 => TE::Transit(32), 17 => TE::Transit(33), 18 => TE::Transit(18), 21 => TE::Transit(19), 26 => TE::Transit(26), 27 => TE::Transit(27), 28 => TE::Transit(25), 29 => TE::Transit(28), 30 => TE::Transit(10), 31 => TE::Transit(38), 32 => TE::Transit(20), 43 => TE::Shift(14), 44 => TE::Shift(22), 45 => TE::Shift(23), 46 => TE::Reduce(13), 47 => TE::Shift(24), 49 => TE::Shift(34), 51 => TE::Shift(35), 52 => TE::Shift(36), 54 => TE::Shift(37), 56 => TE::Shift(30), 57 => TE::Shift(31), 58 => TE::Shift(29), 59 => TE::Shift(11), 60 => TE::Shift(12), 61 => TE::Shift(13), 62 => TE::Shift(15), 63 => TE::Shift(39), 64 => TE::Shift(40), 65 => TE::Shift(41), 66 => TE::Shift(42) },
    hashmap! { 6 => TE::Transit(8), 7 => TE::Transit(59), 8 => TE::Transit(68), 12 => TE::Transit(9), 13 => TE::Transit(21), 14 => TE::Transit(16), 15 => TE::Transit(17), 16 => TE::Transit(32), 17 => TE::Transit(33), 18 => TE::Transit(18), 21 => TE::Transit(19), 26 => TE::Transit(26), 27 => TE::Transit(27), 28 => TE::Transit(25), 29 => TE::Transit(28), 30 => TE::Transit(10), 31 => TE::Transit(38), 32 => TE::Transit(20), 33 => TE::Transit(65), 34 => TE::Transit(66), 35 => TE::Transit(67), 43 => TE::Shift(14), 44 => TE::Shift(22), 45 => TE::Shift(23), 47 => TE::Shift(24), 48 => TE::Reduce(63), 49 => TE::Shift(34), 51 => TE::Shift(35), 52 => TE::Shift(36), 54 => TE::Shift(37), 56 => TE::Shift(30), 57 => TE::Shift(31), 58 => TE::Shift(29), 59 => TE::Shift(11), 60 => TE::Shift(12), 61 => TE::Shift(13), 62 => TE::Shift(15), 63 => TE::Shift(39), 64 => TE::Shift(40), 65 => TE::Shift(41), 66 => TE::Shift(42), 68 => TE::Shift(69) },
    hashmap! { 41 => TE::Reduce(28), 42 => TE::Reduce(28), 46 => TE::Reduce(28), 48 => TE::Reduce(28), 67 => TE::Reduce(28), 69 => TE::Reduce(28), 70 => TE::Reduce(28), 71 => TE::Reduce(28) },
    hashmap! { 41 => TE::Reduce(29), 42 => TE::Reduce(29), 46 => TE::Reduce(29), 48 => TE::Reduce(29), 67 => TE::Reduce(29), 69 => TE::Reduce(29), 70 => TE::Reduce(29), 71 => TE::Reduce(29) },
    hashmap! { 41 => TE::Reduce(30), 42 => TE::Reduce(30), 46 => TE::Reduce(30), 48 => TE::Reduce(30), 67 => TE::Reduce(30), 69 => TE::Reduce(30), 70 => TE::Reduce(30), 71 => TE::Reduce(30) },
    hashmap! { 41 => TE::Reduce(50), 42 => TE::Reduce(50), 46 => TE::Reduce(50), 48 => TE::Reduce(50), 67 => TE::Reduce(50), 69 => TE::Reduce(50), 70 => TE::Reduce(50), 71 => TE::Reduce(50) },
    hashmap! { 41 => TE::Reduce(51), 42 => TE::Reduce(51), 46 => TE::Reduce(51), 48 => TE::Reduce(51), 67 => TE::Reduce(51), 69 => TE::Reduce(51), 70 => TE::Reduce(51), 71 => TE::Reduce(51) },
    hashmap! { 41 => TE::Reduce(48), 42 => TE::Reduce(48), 46 => TE::Reduce(48), 48 => TE::Reduce(48), 67 => TE::Reduce(48), 69 => TE::Reduce(48), 70 => TE::Reduce(48), 71 => TE::Reduce(48) },
    hashmap! { 24 => TE::Transit(76), 50 => TE::Reduce(45), 55 => TE::Reduce(45) },
    hashmap! { 41 => TE::Reduce(31), 42 => TE::Reduce(31), 46 => TE::Reduce(31), 48 => TE::Reduce(31), 67 => TE::Reduce(31), 69 => TE::Reduce(31), 70 => TE::Reduce(31), 71 => TE::Reduce(31) },
    hashmap! { 41 => TE::Reduce(32), 42 => TE::Reduce(32), 46 => TE::Reduce(32), 48 => TE::Reduce(32), 67 => TE::Reduce(32), 69 => TE::Reduce(32), 70 => TE::Reduce(32), 71 => TE::Reduce(32) },
    hashmap! { 23 => TE::Transit(80), 50 => TE::Reduce(43), 55 => TE::Reduce(43) },
    hashmap! { 41 => TE::Reduce(34), 42 => TE::Reduce(34), 46 => TE::Reduce(34), 48 => TE::Reduce(34), 67 => TE::Reduce(34), 69 => TE::Reduce(34), 70 => TE::Reduce(34), 71 => TE::Reduce(34) },
    hashmap! { 19 => TE::Transit(83), 50 => TE::Reduce(36), 55 => TE::Reduce(36) },
    hashmap! { 22 => TE::Transit(89), 50 => TE::Reduce(41), 55 => TE::Reduce(41) },
    hashmap! { 41 => TE::Reduce(62), 42 => TE::Reduce(62), 46 => TE::Reduce(62), 48 => TE::Reduce(62), 67 => TE::Reduce(62), 69 => TE::Reduce(62), 70 => TE::Reduce(62), 71 => TE::Reduce(62) },
    hashmap! { 41 => TE::Reduce(57), 42 => TE::Reduce(57), 46 => TE::Reduce(57), 48 => TE::Reduce(57), 67 => TE::Reduce(57), 69 => TE::Reduce(57), 70 => TE::Reduce(57), 71 => TE::Reduce(57) },
    hashmap! { 41 => TE::Reduce(58), 42 => TE::Reduce(58), 46 => TE::Reduce(58), 48 => TE::Reduce(58), 67 => TE::Reduce(58), 69 => TE::Reduce(58), 70 => TE::Reduce(58), 71 => TE::Reduce(58) },
    hashmap! { 41 => TE::Reduce(59), 42 => TE::Reduce(59), 46 => TE::Reduce(59), 48 => TE::Reduce(59), 67 => TE::Reduce(59), 69 => TE::Reduce(59), 70 => TE::Reduce(59), 71 => TE::Reduce(59) },
    hashmap! { 41 => TE::Reduce(60), 42 => TE::Reduce(60), 46 => TE::Reduce(60), 48 => TE::Reduce(60), 67 => TE::Reduce(60), 69 => TE::Reduce(60), 70 => TE::Reduce(60), 71 => TE::Reduce(60) },
    hashmap! { 71 => TE::Reduce(2) },
    hashmap! { 3 => TE::Transit(48), 4 => TE::Transit(5), 5 => TE::Transit(6), 6 => TE::Transit(8), 7 => TE::Transit(7), 12 => TE::Transit(9), 13 => TE::Transit(21), 14 => TE::Transit(16), 15 => TE::Transit(17), 16 => TE::Transit(32), 17 => TE::Transit(33), 18 => TE::Transit(18), 21 => TE::Transit(19), 26 => TE::Transit(26), 27 => TE::Transit(27), 28 => TE::Transit(25), 29 => TE::Transit(28), 30 => TE::Transit(10), 31 => TE::Transit(38), 32 => TE::Transit(20), 43 => TE::Shift(14), 44 => TE::Shift(22), 45 => TE::Shift(23), 47 => TE::Shift(24), 49 => TE::Shift(34), 51 => TE::Shift(35), 52 => TE::Shift(36), 54 => TE::Shift(37), 56 => TE::Shift(30), 57 => TE::Shift(31), 58 => TE::Shift(29), 59 => TE::Shift(11), 60 => TE::Shift(12), 61 => TE::Shift(13), 62 => TE::Shift(15), 63 => TE::Shift(39), 64 => TE::Shift(40), 65 => TE::Shift(41), 66 => TE::Shift(42), 70 => TE::Shift(49), 71 => TE::Reduce(70) },
    hashmap! { 43 => TE::Reduce(76), 44 => TE::Reduce(76), 45 => TE::Reduce(76), 47 => TE::Reduce(76), 49 => TE::Reduce(76), 51 => TE::Reduce(76), 52 => TE::Reduce(76), 54 => TE::Reduce(76), 56 => TE::Reduce(76), 57 => TE::Reduce(76), 58 => TE::Reduce(76), 59 => TE::Reduce(76), 60 => TE::Reduce(76), 61 => TE::Reduce(76), 62 => TE::Reduce(76), 63 => TE::Reduce(76), 64 => TE::Reduce(76), 65 => TE::Reduce(76), 66 => TE::Reduce(76), 70 => TE::Reduce(76), 71 => TE::Reduce(76) },
    hashmap! { 43 => TE::Reduce(74), 44 => TE::Reduce(74), 45 => TE::Reduce(74), 47 => TE::Reduce(74), 49 => TE::Reduce(74), 51 => TE::Reduce(74), 52 => TE::Reduce(74), 54 => TE::Reduce(74), 56 => TE::Reduce(74), 57 => TE::Reduce(74), 58 => TE::Reduce(74), 59 => TE::Reduce(74), 60 => TE::Reduce(74), 61 => TE::Reduce(74), 62 => TE::Reduce(74), 63 => TE::Reduce(74), 64 => TE::Reduce(74), 65 => TE::Reduce(74), 66 => TE::Reduce(74), 70 => TE::Reduce(74), 71 => TE::Reduce(74) },
    hashmap! { 43 => TE::Reduce(75), 44 => TE::Reduce(75), 45 => TE::Reduce(75), 47 => TE::Reduce(75), 49 => TE::Reduce(75), 51 => TE::Reduce(75), 52 => TE::Reduce(75), 54 => TE::Reduce(75), 56 => TE::Reduce(75), 57 => TE::Reduce(75), 58 => TE::Reduce(75), 59 => TE::Reduce(75), 60 => TE::Reduce(75), 61 => TE::Reduce(75), 62 => TE::Reduce(75), 63 => TE::Reduce(75), 64 => TE::Reduce(75), 65 => TE::Reduce(75), 66 => TE::Reduce(75), 70 => TE::Reduce(75), 71 => TE::Reduce(75) },
    hashmap! { 69 => TE::Reduce(5), 70 => TE::Reduce(5), 71 => TE::Reduce(5) },
    hashmap! { 43 => TE::Reduce(77), 44 => TE::Reduce(77), 45 => TE::Reduce(77), 47 => TE::Reduce(77), 49 => TE::Reduce(77), 51 => TE::Reduce(77), 52 => TE::Reduce(77), 54 => TE::Reduce(77), 56 => TE::Reduce(77), 57 => TE::Reduce(77), 58 => TE::Reduce(77), 59 => TE::Reduce(77), 60 => TE::Reduce(77), 61 => TE::Reduce(77), 62 => TE::Reduce(77), 63 => TE::Reduce(77), 64 => TE::Reduce(77), 65 => TE::Reduce(77), 66 => TE::Reduce(77), 70 => TE::Reduce(77), 71 => TE::Reduce(77) },
    hashmap! { 6 => TE::Transit(8), 7 => TE::Transit(52), 10 => TE::Transit(51), 12 => TE::Transit(9), 13 => TE::Transit(21), 14 => TE::Transit(16), 15 => TE::Transit(17), 16 => TE::Transit(32), 17 => TE::Transit(33), 18 => TE::Transit(18), 21 => TE::Transit(19), 26 => TE::Transit(26), 27 => TE::Transit(27), 28 => TE::Transit(25), 29 => TE::Transit(28), 30 => TE::Transit(10), 31 => TE::Transit(38), 32 => TE::Transit(20), 43 => TE::Shift(14), 44 => TE::Shift(22), 45 => TE::Shift(23), 47 => TE::Shift(24), 49 => TE::Shift(34), 51 => TE::Shift(35), 52 => TE::Shift(36), 54 => TE::Shift(37), 56 => TE::Shift(30), 57 => TE::Shift(31), 58 => TE::Shift(29), 59 => TE::Shift(11), 60 => TE::Shift(12), 61 => TE::Shift(13), 62 => TE::Shift(15), 63 => TE::Shift(39), 64 => TE::Shift(40), 65 => TE::Shift(41), 66 => TE::Shift(42) },
    hashmap! { 41 => TE::Reduce(10), 46 => TE::Reduce(10), 48 => TE::Reduce(10), 67 => TE::Reduce(10), 69 => TE::Reduce(10), 70 => TE::Reduce(10), 71 => TE::Reduce(10) },
    hashmap! { 41 => TE::Reduce(15), 46 => TE::Reduce(15), 48 => TE::Reduce(15), 67 => TE::Reduce(15), 69 => TE::Reduce(15), 70 => TE::Reduce(15), 71 => TE::Reduce(15) },
    hashmap! { 43 => TE::Shift(54) },
    hashmap! { 41 => TE::Reduce(23), 42 => TE::Reduce(23), 46 => TE::Reduce(23), 48 => TE::Reduce(23), 67 => TE::Reduce(23), 69 => TE::Reduce(23), 70 => TE::Reduce(23), 71 => TE::Reduce(23) },
    hashmap! { 41 => TE::Reduce(24), 42 => TE::Reduce(24), 46 => TE::Reduce(24), 48 => TE::Reduce(24), 67 => TE::Reduce(24), 69 => TE::Reduce(24), 70 => TE::Reduce(24), 71 => TE::Reduce(24) },
    hashmap! { 46 => TE::Shift(60) },
    hashmap! { 37 => TE::Transit(61), 41 => TE::Shift(62), 46 => TE::Reduce(71), 69 => TE::Shift(63) },
    hashmap! { 41 => TE::Reduce(16), 46 => TE::Reduce(16), 69 => TE::Reduce(16) },
    hashmap! { 41 => TE::Reduce(12), 46 => TE::Reduce(12), 48 => TE::Reduce(12), 67 => TE::Reduce(12), 69 => TE::Reduce(12) },
    hashmap! { 41 => TE::Reduce(25), 42 => TE::Reduce(25), 46 => TE::Reduce(25), 48 => TE::Reduce(25), 67 => TE::Reduce(25), 69 => TE::Reduce(25), 70 => TE::Reduce(25), 71 => TE::Reduce(25) },
    hashmap! { 46 => TE::Reduce(14) },
    hashmap! { 6 => TE::Transit(8), 7 => TE::Transit(59), 8 => TE::Transit(64), 12 => TE::Transit(9), 13 => TE::Transit(21), 14 => TE::Transit(16), 15 => TE::Transit(17), 16 => TE::Transit(32), 17 => TE::Transit(33), 18 => TE::Transit(18), 21 => TE::Transit(19), 26 => TE::Transit(26), 27 => TE::Transit(27), 28 => TE::Transit(25), 29 => TE::Transit(28), 30 => TE::Transit(10), 31 => TE::Transit(38), 32 => TE::Transit(20), 43 => TE::Shift(14), 44 => TE::Shift(22), 45 => TE::Shift(23), 46 => TE::Reduce(73), 47 => TE::Shift(24), 49 => TE::Shift(34), 51 => TE::Shift(35), 52 => TE::Shift(36), 54 => TE::Shift(37), 56 => TE::Shift(30), 57 => TE::Shift(31), 58 => TE::Shift(29), 59 => TE::Shift(11), 60 => TE::Shift(12), 61 => TE::Shift(13), 62 => TE::Shift(15), 63 => TE::Shift(39), 64 => TE::Shift(40), 65 => TE::Shift(41), 66 => TE::Shift(42) },
    hashmap! { 46 => TE::Reduce(72), 48 => TE::Reduce(72) },
    hashmap! { 41 => TE::Reduce(17), 46 => TE::Reduce(17), 69 => TE::Reduce(17) },
    hashmap! { 48 => TE::Shift(70) },
    hashmap! { 37 => TE::Transit(71), 41 => TE::Shift(72), 48 => TE::Reduce(71), 69 => TE::Shift(63) },
    hashmap! { 41 => TE::Reduce(65), 48 => TE::Reduce(65), 69 => TE::Reduce(65) },
    hashmap! { 67 => TE::Shift(74) },
    hashmap! { 6 => TE::Transit(8), 7 => TE::Transit(59), 8 => TE::Transit(93), 12 => TE::Transit(9), 13 => TE::Transit(21), 14 => TE::Transit(16), 15 => TE::Transit(17), 16 => TE::Transit(32), 17 => TE::Transit(33), 18 => TE::Transit(18), 21 => TE::Transit(19), 26 => TE::Transit(26), 27 => TE::Transit(27), 28 => TE::Transit(25), 29 => TE::Transit(28), 30 => TE::Transit(10), 31 => TE::Transit(38), 32 => TE::Transit(20), 43 => TE::Shift(14), 44 => TE::Shift(22), 45 => TE::Shift(23), 47 => TE::Shift(24), 49 => TE::Shift(34), 51 => TE::Shift(35), 52 => TE::Shift(36), 54 => TE::Shift(37), 56 => TE::Shift(30), 57 => TE::Shift(31), 58 => TE::Shift(29), 59 => TE::Shift(11), 60 => TE::Shift(12), 61 => TE::Shift(13), 62 => TE::Shift(15), 63 => TE::Shift(39), 64 => TE::Shift(40), 65 => TE::Shift(41), 66 => TE::Shift(42) },
    hashmap! { 41 => TE::Reduce(26), 42 => TE::Reduce(26), 46 => TE::Reduce(26), 48 => TE::Reduce(26), 67 => TE::Reduce(26), 69 => TE::Reduce(26), 70 => TE::Reduce(26), 71 => TE::Reduce(26) },
    hashmap! { 48 => TE::Reduce(64) },
    hashmap! { 6 => TE::Transit(8), 7 => TE::Transit(59), 8 => TE::Transit(68), 12 => TE::Transit(9), 13 => TE::Transit(21), 14 => TE::Transit(16), 15 => TE::Transit(17), 16 => TE::Transit(32), 17 => TE::Transit(33), 18 => TE::Transit(18), 21 => TE::Transit(19), 26 => TE::Transit(26), 27 => TE::Transit(27), 28 => TE::Transit(25), 29 => TE::Transit(28), 30 => TE::Transit(10), 31 => TE::Transit(38), 32 => TE::Transit(20), 35 => TE::Transit(73), 43 => TE::Shift(14), 44 => TE::Shift(22), 45 => TE::Shift(23), 47 => TE::Shift(24), 48 => TE::Reduce(73), 49 => TE::Shift(34), 51 => TE::Shift(35), 52 => TE::Shift(36), 54 => TE::Shift(37), 56 => TE::Shift(30), 57 => TE::Shift(31), 58 => TE::Shift(29), 59 => TE::Shift(11), 60 => TE::Shift(12), 61 => TE::Shift(13), 62 => TE::Shift(15), 63 => TE::Shift(39), 64 => TE::Shift(40), 65 => TE::Shift(41), 66 => TE::Shift(42), 68 => TE::Shift(69) },
    hashmap! { 41 => TE::Reduce(66), 48 => TE::Reduce(66), 69 => TE::Reduce(66) },
    hashmap! { 6 => TE::Transit(8), 7 => TE::Transit(59), 8 => TE::Transit(75), 12 => TE::Transit(9), 13 => TE::Transit(21), 14 => TE::Transit(16), 15 => TE::Transit(17), 16 => TE::Transit(32), 17 => TE::Transit(33), 18 => TE::Transit(18), 21 => TE::Transit(19), 26 => TE::Transit(26), 27 => TE::Transit(27), 28 => TE::Transit(25), 29 => TE::Transit(28), 30 => TE::Transit(10), 31 => TE::Transit(38), 32 => TE::Transit(20), 43 => TE::Shift(14), 44 => TE::Shift(22), 45 => TE::Shift(23), 47 => TE::Shift(24), 49 => TE::Shift(34), 51 => TE::Shift(35), 52 => TE::Shift(36), 54 => TE::Shift(37), 56 => TE::Shift(30), 57 => TE::Shift(31), 58 => TE::Shift(29), 59 => TE::Shift(11), 60 => TE::Shift(12), 61 => TE::Shift(13), 62 => TE::Shift(15), 63 => TE::Shift(39), 64 => TE::Shift(40), 65 => TE::Shift(41), 66 => TE::Shift(42) },
    hashmap! { 41 => TE::Reduce(67), 48 => TE::Reduce(67), 69 => TE::Reduce(67) },
    hashmap! { 25 => TE::Transit(78), 50 => TE::Shift(77), 55 => TE::Shift(79) },
    hashmap! { 41 => TE::Reduce(49), 42 => TE::Reduce(49), 46 => TE::Reduce(49), 48 => TE::Reduce(49), 67 => TE::Reduce(49), 69 => TE::Reduce(49), 70 => TE::Reduce(49), 71 => TE::Reduce(49) },
    hashmap! { 50 => TE::Reduce(46), 55 => TE::Reduce(46) },
    hashmap! { 50 => TE::Reduce(47), 53 => TE::Reduce(47), 55 => TE::Reduce(47) },
    hashmap! { 25 => TE::Transit(82), 50 => TE::Shift(81), 55 => TE::Shift(79) },
    hashmap! { 41 => TE::Reduce(33), 42 => TE::Reduce(33), 46 => TE::Reduce(33), 48 => TE::Reduce(33), 67 => TE::Reduce(33), 69 => TE::Reduce(33), 70 => TE::Reduce(33), 71 => TE::Reduce(33) },
    hashmap! { 50 => TE::Reduce(44), 55 => TE::Reduce(44) },
    hashmap! { 20 => TE::Transit(85), 25 => TE::Transit(86), 50 => TE::Shift(84), 55 => TE::Shift(79) },
    hashmap! { 41 => TE::Reduce(35), 42 => TE::Reduce(35), 46 => TE::Reduce(35), 48 => TE::Reduce(35), 67 => TE::Reduce(35), 69 => TE::Reduce(35), 70 => TE::Reduce(35), 71 => TE::Reduce(35) },
    hashmap! { 25 => TE::Transit(88), 53 => TE::Shift(87), 55 => TE::Shift(79) },
    hashmap! { 53 => TE::Reduce(38), 55 => TE::Reduce(38) },
    hashmap! { 50 => TE::Reduce(37), 55 => TE::Reduce(37) },
    hashmap! { 53 => TE::Reduce(39), 55 => TE::Reduce(39) },
    hashmap! { 50 => TE::Shift(90), 55 => TE::Shift(91) },
    hashmap! { 41 => TE::Reduce(40), 42 => TE::Reduce(40), 46 => TE::Reduce(40), 48 => TE::Reduce(40), 67 => TE::Reduce(40), 69 => TE::Reduce(40), 70 => TE::Reduce(40), 71 => TE::Reduce(40) },
    hashmap! { 53 => TE::Shift(92) },
    hashmap! { 50 => TE::Reduce(42), 55 => TE::Reduce(42) },
    hashmap! { 41 => TE::Reduce(68), 48 => TE::Reduce(68), 69 => TE::Reduce(68) }
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

use token::token::Token as InteriorToken;
use parser::token::Token;
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
    handlers: [fn(&mut Parser) -> SV; 78],
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
    Parser::_handler65,
    Parser::_handler66,
    Parser::_handler67,
    Parser::_handler68,
    Parser::_handler69,
    Parser::_handler70,
    Parser::_handler71,
    Parser::_handler72,
    Parser::_handler73,
    Parser::_handler74,
    Parser::_handler75,
    Parser::_handler76,
    Parser::_handler77
],
        }
    }

    /**
     * Parses a string.
     */
    pub fn parse(&mut self, string: &str) -> TResult {
        

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
let mut _3 = pop!(self.values_stack, _0);
let mut _2 = pop!(self.values_stack, _0);
let mut _1 = pop!(self.values_stack, _2);

let __ = node::const_fetch(_1, *_2.interior_token, *_3.interior_token);
SV::_2(__)
}


fn _handler24(&mut self) -> SV {

    println!("   *** PARSER: _handler24");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.
let mut _2 = pop!(self.values_stack, _0);
let mut _1 = pop!(self.values_stack, _0);

let __ = node::const_global(*_1.interior_token, *_2.interior_token);
SV::_2(__)
}


fn _handler25(&mut self) -> SV {

    println!("   *** PARSER: _handler25");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.
let mut _3 = pop!(self.values_stack, _0);
let mut _2 = pop!(self.values_stack, _2);
let mut _1 = pop!(self.values_stack, _0);

let __ = node::array(_2);
SV::_2(__)
}


fn _handler26(&mut self) -> SV {

    println!("   *** PARSER: _handler26");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.
let mut _3 = pop!(self.values_stack, _0);
let mut _2 = pop!(self.values_stack, _1);
let mut _1 = pop!(self.values_stack, _0);

let __ = node::associate(_2);
SV::_2(__)
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
let mut _1 = self.values_stack.pop().unwrap();

let __ = _1;
__
}


fn _handler29(&mut self) -> SV {

    println!("   *** PARSER: _handler29");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.
let mut _1 = self.values_stack.pop().unwrap();

let __ = _1;
__
}


fn _handler30(&mut self) -> SV {

    println!("   *** PARSER: _handler30");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.
let mut _1 = self.values_stack.pop().unwrap();

let __ = _1;
__
}


fn _handler31(&mut self) -> SV {

    println!("   *** PARSER: _handler31");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.
let mut _1 = pop!(self.values_stack, _2);

let __ = node::string_compose(_1);
SV::_2(__)
}


fn _handler32(&mut self) -> SV {

    println!("   *** PARSER: _handler32");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.
let mut _1 = pop!(self.values_stack, _2);

let __ = Node::Nodes(vec![_1]);
SV::_2(__)
}


fn _handler33(&mut self) -> SV {

    println!("   *** PARSER: _handler33");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.
let mut _3 = pop!(self.values_stack, _0);
let mut _2 = pop!(self.values_stack, _2);
let mut _1 = pop!(self.values_stack, _0);

let __ = node::string_compose(_2);
        // TODO dedent_string;
SV::_2(__)
}


fn _handler34(&mut self) -> SV {

    println!("   *** PARSER: _handler34");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.
let mut _1 = pop!(self.values_stack, _0);

let __;
        if let InteriorToken::T_STRING(string_value) = *_1.interior_token {
            __ = Node::Str(string_value);
        } else { unreachable!(); }
        // TODO builder.dedent_string;
SV::_2(__)
}


fn _handler35(&mut self) -> SV {

    println!("   *** PARSER: _handler35");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.
self.values_stack.pop();
let mut _2 = pop!(self.values_stack, _2);
self.values_stack.pop();

let __ = node::words_compose(_2);
SV::_2(__)
}


fn _handler36(&mut self) -> SV {

    println!("   *** PARSER: _handler36");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.


let __ = Node::Nodes(vec![]);
SV::_2(__)
}


fn _handler37(&mut self) -> SV {

    println!("   *** PARSER: _handler37");
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


fn _handler38(&mut self) -> SV {

    println!("   *** PARSER: _handler38");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.
let mut _1 = pop!(self.values_stack, _2);

let __ = Node::Nodes(vec![_1]);
SV::_2(__)
}


fn _handler39(&mut self) -> SV {

    println!("   *** PARSER: _handler39");
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


fn _handler40(&mut self) -> SV {

    println!("   *** PARSER: _handler40");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.
self.values_stack.pop();
let mut _2 = pop!(self.values_stack, _2);
self.values_stack.pop();

let __ = node::words_compose(_2);
SV::_2(__)
}


fn _handler41(&mut self) -> SV {

    println!("   *** PARSER: _handler41");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.


let __ = Node::Nodes(vec![]);
SV::_2(__)
}


fn _handler42(&mut self) -> SV {

    println!("   *** PARSER: _handler42");
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


fn _handler43(&mut self) -> SV {

    println!("   *** PARSER: _handler43");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.


let __ = Node::Nodes(vec![]);
SV::_2(__)
}


fn _handler44(&mut self) -> SV {

    println!("   *** PARSER: _handler44");
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


fn _handler45(&mut self) -> SV {

    println!("   *** PARSER: _handler45");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.


let __ = Node::Nodes(vec![]);
SV::_2(__)
}


fn _handler46(&mut self) -> SV {

    println!("   *** PARSER: _handler46");
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


fn _handler47(&mut self) -> SV {

    println!("   *** PARSER: _handler47");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.
let mut _1 = pop!(self.values_stack, _0);

let __;
        if let InteriorToken::T_STRING_CONTENT(string_value) = *_1.interior_token {
            __ = Node::Str(string_value);
        } else { unreachable!(); };
SV::_2(__)
}


fn _handler48(&mut self) -> SV {

    println!("   *** PARSER: _handler48");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.
let mut _1 = pop!(self.values_stack, _0);

// TODO lexer.state
        let __ = node::symbol(*_1.interior_token);
SV::_2(__)
}


fn _handler49(&mut self) -> SV {

    println!("   *** PARSER: _handler49");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.
let mut _3 = pop!(self.values_stack, _0);
let mut _2 = pop!(self.values_stack, _2);
let mut _1 = pop!(self.values_stack, _0);

// TODO lexer.state
        let __ = node::symbol_compose(_2);
SV::_2(__)
}


fn _handler50(&mut self) -> SV {

    println!("   *** PARSER: _handler50");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.
let mut _1 = self.values_stack.pop().unwrap();

let __ = _1;
__
}


fn _handler51(&mut self) -> SV {

    println!("   *** PARSER: _handler51");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.
let mut _1 = self.values_stack.pop().unwrap();

let __;
        if let SV::_0(token) = _1 {
            if let InteriorToken::T_INTEGER(value) = *token.interior_token {
                __ = Node::Int(value);
            } else { unreachable!(); }
        } else { unreachable!(); };
SV::_2(__)
}


fn _handler52(&mut self) -> SV {

    println!("   *** PARSER: _handler52");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.
let mut _1 = pop!(self.values_stack, _0);

let __ = node::ident(*_1.interior_token);
SV::_2(__)
}


fn _handler53(&mut self) -> SV {

    println!("   *** PARSER: _handler53");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.
let mut _1 = pop!(self.values_stack, _0);

let __ = node::ivar(*_1.interior_token);
SV::_2(__)
}


fn _handler54(&mut self) -> SV {

    println!("   *** PARSER: _handler54");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.
let mut _1 = pop!(self.values_stack, _0);

let __ = node::gvar(*_1.interior_token);
SV::_2(__)
}


fn _handler55(&mut self) -> SV {

    println!("   *** PARSER: _handler55");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.
let mut _1 = pop!(self.values_stack, _0);

let __ = node::build_const(*_1.interior_token);
SV::_2(__)
}


fn _handler56(&mut self) -> SV {

    println!("   *** PARSER: _handler56");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.
let mut _1 = pop!(self.values_stack, _0);

let __ = node::cvar(*_1.interior_token);
SV::_2(__)
}


fn _handler57(&mut self) -> SV {

    println!("   *** PARSER: _handler57");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.
self.values_stack.pop();

let __ = Node::Nil;
SV::_2(__)
}


fn _handler58(&mut self) -> SV {

    println!("   *** PARSER: _handler58");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.
self.values_stack.pop();

let __ = Node::NSelf;
SV::_2(__)
}


fn _handler59(&mut self) -> SV {

    println!("   *** PARSER: _handler59");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.
self.values_stack.pop();

let __ = Node::True;
SV::_2(__)
}


fn _handler60(&mut self) -> SV {

    println!("   *** PARSER: _handler60");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.
self.values_stack.pop();

let __ = Node::False;
SV::_2(__)
}


fn _handler61(&mut self) -> SV {

    println!("   *** PARSER: _handler61");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.
let mut _1 = pop!(self.values_stack, _2);

let __ = node::accessible(_1);
SV::_2(__)
}


fn _handler62(&mut self) -> SV {

    println!("   *** PARSER: _handler62");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.
let mut _1 = pop!(self.values_stack, _2);

let __ = node::accessible(_1);
SV::_2(__)
}


fn _handler63(&mut self) -> SV {

    println!("   *** PARSER: _handler63");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.


let __ = vec![];
SV::_1(__)
}


fn _handler64(&mut self) -> SV {

    println!("   *** PARSER: _handler64");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.
self.values_stack.pop();
let mut _1 = self.values_stack.pop().unwrap();

let __ = _1;
__
}


fn _handler65(&mut self) -> SV {

    println!("   *** PARSER: _handler65");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.
let mut _1 = pop!(self.values_stack, _2);

let __ = vec![_1];
SV::_1(__)
}


fn _handler66(&mut self) -> SV {

    println!("   *** PARSER: _handler66");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.
let mut _3 = pop!(self.values_stack, _2);
let mut _2 = pop!(self.values_stack, _0);
let mut _1 = pop!(self.values_stack, _1);

_1.push(_3);
        let __ = _1;
SV::_1(__)
}


fn _handler67(&mut self) -> SV {

    println!("   *** PARSER: _handler67");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.
let mut _3 = pop!(self.values_stack, _2);
let mut _2 = pop!(self.values_stack, _0);
let mut _1 = pop!(self.values_stack, _2);

let __ = node::pair(_1, *_2.interior_token, _3);
SV::_2(__)
}


fn _handler68(&mut self) -> SV {

    println!("   *** PARSER: _handler68");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.
let mut _2 = pop!(self.values_stack, _2);
let mut _1 = pop!(self.values_stack, _0);

let __ = node::pair_keyword(*_1.interior_token, _2);
SV::_2(__)
}


fn _handler69(&mut self) -> SV {

    println!("   *** PARSER: _handler69");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.


let __ = SV::Undefined;
__
}


fn _handler70(&mut self) -> SV {

    println!("   *** PARSER: _handler70");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.
let mut _1 = self.values_stack.pop().unwrap();

let __ = _1;
__
}


fn _handler71(&mut self) -> SV {

    println!("   *** PARSER: _handler71");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.


let __ = SV::Undefined;
__
}


fn _handler72(&mut self) -> SV {

    println!("   *** PARSER: _handler72");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.
let mut _1 = self.values_stack.pop().unwrap();

let __ = _1;
__
}


fn _handler73(&mut self) -> SV {

    println!("   *** PARSER: _handler73");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.
let mut _1 = self.values_stack.pop().unwrap();

let __ = _1;
__
}


fn _handler74(&mut self) -> SV {

    println!("   *** PARSER: _handler74");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.
let mut _1 = self.values_stack.pop().unwrap();

let __ = _1;
__
}


fn _handler75(&mut self) -> SV {

    println!("   *** PARSER: _handler75");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.
let mut _1 = self.values_stack.pop().unwrap();

let __ = _1;
__
}


fn _handler76(&mut self) -> SV {

    println!("   *** PARSER: _handler76");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.
let mut _1 = self.values_stack.pop().unwrap();

let __ = _1;
__
}


fn _handler77(&mut self) -> SV {

    println!("   *** PARSER: _handler77");
    println!("   values_stack: {:?}", self.values_stack);
  // Semantic values prologue.
self.values_stack.pop();
self.values_stack.pop();

let __ = SV::Undefined;
__
}
}
