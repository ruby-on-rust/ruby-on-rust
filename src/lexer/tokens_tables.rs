use std::collections::HashMap;

use lexer::Lexer;

use parser::parser::Token;

// 
// mapping tables of strings literal -> token
// 
// 
// PUNCTUATION = {
//     '='   => :tEQL,     '&'   => :tAMPER2,  '|'   => :tPIPE,
//     '!'   => :tBANG,    '^'   => :tCARET,   '+'   => :tPLUS,
//     '-'   => :tMINUS,   '*'   => :tSTAR2,   '/'   => :tDIVIDE,
//     '%'   => :tPERCENT, '~'   => :tTILDE,   ','   => :tCOMMA,
//     ';'   => :tSEMI,    '.'   => :tDOT,     '..'  => :tDOT2,
//     '...' => :tDOT3,    '['   => :tLBRACK2, ']'   => :tRBRACK,
//     '('   => :tLPAREN2, ')'   => :tRPAREN,  '?'   => :tEH,
//     ':'   => :tCOLON,   '&&'  => :tANDOP,   '||'  => :tOROP,
//     '-@'  => :tUMINUS,  '+@'  => :tUPLUS,   '~@'  => :tTILDE,
//     '**'  => :tPOW,     '->'  => :tLAMBDA,  '=~'  => :tMATCH,
//     '!~'  => :tNMATCH,  '=='  => :tEQ,      '!='  => :tNEQ,
//     '>'   => :tGT,      '>>'  => :tRSHFT,   '>='  => :tGEQ,
//     '<'   => :tLT,      '<<'  => :tLSHFT,   '<='  => :tLEQ,
//     '=>'  => :tASSOC,   '::'  => :tCOLON2,  '===' => :tEQQ,
//     '<=>' => :tCMP,     '[]'  => :tAREF,    '[]=' => :tASET,
//     '{'   => :tLCURLY,  '}'   => :tRCURLY,  '`'   => :tBACK_REF2,
//     '!@'  => :tBANG,    '&.'  => :tANDDOT,
// }

// PUNCTUATION_BEGIN = {
//     '&'   => :tAMPER,   '*'   => :tSTAR,    '**'  => :tDSTAR,
//     '+'   => :tUPLUS,   '-'   => :tUMINUS,  '::'  => :tCOLON3,
//     '('   => :tLPAREN,  '{'   => :tLBRACE,  '['   => :tLBRACK,
// }

// KEYWORDS = {
//     'if'     => :kIF_MOD,      'unless'   => :kUNLESS_MOD,
//     'while'  => :kWHILE_MOD,   'until'    => :kUNTIL_MOD,
//     'rescue' => :kRESCUE_MOD,  'defined?' => :kDEFINED,
//     'BEGIN'  => :klBEGIN,      'END'      => :klEND,
// }

// KEYWORDS_BEGIN = {
//     'if'     => :kIF,          'unless'   => :kUNLESS,
//     'while'  => :kWHILE,       'until'    => :kUNTIL,
//     'rescue' => :kRESCUE,      'defined?' => :kDEFINED,
// }

// %w(class module def undef begin end then elsif else ensure case when
//     for break next redo retry in do return yield super self nil true
//     false and or not alias __FILE__ __LINE__ __ENCODING__).each do |keyword|
//     KEYWORDS_BEGIN[keyword] = KEYWORDS[keyword] = :"k#{keyword.upcase}"
// end

pub fn construct() -> HashMap<&'static str, HashMap<&'static str, Token>> {
    let keywords: HashMap<&'static str, Token> = vec![
        ( "true", Token::K_TRUE ),
    ].into_iter().collect();

    let tables: HashMap<&'static str, HashMap<&str, Token>> = vec![
        ( "keywords", keywords ),
    ].into_iter().collect();

    tables
}
