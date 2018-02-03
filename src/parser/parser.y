%type T_INTEGER { i64 }

%include {
}

%derive_token { Debug }

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
