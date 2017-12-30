#![allow(dead_code)]
#![allow(unused_variables)]
/* TMPL: %include */

/* extra include */

#[derive(Debug)]
pub enum Operator {
    Addition,
    Substraction,
}

#[derive(Debug)]
pub enum Expression {
    Number(i64),
    Binary(Operator, Box<Expression>, Box<Expression>),
}


/* TMPL: makeheader cruft */


/* TMPL: types */

type YYCODETYPE = i8;
const YYNOCODE: i32 = 10;
type YYACTIONTYPE = u8;
const YYWILDCARD: YYCODETYPE = 0;
enum YYMinorType {
    YY0,
    YY5(Expression),
    YY14(i64),
}
const YYNSTATE: i32 = 10;
const YYNRULE: i32 = 5;
const YYERRORSYMBOL: i32 = 0;

//const YY_NO_ACTION: i32 = YYNSTATE+YYNRULE+2;
//const YY_ACCEPT_ACTION: i32 = YYNSTATE+YYNRULE+1;
//const YY_ERROR_ACTION: i32 = YYNSTATE+YYNRULE+1;

/* TMPL: action tables */

#[derive( Debug 
)]
pub enum Token {
    EOI, //0
    VALUE( i64 ), //1
    OP_PLUS, //2
    OP_MINUS, //3
    LPAREN, //4
    RPAREN, //5
}
pub const TOKEN_EOI: i32 = 0;
pub const TOKEN_VALUE: i32 = 1;
pub const TOKEN_OP_PLUS: i32 = 2;
pub const TOKEN_OP_MINUS: i32 = 3;
pub const TOKEN_LPAREN: i32 = 4;
pub const TOKEN_RPAREN: i32 = 5;
#[inline]
fn token_value(t: Token) -> (i32, YYMinorType) {
  match t {
        Token::EOI => (0, YYMinorType::YY0),
        Token::VALUE(x) => (1, YYMinorType::YY14(x)),
        Token::OP_PLUS => (2, YYMinorType::YY0),
        Token::OP_MINUS => (3, YYMinorType::YY0),
        Token::LPAREN => (4, YYMinorType::YY0),
        Token::RPAREN => (5, YYMinorType::YY0),
  }
}
const YY_ACTTAB_COUNT: i32 = 13;
const YY_ACTION: [YYACTIONTYPE; 13] = [
 /*     0 */    10,    9,    3,    2,    8,    3,    2,    6,    7,    4,
 /*    10 */     1,    5,   16,
];
const YY_LOOKAHEAD: [YYCODETYPE; 13] = [
 /*     0 */     0,    7,    2,    3,    7,    2,    3,    1,    5,    7,
 /*    10 */     4,    7,    8,
];
const YY_SHIFT_USE_DFLT: i32 = -1;
const YY_SHIFT_COUNT: i32 = 5;
const YY_SHIFT_MIN: i32 = 0;
const YY_SHIFT_MAX: i32 = 6;
const YY_SHIFT_OFST: [i8; 6] = [
 /*     0 */     6,    6,    6,    6,    3,    0,
];
const YY_REDUCE_USE_DFLT: i32 = -7;
const YY_REDUCE_COUNT: i32 = 3;
const YY_REDUCE_MIN: i32 = -6;
const YY_REDUCE_MAX: i32 = 4;
const YY_REDUCE_OFST: [i8; 4] = [
 /*     0 */     4,    2,   -3,   -6,
];
const YY_DEFAULT: [YYACTIONTYPE; 10] = [
 /*     0 */    15,   15,   15,   15,   15,   15,   14,   13,   12,   11,
];

/* TMPL: fallback tokens */

const YY_FALLBACK: [i32; 0] = [
];

/* TMPL: symbol names */


/* TMPL: rules */


/* TMPL: destructors */


/* TMPL: stack-overflow */


/* TMPL: stack-overflow */

const YY_RULE_INFO: [YYCODETYPE; 5] = [
  8,
  7,
  7,
  7,
  7,
];

struct YYStackEntry {
    stateno: i32, /* The state-number */
    major: i32,     /* The major token value.  This is the code
                            ** number for the token at this stack level */
    minor: YYMinorType,    /* The user-supplied minor token value.  This
                            ** is the value of the token  */
}

pub struct Parser {
    yyerrcnt: i32, /* Shifts left before out of the error */
    yystack: Vec<YYStackEntry>,
    extra:  Option<Expression> ,
}

impl Parser {

    pub fn new(
            extra:  Option<Expression> ,
        ) -> Parser {
        let mut p = Parser { yyerrcnt: -1, yystack: Vec::new(), extra: extra};
        p.yystack.push(YYStackEntry{stateno: 0, major: 0, minor: YYMinorType::YY0});
        p
    }

    pub fn into_extra(self) ->  Option<Expression>  {
        self.extra
    }
    pub fn extra(&self) -> & Option<Expression>  {
        &self.extra
    }

    pub fn parse(&mut self, token: Token) {
        let (yymajor, yyminor) = token_value(token);
        let yyendofinput = yymajor==0;
        let mut yyerrorhit = false;
        while !self.yystack.is_empty() {
            let yyact = self.find_shift_action(yymajor);
            if yyact < YYNSTATE {
                assert!(!yyendofinput);  /* Impossible to shift the $ token */
                self.yy_shift(yyact, yymajor, yyminor);
                self.yyerrcnt -= 1;
                break;
            } else if yyact < YYNSTATE + YYNRULE {
                self.yy_reduce(yyact - YYNSTATE);
            } else {
                /* A syntax error has occurred.
                 ** The response to an error depends upon whether or not the
                 ** grammar defines an error token "ERROR".
                 */
                assert!(yyact == YYNSTATE+YYNRULE);
                if YYERRORSYMBOL != 0 {
                    /* This is what we do if the grammar does define ERROR:
                     **
                     **  * Call the %syntax_error function.
                     **
                     **  * Begin popping the stack until we enter a state where
                     **    it is legal to shift the error symbol, then shift
                     **    the error symbol.
                     **
                     **  * Set the error count to three.
                     **
                     **  * Begin accepting and shifting new tokens.  No new error
                     **    processing will occur until three tokens have been
                     **    shifted successfully.
                     **
                     */
                    if self.yyerrcnt < 0 {
                        self.yy_syntax_error(yymajor, &yyminor);
                    }
                    let yymx = self.yystack[self.yystack.len() - 1].major;
                    if yymx==YYERRORSYMBOL || yyerrorhit {
                        break;
                    } else {
                        let mut yyact;
                        while !self.yystack.is_empty() {
                            yyact = self.find_reduce_action(YYERRORSYMBOL);
                            if yyact < YYNSTATE {
                                if !yyendofinput {
                                    self.yy_shift(yyact, YYERRORSYMBOL, YYMinorType::YY0);
                                }
                                break;
                            }
                            self.yystack.pop().unwrap();
                        }
                        if self.yystack.is_empty() || yyendofinput {
                            self.yy_parse_failed();
                            break;
                        }
                    }
                    self.yyerrcnt = 3;
                    yyerrorhit = true;
                } else {
                    /* This is what we do if the grammar does not define ERROR:
                     **
                     **  * Report an error message, and throw away the input token.
                     **
                     **  * If the input token is $, then fail the parse.
                     **
                     ** As before, subsequent error messages are suppressed until
                     ** three input tokens have been successfully shifted.
                     */
                    if self.yyerrcnt <= 0 {
                        self.yy_syntax_error(yymajor, &yyminor);
                    }
                    self.yyerrcnt = 3;
                    if yyendofinput {
                        self.yy_parse_failed();
                    }
                    break;
                }
            }
        }
    }

    /*
    ** Find the appropriate action for a parser given the terminal
    ** look-ahead token look_ahead.
    */
    fn find_shift_action(&self, look_ahead: i32) -> i32 {

        let stateno = self.yystack[self.yystack.len() - 1].stateno;

        if stateno > YY_SHIFT_COUNT {
            return YY_DEFAULT[stateno as usize] as i32;
        }
        let i = YY_SHIFT_OFST[stateno as usize] as i32;
        if i == YY_SHIFT_USE_DFLT {
            return YY_DEFAULT[stateno as usize] as i32;
        }
        assert!(look_ahead != YYNOCODE);
        let i = i + look_ahead;

        if i < 0 || i >= YY_ACTTAB_COUNT || YY_LOOKAHEAD[i as usize] as i32 != look_ahead {
            if look_ahead > 0 {
                if (look_ahead as usize) < YY_FALLBACK.len() {
                    let fallback = YY_FALLBACK[look_ahead as usize];
                    if fallback != 0 {
                        println!("FALLBACK");
                        return self.find_shift_action(fallback);
                    }
                }
                if YYWILDCARD > 0 {
                    let j = i - look_ahead + (YYWILDCARD as i32);
                    if j >= 0 && j < YY_ACTTAB_COUNT && YY_LOOKAHEAD[j as usize]==YYWILDCARD {
                        println!("WILDCARD");
                        return YY_ACTION[j as usize] as i32;
                    }
                }
            }
            return YY_DEFAULT[stateno as usize] as i32;
        } else {
            return YY_ACTION[i as usize] as i32;
        }
    }

    /*
    ** Find the appropriate action for a parser given the non-terminal
    ** look-ahead token iLookAhead.
    */
    fn find_reduce_action(&self, look_ahead: i32) -> i32 {
        let stateno = self.yystack[self.yystack.len() - 1].stateno;
        if YYERRORSYMBOL != 0 && stateno > YY_REDUCE_COUNT {
            return YY_DEFAULT[stateno as usize] as i32;
        }
        assert!(stateno <= YY_REDUCE_COUNT);
        let i = YY_REDUCE_OFST[stateno as usize] as i32;
        assert!(i != YY_REDUCE_USE_DFLT);
        assert!(look_ahead != YYNOCODE );
        let i = i + look_ahead;
        if YYERRORSYMBOL != 0 && (i < 0 || i >= YY_ACTTAB_COUNT || YY_LOOKAHEAD[i as usize] as i32 != look_ahead) {
            return YY_DEFAULT[stateno as usize] as i32;
        }
        assert!(i >= 0 && i < YY_ACTTAB_COUNT);
        assert!(YY_LOOKAHEAD[i as usize] as i32 == look_ahead);
        return YY_ACTION[i as usize] as i32;
    }

    fn yy_shift(&mut self, new_state: i32, major: i32, minor: YYMinorType) {
        self.yystack.push(YYStackEntry{stateno: new_state, major: major, minor: minor});
    }

    fn yy_reduce(&mut self, yyruleno: i32) {

        let yygotominor: YYMinorType = match yyruleno {
            /* Beginning here are the reduction cases.  */
            0 /* input ::= expr */
            => 
{
let yyp0 = self.yystack.pop().unwrap();
match (yyp0.minor,) {
 (YYMinorType::YY5(yy0),) => {

    self.extra = Some(yy0);

},    _ => unreachable!() };
 YYMinorType::YY0
}
            ,
            1 /* expr ::= expr OP_PLUS expr */
            => 
{
let yyres :  Expression ;
let yyp2 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
let yyp0 = self.yystack.pop().unwrap();
match (yyp0.minor,yyp2.minor,) {
 (YYMinorType::YY5(yy0),YYMinorType::YY5(yy2),) => {

    yyres = Expression::Binary(Operator::Addition, Box::new(yy0), Box::new(yy2));

},    _ => unreachable!() };
 YYMinorType::YY5(yyres)
}
            ,
            2 /* expr ::= expr OP_MINUS expr */
            => 
{
let yyres :  Expression ;
let yyp2 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
let yyp0 = self.yystack.pop().unwrap();
match (yyp0.minor,yyp2.minor,) {
 (YYMinorType::YY5(yy0),YYMinorType::YY5(yy2),) => {

    yyres = Expression::Binary(Operator::Substraction, Box::new(yy0), Box::new(yy2));

},    _ => unreachable!() };
 YYMinorType::YY5(yyres)
}
            ,
            3 /* expr ::= LPAREN expr RPAREN */
            => 
{
let yyres :  Expression ;
self.yystack.pop().unwrap();
let yyp1 = self.yystack.pop().unwrap();
self.yystack.pop().unwrap();
match (yyp1.minor,) {
 (YYMinorType::YY5(yy1),) => {

    yyres = yy1;

},    _ => unreachable!() };
 YYMinorType::YY5(yyres)
}
            ,
            4 /* expr ::= VALUE */
            => 
{
let yyres :  Expression ;
let yyp0 = self.yystack.pop().unwrap();
match (yyp0.minor,) {
 (YYMinorType::YY14(yy0),) => {

    yyres = Expression::Number(yy0);

},    _ => unreachable!() };
 YYMinorType::YY5(yyres)
}
            ,
            _ => unreachable!(),
        };
        let yygoto = YY_RULE_INFO[yyruleno as usize] as i32;
        let yyact = self.find_reduce_action(yygoto);
        if yyact < YYNSTATE {
            self.yy_shift(yyact, yygoto, yygotominor);
        } else {
            assert!(yyact == YYNSTATE + YYNRULE + 1);
            self.yy_accept();
        }
    }

    fn yy_parse_failed(&mut self) {
        self.yystack.clear();

    println!("parse_failure!");
    }

    fn yy_syntax_error(&mut self, yymajor: i32, yyminor: &YYMinorType) {
 println!("syntax error"); 
    }

    fn yy_accept(&mut self) {
        self.yystack.clear();

    println!("parse_accept");
    }
}

