%type T_INTEGER { i64 }

%type T_GVAR { TokenString }
%type T_CONSTANT { TokenString }
%type T_IDENTIFIER { TokenString }
%type T_LABLE { TokenString }
%type T_IVAR { TokenString }
%type T_FID { TokenString }

%include {

pub type TokenString = String;

}

%derive_token { Debug, Clone }

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

// NOTE
// fake rules for generating tokens for lexer

input ::= K_ALIAS.
input ::= K_AND.
input ::= K_BEGIN.
input ::= K_BREAK.
input ::= K_CASE.
input ::= K_CLASS.
input ::= K_DEF.
input ::= K_DEFINED.
input ::= K_DO.
input ::= K_DO_BLOCK.
input ::= K_DO_COND.
input ::= K_ELSE.
input ::= K_ELSIF.
input ::= K_END.
input ::= K_ENSURE.
input ::= K_FALSE.
input ::= K_FOR.
input ::= K_IF.
input ::= K_IF_MOD.
input ::= K_IN.
input ::= K_LBEGIN.
input ::= K_LEND.
input ::= K_MODULE.
input ::= K_NEXT.
input ::= K_NIL.
input ::= K_NOT.
input ::= K_OR.
input ::= K_REDO.
input ::= K_RESCUE.
input ::= K_RESCUE_MOD.
input ::= K_RETRY.
input ::= K_RETURN.
input ::= K_SELF.
input ::= K_SUPER.
input ::= K_THEN.
input ::= K_TRUE.
input ::= K_UNDEF.
input ::= K_UNLESS.
input ::= K_UNLESS_MOD.
input ::= K_UNTIL.
input ::= K_UNTIL_MOD.
input ::= K_WHEN.
input ::= K_WHILE.
input ::= K_WHILE_MOD.
input ::= K_YIELD.
input ::= K__ENCODING__.
input ::= K__FILE__.
input ::= K__LINE__.
input ::= T_AMPER.
input ::= T_AMPER2.
input ::= T_ANDDOT.
input ::= T_ANDOP.
input ::= T_AREF.
input ::= T_ASET.
input ::= T_ASSOC.
input ::= T_BACK_REF2.
input ::= T_BANG.
input ::= T_CARET.
input ::= T_CMP.
input ::= T_COLON.
input ::= T_COLON2.
input ::= T_COLON3.
input ::= T_COMMA.
input ::= T_DIVIDE.
input ::= T_DOT.
input ::= T_DOT2.
input ::= T_DOT3.
input ::= T_DSTAR.
input ::= T_EH.
input ::= T_EQ.
input ::= T_EQL.
input ::= T_EQQ.
input ::= T_GEQ.
input ::= T_GT.
input ::= T_LAMBEG.
input ::= T_LAMBDA.
input ::= T_LBRACE.
input ::= T_LBRACE_ARG.
input ::= T_LBRACK.
input ::= T_LBRACK2.
input ::= T_LCURLY.
input ::= T_LEQ.
input ::= T_LPAREN.
input ::= T_LPAREN2.
input ::= T_LPAREN_ARG.
input ::= T_LSHFT.
input ::= T_LT.
input ::= T_MATCH.
input ::= T_MINUS.
input ::= T_NEQ.
input ::= T_NMATCH.
input ::= T_OROP.
input ::= T_PERCENT.
input ::= T_PIPE.
input ::= T_PLUS.
input ::= T_POW.
input ::= T_RBRACK.
input ::= T_RCURLY.
input ::= T_RPAREN.
input ::= T_RSHFT.
input ::= T_SEMI.
input ::= T_STAR.
input ::= T_STAR2.
input ::= T_TILDE.
input ::= T_UMINUS.
input ::= T_UPLUS.
