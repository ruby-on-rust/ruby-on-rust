use crate::token::token::Token as InteriorToken;

use std::collections::HashMap;

impl InteriorToken {
    pub fn wrap_as_token(&self) -> Token {
        println!("#wrap_as_token invoked, self: {:?}", self);

// STARTING OF TOKENS_MAP
let tokens_map: HashMap<&str, isize> = hashmap! {"K_lBEGIN"=>186, "T_LCURLY"=>187, "T_RCURLY"=>188, "K_ALIAS"=>189, "T_GVAR"=>190, "T_BACK_REF"=>191, "T_NTH_REF"=>192, "K_UNDEF"=>193, "K_IF_MOD"=>194, "K_UNLESS_MOD"=>195, "K_WHILE_MOD"=>196, "K_UNTIL_MOD"=>197, "K_RESCUE_MOD"=>198, "K_lEND"=>199, "T_EQL"=>200, "T_OP_ASGN"=>201, "T_LBRACK2"=>202, "T_IDENTIFIER"=>203, "T_CONSTANT"=>204, "T_COLON2"=>205, "K_AND"=>206, "K_OR"=>207, "K_NOT"=>208, "T_BANG"=>209, "T_LBRACE_ARG"=>210, "K_SUPER"=>211, "K_YIELD"=>212, "K_BREAK"=>213, "K_NEXT"=>214, "T_LPAREN"=>215, "T_STAR"=>216, "T_COMMA"=>217, "T_COLON3"=>218, "T_FID"=>219, "T_PIPE"=>220, "T_CARET"=>221, "T_AMPER2"=>222, "T_CMP"=>223, "T_EQ"=>224, "T_EQQ"=>225, "T_MATCH"=>226, "T_NMATCH"=>227, "T_GT"=>228, "T_GEQ"=>229, "T_LT"=>230, "T_LEQ"=>231, "T_NEQ"=>232, "T_LSHFT"=>233, "T_RSHFT"=>234, "T_PLUS"=>235, "T_MINUS"=>236, "T_STAR2"=>237, "T_DIVIDE"=>238, "T_PERCENT"=>239, "T_POW"=>240, "T_TILDE"=>241, "T_UPLUS"=>242, "T_UMINUS"=>243, "T_AREF"=>244, "T_ASET"=>245, "T_DSTAR"=>246, "T_BACK_REF2"=>247, "K___LINE__"=>248, "K___FILE__"=>249, "K___ENCODING__"=>250, "K_BEGIN"=>251, "K_CASE"=>252, "K_CLASS"=>253, "K_DEF"=>254, "K_DEFINED"=>255, "K_DO"=>256, "K_ELSE"=>257, "K_ELSIF"=>258, "K_END"=>259, "K_ENSURE"=>260, "K_FALSE"=>261, "K_FOR"=>262, "K_IN"=>263, "K_MODULE"=>264, "K_NIL"=>265, "K_REDO"=>266, "K_RESCUE"=>267, "K_RETRY"=>268, "K_RETURN"=>269, "K_SELF"=>270, "K_THEN"=>271, "K_TRUE"=>272, "K_WHEN"=>273, "K_IF"=>274, "K_UNLESS"=>275, "K_WHILE"=>276, "K_UNTIL"=>277, "T_DOT2"=>278, "T_DOT3"=>279, "T_UNARY_NUM"=>280, "T_ANDOP"=>281, "T_OROP"=>282, "T_EH"=>283, "T_COLON"=>284, "T_LPAREN2"=>285, "T_AMPER"=>286, "T_LPAREN_ARG"=>287, "T_RPAREN"=>288, "T_LBRACK"=>289, "T_RBRACK"=>290, "T_LBRACE"=>291, "T_LAMBDA"=>292, "K_DO_COND"=>293, "T_SEMI"=>294, "T_LAMBEG"=>295, "K_DO_LAMBDA"=>296, "K_DO_BLOCK"=>297, "T_ASSOC"=>298, "T_STRING_BEG"=>299, "T_STRING_END"=>300, "T_STRING"=>301, "T_CHARACTER"=>302, "T_XSTRING_BEG"=>303, "T_REGEXP_BEG"=>304, "T_REGEXP_OPT"=>305, "T_WORDS_BEG"=>306, "T_SPACE"=>307, "T_SYMBOLS_BEG"=>308, "T_QWORDS_BEG"=>309, "T_QSYMBOLS_BEG"=>310, "T_STRING_CONTENT"=>311, "T_STRING_DVAR"=>312, "T_STRING_DBEG"=>313, "T_STRING_DEND"=>314, "T_IVAR"=>315, "T_CVAR"=>316, "T_SYMBOL"=>317, "T_SYMBEG"=>318, "T_INTEGER"=>319, "T_FLOAT"=>320, "T_RATIONAL"=>321, "T_IMAGINARY"=>322, "T_LABEL"=>323, "T_LABEL_END"=>324, "T_DOT"=>325, "T_ANDDOT"=>326, "T_NL"=>327, "$"=>328};
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
let tokens_map: HashMap<&str, isize> = hashmap! {"K_lBEGIN"=>186, "T_LCURLY"=>187, "T_RCURLY"=>188, "K_ALIAS"=>189, "T_GVAR"=>190, "T_BACK_REF"=>191, "T_NTH_REF"=>192, "K_UNDEF"=>193, "K_IF_MOD"=>194, "K_UNLESS_MOD"=>195, "K_WHILE_MOD"=>196, "K_UNTIL_MOD"=>197, "K_RESCUE_MOD"=>198, "K_lEND"=>199, "T_EQL"=>200, "T_OP_ASGN"=>201, "T_LBRACK2"=>202, "T_IDENTIFIER"=>203, "T_CONSTANT"=>204, "T_COLON2"=>205, "K_AND"=>206, "K_OR"=>207, "K_NOT"=>208, "T_BANG"=>209, "T_LBRACE_ARG"=>210, "K_SUPER"=>211, "K_YIELD"=>212, "K_BREAK"=>213, "K_NEXT"=>214, "T_LPAREN"=>215, "T_STAR"=>216, "T_COMMA"=>217, "T_COLON3"=>218, "T_FID"=>219, "T_PIPE"=>220, "T_CARET"=>221, "T_AMPER2"=>222, "T_CMP"=>223, "T_EQ"=>224, "T_EQQ"=>225, "T_MATCH"=>226, "T_NMATCH"=>227, "T_GT"=>228, "T_GEQ"=>229, "T_LT"=>230, "T_LEQ"=>231, "T_NEQ"=>232, "T_LSHFT"=>233, "T_RSHFT"=>234, "T_PLUS"=>235, "T_MINUS"=>236, "T_STAR2"=>237, "T_DIVIDE"=>238, "T_PERCENT"=>239, "T_POW"=>240, "T_TILDE"=>241, "T_UPLUS"=>242, "T_UMINUS"=>243, "T_AREF"=>244, "T_ASET"=>245, "T_DSTAR"=>246, "T_BACK_REF2"=>247, "K___LINE__"=>248, "K___FILE__"=>249, "K___ENCODING__"=>250, "K_BEGIN"=>251, "K_CASE"=>252, "K_CLASS"=>253, "K_DEF"=>254, "K_DEFINED"=>255, "K_DO"=>256, "K_ELSE"=>257, "K_ELSIF"=>258, "K_END"=>259, "K_ENSURE"=>260, "K_FALSE"=>261, "K_FOR"=>262, "K_IN"=>263, "K_MODULE"=>264, "K_NIL"=>265, "K_REDO"=>266, "K_RESCUE"=>267, "K_RETRY"=>268, "K_RETURN"=>269, "K_SELF"=>270, "K_THEN"=>271, "K_TRUE"=>272, "K_WHEN"=>273, "K_IF"=>274, "K_UNLESS"=>275, "K_WHILE"=>276, "K_UNTIL"=>277, "T_DOT2"=>278, "T_DOT3"=>279, "T_UNARY_NUM"=>280, "T_ANDOP"=>281, "T_OROP"=>282, "T_EH"=>283, "T_COLON"=>284, "T_LPAREN2"=>285, "T_AMPER"=>286, "T_LPAREN_ARG"=>287, "T_RPAREN"=>288, "T_LBRACK"=>289, "T_RBRACK"=>290, "T_LBRACE"=>291, "T_LAMBDA"=>292, "K_DO_COND"=>293, "T_SEMI"=>294, "T_LAMBEG"=>295, "K_DO_LAMBDA"=>296, "K_DO_BLOCK"=>297, "T_ASSOC"=>298, "T_STRING_BEG"=>299, "T_STRING_END"=>300, "T_STRING"=>301, "T_CHARACTER"=>302, "T_XSTRING_BEG"=>303, "T_REGEXP_BEG"=>304, "T_REGEXP_OPT"=>305, "T_WORDS_BEG"=>306, "T_SPACE"=>307, "T_SYMBOLS_BEG"=>308, "T_QWORDS_BEG"=>309, "T_QSYMBOLS_BEG"=>310, "T_STRING_CONTENT"=>311, "T_STRING_DVAR"=>312, "T_STRING_DBEG"=>313, "T_STRING_DEND"=>314, "T_IVAR"=>315, "T_CVAR"=>316, "T_SYMBOL"=>317, "T_SYMBEG"=>318, "T_INTEGER"=>319, "T_FLOAT"=>320, "T_RATIONAL"=>321, "T_IMAGINARY"=>322, "T_LABEL"=>323, "T_LABEL_END"=>324, "T_DOT"=>325, "T_ANDDOT"=>326, "T_NL"=>327, "$"=>328};
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
