use std::collections::HashMap;

use parser::token::Token;

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

    let punctuation: HashMap<&'static str, Token> = vec![
        ( "=", Token::T_EQL ),
        ( "&", Token::T_AMPER2 ),
        ( "|", Token::T_PIPE ),
        ( "!", Token::T_BANG ),
        ( "^", Token::T_CARET ),
        ( "+", Token::T_PLUS ),
        ( "-", Token::T_MINUS ),
        ( "*", Token::T_STAR2 ),
        ( "/", Token::T_DIVIDE ),
        ( "%", Token::T_PERCENT ),
        ( "~", Token::T_TILDE ),
        ( ",", Token::T_COMMA ),
        ( ";", Token::T_SEMI ),
        ( ".", Token::T_DOT ),
        ( "..", Token::T_DOT2 ),
        ( "...", Token::T_DOT3 ),
        ( "[", Token::T_LBRACK2 ),
        ( "]", Token::T_RBRACK ),
        ( "(", Token::T_LPAREN2 ),
        ( ")", Token::T_RPAREN ),
        ( "?", Token::T_EH ),
        ( ":", Token::T_COLON ),
        ( "&&", Token::T_ANDOP ),
        ( "||", Token::T_OROP ),
        ( "-@", Token::T_UMINUS ),
        ( "+@", Token::T_UPLUS ),
        ( "~@", Token::T_TILDE ),
        ( "**", Token::T_POW ),
        ( "->", Token::T_LAMBDA ),
        ( "=~", Token::T_MATCH ),
        ( "!~", Token::T_NMATCH ),
        ( "==", Token::T_EQ ),
        ( "!=", Token::T_NEQ ),
        ( ">", Token::T_GT ),
        ( ">>", Token::T_RSHFT ),
        ( ">=", Token::T_GEQ ),
        ( "<", Token::T_LT ),
        ( "<<", Token::T_LSHFT ),
        ( "<=", Token::T_LEQ ),
        ( "=>", Token::T_ASSOC ),
        ( "::", Token::T_COLON2 ),
        ( "===", Token::T_EQQ ),
        ( "<=>", Token::T_CMP ),
        ( "[]", Token::T_AREF ),
        ( "[]=", Token::T_ASET ),
        ( "{", Token::T_LCURLY ),
        ( "}", Token::T_RCURLY ),
        ( "`", Token::T_BACK_REF2 ),
        ( "!@", Token::T_BANG ),
        ( "&.", Token::T_ANDDOT ),
    ].into_iter().collect();

    let punctuation_begin: HashMap<&'static str, Token> = vec![
        ( "&" , Token::T_AMPER ),
        ( "*" , Token::T_STAR ),
        ( "**", Token::T_DSTAR ),
        ( "+" , Token::T_UPLUS ),
        ( "-" , Token::T_UMINUS ),
        ( "::", Token::T_COLON3 ),
        ( "(" , Token::T_LPAREN ),
        ( "{" , Token::T_LBRACE ),
        ( "[" , Token::T_LBRACK ),
    ].into_iter().collect();

    let keywords: HashMap<&'static str, Token> = vec![
        ( "if" , Token::K_IF_MOD ),
        ( "unless" , Token::K_UNLESS_MOD ),
        ( "while" , Token::K_WHILE_MOD ),
        ( "until" , Token::K_UNTIL_MOD ),
        ( "rescue" , Token::K_RESCUE_MOD ),
        ( "defined?" , Token::K_DEFINED ),
        ( "BEGIN" , Token::K_LBEGIN ),
        ( "END" , Token::K_LEND ),
        ( "class", Token::K_CLASS ),
        ( "module", Token::K_MODULE ),
        ( "def", Token::K_DEF ),
        ( "undef", Token::K_UNDEF ),
        ( "begin", Token::K_BEGIN ),
        ( "end", Token::K_END ),
        ( "then", Token::K_THEN ),
        ( "elsif", Token::K_ELSIF ),
        ( "else", Token::K_ELSE ),
        ( "ensure", Token::K_ENSURE ),
        ( "case", Token::K_CASE ),
        ( "when", Token::K_WHEN ),
        ( "for", Token::K_FOR ),
        ( "break", Token::K_BREAK ),
        ( "next", Token::K_NEXT ),
        ( "redo", Token::K_REDO ),
        ( "retry", Token::K_RETRY ),
        ( "in", Token::K_IN ),
        ( "do", Token::K_DO ),
        ( "return", Token::K_RETURN ),
        ( "yield", Token::K_YIELD ),
        ( "super", Token::K_SUPER ),
        ( "self", Token::K_SELF ),
        ( "nil", Token::K_NIL ),
        ( "true", Token::K_TRUE ),
        ( "false", Token::K_FALSE ),
        ( "and", Token::K_AND ),
        ( "or", Token::K_OR ),
        ( "not", Token::K_NOT ),
        ( "alias", Token::K_ALIAS ),
        ( "__FILE__", Token::K__FILE__ ),
        ( "__LINE__", Token::K__LINE__ ),
        ( "__ENCODING__", Token::K__ENCODING__ ),
    ].into_iter().collect();

    let keywords_begin: HashMap<&'static str, Token> = vec![
        ( "if", Token::K_IF ),
        ( "unless", Token::K_UNLESS ),
        ( "while", Token::K_WHILE ),
        ( "until", Token::K_UNTIL ),
        ( "rescue", Token::K_RESCUE ),
        ( "defined?", Token::K_DEFINED ),
        ( "class", Token::K_CLASS ),
        ( "module", Token::K_MODULE ),
        ( "def", Token::K_DEF ),
        ( "undef", Token::K_UNDEF ),
        ( "begin", Token::K_BEGIN ),
        ( "end", Token::K_END ),
        ( "then", Token::K_THEN ),
        ( "elsif", Token::K_ELSIF ),
        ( "else", Token::K_ELSE ),
        ( "ensure", Token::K_ENSURE ),
        ( "case", Token::K_CASE ),
        ( "when", Token::K_WHEN ),
        ( "for", Token::K_FOR ),
        ( "break", Token::K_BREAK ),
        ( "next", Token::K_NEXT ),
        ( "redo", Token::K_REDO ),
        ( "retry", Token::K_RETRY ),
        ( "in", Token::K_IN ),
        ( "do", Token::K_DO ),
        ( "return", Token::K_RETURN ),
        ( "yield", Token::K_YIELD ),
        ( "super", Token::K_SUPER ),
        ( "self", Token::K_SELF ),
        ( "nil", Token::K_NIL ),
        ( "true", Token::K_TRUE ),
        ( "false", Token::K_FALSE ),
        ( "and", Token::K_AND ),
        ( "or", Token::K_OR ),
        ( "not", Token::K_NOT ),
        ( "alias", Token::K_ALIAS ),
        ( "__FILE__", Token::K__FILE__ ),
        ( "__LINE__", Token::K__LINE__ ),
        ( "__ENCODING__", Token::K__ENCODING__ ),

    ].into_iter().collect();

    let tables: HashMap<&'static str, HashMap<&str, Token>> = vec![
        ( "punctuation", punctuation ),
        ( "punctuation_begin", punctuation_begin ),
        ( "keywords", keywords ),
        ( "keywords_begin", keywords_begin )
    ].into_iter().collect();

    tables
}
