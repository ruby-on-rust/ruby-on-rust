use crate::token::token::Token as InteriorToken;

use std::collections::HashMap;

impl InteriorToken {
    pub fn wrap_as_token(&self) -> Token {
        println!("#wrap_as_token invoked, self: {:?}", self);

// STARTING OF TOKENS_MAP
let tokens_map: HashMap<&str, isize> = hashmap! {"K_lBEGIN"=>183, "T_LCURLY"=>184, "T_RCURLY"=>185, "K_ALIAS"=>186, "T_GVAR"=>187, "T_BACK_REF"=>188, "T_NTH_REF"=>189, "K_UNDEF"=>190, "K_IF_MOD"=>191, "K_UNLESS_MOD"=>192, "K_WHILE_MOD"=>193, "K_UNTIL_MOD"=>194, "K_RESCUE_MOD"=>195, "K_lEND"=>196, "T_EQL"=>197, "T_OP_ASGN"=>198, "T_LBRACK2"=>199, "T_IDENTIFIER"=>200, "T_CONSTANT"=>201, "T_COLON2"=>202, "K_AND"=>203, "K_OR"=>204, "K_NOT"=>205, "T_BANG"=>206, "T_LBRACE_ARG"=>207, "K_SUPER"=>208, "K_YIELD"=>209, "K_BREAK"=>210, "K_NEXT"=>211, "T_LPAREN"=>212, "T_STAR"=>213, "T_COMMA"=>214, "T_COLON3"=>215, "T_FID"=>216, "T_PIPE"=>217, "T_CARET"=>218, "T_AMPER2"=>219, "T_CMP"=>220, "T_EQ"=>221, "T_EQQ"=>222, "T_MATCH"=>223, "T_NMATCH"=>224, "T_GT"=>225, "T_GEQ"=>226, "T_LT"=>227, "T_LEQ"=>228, "T_NEQ"=>229, "T_LSHFT"=>230, "T_RSHFT"=>231, "T_PLUS"=>232, "T_MINUS"=>233, "T_STAR2"=>234, "T_DIVIDE"=>235, "T_PERCENT"=>236, "T_POW"=>237, "T_TILDE"=>238, "T_UPLUS"=>239, "T_UMINUS"=>240, "T_AREF"=>241, "T_ASET"=>242, "T_DSTAR"=>243, "T_BACK_REF2"=>244, "K___LINE__"=>245, "K___FILE__"=>246, "K___ENCODING__"=>247, "K_BEGIN"=>248, "K_CASE"=>249, "K_CLASS"=>250, "K_DEF"=>251, "K_DEFINED"=>252, "K_DO"=>253, "K_ELSE"=>254, "K_ELSIF"=>255, "K_END"=>256, "K_ENSURE"=>257, "K_FALSE"=>258, "K_FOR"=>259, "K_IN"=>260, "K_MODULE"=>261, "K_NIL"=>262, "K_REDO"=>263, "K_RESCUE"=>264, "K_RETRY"=>265, "K_RETURN"=>266, "K_SELF"=>267, "K_THEN"=>268, "K_TRUE"=>269, "K_WHEN"=>270, "K_IF"=>271, "K_UNLESS"=>272, "K_WHILE"=>273, "K_UNTIL"=>274, "T_DOT2"=>275, "T_DOT3"=>276, "T_UNARY_NUM"=>277, "T_ANDOP"=>278, "T_OROP"=>279, "T_EH"=>280, "T_COLON"=>281, "T_LPAREN2"=>282, "T_AMPER"=>283, "T_LPAREN_ARG"=>284, "T_RPAREN"=>285, "T_LBRACK"=>286, "T_RBRACK"=>287, "T_LBRACE"=>288, "T_LAMBDA"=>289, "K_DO_COND"=>290, "T_SEMI"=>291, "T_LAMBEG"=>292, "K_DO_LAMBDA"=>293, "K_DO_BLOCK"=>294, "T_ASSOC"=>295, "T_STRING_BEG"=>296, "T_STRING_END"=>297, "T_STRING"=>298, "T_CHARACTER"=>299, "T_XSTRING_BEG"=>300, "T_REGEXP_BEG"=>301, "T_REGEXP_OPT"=>302, "T_WORDS_BEG"=>303, "T_SPACE"=>304, "T_SYMBOLS_BEG"=>305, "T_QWORDS_BEG"=>306, "T_QSYMBOLS_BEG"=>307, "T_STRING_CONTENT"=>308, "T_STRING_DVAR"=>309, "T_STRING_DBEG"=>310, "T_STRING_DEND"=>311, "T_IVAR"=>312, "T_CVAR"=>313, "T_SYMBOL"=>314, "T_SYMBEG"=>315, "T_INTEGER"=>316, "T_FLOAT"=>317, "T_RATIONAL"=>318, "T_IMAGINARY"=>319, "T_LABEL"=>320, "T_LABEL_END"=>321, "T_DOT"=>322, "T_ANDDOT"=>323, "T_NL"=>324, "$"=>325};
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
let tokens_map: HashMap<&str, isize> = hashmap! {"K_lBEGIN"=>183, "T_LCURLY"=>184, "T_RCURLY"=>185, "K_ALIAS"=>186, "T_GVAR"=>187, "T_BACK_REF"=>188, "T_NTH_REF"=>189, "K_UNDEF"=>190, "K_IF_MOD"=>191, "K_UNLESS_MOD"=>192, "K_WHILE_MOD"=>193, "K_UNTIL_MOD"=>194, "K_RESCUE_MOD"=>195, "K_lEND"=>196, "T_EQL"=>197, "T_OP_ASGN"=>198, "T_LBRACK2"=>199, "T_IDENTIFIER"=>200, "T_CONSTANT"=>201, "T_COLON2"=>202, "K_AND"=>203, "K_OR"=>204, "K_NOT"=>205, "T_BANG"=>206, "T_LBRACE_ARG"=>207, "K_SUPER"=>208, "K_YIELD"=>209, "K_BREAK"=>210, "K_NEXT"=>211, "T_LPAREN"=>212, "T_STAR"=>213, "T_COMMA"=>214, "T_COLON3"=>215, "T_FID"=>216, "T_PIPE"=>217, "T_CARET"=>218, "T_AMPER2"=>219, "T_CMP"=>220, "T_EQ"=>221, "T_EQQ"=>222, "T_MATCH"=>223, "T_NMATCH"=>224, "T_GT"=>225, "T_GEQ"=>226, "T_LT"=>227, "T_LEQ"=>228, "T_NEQ"=>229, "T_LSHFT"=>230, "T_RSHFT"=>231, "T_PLUS"=>232, "T_MINUS"=>233, "T_STAR2"=>234, "T_DIVIDE"=>235, "T_PERCENT"=>236, "T_POW"=>237, "T_TILDE"=>238, "T_UPLUS"=>239, "T_UMINUS"=>240, "T_AREF"=>241, "T_ASET"=>242, "T_DSTAR"=>243, "T_BACK_REF2"=>244, "K___LINE__"=>245, "K___FILE__"=>246, "K___ENCODING__"=>247, "K_BEGIN"=>248, "K_CASE"=>249, "K_CLASS"=>250, "K_DEF"=>251, "K_DEFINED"=>252, "K_DO"=>253, "K_ELSE"=>254, "K_ELSIF"=>255, "K_END"=>256, "K_ENSURE"=>257, "K_FALSE"=>258, "K_FOR"=>259, "K_IN"=>260, "K_MODULE"=>261, "K_NIL"=>262, "K_REDO"=>263, "K_RESCUE"=>264, "K_RETRY"=>265, "K_RETURN"=>266, "K_SELF"=>267, "K_THEN"=>268, "K_TRUE"=>269, "K_WHEN"=>270, "K_IF"=>271, "K_UNLESS"=>272, "K_WHILE"=>273, "K_UNTIL"=>274, "T_DOT2"=>275, "T_DOT3"=>276, "T_UNARY_NUM"=>277, "T_ANDOP"=>278, "T_OROP"=>279, "T_EH"=>280, "T_COLON"=>281, "T_LPAREN2"=>282, "T_AMPER"=>283, "T_LPAREN_ARG"=>284, "T_RPAREN"=>285, "T_LBRACK"=>286, "T_RBRACK"=>287, "T_LBRACE"=>288, "T_LAMBDA"=>289, "K_DO_COND"=>290, "T_SEMI"=>291, "T_LAMBEG"=>292, "K_DO_LAMBDA"=>293, "K_DO_BLOCK"=>294, "T_ASSOC"=>295, "T_STRING_BEG"=>296, "T_STRING_END"=>297, "T_STRING"=>298, "T_CHARACTER"=>299, "T_XSTRING_BEG"=>300, "T_REGEXP_BEG"=>301, "T_REGEXP_OPT"=>302, "T_WORDS_BEG"=>303, "T_SPACE"=>304, "T_SYMBOLS_BEG"=>305, "T_QWORDS_BEG"=>306, "T_QSYMBOLS_BEG"=>307, "T_STRING_CONTENT"=>308, "T_STRING_DVAR"=>309, "T_STRING_DBEG"=>310, "T_STRING_DEND"=>311, "T_IVAR"=>312, "T_CVAR"=>313, "T_SYMBOL"=>314, "T_SYMBEG"=>315, "T_INTEGER"=>316, "T_FLOAT"=>317, "T_RATIONAL"=>318, "T_IMAGINARY"=>319, "T_LABEL"=>320, "T_LABEL_END"=>321, "T_DOT"=>322, "T_ANDDOT"=>323, "T_NL"=>324, "$"=>325};
// END OF TOKENS_MAP

    Token {
        kind: *tokens_map.get("$").unwrap() as i32,
        value: "$",

        // interior_token: Box::new(InteriorToken::T_EOF),
        // TODO REF https://github.com/rust-lang/rust/issues/49683
        interior_token: Box::new(InteriorToken::T_EOF),

        start_offset: 0,
        end_offset: 0,
        start_line: 0,
        end_line: 0,
        start_column: 0,
        end_column: 0,
    }
}
