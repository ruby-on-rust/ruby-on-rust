use ::token::token::Token as InteriorToken;

use std::collections::HashMap;

impl InteriorToken {
    pub fn wrap_as_token(&self) -> Token {
        println!("#wrap_as_token invoked, self: {:?}", self);

// STARTING OF TOKENS_MAP
let tokens_map: HashMap<&str, isize> = hashmap! {"T_EQL"=>40, "T_COMMA"=>41, "T_COLON2"=>42, "T_CONSTANT"=>43, "T_COLON3"=>44, "T_LBRACK"=>45, "T_RBRACK"=>46, "T_LBRACE"=>47, "T_RCURLY"=>48, "T_STRING_BEG"=>49, "T_STRING_END"=>50, "T_STRING"=>51, "T_WORDS_BEG"=>52, "T_SPACE"=>53, "T_QWORDS_BEG"=>54, "T_STRING_CONTENT"=>55, "T_SYMBOL"=>56, "T_SYMBEG"=>57, "T_INTEGER"=>58, "T_IDENTIFIER"=>59, "T_IVAR"=>60, "T_GVAR"=>61, "T_CVAR"=>62, "K_NIL"=>63, "K_SELF"=>64, "K_TRUE"=>65, "K_FALSE"=>66, "T_ASSOC"=>67, "T_LABEL"=>68, "T_NL"=>69, "T_SEMI"=>70, "$"=>71};
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
let tokens_map: HashMap<&str, isize> = hashmap! {"T_EQL"=>40, "T_COMMA"=>41, "T_COLON2"=>42, "T_CONSTANT"=>43, "T_COLON3"=>44, "T_LBRACK"=>45, "T_RBRACK"=>46, "T_LBRACE"=>47, "T_RCURLY"=>48, "T_STRING_BEG"=>49, "T_STRING_END"=>50, "T_STRING"=>51, "T_WORDS_BEG"=>52, "T_SPACE"=>53, "T_QWORDS_BEG"=>54, "T_STRING_CONTENT"=>55, "T_SYMBOL"=>56, "T_SYMBEG"=>57, "T_INTEGER"=>58, "T_IDENTIFIER"=>59, "T_IVAR"=>60, "T_GVAR"=>61, "T_CVAR"=>62, "K_NIL"=>63, "K_SELF"=>64, "K_TRUE"=>65, "K_FALSE"=>66, "T_ASSOC"=>67, "T_LABEL"=>68, "T_NL"=>69, "T_SEMI"=>70, "$"=>71};
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
