use ::token::token::Token as InteriorToken;

use std::collections::HashMap;

impl InteriorToken {
    pub fn wrap_as_token(&self) -> Token {
        println!("#wrap_as_token invoked, self: {:?}", self);

// STARTING OF TOKENS_MAP
let tokens_map: HashMap<&str, isize> = hashmap! {"K_lBEGIN"=>183, "T_LCURLY"=>184, "T_RCURLY"=>185, "K_ALIAS"=>186, "T_GVAR"=>187, "T_BACK_REF"=>188, "T_NTH_REF"=>189, "K_UNDEF"=>190, "K_IF_MOD"=>191, "K_UNLESS_MOD"=>192, "K_WHILE_MOD"=>193, "K_UNTIL_MOD"=>194, "K_RESCUE_MOD"=>195, "K_lEND"=>196, "T_EQL"=>197, "T_OP_ASGN"=>198, "T_LBRACK2"=>199, "T_IDENTIFIER"=>200, "T_CONSTANT"=>201, "T_COLON2"=>202, "K_AND"=>203, "K_OR"=>204, "K_NOT"=>205, "T_BANG"=>206, "T_LBRACE_ARG"=>207, "K_SUPER"=>208, "K_YIELD"=>209, "K_RETURN"=>210, "K_BREAK"=>211, "K_NEXT"=>212, "T_LPAREN"=>213, "T_STAR"=>214, "T_COMMA"=>215, "T_COLON3"=>216, "T_FID"=>217, "T_PIPE"=>218, "T_CARET"=>219, "T_AMPER2"=>220, "T_CMP"=>221, "T_EQ"=>222, "T_EQQ"=>223, "T_MATCH"=>224, "T_NMATCH"=>225, "T_GT"=>226, "T_GEQ"=>227, "T_LT"=>228, "T_LEQ"=>229, "T_NEQ"=>230, "T_LSHFT"=>231, "T_RSHFT"=>232, "T_PLUS"=>233, "T_MINUS"=>234, "T_STAR2"=>235, "T_DIVIDE"=>236, "T_PERCENT"=>237, "T_POW"=>238, "T_TILDE"=>239, "T_UPLUS"=>240, "T_UMINUS"=>241, "T_AREF"=>242, "T_ASET"=>243, "T_DSTAR"=>244, "T_BACK_REF2"=>245, "K___LINE__"=>246, "K___FILE__"=>247, "K___ENCODING__"=>248, "K_BEGIN"=>249, "K_CASE"=>250, "K_CLASS"=>251, "K_DEF"=>252, "K_DEFINED"=>253, "K_DO"=>254, "K_ELSE"=>255, "K_ELSIF"=>256, "K_END"=>257, "K_ENSURE"=>258, "K_FALSE"=>259, "K_FOR"=>260, "K_IN"=>261, "K_MODULE"=>262, "K_NIL"=>263, "K_REDO"=>264, "K_RESCUE"=>265, "K_RETRY"=>266, "K_SELF"=>267, "K_THEN"=>268, "K_TRUE"=>269, "K_WHEN"=>270, "K_IF"=>271, "K_UNLESS"=>272, "K_WHILE"=>273, "K_UNTIL"=>274, "T_DOT2"=>275, "T_DOT3"=>276, "T_UNARY_NUM"=>277, "T_ANDOP"=>278, "T_OROP"=>279, "T_EH"=>280, "T_COLON"=>281, "T_LPAREN2"=>282, "T_AMPER"=>283, "T_LPAREN_ARG"=>284, "T_RPAREN"=>285, "T_LBRACK"=>286, "T_RBRACK"=>287, "T_LBRACE"=>288, "T_LAMBDA"=>289, "K_DO_COND"=>290, "T_SEMI"=>291, "T_LAMBEG"=>292, "K_DO_LAMBDA"=>293, "K_DO_BLOCK"=>294, "T_ASSOC"=>295, "T_STRING_BEG"=>296, "T_STRING_END"=>297, "T_STRING"=>298, "T_CHARACTER"=>299, "T_XSTRING_BEG"=>300, "T_REGEXP_BEG"=>301, "T_REGEXP_OPT"=>302, "T_WORDS_BEG"=>303, "T_SPACE"=>304, "T_SYMBOLS_BEG"=>305, "T_QWORDS_BEG"=>306, "T_QSYMBOLS_BEG"=>307, "T_STRING_CONTENT"=>308, "T_STRING_DVAR"=>309, "T_STRING_DBEG"=>310, "T_STRING_DEND"=>311, "T_IVAR"=>312, "T_CVAR"=>313, "T_SYMBOL"=>314, "T_SYMBEG"=>315, "T_INTEGER"=>316, "T_FLOAT"=>317, "T_RATIONAL"=>318, "T_IMAGINARY"=>319, "T_LABEL"=>320, "T_LABEL_END"=>321, "T_DOT"=>322, "T_ANDDOT"=>323, "T_NL"=>324, "$"=>325};
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
let tokens_map: HashMap<&str, isize> = hashmap! {"K_lBEGIN"=>183, "T_LCURLY"=>184, "T_RCURLY"=>185, "K_ALIAS"=>186, "T_GVAR"=>187, "T_BACK_REF"=>188, "T_NTH_REF"=>189, "K_UNDEF"=>190, "K_IF_MOD"=>191, "K_UNLESS_MOD"=>192, "K_WHILE_MOD"=>193, "K_UNTIL_MOD"=>194, "K_RESCUE_MOD"=>195, "K_lEND"=>196, "T_EQL"=>197, "T_OP_ASGN"=>198, "T_LBRACK2"=>199, "T_IDENTIFIER"=>200, "T_CONSTANT"=>201, "T_COLON2"=>202, "K_AND"=>203, "K_OR"=>204, "K_NOT"=>205, "T_BANG"=>206, "T_LBRACE_ARG"=>207, "K_SUPER"=>208, "K_YIELD"=>209, "K_RETURN"=>210, "K_BREAK"=>211, "K_NEXT"=>212, "T_LPAREN"=>213, "T_STAR"=>214, "T_COMMA"=>215, "T_COLON3"=>216, "T_FID"=>217, "T_PIPE"=>218, "T_CARET"=>219, "T_AMPER2"=>220, "T_CMP"=>221, "T_EQ"=>222, "T_EQQ"=>223, "T_MATCH"=>224, "T_NMATCH"=>225, "T_GT"=>226, "T_GEQ"=>227, "T_LT"=>228, "T_LEQ"=>229, "T_NEQ"=>230, "T_LSHFT"=>231, "T_RSHFT"=>232, "T_PLUS"=>233, "T_MINUS"=>234, "T_STAR2"=>235, "T_DIVIDE"=>236, "T_PERCENT"=>237, "T_POW"=>238, "T_TILDE"=>239, "T_UPLUS"=>240, "T_UMINUS"=>241, "T_AREF"=>242, "T_ASET"=>243, "T_DSTAR"=>244, "T_BACK_REF2"=>245, "K___LINE__"=>246, "K___FILE__"=>247, "K___ENCODING__"=>248, "K_BEGIN"=>249, "K_CASE"=>250, "K_CLASS"=>251, "K_DEF"=>252, "K_DEFINED"=>253, "K_DO"=>254, "K_ELSE"=>255, "K_ELSIF"=>256, "K_END"=>257, "K_ENSURE"=>258, "K_FALSE"=>259, "K_FOR"=>260, "K_IN"=>261, "K_MODULE"=>262, "K_NIL"=>263, "K_REDO"=>264, "K_RESCUE"=>265, "K_RETRY"=>266, "K_SELF"=>267, "K_THEN"=>268, "K_TRUE"=>269, "K_WHEN"=>270, "K_IF"=>271, "K_UNLESS"=>272, "K_WHILE"=>273, "K_UNTIL"=>274, "T_DOT2"=>275, "T_DOT3"=>276, "T_UNARY_NUM"=>277, "T_ANDOP"=>278, "T_OROP"=>279, "T_EH"=>280, "T_COLON"=>281, "T_LPAREN2"=>282, "T_AMPER"=>283, "T_LPAREN_ARG"=>284, "T_RPAREN"=>285, "T_LBRACK"=>286, "T_RBRACK"=>287, "T_LBRACE"=>288, "T_LAMBDA"=>289, "K_DO_COND"=>290, "T_SEMI"=>291, "T_LAMBEG"=>292, "K_DO_LAMBDA"=>293, "K_DO_BLOCK"=>294, "T_ASSOC"=>295, "T_STRING_BEG"=>296, "T_STRING_END"=>297, "T_STRING"=>298, "T_CHARACTER"=>299, "T_XSTRING_BEG"=>300, "T_REGEXP_BEG"=>301, "T_REGEXP_OPT"=>302, "T_WORDS_BEG"=>303, "T_SPACE"=>304, "T_SYMBOLS_BEG"=>305, "T_QWORDS_BEG"=>306, "T_QSYMBOLS_BEG"=>307, "T_STRING_CONTENT"=>308, "T_STRING_DVAR"=>309, "T_STRING_DBEG"=>310, "T_STRING_DEND"=>311, "T_IVAR"=>312, "T_CVAR"=>313, "T_SYMBOL"=>314, "T_SYMBEG"=>315, "T_INTEGER"=>316, "T_FLOAT"=>317, "T_RATIONAL"=>318, "T_IMAGINARY"=>319, "T_LABEL"=>320, "T_LABEL_END"=>321, "T_DOT"=>322, "T_ANDDOT"=>323, "T_NL"=>324, "$"=>325};
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
