use crate::token::token::Token as InteriorToken;

use std::collections::HashMap;

impl InteriorToken {
    pub fn wrap_as_token(&self) -> Token {
        println!("#wrap_as_token invoked, self: {:?}", self);

// STARTING OF TOKENS_MAP
let tokens_map: HashMap<&str, isize> = hashmap! {"K_lBEGIN"=>187, "T_LCURLY"=>188, "T_RCURLY"=>189, "K_ALIAS"=>190, "T_GVAR"=>191, "T_BACK_REF"=>192, "T_NTH_REF"=>193, "K_UNDEF"=>194, "K_IF_MOD"=>195, "K_UNLESS_MOD"=>196, "K_WHILE_MOD"=>197, "K_UNTIL_MOD"=>198, "K_RESCUE_MOD"=>199, "K_lEND"=>200, "T_EQL"=>201, "T_OP_ASGN"=>202, "T_LBRACK2"=>203, "T_IDENTIFIER"=>204, "T_CONSTANT"=>205, "T_COLON2"=>206, "K_AND"=>207, "K_OR"=>208, "K_NOT"=>209, "T_BANG"=>210, "T_LBRACE_ARG"=>211, "K_SUPER"=>212, "K_YIELD"=>213, "K_BREAK"=>214, "K_NEXT"=>215, "T_LPAREN"=>216, "T_STAR"=>217, "T_COMMA"=>218, "T_COLON3"=>219, "T_FID"=>220, "T_PIPE"=>221, "T_CARET"=>222, "T_AMPER2"=>223, "T_CMP"=>224, "T_EQ"=>225, "T_EQQ"=>226, "T_MATCH"=>227, "T_NMATCH"=>228, "T_GT"=>229, "T_GEQ"=>230, "T_LT"=>231, "T_LEQ"=>232, "T_NEQ"=>233, "T_LSHFT"=>234, "T_RSHFT"=>235, "T_PLUS"=>236, "T_MINUS"=>237, "T_STAR2"=>238, "T_DIVIDE"=>239, "T_PERCENT"=>240, "T_POW"=>241, "T_TILDE"=>242, "T_UPLUS"=>243, "T_UMINUS"=>244, "T_AREF"=>245, "T_ASET"=>246, "T_DSTAR"=>247, "T_BACK_REF2"=>248, "K___LINE__"=>249, "K___FILE__"=>250, "K___ENCODING__"=>251, "K_BEGIN"=>252, "K_CASE"=>253, "K_CLASS"=>254, "K_DEF"=>255, "K_DEFINED"=>256, "K_DO"=>257, "K_ELSE"=>258, "K_ELSIF"=>259, "K_END"=>260, "K_ENSURE"=>261, "K_FALSE"=>262, "K_FOR"=>263, "K_IN"=>264, "K_MODULE"=>265, "K_NIL"=>266, "K_REDO"=>267, "K_RESCUE"=>268, "K_RETRY"=>269, "K_RETURN"=>270, "K_SELF"=>271, "K_THEN"=>272, "K_TRUE"=>273, "K_WHEN"=>274, "K_IF"=>275, "K_UNLESS"=>276, "K_WHILE"=>277, "K_UNTIL"=>278, "T_DOT2"=>279, "T_DOT3"=>280, "T_UNARY_NUM"=>281, "T_ANDOP"=>282, "T_OROP"=>283, "T_EH"=>284, "T_COLON"=>285, "T_LPAREN2"=>286, "T_AMPER"=>287, "T_LPAREN_ARG"=>288, "T_RPAREN"=>289, "T_LBRACK"=>290, "T_RBRACK"=>291, "T_LBRACE"=>292, "T_LAMBDA"=>293, "K_DO_COND"=>294, "T_SEMI"=>295, "T_LAMBEG"=>296, "K_DO_LAMBDA"=>297, "K_DO_BLOCK"=>298, "T_ASSOC"=>299, "T_STRING_BEG"=>300, "T_STRING_END"=>301, "T_STRING"=>302, "T_CHARACTER"=>303, "T_XSTRING_BEG"=>304, "T_REGEXP_BEG"=>305, "T_REGEXP_OPT"=>306, "T_WORDS_BEG"=>307, "T_SPACE"=>308, "T_SYMBOLS_BEG"=>309, "T_QWORDS_BEG"=>310, "T_QSYMBOLS_BEG"=>311, "T_STRING_CONTENT"=>312, "T_STRING_DVAR"=>313, "T_STRING_DBEG"=>314, "T_STRING_DEND"=>315, "T_IVAR"=>316, "T_CVAR"=>317, "T_SYMBOL"=>318, "T_SYMBEG"=>319, "T_INTEGER"=>320, "T_FLOAT"=>321, "T_RATIONAL"=>322, "T_IMAGINARY"=>323, "T_LABEL"=>324, "T_LABEL_END"=>325, "T_DOT"=>326, "T_ANDDOT"=>327, "T_NL"=>328, "$"=>329};
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
let tokens_map: HashMap<&str, isize> = hashmap! {"K_lBEGIN"=>187, "T_LCURLY"=>188, "T_RCURLY"=>189, "K_ALIAS"=>190, "T_GVAR"=>191, "T_BACK_REF"=>192, "T_NTH_REF"=>193, "K_UNDEF"=>194, "K_IF_MOD"=>195, "K_UNLESS_MOD"=>196, "K_WHILE_MOD"=>197, "K_UNTIL_MOD"=>198, "K_RESCUE_MOD"=>199, "K_lEND"=>200, "T_EQL"=>201, "T_OP_ASGN"=>202, "T_LBRACK2"=>203, "T_IDENTIFIER"=>204, "T_CONSTANT"=>205, "T_COLON2"=>206, "K_AND"=>207, "K_OR"=>208, "K_NOT"=>209, "T_BANG"=>210, "T_LBRACE_ARG"=>211, "K_SUPER"=>212, "K_YIELD"=>213, "K_BREAK"=>214, "K_NEXT"=>215, "T_LPAREN"=>216, "T_STAR"=>217, "T_COMMA"=>218, "T_COLON3"=>219, "T_FID"=>220, "T_PIPE"=>221, "T_CARET"=>222, "T_AMPER2"=>223, "T_CMP"=>224, "T_EQ"=>225, "T_EQQ"=>226, "T_MATCH"=>227, "T_NMATCH"=>228, "T_GT"=>229, "T_GEQ"=>230, "T_LT"=>231, "T_LEQ"=>232, "T_NEQ"=>233, "T_LSHFT"=>234, "T_RSHFT"=>235, "T_PLUS"=>236, "T_MINUS"=>237, "T_STAR2"=>238, "T_DIVIDE"=>239, "T_PERCENT"=>240, "T_POW"=>241, "T_TILDE"=>242, "T_UPLUS"=>243, "T_UMINUS"=>244, "T_AREF"=>245, "T_ASET"=>246, "T_DSTAR"=>247, "T_BACK_REF2"=>248, "K___LINE__"=>249, "K___FILE__"=>250, "K___ENCODING__"=>251, "K_BEGIN"=>252, "K_CASE"=>253, "K_CLASS"=>254, "K_DEF"=>255, "K_DEFINED"=>256, "K_DO"=>257, "K_ELSE"=>258, "K_ELSIF"=>259, "K_END"=>260, "K_ENSURE"=>261, "K_FALSE"=>262, "K_FOR"=>263, "K_IN"=>264, "K_MODULE"=>265, "K_NIL"=>266, "K_REDO"=>267, "K_RESCUE"=>268, "K_RETRY"=>269, "K_RETURN"=>270, "K_SELF"=>271, "K_THEN"=>272, "K_TRUE"=>273, "K_WHEN"=>274, "K_IF"=>275, "K_UNLESS"=>276, "K_WHILE"=>277, "K_UNTIL"=>278, "T_DOT2"=>279, "T_DOT3"=>280, "T_UNARY_NUM"=>281, "T_ANDOP"=>282, "T_OROP"=>283, "T_EH"=>284, "T_COLON"=>285, "T_LPAREN2"=>286, "T_AMPER"=>287, "T_LPAREN_ARG"=>288, "T_RPAREN"=>289, "T_LBRACK"=>290, "T_RBRACK"=>291, "T_LBRACE"=>292, "T_LAMBDA"=>293, "K_DO_COND"=>294, "T_SEMI"=>295, "T_LAMBEG"=>296, "K_DO_LAMBDA"=>297, "K_DO_BLOCK"=>298, "T_ASSOC"=>299, "T_STRING_BEG"=>300, "T_STRING_END"=>301, "T_STRING"=>302, "T_CHARACTER"=>303, "T_XSTRING_BEG"=>304, "T_REGEXP_BEG"=>305, "T_REGEXP_OPT"=>306, "T_WORDS_BEG"=>307, "T_SPACE"=>308, "T_SYMBOLS_BEG"=>309, "T_QWORDS_BEG"=>310, "T_QSYMBOLS_BEG"=>311, "T_STRING_CONTENT"=>312, "T_STRING_DVAR"=>313, "T_STRING_DBEG"=>314, "T_STRING_DEND"=>315, "T_IVAR"=>316, "T_CVAR"=>317, "T_SYMBOL"=>318, "T_SYMBEG"=>319, "T_INTEGER"=>320, "T_FLOAT"=>321, "T_RATIONAL"=>322, "T_IMAGINARY"=>323, "T_LABEL"=>324, "T_LABEL_END"=>325, "T_DOT"=>326, "T_ANDDOT"=>327, "T_NL"=>328, "$"=>329};
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
