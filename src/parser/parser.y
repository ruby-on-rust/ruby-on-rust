%type T_INTEGER { i64 }

%include {
}

%derive_token { Debug, Copy, Clone }

%extra_argument { Option<i64> }

%syntax_error { println!("syntax error"); }

%parse_accept {
    println!("parse_accept");
}

%parse_failure {
    println!("parse_failure!");
}

input ::= T_INTEGER(A). {
    self.extra = Some(A);
}

input ::= K_TRUE.

// NOTE
// fake rules for generating tokens for lexer
input ::= K_IF_MOD.
input ::= K_IF.
