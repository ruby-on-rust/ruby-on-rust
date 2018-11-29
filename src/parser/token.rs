use crate::token::token::Token as InteriorToken;

use std::collections::HashMap;

impl InteriorToken {
    pub fn wrap_as_token(&self) -> Token {

// STARTING OF TOKENS_MAP
let tokens_map: HashMap<&str, isize> = hashmap! {"K_lBEGIN"=>182, "T_LCURLY"=>183, "T_RCURLY"=>184, "K_ALIAS"=>185, "T_GVAR"=>186, "T_BACK_REF"=>187, "T_NTH_REF"=>188, "K_UNDEF"=>189, "K_IF_MOD"=>190, "K_UNLESS_MOD"=>191, "K_WHILE_MOD"=>192, "K_UNTIL_MOD"=>193, "K_RESCUE_MOD"=>194, "K_lEND"=>195, "T_EQL"=>196, "T_OP_ASGN"=>197, "T_LBRACK2"=>198, "T_IDENTIFIER"=>199, "T_CONSTANT"=>200, "T_COLON2"=>201, "K_AND"=>202, "K_OR"=>203, "K_NOT"=>204, "T_BANG"=>205, "T_LBRACE_ARG"=>206, "K_SUPER"=>207, "K_YIELD"=>208, "K_BREAK"=>209, "K_NEXT"=>210, "T_LPAREN"=>211, "T_STAR"=>212, "T_COMMA"=>213, "T_COLON3"=>214, "T_FID"=>215, "T_PIPE"=>216, "T_CARET"=>217, "T_AMPER2"=>218, "T_CMP"=>219, "T_EQ"=>220, "T_EQQ"=>221, "T_MATCH"=>222, "T_NMATCH"=>223, "T_GT"=>224, "T_GEQ"=>225, "T_LT"=>226, "T_LEQ"=>227, "T_NEQ"=>228, "T_LSHFT"=>229, "T_RSHFT"=>230, "T_PLUS"=>231, "T_MINUS"=>232, "T_STAR2"=>233, "T_DIVIDE"=>234, "T_PERCENT"=>235, "T_POW"=>236, "T_TILDE"=>237, "T_UPLUS"=>238, "T_UMINUS"=>239, "T_AREF"=>240, "T_ASET"=>241, "T_DSTAR"=>242, "T_BACK_REF2"=>243, "K___LINE__"=>244, "K___FILE__"=>245, "K___ENCODING__"=>246, "K_BEGIN"=>247, "K_CASE"=>248, "K_CLASS"=>249, "K_DEF"=>250, "K_DEFINED"=>251, "K_DO"=>252, "K_ELSE"=>253, "K_ELSIF"=>254, "K_END"=>255, "K_ENSURE"=>256, "K_FALSE"=>257, "K_FOR"=>258, "K_IN"=>259, "K_MODULE"=>260, "K_NIL"=>261, "K_REDO"=>262, "K_RESCUE"=>263, "K_RETRY"=>264, "K_RETURN"=>265, "K_SELF"=>266, "K_THEN"=>267, "K_TRUE"=>268, "K_WHEN"=>269, "K_IF"=>270, "K_UNLESS"=>271, "K_WHILE"=>272, "K_UNTIL"=>273, "T_DOT2"=>274, "T_DOT3"=>275, "T_UNARY_NUM"=>276, "T_ANDOP"=>277, "T_OROP"=>278, "T_EH"=>279, "T_COLON"=>280, "T_LPAREN2"=>281, "T_AMPER"=>282, "T_LPAREN_ARG"=>283, "T_RPAREN"=>284, "T_LBRACK"=>285, "T_RBRACK"=>286, "T_LBRACE"=>287, "T_LAMBDA"=>288, "K_DO_COND"=>289, "T_SEMI"=>290, "T_LAMBEG"=>291, "K_DO_LAMBDA"=>292, "K_DO_BLOCK"=>293, "T_ASSOC"=>294, "T_STRING_BEG"=>295, "T_STRING_END"=>296, "T_STRING"=>297, "T_CHARACTER"=>298, "T_XSTRING_BEG"=>299, "T_REGEXP_BEG"=>300, "T_REGEXP_OPT"=>301, "T_WORDS_BEG"=>302, "T_SPACE"=>303, "T_SYMBOLS_BEG"=>304, "T_QWORDS_BEG"=>305, "T_QSYMBOLS_BEG"=>306, "T_STRING_CONTENT"=>307, "T_STRING_DVAR"=>308, "T_STRING_DBEG"=>309, "T_STRING_DEND"=>310, "T_IVAR"=>311, "T_CVAR"=>312, "T_SYMBOL"=>313, "T_SYMBEG"=>314, "T_INTEGER"=>315, "T_FLOAT"=>316, "T_RATIONAL"=>317, "T_IMAGINARY"=>318, "T_LABEL"=>319, "T_LABEL_END"=>320, "T_DOT"=>321, "T_ANDDOT"=>322, "T_NL"=>323, "$"=>324};
// END OF TOKENS_MAP

        let token_variant = self.as_ref();
        let kind = tokens_map.get(&token_variant).expect(&format!("unknown token type {}", token_variant));

        Token {
            kind: *kind as i32,
            value: "",

            interior_token: Box::new(self.clone()),

            start_offset: 0,
            end_offset: 0,
            start_line: 0,
            end_line: 0,
            start_column: 0,
            end_column: 0,
        }

    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: i32,
    pub value: &'static str,

    pub interior_token: Box<InteriorToken>,

    pub start_offset: i32,
    pub end_offset: i32,
    pub start_line: i32,
    pub end_line: i32,
    pub start_column: i32,
    pub end_column: i32,
}

pub fn get_an_eof_token() -> Token {
// STARTING OF TOKENS_MAP
let tokens_map: HashMap<&str, isize> = hashmap! {"K_lBEGIN"=>182, "T_LCURLY"=>183, "T_RCURLY"=>184, "K_ALIAS"=>185, "T_GVAR"=>186, "T_BACK_REF"=>187, "T_NTH_REF"=>188, "K_UNDEF"=>189, "K_IF_MOD"=>190, "K_UNLESS_MOD"=>191, "K_WHILE_MOD"=>192, "K_UNTIL_MOD"=>193, "K_RESCUE_MOD"=>194, "K_lEND"=>195, "T_EQL"=>196, "T_OP_ASGN"=>197, "T_LBRACK2"=>198, "T_IDENTIFIER"=>199, "T_CONSTANT"=>200, "T_COLON2"=>201, "K_AND"=>202, "K_OR"=>203, "K_NOT"=>204, "T_BANG"=>205, "T_LBRACE_ARG"=>206, "K_SUPER"=>207, "K_YIELD"=>208, "K_BREAK"=>209, "K_NEXT"=>210, "T_LPAREN"=>211, "T_STAR"=>212, "T_COMMA"=>213, "T_COLON3"=>214, "T_FID"=>215, "T_PIPE"=>216, "T_CARET"=>217, "T_AMPER2"=>218, "T_CMP"=>219, "T_EQ"=>220, "T_EQQ"=>221, "T_MATCH"=>222, "T_NMATCH"=>223, "T_GT"=>224, "T_GEQ"=>225, "T_LT"=>226, "T_LEQ"=>227, "T_NEQ"=>228, "T_LSHFT"=>229, "T_RSHFT"=>230, "T_PLUS"=>231, "T_MINUS"=>232, "T_STAR2"=>233, "T_DIVIDE"=>234, "T_PERCENT"=>235, "T_POW"=>236, "T_TILDE"=>237, "T_UPLUS"=>238, "T_UMINUS"=>239, "T_AREF"=>240, "T_ASET"=>241, "T_DSTAR"=>242, "T_BACK_REF2"=>243, "K___LINE__"=>244, "K___FILE__"=>245, "K___ENCODING__"=>246, "K_BEGIN"=>247, "K_CASE"=>248, "K_CLASS"=>249, "K_DEF"=>250, "K_DEFINED"=>251, "K_DO"=>252, "K_ELSE"=>253, "K_ELSIF"=>254, "K_END"=>255, "K_ENSURE"=>256, "K_FALSE"=>257, "K_FOR"=>258, "K_IN"=>259, "K_MODULE"=>260, "K_NIL"=>261, "K_REDO"=>262, "K_RESCUE"=>263, "K_RETRY"=>264, "K_RETURN"=>265, "K_SELF"=>266, "K_THEN"=>267, "K_TRUE"=>268, "K_WHEN"=>269, "K_IF"=>270, "K_UNLESS"=>271, "K_WHILE"=>272, "K_UNTIL"=>273, "T_DOT2"=>274, "T_DOT3"=>275, "T_UNARY_NUM"=>276, "T_ANDOP"=>277, "T_OROP"=>278, "T_EH"=>279, "T_COLON"=>280, "T_LPAREN2"=>281, "T_AMPER"=>282, "T_LPAREN_ARG"=>283, "T_RPAREN"=>284, "T_LBRACK"=>285, "T_RBRACK"=>286, "T_LBRACE"=>287, "T_LAMBDA"=>288, "K_DO_COND"=>289, "T_SEMI"=>290, "T_LAMBEG"=>291, "K_DO_LAMBDA"=>292, "K_DO_BLOCK"=>293, "T_ASSOC"=>294, "T_STRING_BEG"=>295, "T_STRING_END"=>296, "T_STRING"=>297, "T_CHARACTER"=>298, "T_XSTRING_BEG"=>299, "T_REGEXP_BEG"=>300, "T_REGEXP_OPT"=>301, "T_WORDS_BEG"=>302, "T_SPACE"=>303, "T_SYMBOLS_BEG"=>304, "T_QWORDS_BEG"=>305, "T_QSYMBOLS_BEG"=>306, "T_STRING_CONTENT"=>307, "T_STRING_DVAR"=>308, "T_STRING_DBEG"=>309, "T_STRING_DEND"=>310, "T_IVAR"=>311, "T_CVAR"=>312, "T_SYMBOL"=>313, "T_SYMBEG"=>314, "T_INTEGER"=>315, "T_FLOAT"=>316, "T_RATIONAL"=>317, "T_IMAGINARY"=>318, "T_LABEL"=>319, "T_LABEL_END"=>320, "T_DOT"=>321, "T_ANDDOT"=>322, "T_NL"=>323, "$"=>324};
// END OF TOKENS_MAP

    Token {
        kind: *tokens_map.get("$").unwrap() as i32,
        value: "$",

        interior_token: Box::new(InteriorToken::T_EOF),

        start_offset: 0,
        end_offset: 0,
        start_line: 0,
        end_line: 0,
        start_column: 0,
        end_column: 0,
    }
}
