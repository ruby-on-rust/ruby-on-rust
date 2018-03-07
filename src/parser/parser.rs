// https based on github.com/whitequark/parser/blob/2a73841d6da04a5ab9bd270561165fd766722d43/lib/parser/ruby25.y

use lexer::lexing_state::LexingState;
use lexer::Lexer;
use parser::token::Token;
use ast::node;
use ast::node::Node;

// TODO dont rewrite this macro here
macro_rules! state { ($state_name:expr) => { $state_name.parse::<LexingState>().unwrap() }; }

pub struct Parser {
    lexer: Lexer,

    tokens: Vec<Token>,
    current_p: usize, // TODO NOTE
}

impl Parser {
    pub fn new(input: String) -> Parser {
        Parser {
            lexer: Lexer::new(input),

            tokens: vec![],
            current_p: 0,
        }
    }

    pub fn parse(&mut self) -> Node {
        self.p_program().expect("failed to parse")
    }

    // TODO
    // shared match-and-consume
    // fn match_token() -> bool {
    // }

    // TODO wrap in Result
    // get a new one if necessary
    fn current_token(&mut self) -> Token {
        if self.tokens.get(self.current_p).is_none() {
            self.tokens.push(self.lexer.advance().expect("no token emitted after lexer.advance()"));
        }

        let token = (*self.tokens.get(self.current_p).expect("no current token for current_p")).clone();

        token
    }

    // TODO handle no more token
    fn consume_current_token(&mut self) -> Token {
        let original_p = self.current_p;
        self.current_p += 1;
        return self.tokens.get(original_p).unwrap().clone();
    }

    // fn try_to_consume_token(&mut self, token: Token) -> Result<Token> {
    //     Ok(token)
    // }

    // match and consume one token
    // TODO REFINE
    // TODO cant handle token with value for now
    fn match_1_token(&mut self, token: Token) -> Option<Token> {

        let current_token = self.current_token();
        if current_token == token {
            self.consume_current_token();
            Some(current_token)
        } else {
            None
        }
    }

    // ===
    fn p_program(&mut self) -> Option<Node> { self.p_stmt() }

    // top_compstmt: top_stmts opt_terms
    //                 {
    //                   result = @builder.compstmt(val[0])
    //                 }
    // fn p_top_compstmt(&mut self) -> Node {
    // }

    // fn p_top_stmts(&mut self) -> Node {
    // }

    // TODO INCOMPLETE
    // fn p_stmts(&mut self) -> Option<Node> {
    //     if let Some(n_stmt) = self.p_stmt() { return Some(n_stmt); }
    //     None
    // }

    //    stmt_or_begin: stmt
    //                 | klBEGIN tLCURLY top_compstmt tRCURLY
    //                     {
    //                       diagnostic :error, :begin_in_method, nil, val[0]
    //                     }
    // TODO INCOMPLETE
    // fn p_stmt_or_begin(&mut self) -> Option<Node> {
    //     if let Some(n_stmt) = self.p_stmt() { return Some(n_stmt); }
    //     None
    // }

    // TODO INCOMPLETE
    fn p_stmt(&mut self) -> Option<Node> {

        // expr
        if let Some(n_expr) = self.p_expr() { return Some(n_expr); }

        None
    }

    // TODO INCOMPLETE
    fn p_expr(&mut self) -> Option<Node> {
        if let Some(n_arg) = self.p_arg() { return Some(n_arg); }
        None
    }

    // TODO INCOMPLETE
    fn p_arg(&mut self) -> Option<Node> {

        //  arg: lhs tEQL arg_rhs
        //         {
        //           result = @builder.assign(val[0], val[1], val[2])
        //         }
        if let Some(n_lhs) = self.p_lhs() {
            if let Some(_) = self.match_1_token(Token::T_EQL) {
                if let Some(n_arg_rhs) = self.p_arg_rhs() {
                    return Some(Node::Assign( box n_lhs, Token::T_EQL, box n_arg_rhs ));
                }
            }
        }

        // | primary
        if let Some(n_primary) = self.p_primary() { return Some(n_primary); }

        None
    }

    // TODO INCOMPLETE
    fn p_lhs(&mut self) -> Option<Node> {
        //  user_variable
        //         {
        //           result = @builder.assignable(val[0])
        //         }
        if let Some(n_user_variable) = self.p_user_variable() {
            // TODO value in assignable
            return Some(Node::Assignable);
        }

        None
    }

    //    user_variable: tIDENTIFIER
    //                     {
    //                       result = @builder.ident(val[0])
    //                     }
    //                 | tIVAR
    //                     {
    //                       result = @builder.ivar(val[0])
    //                     }
    //                 | tGVAR
    //                     {
    //                       result = @builder.gvar(val[0])
    //                     }
    //                 | tCONSTANT
    //                     {
    //                       result = @builder.const(val[0])
    //                     }
    //                 | tCVAR
    //                     {
    //                       result = @builder.cvar(val[0])
    //                     }
    // TODO INCOMPLETE
    fn p_user_variable(&mut self) -> Option<Node> {
        let current_token = self.current_token();

        match current_token {
            Token::T_IDENTIFIER(_) => { self.consume_current_token(); return Some(Node::Ident(current_token)); },
            _ => { return None; }
        }
    }

    // keyword_variable: kNIL
    //                     {
    //                       result = @builder.nil(val[0])
    //                     }
    //                 | kSELF
    //                     {
    //                       result = @builder.self(val[0])
    //                     }
    //                 | kTRUE
    //                     {
    //                       result = @builder.true(val[0])
    //                     }
    //                 | kFALSE
    //                     {
    //                       result = @builder.false(val[0])
    //                     }
    //                 | k__FILE__
    //                     {
    //                       result = @builder.__FILE__(val[0])
    //                     }
    //                 | k__LINE__
    //                     {
    //                       result = @builder.__LINE__(val[0])
    //                     }
    //                 | k__ENCODING__
    //                     {
    //                       result = @builder.__ENCODING__(val[0])
    //                     }
    // TODO INCOMPLETE
    fn p_keyword_variable(&mut self) -> Option<Node> {
        if let Some(_) = self.match_1_token(Token::K_NIL) { return Some(Node::Nil); }

        if let Some(_) = self.match_1_token(Token::K_TRUE) { return Some(Node::True); }
        if let Some(_) = self.match_1_token(Token::K_FALSE) { return Some(Node::False); }

        None
    }

    //  var_ref: user_variable
    //             {
    //               result = @builder.accessible(val[0])
    //             }
    //         | keyword_variable
    //             {
    //               result = @builder.accessible(val[0])
    //             }
    // TODO INCOMPLETE
    fn p_var_ref(&mut self) -> Option<Node> {
        if let Some(n_user_variable) = self.p_user_variable() { return Some(node::accessible(n_user_variable)); }

        if let Some(n_keyword_variable) = self.p_keyword_variable() { return Some(node::accessible(n_keyword_variable)); }

        None
    }

    //  arg_rhs: arg =tOP_ASGN
    //         | arg kRESCUE_MOD arg
    //             {
    //               rescue_body = @builder.rescue_body(val[1],
    //                                 nil, nil, nil,
    //                                 nil, val[2])

    //               result = @builder.begin_body(val[0], [ rescue_body ])
    //             }
    // TODO INCOMPLETE
    // TODO handle %prec
    fn p_arg_rhs(&mut self) -> Option<Node> {
        // TODO DUMMY
        if let Some(n_primary) = self.p_primary() { return Some(n_primary); }

        None
    }


    //  primary: literal
    //         | strings
    //         | xstring
    //         | regexp
    //         | words
    //         | qwords
    //         | symbols
    //         | qsymbols
    //         | var_ref
    //         | backref
    //         | tFID
    //             {
    //               result = @builder.call_method(nil, nil, val[0])
    //             }
    //         | kBEGIN
    //             {
    //               result = @lexer.cmdarg.dup
    //               @lexer.cmdarg.clear
    //             }
    //             bodystmt kEND
    //             {
    //               @lexer.cmdarg = val[1]

    //               result = @builder.begin_keyword(val[0], val[2], val[3])
    //             }
    //         | tLPAREN_ARG
    //             {
    //               result = @lexer.cmdarg.dup
    //               @lexer.cmdarg.clear
    //             }
    //             stmt
    //             {
    //               @lexer.state = :expr_endarg
    //             }
    //             rparen
    //             {
    //               @lexer.cmdarg = val[1]

    //               result = @builder.begin(val[0], val[2], val[4])
    //             }
    //         | tLPAREN_ARG
    //             {
    //               @lexer.state = :expr_endarg
    //             }
    //             opt_nl tRPAREN
    //             {
    //               result = @builder.begin(val[0], nil, val[3])
    //             }
    //         | tLPAREN compstmt tRPAREN
    //             {
    //               result = @builder.begin(val[0], val[1], val[2])
    //             }
    //         | primary_value tCOLON2 tCONSTANT
    //             {
    //               result = @builder.const_fetch(val[0], val[1], val[2])
    //             }
    //         | tCOLON3 tCONSTANT
    //             {
    //               result = @builder.const_global(val[0], val[1])
    //             }
    //         | tLBRACK aref_args tRBRACK
    //             {
    //               result = @builder.array(val[0], val[1], val[2])
    //             }
    //         | tLBRACE assoc_list tRCURLY
    //             {
    //               result = @builder.associate(val[0], val[1], val[2])
    //             }
    //         | kRETURN
    //             {
    //               result = @builder.keyword_cmd(:return, val[0])
    //             }
    //         | kYIELD tLPAREN2 call_args rparen
    //             {
    //               result = @builder.keyword_cmd(:yield, val[0], val[1], val[2], val[3])
    //             }
    //         | kYIELD tLPAREN2 rparen
    //             {
    //               result = @builder.keyword_cmd(:yield, val[0], val[1], [], val[2])
    //             }
    //         | kYIELD
    //             {
    //               result = @builder.keyword_cmd(:yield, val[0])
    //             }
    //         | kDEFINED opt_nl tLPAREN2 expr rparen
    //             {
    //               result = @builder.keyword_cmd(:defined?, val[0],
    //                                             val[2], [ val[3] ], val[4])
    //             }
    //         | kNOT tLPAREN2 expr rparen
    //             {
    //               result = @builder.not_op(val[0], val[1], val[2], val[3])
    //             }
    //         | kNOT tLPAREN2 rparen
    //             {
    //               result = @builder.not_op(val[0], val[1], nil, val[2])
    //             }
    //         | fcall brace_block
    //             {
    //               method_call = @builder.call_method(nil, nil, val[0])

    //               begin_t, args, body, end_t = val[1]
    //               result      = @builder.block(method_call,
    //                               begin_t, args, body, end_t)
    //             }
    //         | method_call
    //         | method_call brace_block
    //             {
    //               begin_t, args, body, end_t = val[1]
    //               result      = @builder.block(val[0],
    //                               begin_t, args, body, end_t)
    //             }
    //         | tLAMBDA lambda
    //             {
    //               lambda_call = @builder.call_lambda(val[0])

    //               args, (begin_t, body, end_t) = val[1]
    //               result      = @builder.block(lambda_call,
    //                               begin_t, args, body, end_t)
    //             }
    //         | kIF expr_value then compstmt if_tail kEND
    //             {
    //               else_t, else_ = val[4]
    //               result = @builder.condition(val[0], val[1], val[2],
    //                                           val[3], else_t,
    //                                           else_,  val[5])
    //             }
    //         | kUNLESS expr_value then compstmt opt_else kEND
    //             {
    //               else_t, else_ = val[4]
    //               result = @builder.condition(val[0], val[1], val[2],
    //                                           else_,  else_t,
    //                                           val[3], val[5])
    //             }
    //         | kWHILE
    //             {
    //               @lexer.cond.push(true)
    //             }
    //             expr_value do
    //             {
    //               @lexer.cond.pop
    //             }
    //             compstmt kEND
    //             {
    //               result = @builder.loop(:while, val[0], val[2], val[3],
    //                                      val[5], val[6])
    //             }
    //         | kUNTIL
    //             {
    //               @lexer.cond.push(true)
    //             }
    //             expr_value do
    //             {
    //               @lexer.cond.pop
    //             }
    //             compstmt kEND
    //             {
    //               result = @builder.loop(:until, val[0], val[2], val[3],
    //                                      val[5], val[6])
    //             }
    //         | kCASE expr_value opt_terms case_body kEND
    //             {
    //               *when_bodies, (else_t, else_body) = *val[3]

    //               result = @builder.case(val[0], val[1],
    //                                      when_bodies, else_t, else_body,
    //                                      val[4])
    //             }
    //         | kCASE            opt_terms case_body kEND
    //             {
    //               *when_bodies, (else_t, else_body) = *val[2]

    //               result = @builder.case(val[0], nil,
    //                                      when_bodies, else_t, else_body,
    //                                      val[3])
    //             }
    //         | kFOR for_var kIN
    //             {
    //               @lexer.cond.push(true)
    //             }
    //             expr_value do
    //             {
    //               @lexer.cond.pop
    //             }
    //             compstmt kEND
    //             {
    //               result = @builder.for(val[0], val[1],
    //                                     val[2], val[4],
    //                                     val[5], val[7], val[8])
    //             }
    //         | kCLASS cpath superclass
    //             {
    //               @static_env.extend_static
    //               @lexer.push_cmdarg
    //             }
    //             bodystmt kEND
    //             {
    //               if in_def?
    //                 diagnostic :error, :class_in_def, nil, val[0]
    //               end

    //               lt_t, superclass = val[2]
    //               result = @builder.def_class(val[0], val[1],
    //                                           lt_t, superclass,
    //                                           val[4], val[5])

    //               @lexer.pop_cmdarg
    //               @static_env.unextend
    //             }
    //         | kCLASS tLSHFT expr term
    //             {
    //               result = @def_level
    //               @def_level = 0

    //               @static_env.extend_static
    //               @lexer.push_cmdarg
    //             }
    //             bodystmt kEND
    //             {
    //               result = @builder.def_sclass(val[0], val[1], val[2],
    //                                            val[5], val[6])

    //               @lexer.pop_cmdarg
    //               @static_env.unextend

    //               @def_level = val[4]
    //             }
    //         | kMODULE cpath
    //             {
    //               @static_env.extend_static
    //               @lexer.push_cmdarg
    //             }
    //             bodystmt kEND
    //             {
    //               if in_def?
    //                 diagnostic :error, :module_in_def, nil, val[0]
    //               end

    //               result = @builder.def_module(val[0], val[1],
    //                                            val[3], val[4])

    //               @lexer.pop_cmdarg
    //               @static_env.unextend
    //             }
    //         | kDEF fname
    //             {
    //               @def_level += 1
    //               @static_env.extend_static
    //               @lexer.push_cmdarg
    //             }
    //             f_arglist bodystmt kEND
    //             {
    //               result = @builder.def_method(val[0], val[1],
    //                           val[3], val[4], val[5])

    //               @lexer.pop_cmdarg
    //               @static_env.unextend
    //               @def_level -= 1
    //             }
    //         | kDEF singleton dot_or_colon
    //             {
    //               @lexer.state = :expr_fname
    //             }
    //             fname
    //             {
    //               @def_level += 1
    //               @static_env.extend_static
    //               @lexer.push_cmdarg
    //             }
    //             f_arglist bodystmt kEND
    //             {
    //               result = @builder.def_singleton(val[0], val[1], val[2],
    //                           val[4], val[6], val[7], val[8])

    //               @lexer.pop_cmdarg
    //               @static_env.unextend
    //               @def_level -= 1
    //             }
    //         | kBREAK
    //             {
    //               result = @builder.keyword_cmd(:break, val[0])
    //             }
    //         | kNEXT
    //             {
    //               result = @builder.keyword_cmd(:next, val[0])
    //             }
    //         | kREDO
    //             {
    //               result = @builder.keyword_cmd(:redo, val[0])
    //             }
    //         | kRETRY
    //             {
    //               result = @builder.keyword_cmd(:retry, val[0])
    //             }
    // TODO INCOMPLETE
    fn p_primary(&mut self) -> Option<Node> {
        // TODO DUMMY
        if let Some(n_literal) = self.p_literal() { return Some(n_literal); }

        // TODO DUMMY should be `strings`
        if let Some(n_string) = self.p_string() { return Some(n_string); }


        if let Some(n_var_ref) = self.p_var_ref() { return Some(n_var_ref); }

        None
    }

    //  literal: numeric
    //         | symbol
    //         | dsym
    // TODO INCOMPLETE
    // TODO DUMMY
    fn p_literal(&mut self) -> Option<Node> {
        if let Some(n_numeric) = self.p_numeric() { return Some(n_numeric); }
        if let Some(n_symbol) = self.p_symbol() { return Some(n_symbol); }

        None
    }

    // TODO DUMMY
    fn p_string(&mut self) -> Option<Node> {
        if let Token::T_STRING(token_string) = self.current_token() {
            return Some( Node::Str( token_string ) );
        }

       None
    }

    //  numeric: simple_numeric
    //             {
    //               result = val[0]
    //             }
    // TODO IMCOMPLETE
    fn p_numeric(&mut self) -> Option<Node> {
        if let Some(n_simple_numeric) = self.p_simple_numeric() { return Some(n_simple_numeric); }

        //         | tUNARY_NUM simple_numeric =tLOWEST
        //             {
        //               if @builder.respond_to? :negate
        //                 # AST builder interface compatibility
        //                 result = @builder.negate(val[0], val[1])
        //               else
        //                 result = @builder.unary_num(val[0], val[1])
        //               end
        //             }
        // TODO HANDLE %prec
        if let Token::T_UNARY_NUM(_) = self.current_token() {
            let t_unary_num = self.consume_current_token();
            if let Some(n_simple_numeric) = self.p_simple_numeric() {
                return Some(node::unary_num(t_unary_num, n_simple_numeric));
            }
        }

        None
    }

    //   simple_numeric: tINTEGER
    //                     {
    //                       @lexer.state = :expr_end
    //                       result = @builder.integer(val[0])
    //                     }
    //                 | tFLOAT
    //                     {
    //                       @lexer.state = :expr_end
    //                       result = @builder.float(val[0])
    //                     }
    //                 | tRATIONAL
    //                     {
    //                       @lexer.state = :expr_end
    //                       result = @builder.rational(val[0])
    //                     }
    //                 | tIMAGINARY
    //                     {
    //                       @lexer.state = :expr_end
    //                       result = @builder.complex(val[0])
    //                     }
    fn p_simple_numeric(&mut self) -> Option<Node> {
        match self.current_token() {
            Token::T_INTEGER(i) => {
                self.lexer.set_state(state!("expr_end"));

                self.consume_current_token();

                return Some(Node::Int(i));
            },
            _ => { return None; }
        }
    }


    //   symbol: tSYMBOL
    //             {
    //               @lexer.state = :expr_endarg
    //               result = @builder.symbol(val[0])
    //             }
    fn p_symbol(&mut self) -> Option<Node> {
        if let Token::T_SYMBOL(symbol_string) = self.current_token() {
            let _t_symbol = self.consume_current_token();

            self.lexer.set_state(state!("expr_endarg"));

            return Some(Node::Sym(symbol_string));
        }

        None
    }

    // TODO impl corresponding `none` rule from original grammar
    // fn p_none(&mut self) -> Option<Node> {
    //     Some(Node::None)
    // }
}
