%type expr { Expression }
%type VALUE { i32 }
%left PLUS MINUS.
%left TIMES DIV.
%token_class id PLUS VALUE.

%include {
/* extra include */

#[derive(Debug)]
pub enum Operator {
    Addition,
    Substraction,
    Multiplication,
    Division
}

#[derive(Debug)]
pub enum Expression {
    Number(i32),
    Binary(Operator, Box<Expression>, Box<Expression>),
}


}

%derive_token { Debug }

%extra_argument { Option<Expression> }
%syntax_error { println!("syntax error"); }

%parse_accept {
    println!("parse_accept");
}

%parse_failure {
    println!("parse_failure!");
}

input ::= expr(A).                  {
      self.extra = Some(A);
}
expr(A) ::= expr(B) PLUS expr(C).   {
    A = Expression::Binary(Operator::Addition, Box::new(B), Box::new(C));
}
expr(A) ::= expr(B) MINUS expr(C).  {
    A = Expression::Binary(Operator::Substraction, Box::new(B), Box::new(C));
}
expr(A) ::= expr(B) TIMES expr(C).  {
    A = Expression::Binary(Operator::Multiplication, Box::new(B), Box::new(C));
}
expr(A) ::= expr(B) DIV expr(C).    {
    A = Expression::Binary(Operator::Division, Box::new(B), Box::new(C));
}
expr(A) ::= LPAREN expr(B) RPAREN.  {
    A = B;
}
expr(A) ::= VALUE(B).               {
    A = Expression::Number(B);
}
