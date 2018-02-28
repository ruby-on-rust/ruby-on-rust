#![allow(dead_code)]
#![allow(unused_variables)]
/* TMPL: %include */


pub type TokenString = String;

/* TMPL: makeheader cruft */


/* TMPL: types */

type YYCODETYPE = i8;
const YYNOCODE: i32 = 122;
type YYACTIONTYPE = u8;
const YYWILDCARD: YYCODETYPE = 0;
enum YYMinorType {
    YY0,
    YY139(TokenString),
    YY200(i64),
}
const YYNSTATE: i32 = 111;
const YYNRULE: i32 = 110;
const YYERRORSYMBOL: i32 = 0;

//const YY_NO_ACTION: i32 = YYNSTATE+YYNRULE+2;
//const YY_ACCEPT_ACTION: i32 = YYNSTATE+YYNRULE+1;
//const YY_ERROR_ACTION: i32 = YYNSTATE+YYNRULE+1;

/* TMPL: action tables */

#[derive( Debug, Clone, PartialEq 
)]
pub enum Token {
    EOI, //0
    T_INTEGER( i64 ), //1
    T_GVAR( TokenString ), //2
    T_CONSTANT( TokenString ), //3
    T_IDENTIFIER( TokenString ), //4
    T_LABLE( TokenString ), //5
    T_IVAR( TokenString ), //6
    T_FID( TokenString ), //7
    T_UNARY_NUM( TokenString ), //8
    T_SYMBOL( TokenString ), //9
    K_ALIAS, //10
    K_AND, //11
    K_BEGIN, //12
    K_BREAK, //13
    K_CASE, //14
    K_CLASS, //15
    K_DEF, //16
    K_DEFINED, //17
    K_DO, //18
    K_DO_BLOCK, //19
    K_DO_COND, //20
    K_DO_LAMBDA, //21
    K_ELSE, //22
    K_ELSIF, //23
    K_END, //24
    K_ENSURE, //25
    K_FALSE, //26
    K_FOR, //27
    K_IF, //28
    K_IF_MOD, //29
    K_IN, //30
    K_LBEGIN, //31
    K_LEND, //32
    K_MODULE, //33
    K_NEXT, //34
    K_NIL, //35
    K_NOT, //36
    K_OR, //37
    K_REDO, //38
    K_RESCUE, //39
    K_RESCUE_MOD, //40
    K_RETRY, //41
    K_RETURN, //42
    K_SELF, //43
    K_SUPER, //44
    K_THEN, //45
    K_TRUE, //46
    K_UNDEF, //47
    K_UNLESS, //48
    K_UNLESS_MOD, //49
    K_UNTIL, //50
    K_UNTIL_MOD, //51
    K_WHEN, //52
    K_WHILE, //53
    K_WHILE_MOD, //54
    K_YIELD, //55
    K__ENCODING__, //56
    K__FILE__, //57
    K__LINE__, //58
    T_AMPER, //59
    T_AMPER2, //60
    T_ANDDOT, //61
    T_ANDOP, //62
    T_AREF, //63
    T_ASET, //64
    T_ASSOC, //65
    T_BACK_REF2, //66
    T_BANG, //67
    T_CARET, //68
    T_CMP, //69
    T_COLON, //70
    T_COLON2, //71
    T_COLON3, //72
    T_COMMA, //73
    T_DIVIDE, //74
    T_DOT, //75
    T_DOT2, //76
    T_DOT3, //77
    T_DSTAR, //78
    T_EH, //79
    T_EQ, //80
    T_EQL, //81
    T_EQQ, //82
    T_GEQ, //83
    T_GT, //84
    T_LAMBEG, //85
    T_LAMBDA, //86
    T_LBRACE, //87
    T_LBRACE_ARG, //88
    T_LBRACK, //89
    T_LBRACK2, //90
    T_LCURLY, //91
    T_LEQ, //92
    T_LPAREN, //93
    T_LPAREN2, //94
    T_LPAREN_ARG, //95
    T_LSHFT, //96
    T_LT, //97
    T_MATCH, //98
    T_MINUS, //99
    T_NEQ, //100
    T_NL, //101
    T_NMATCH, //102
    T_OROP, //103
    T_OP_ASGN, //104
    T_PERCENT, //105
    T_PIPE, //106
    T_PLUS, //107
    T_POW, //108
    T_RBRACK, //109
    T_RCURLY, //110
    T_RPAREN, //111
    T_RSHFT, //112
    T_SEMI, //113
    T_STAR, //114
    T_STAR2, //115
    T_TILDE, //116
    T_UMINUS, //117
    T_UPLUS, //118
}
pub const TOKEN_EOI: i32 = 0;
pub const TOKEN_T_INTEGER: i32 = 1;
pub const TOKEN_T_GVAR: i32 = 2;
pub const TOKEN_T_CONSTANT: i32 = 3;
pub const TOKEN_T_IDENTIFIER: i32 = 4;
pub const TOKEN_T_LABLE: i32 = 5;
pub const TOKEN_T_IVAR: i32 = 6;
pub const TOKEN_T_FID: i32 = 7;
pub const TOKEN_T_UNARY_NUM: i32 = 8;
pub const TOKEN_T_SYMBOL: i32 = 9;
pub const TOKEN_K_ALIAS: i32 = 10;
pub const TOKEN_K_AND: i32 = 11;
pub const TOKEN_K_BEGIN: i32 = 12;
pub const TOKEN_K_BREAK: i32 = 13;
pub const TOKEN_K_CASE: i32 = 14;
pub const TOKEN_K_CLASS: i32 = 15;
pub const TOKEN_K_DEF: i32 = 16;
pub const TOKEN_K_DEFINED: i32 = 17;
pub const TOKEN_K_DO: i32 = 18;
pub const TOKEN_K_DO_BLOCK: i32 = 19;
pub const TOKEN_K_DO_COND: i32 = 20;
pub const TOKEN_K_DO_LAMBDA: i32 = 21;
pub const TOKEN_K_ELSE: i32 = 22;
pub const TOKEN_K_ELSIF: i32 = 23;
pub const TOKEN_K_END: i32 = 24;
pub const TOKEN_K_ENSURE: i32 = 25;
pub const TOKEN_K_FALSE: i32 = 26;
pub const TOKEN_K_FOR: i32 = 27;
pub const TOKEN_K_IF: i32 = 28;
pub const TOKEN_K_IF_MOD: i32 = 29;
pub const TOKEN_K_IN: i32 = 30;
pub const TOKEN_K_LBEGIN: i32 = 31;
pub const TOKEN_K_LEND: i32 = 32;
pub const TOKEN_K_MODULE: i32 = 33;
pub const TOKEN_K_NEXT: i32 = 34;
pub const TOKEN_K_NIL: i32 = 35;
pub const TOKEN_K_NOT: i32 = 36;
pub const TOKEN_K_OR: i32 = 37;
pub const TOKEN_K_REDO: i32 = 38;
pub const TOKEN_K_RESCUE: i32 = 39;
pub const TOKEN_K_RESCUE_MOD: i32 = 40;
pub const TOKEN_K_RETRY: i32 = 41;
pub const TOKEN_K_RETURN: i32 = 42;
pub const TOKEN_K_SELF: i32 = 43;
pub const TOKEN_K_SUPER: i32 = 44;
pub const TOKEN_K_THEN: i32 = 45;
pub const TOKEN_K_TRUE: i32 = 46;
pub const TOKEN_K_UNDEF: i32 = 47;
pub const TOKEN_K_UNLESS: i32 = 48;
pub const TOKEN_K_UNLESS_MOD: i32 = 49;
pub const TOKEN_K_UNTIL: i32 = 50;
pub const TOKEN_K_UNTIL_MOD: i32 = 51;
pub const TOKEN_K_WHEN: i32 = 52;
pub const TOKEN_K_WHILE: i32 = 53;
pub const TOKEN_K_WHILE_MOD: i32 = 54;
pub const TOKEN_K_YIELD: i32 = 55;
pub const TOKEN_K__ENCODING__: i32 = 56;
pub const TOKEN_K__FILE__: i32 = 57;
pub const TOKEN_K__LINE__: i32 = 58;
pub const TOKEN_T_AMPER: i32 = 59;
pub const TOKEN_T_AMPER2: i32 = 60;
pub const TOKEN_T_ANDDOT: i32 = 61;
pub const TOKEN_T_ANDOP: i32 = 62;
pub const TOKEN_T_AREF: i32 = 63;
pub const TOKEN_T_ASET: i32 = 64;
pub const TOKEN_T_ASSOC: i32 = 65;
pub const TOKEN_T_BACK_REF2: i32 = 66;
pub const TOKEN_T_BANG: i32 = 67;
pub const TOKEN_T_CARET: i32 = 68;
pub const TOKEN_T_CMP: i32 = 69;
pub const TOKEN_T_COLON: i32 = 70;
pub const TOKEN_T_COLON2: i32 = 71;
pub const TOKEN_T_COLON3: i32 = 72;
pub const TOKEN_T_COMMA: i32 = 73;
pub const TOKEN_T_DIVIDE: i32 = 74;
pub const TOKEN_T_DOT: i32 = 75;
pub const TOKEN_T_DOT2: i32 = 76;
pub const TOKEN_T_DOT3: i32 = 77;
pub const TOKEN_T_DSTAR: i32 = 78;
pub const TOKEN_T_EH: i32 = 79;
pub const TOKEN_T_EQ: i32 = 80;
pub const TOKEN_T_EQL: i32 = 81;
pub const TOKEN_T_EQQ: i32 = 82;
pub const TOKEN_T_GEQ: i32 = 83;
pub const TOKEN_T_GT: i32 = 84;
pub const TOKEN_T_LAMBEG: i32 = 85;
pub const TOKEN_T_LAMBDA: i32 = 86;
pub const TOKEN_T_LBRACE: i32 = 87;
pub const TOKEN_T_LBRACE_ARG: i32 = 88;
pub const TOKEN_T_LBRACK: i32 = 89;
pub const TOKEN_T_LBRACK2: i32 = 90;
pub const TOKEN_T_LCURLY: i32 = 91;
pub const TOKEN_T_LEQ: i32 = 92;
pub const TOKEN_T_LPAREN: i32 = 93;
pub const TOKEN_T_LPAREN2: i32 = 94;
pub const TOKEN_T_LPAREN_ARG: i32 = 95;
pub const TOKEN_T_LSHFT: i32 = 96;
pub const TOKEN_T_LT: i32 = 97;
pub const TOKEN_T_MATCH: i32 = 98;
pub const TOKEN_T_MINUS: i32 = 99;
pub const TOKEN_T_NEQ: i32 = 100;
pub const TOKEN_T_NL: i32 = 101;
pub const TOKEN_T_NMATCH: i32 = 102;
pub const TOKEN_T_OROP: i32 = 103;
pub const TOKEN_T_OP_ASGN: i32 = 104;
pub const TOKEN_T_PERCENT: i32 = 105;
pub const TOKEN_T_PIPE: i32 = 106;
pub const TOKEN_T_PLUS: i32 = 107;
pub const TOKEN_T_POW: i32 = 108;
pub const TOKEN_T_RBRACK: i32 = 109;
pub const TOKEN_T_RCURLY: i32 = 110;
pub const TOKEN_T_RPAREN: i32 = 111;
pub const TOKEN_T_RSHFT: i32 = 112;
pub const TOKEN_T_SEMI: i32 = 113;
pub const TOKEN_T_STAR: i32 = 114;
pub const TOKEN_T_STAR2: i32 = 115;
pub const TOKEN_T_TILDE: i32 = 116;
pub const TOKEN_T_UMINUS: i32 = 117;
pub const TOKEN_T_UPLUS: i32 = 118;
#[inline]
fn token_value(t: Token) -> (i32, YYMinorType) {
  match t {
        Token::EOI => (0, YYMinorType::YY0),
        Token::T_INTEGER(x) => (1, YYMinorType::YY200(x)),
        Token::T_GVAR(x) => (2, YYMinorType::YY139(x)),
        Token::T_CONSTANT(x) => (3, YYMinorType::YY139(x)),
        Token::T_IDENTIFIER(x) => (4, YYMinorType::YY139(x)),
        Token::T_LABLE(x) => (5, YYMinorType::YY139(x)),
        Token::T_IVAR(x) => (6, YYMinorType::YY139(x)),
        Token::T_FID(x) => (7, YYMinorType::YY139(x)),
        Token::T_UNARY_NUM(x) => (8, YYMinorType::YY139(x)),
        Token::T_SYMBOL(x) => (9, YYMinorType::YY139(x)),
        Token::K_ALIAS => (10, YYMinorType::YY0),
        Token::K_AND => (11, YYMinorType::YY0),
        Token::K_BEGIN => (12, YYMinorType::YY0),
        Token::K_BREAK => (13, YYMinorType::YY0),
        Token::K_CASE => (14, YYMinorType::YY0),
        Token::K_CLASS => (15, YYMinorType::YY0),
        Token::K_DEF => (16, YYMinorType::YY0),
        Token::K_DEFINED => (17, YYMinorType::YY0),
        Token::K_DO => (18, YYMinorType::YY0),
        Token::K_DO_BLOCK => (19, YYMinorType::YY0),
        Token::K_DO_COND => (20, YYMinorType::YY0),
        Token::K_DO_LAMBDA => (21, YYMinorType::YY0),
        Token::K_ELSE => (22, YYMinorType::YY0),
        Token::K_ELSIF => (23, YYMinorType::YY0),
        Token::K_END => (24, YYMinorType::YY0),
        Token::K_ENSURE => (25, YYMinorType::YY0),
        Token::K_FALSE => (26, YYMinorType::YY0),
        Token::K_FOR => (27, YYMinorType::YY0),
        Token::K_IF => (28, YYMinorType::YY0),
        Token::K_IF_MOD => (29, YYMinorType::YY0),
        Token::K_IN => (30, YYMinorType::YY0),
        Token::K_LBEGIN => (31, YYMinorType::YY0),
        Token::K_LEND => (32, YYMinorType::YY0),
        Token::K_MODULE => (33, YYMinorType::YY0),
        Token::K_NEXT => (34, YYMinorType::YY0),
        Token::K_NIL => (35, YYMinorType::YY0),
        Token::K_NOT => (36, YYMinorType::YY0),
        Token::K_OR => (37, YYMinorType::YY0),
        Token::K_REDO => (38, YYMinorType::YY0),
        Token::K_RESCUE => (39, YYMinorType::YY0),
        Token::K_RESCUE_MOD => (40, YYMinorType::YY0),
        Token::K_RETRY => (41, YYMinorType::YY0),
        Token::K_RETURN => (42, YYMinorType::YY0),
        Token::K_SELF => (43, YYMinorType::YY0),
        Token::K_SUPER => (44, YYMinorType::YY0),
        Token::K_THEN => (45, YYMinorType::YY0),
        Token::K_TRUE => (46, YYMinorType::YY0),
        Token::K_UNDEF => (47, YYMinorType::YY0),
        Token::K_UNLESS => (48, YYMinorType::YY0),
        Token::K_UNLESS_MOD => (49, YYMinorType::YY0),
        Token::K_UNTIL => (50, YYMinorType::YY0),
        Token::K_UNTIL_MOD => (51, YYMinorType::YY0),
        Token::K_WHEN => (52, YYMinorType::YY0),
        Token::K_WHILE => (53, YYMinorType::YY0),
        Token::K_WHILE_MOD => (54, YYMinorType::YY0),
        Token::K_YIELD => (55, YYMinorType::YY0),
        Token::K__ENCODING__ => (56, YYMinorType::YY0),
        Token::K__FILE__ => (57, YYMinorType::YY0),
        Token::K__LINE__ => (58, YYMinorType::YY0),
        Token::T_AMPER => (59, YYMinorType::YY0),
        Token::T_AMPER2 => (60, YYMinorType::YY0),
        Token::T_ANDDOT => (61, YYMinorType::YY0),
        Token::T_ANDOP => (62, YYMinorType::YY0),
        Token::T_AREF => (63, YYMinorType::YY0),
        Token::T_ASET => (64, YYMinorType::YY0),
        Token::T_ASSOC => (65, YYMinorType::YY0),
        Token::T_BACK_REF2 => (66, YYMinorType::YY0),
        Token::T_BANG => (67, YYMinorType::YY0),
        Token::T_CARET => (68, YYMinorType::YY0),
        Token::T_CMP => (69, YYMinorType::YY0),
        Token::T_COLON => (70, YYMinorType::YY0),
        Token::T_COLON2 => (71, YYMinorType::YY0),
        Token::T_COLON3 => (72, YYMinorType::YY0),
        Token::T_COMMA => (73, YYMinorType::YY0),
        Token::T_DIVIDE => (74, YYMinorType::YY0),
        Token::T_DOT => (75, YYMinorType::YY0),
        Token::T_DOT2 => (76, YYMinorType::YY0),
        Token::T_DOT3 => (77, YYMinorType::YY0),
        Token::T_DSTAR => (78, YYMinorType::YY0),
        Token::T_EH => (79, YYMinorType::YY0),
        Token::T_EQ => (80, YYMinorType::YY0),
        Token::T_EQL => (81, YYMinorType::YY0),
        Token::T_EQQ => (82, YYMinorType::YY0),
        Token::T_GEQ => (83, YYMinorType::YY0),
        Token::T_GT => (84, YYMinorType::YY0),
        Token::T_LAMBEG => (85, YYMinorType::YY0),
        Token::T_LAMBDA => (86, YYMinorType::YY0),
        Token::T_LBRACE => (87, YYMinorType::YY0),
        Token::T_LBRACE_ARG => (88, YYMinorType::YY0),
        Token::T_LBRACK => (89, YYMinorType::YY0),
        Token::T_LBRACK2 => (90, YYMinorType::YY0),
        Token::T_LCURLY => (91, YYMinorType::YY0),
        Token::T_LEQ => (92, YYMinorType::YY0),
        Token::T_LPAREN => (93, YYMinorType::YY0),
        Token::T_LPAREN2 => (94, YYMinorType::YY0),
        Token::T_LPAREN_ARG => (95, YYMinorType::YY0),
        Token::T_LSHFT => (96, YYMinorType::YY0),
        Token::T_LT => (97, YYMinorType::YY0),
        Token::T_MATCH => (98, YYMinorType::YY0),
        Token::T_MINUS => (99, YYMinorType::YY0),
        Token::T_NEQ => (100, YYMinorType::YY0),
        Token::T_NL => (101, YYMinorType::YY0),
        Token::T_NMATCH => (102, YYMinorType::YY0),
        Token::T_OROP => (103, YYMinorType::YY0),
        Token::T_OP_ASGN => (104, YYMinorType::YY0),
        Token::T_PERCENT => (105, YYMinorType::YY0),
        Token::T_PIPE => (106, YYMinorType::YY0),
        Token::T_PLUS => (107, YYMinorType::YY0),
        Token::T_POW => (108, YYMinorType::YY0),
        Token::T_RBRACK => (109, YYMinorType::YY0),
        Token::T_RCURLY => (110, YYMinorType::YY0),
        Token::T_RPAREN => (111, YYMinorType::YY0),
        Token::T_RSHFT => (112, YYMinorType::YY0),
        Token::T_SEMI => (113, YYMinorType::YY0),
        Token::T_STAR => (114, YYMinorType::YY0),
        Token::T_STAR2 => (115, YYMinorType::YY0),
        Token::T_TILDE => (116, YYMinorType::YY0),
        Token::T_UMINUS => (117, YYMinorType::YY0),
        Token::T_UPLUS => (118, YYMinorType::YY0),
  }
}
const YY_ACTTAB_COUNT: i32 = 221;
const YY_ACTION: [YYACTIONTYPE; 221] = [
 /*     0 */   110,  111,  112,  113,  114,  115,  116,  117,  118,  109,
 /*    10 */   108,  107,  106,  105,  104,  103,  102,  101,  100,   99,
 /*    20 */    98,   97,   96,   95,   94,   93,   92,   91,   90,   89,
 /*    30 */    88,   87,   86,   85,   84,   83,   82,   81,   80,   79,
 /*    40 */    78,   77,   76,   75,   74,   73,   72,   71,   70,   69,
 /*    50 */    68,   67,   66,   65,   64,   63,   62,   61,   60,   59,
 /*    60 */    58,   57,   56,   55,   54,   53,   52,   51,   50,   49,
 /*    70 */    48,   47,   46,   45,   44,   43,   42,   41,   40,   39,
 /*    80 */    38,   37,   36,   35,   34,   33,   32,   31,   30,   29,
 /*    90 */    28,   27,   26,   25,   24,   23,   22,   21,   20,   19,
 /*   100 */    18,   17,   16,   15,   14,   13,   12,   11,   10,    9,
 /*   110 */     8,    7,    6,    5,    4,    3,    2,    1,  119,  120,
 /*   120 */   121,  122,  123,  124,  125,  126,  127,  128,  129,  130,
 /*   130 */   131,  132,  133,  134,  135,  136,  137,  138,  139,  140,
 /*   140 */   141,  142,  143,  144,  145,  146,  147,  148,  149,  150,
 /*   150 */   151,  152,  153,  154,  155,  156,  157,  158,  159,  160,
 /*   160 */   161,  162,  163,  164,  165,  166,  167,  168,  169,  170,
 /*   170 */   171,  172,  173,  174,  175,  176,  177,  178,  179,  180,
 /*   180 */   181,  182,  183,  184,  185,  186,  187,  188,  189,  190,
 /*   190 */   191,  192,  193,  194,  195,  196,  197,  198,  199,  200,
 /*   200 */   201,  202,  203,  204,  205,  206,  207,  208,  209,  210,
 /*   210 */   211,  212,  213,  214,  215,  216,  217,  218,  219,  220,
 /*   220 */   222,
];
const YY_LOOKAHEAD: [YYCODETYPE; 221] = [
 /*     0 */     1,    0,    0,    0,    0,    0,    0,    0,    0,   10,
 /*    10 */    11,   12,   13,   14,   15,   16,   17,   18,   19,   20,
 /*    20 */    21,   22,   23,   24,   25,   26,   27,   28,   29,   30,
 /*    30 */    31,   32,   33,   34,   35,   36,   37,   38,   39,   40,
 /*    40 */    41,   42,   43,   44,   45,   46,   47,   48,   49,   50,
 /*    50 */    51,   52,   53,   54,   55,   56,   57,   58,   59,   60,
 /*    60 */    61,   62,   63,   64,   65,   66,   67,   68,   69,   70,
 /*    70 */    71,   72,   73,   74,   75,   76,   77,   78,   79,   80,
 /*    80 */    81,   82,   83,   84,   85,   86,   87,   88,   89,   90,
 /*    90 */    91,   92,   93,   94,   95,   96,   97,   98,   99,  100,
 /*   100 */   101,  102,  103,  104,  105,  106,  107,  108,  109,  110,
 /*   110 */   111,  112,  113,  114,  115,  116,  117,  118,    0,    0,
 /*   120 */     0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
 /*   130 */     0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
 /*   140 */     0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
 /*   150 */     0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
 /*   160 */     0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
 /*   170 */     0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
 /*   180 */     0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
 /*   190 */     0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
 /*   200 */     0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
 /*   210 */     0,    0,    0,    0,    0,    0,    0,    0,    0,    0,
 /*   220 */   120,
];
const YY_SHIFT_USE_DFLT: i32 = -2;
const YY_SHIFT_COUNT: i32 = 110;
const YY_SHIFT_MIN: i32 = -1;
const YY_SHIFT_MAX: i32 = 219;
const YY_SHIFT_OFST: [i16; 111] = [
 /*     0 */    -1,  219,  218,  217,  216,  215,  214,  213,  212,  211,
 /*    10 */   210,  209,  208,  207,  206,  205,  204,  203,  202,  201,
 /*    20 */   200,  199,  198,  197,  196,  195,  194,  193,  192,  191,
 /*    30 */   190,  189,  188,  187,  186,  185,  184,  183,  182,  181,
 /*    40 */   180,  179,  178,  177,  176,  175,  174,  173,  172,  171,
 /*    50 */   170,  169,  168,  167,  166,  165,  164,  163,  162,  161,
 /*    60 */   160,  159,  158,  157,  156,  155,  154,  153,  152,  151,
 /*    70 */   150,  149,  148,  147,  146,  145,  144,  143,  142,  141,
 /*    80 */   140,  139,  138,  137,  136,  135,  134,  133,  132,  131,
 /*    90 */   130,  129,  128,  127,  126,  125,  124,  123,  122,  121,
 /*   100 */   120,  119,  118,    8,    7,    6,    5,    4,    3,    2,
 /*   110 */     1,
];
const YY_REDUCE_USE_DFLT: i32 = -1;
const YY_REDUCE_COUNT: i32 = 0;
const YY_REDUCE_MIN: i32 = 0;
const YY_REDUCE_MAX: i32 = 100;
const YY_REDUCE_OFST: [i8; 1] = [
 /*     0 */   100,
];
const YY_DEFAULT: [YYACTIONTYPE; 111] = [
 /*     0 */   221,  221,  221,  221,  221,  221,  221,  221,  221,  221,
 /*    10 */   221,  221,  221,  221,  221,  221,  221,  221,  221,  221,
 /*    20 */   221,  221,  221,  221,  221,  221,  221,  221,  221,  221,
 /*    30 */   221,  221,  221,  221,  221,  221,  221,  221,  221,  221,
 /*    40 */   221,  221,  221,  221,  221,  221,  221,  221,  221,  221,
 /*    50 */   221,  221,  221,  221,  221,  221,  221,  221,  221,  221,
 /*    60 */   221,  221,  221,  221,  221,  221,  221,  221,  221,  221,
 /*    70 */   221,  221,  221,  221,  221,  221,  221,  221,  221,  221,
 /*    80 */   221,  221,  221,  221,  221,  221,  221,  221,  221,  221,
 /*    90 */   221,  221,  221,  221,  221,  221,  221,  221,  221,  221,
 /*   100 */   221,  221,  221,  221,  221,  221,  221,  221,  221,  221,
 /*   110 */   221,
];

/* TMPL: fallback tokens */

const YY_FALLBACK: [i32; 0] = [
];

/* TMPL: symbol names */


/* TMPL: rules */


/* TMPL: destructors */


/* TMPL: stack-overflow */


/* TMPL: stack-overflow */

const YY_RULE_INFO: [YYCODETYPE; 110] = [
  120,
  120,
  120,
  120,
  120,
  120,
  120,
  120,
  120,
  120,
  120,
  120,
  120,
  120,
  120,
  120,
  120,
  120,
  120,
  120,
  120,
  120,
  120,
  120,
  120,
  120,
  120,
  120,
  120,
  120,
  120,
  120,
  120,
  120,
  120,
  120,
  120,
  120,
  120,
  120,
  120,
  120,
  120,
  120,
  120,
  120,
  120,
  120,
  120,
  120,
  120,
  120,
  120,
  120,
  120,
  120,
  120,
  120,
  120,
  120,
  120,
  120,
  120,
  120,
  120,
  120,
  120,
  120,
  120,
  120,
  120,
  120,
  120,
  120,
  120,
  120,
  120,
  120,
  120,
  120,
  120,
  120,
  120,
  120,
  120,
  120,
  120,
  120,
  120,
  120,
  120,
  120,
  120,
  120,
  120,
  120,
  120,
  120,
  120,
  120,
  120,
  120,
  120,
  120,
  120,
  120,
  120,
  120,
  120,
  120,
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
    extra:  Option<i64> ,
}

impl Parser {

    pub fn new(
            extra:  Option<i64> ,
        ) -> Parser {
        let mut p = Parser { yyerrcnt: -1, yystack: Vec::new(), extra: extra};
        p.yystack.push(YYStackEntry{stateno: 0, major: 0, minor: YYMinorType::YY0});
        p
    }

    pub fn into_extra(self) ->  Option<i64>  {
        self.extra
    }
    pub fn extra(&self) -> & Option<i64>  {
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
            0 /* input ::= T_INTEGER */
            => 
{
let yyp0 = self.yystack.pop().unwrap();
match (yyp0.minor,) {
 (YYMinorType::YY200(yy0),) => {

    self.extra = Some(yy0);

},    _ => unreachable!() };
 YYMinorType::YY0
}
            ,
            1 /* input ::= K_ALIAS */
          | 2 /* input ::= K_AND */
          | 3 /* input ::= K_BEGIN */
          | 4 /* input ::= K_BREAK */
          | 5 /* input ::= K_CASE */
          | 6 /* input ::= K_CLASS */
          | 7 /* input ::= K_DEF */
          | 8 /* input ::= K_DEFINED */
          | 9 /* input ::= K_DO */
          | 10 /* input ::= K_DO_BLOCK */
          | 11 /* input ::= K_DO_COND */
          | 12 /* input ::= K_DO_LAMBDA */
          | 13 /* input ::= K_ELSE */
          | 14 /* input ::= K_ELSIF */
          | 15 /* input ::= K_END */
          | 16 /* input ::= K_ENSURE */
          | 17 /* input ::= K_FALSE */
          | 18 /* input ::= K_FOR */
          | 19 /* input ::= K_IF */
          | 20 /* input ::= K_IF_MOD */
          | 21 /* input ::= K_IN */
          | 22 /* input ::= K_LBEGIN */
          | 23 /* input ::= K_LEND */
          | 24 /* input ::= K_MODULE */
          | 25 /* input ::= K_NEXT */
          | 26 /* input ::= K_NIL */
          | 27 /* input ::= K_NOT */
          | 28 /* input ::= K_OR */
          | 29 /* input ::= K_REDO */
          | 30 /* input ::= K_RESCUE */
          | 31 /* input ::= K_RESCUE_MOD */
          | 32 /* input ::= K_RETRY */
          | 33 /* input ::= K_RETURN */
          | 34 /* input ::= K_SELF */
          | 35 /* input ::= K_SUPER */
          | 36 /* input ::= K_THEN */
          | 37 /* input ::= K_TRUE */
          | 38 /* input ::= K_UNDEF */
          | 39 /* input ::= K_UNLESS */
          | 40 /* input ::= K_UNLESS_MOD */
          | 41 /* input ::= K_UNTIL */
          | 42 /* input ::= K_UNTIL_MOD */
          | 43 /* input ::= K_WHEN */
          | 44 /* input ::= K_WHILE */
          | 45 /* input ::= K_WHILE_MOD */
          | 46 /* input ::= K_YIELD */
          | 47 /* input ::= K__ENCODING__ */
          | 48 /* input ::= K__FILE__ */
          | 49 /* input ::= K__LINE__ */
          | 50 /* input ::= T_AMPER */
          | 51 /* input ::= T_AMPER2 */
          | 52 /* input ::= T_ANDDOT */
          | 53 /* input ::= T_ANDOP */
          | 54 /* input ::= T_AREF */
          | 55 /* input ::= T_ASET */
          | 56 /* input ::= T_ASSOC */
          | 57 /* input ::= T_BACK_REF2 */
          | 58 /* input ::= T_BANG */
          | 59 /* input ::= T_CARET */
          | 60 /* input ::= T_CMP */
          | 61 /* input ::= T_COLON */
          | 62 /* input ::= T_COLON2 */
          | 63 /* input ::= T_COLON3 */
          | 64 /* input ::= T_COMMA */
          | 65 /* input ::= T_DIVIDE */
          | 66 /* input ::= T_DOT */
          | 67 /* input ::= T_DOT2 */
          | 68 /* input ::= T_DOT3 */
          | 69 /* input ::= T_DSTAR */
          | 70 /* input ::= T_EH */
          | 71 /* input ::= T_EQ */
          | 72 /* input ::= T_EQL */
          | 73 /* input ::= T_EQQ */
          | 74 /* input ::= T_GEQ */
          | 75 /* input ::= T_GT */
          | 76 /* input ::= T_LAMBEG */
          | 77 /* input ::= T_LAMBDA */
          | 78 /* input ::= T_LBRACE */
          | 79 /* input ::= T_LBRACE_ARG */
          | 80 /* input ::= T_LBRACK */
          | 81 /* input ::= T_LBRACK2 */
          | 82 /* input ::= T_LCURLY */
          | 83 /* input ::= T_LEQ */
          | 84 /* input ::= T_LPAREN */
          | 85 /* input ::= T_LPAREN2 */
          | 86 /* input ::= T_LPAREN_ARG */
          | 87 /* input ::= T_LSHFT */
          | 88 /* input ::= T_LT */
          | 89 /* input ::= T_MATCH */
          | 90 /* input ::= T_MINUS */
          | 91 /* input ::= T_NEQ */
          | 92 /* input ::= T_NL */
          | 93 /* input ::= T_NMATCH */
          | 94 /* input ::= T_OROP */
          | 95 /* input ::= T_OP_ASGN */
          | 96 /* input ::= T_PERCENT */
          | 97 /* input ::= T_PIPE */
          | 98 /* input ::= T_PLUS */
          | 99 /* input ::= T_POW */
          | 100 /* input ::= T_RBRACK */
          | 101 /* input ::= T_RCURLY */
          | 102 /* input ::= T_RPAREN */
          | 103 /* input ::= T_RSHFT */
          | 104 /* input ::= T_SEMI */
          | 105 /* input ::= T_STAR */
          | 106 /* input ::= T_STAR2 */
          | 107 /* input ::= T_TILDE */
          | 108 /* input ::= T_UMINUS */
          | 109 /* input ::= T_UPLUS */
            => 
{
self.yystack.pop().unwrap();
match () {
 () => {


} };
 YYMinorType::YY0
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

