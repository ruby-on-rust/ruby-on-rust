// https based on github.com/whitequark/parser/blob/2a73841d6da04a5ab9bd270561165fd766722d43/lib/parser/ruby25.y

// TODO match_token! macro

use lexer::lexing_state::LexingState;
use lexer::Lexer;
use parser::token::Token;
use ast::node;
use ast::node::Node;

// TODO dont rewrite this macro here
macro_rules! state { ($state_name:expr) => { $state_name.parse::<LexingState>().unwrap() }; }

// helpers
fn extract_string_content(token: Token) -> String {
    match token {
        Token::T_STRING(content) | Token::T_STRING_CONTENT(content) => { return content; },
        _ => { panic!("can't extract string content"); }
    }
}

fn extract_nodes(node: Node) -> Vec<Node> {
    if let Node::Nodes(nodes) = node {
        return nodes;
    } else { panic!("can't extract nodes"); }
}

pub struct Parser {
    lexer: Lexer,

    tokens: Vec<Token>,
    current_p: usize, // TODO NOTE

    no_more_tokens: bool,
}

impl Parser {
    pub fn new(input: String) -> Parser {
        Parser {
            lexer: Lexer::new(input),

            tokens: vec![],
            current_p: 0,

            no_more_tokens: false,
        }
    }

    pub fn parse(&mut self) -> Node {
        if let Some(n_program) = self.p_program() {
            return n_program;
        } else {
            println!("PARSRE failed to parse");
            println!("emitted tokens: {:?}", self.tokens);
            panic!("parser: failed to parse");
        }
    }

    // get a new one if necessary
    fn current_token(&mut self) -> Option<Token> {
        // println!("PARSER current_token, current_p: {}", self.current_p);

        if self.no_more_tokens { return None; }

        if let Some(token) = self.tokens.get(self.current_p) {
            return Some(token.clone());
        }

        // try to fetch a new one
        if let Some(token) = self.lexer.advance() {
            self.tokens.push(token.clone());
            return Some(token);
        } else {
            self.no_more_tokens = true;
            return None;
        }
    }

    // TODO handle no more token
    fn consume_current_token(&mut self) -> Token {
        let token_to_consume = self.tokens.get(self.current_p).unwrap().clone();

        self.current_p += 1;

        println!("PARSER comsume_current_token: {:?}", token_to_consume);

        return token_to_consume;
    }

    // match and consume one token
    // TODO REFINE
    // TODO cant handle token with value for now
    fn match_1_token(&mut self, token: Token) -> Option<Token> {
        println!("PARSER match_1_token, current_p: {:?}, current: {:?}, token: {:?}", self.current_p.clone(), self.current_token(), token );

        if let Some(current_token) = self.current_token() {
            if current_token == token {
                self.consume_current_token();
                return Some(current_token);
            }
        }

        return None;
    }

    // ===

    //  program: top_compstmt
    // TODO
    fn p_program(&mut self) -> Option<Node> { self.p_stmt() }

    // top_compstmt: top_stmts opt_terms
    //                 {
    //                   result = @builder.compstmt(val[0])
    //                 }
    // fn p_top_compstmt(&mut self) -> Node {
    // }

    // fn p_top_stmts(&mut self) -> Node {
    // }

    // top_stmt: stmt
    //         | klBEGIN tLCURLY top_compstmt tRCURLY
    //             {
    //               result = @builder.preexe(val[0], val[1], val[2], val[3])
    //             }

    // bodystmt: compstmt opt_rescue opt_else opt_ensure
    //             {
    //               rescue_bodies     = val[1]
    //               else_t,   else_   = val[2]
    //               ensure_t, ensure_ = val[3]
    // 
    //               if rescue_bodies.empty? && !else_.nil?
    //                 diagnostic :warning, :useless_else, nil, else_t
    //               end
    // 
    //               result = @builder.begin_body(val[0],
    //                           rescue_bodies,
    //                           else_t,   else_,
    //                           ensure_t, ensure_)
    //             }

    // compstmt: stmts opt_terms
    //             {
    //               result = @builder.compstmt(val[0])
    //             }
    fn p_compstmt(&mut self) -> Option<Node> {
        // TODO DUMMY
        return self.p_stmt();

        None
    }

    //    stmts: # nothing
    //             {
    //               result = []
    //             }
    //         | stmt_or_begin
    //             {
    //               result = [ val[0] ]
    //             }
    //         | stmts terms stmt_or_begin
    //             {
    //               result = val[0] << val[2]
    //             }
    //         | error stmt
    //             {
    //               result = [ val[1] ]
    //             }
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

    // stmt: kALIAS fitem
    //         {
    //           @lexer.state = :expr_fname
    //         }
    //         fitem
    //         {
    //           result = @builder.alias(val[0], val[1], val[3])
    //         }
    //     | kALIAS tGVAR tGVAR
    //         {
    //           result = @builder.alias(val[0],
    //                       @builder.gvar(val[1]),
    //                       @builder.gvar(val[2]))
    //         }
    //     | kALIAS tGVAR tBACK_REF
    //         {
    //           result = @builder.alias(val[0],
    //                       @builder.gvar(val[1]),
    //                       @builder.back_ref(val[2]))
    //         }
    //     | kALIAS tGVAR tNTH_REF
    //         {
    //           diagnostic :error, :nth_ref_alias, nil, val[2]
    //         }
    //     | kUNDEF undef_list
    //         {
    //           result = @builder.undef_method(val[0], val[1])
    //         }
    //     | stmt kIF_MOD expr_value
    //         {
    //           result = @builder.condition_mod(val[0], nil,
    //                                           val[1], val[2])
    //         }
    //     | stmt kUNLESS_MOD expr_value
    //         {
    //           result = @builder.condition_mod(nil, val[0],
    //                                           val[1], val[2])
    //         }
    //     | stmt kWHILE_MOD expr_value
    //         {
    //           result = @builder.loop_mod(:while, val[0], val[1], val[2])
    //         }
    //     | stmt kUNTIL_MOD expr_value
    //         {
    //           result = @builder.loop_mod(:until, val[0], val[1], val[2])
    //         }
    //     | stmt kRESCUE_MOD stmt
    //         {
    //           rescue_body = @builder.rescue_body(val[1],
    //                             nil, nil, nil,
    //                             nil, val[2])
    // 
    //           result = @builder.begin_body(val[0], [ rescue_body ])
    //         }
    //     | klEND tLCURLY compstmt tRCURLY
    //         {
    //           result = @builder.postexe(val[0], val[1], val[2], val[3])
    //         }
    //     | command_asgn
    //     | mlhs tEQL command_call
    //         {
    //           result = @builder.multi_assign(val[0], val[1], val[2])
    //         }
    //     | lhs tEQL mrhs
    //         {
    //           result = @builder.assign(val[0], val[1],
    //                       @builder.array(nil, val[2], nil))
    //         }
    //     | mlhs tEQL mrhs_arg
    //         {
    //           result = @builder.multi_assign(val[0], val[1], val[2])
    //         }
    //     | expr
    // TODO INCOMPLETE
    fn p_stmt(&mut self) -> Option<Node> {

        // expr
        if let Some(n_expr) = self.p_expr() { return Some(n_expr); }

        None
    }

    // command_asgn: lhs tEQL command_rhs
    //                 {
    //                   result = @builder.assign(val[0], val[1], val[2])
    //                 }
    //             | var_lhs tOP_ASGN command_rhs
    //                 {
    //                   result = @builder.op_assign(val[0], val[1], val[2])
    //                 }
    //             | primary_value tLBRACK2 opt_call_args rbracket tOP_ASGN command_rhs
    //                 {
    //                   result = @builder.op_assign(
    //                               @builder.index(
    //                                 val[0], val[1], val[2], val[3]),
    //                               val[4], val[5])
    //                 }
    //             | primary_value call_op tIDENTIFIER tOP_ASGN command_rhs
    //                 {
    //                   result = @builder.op_assign(
    //                               @builder.call_method(
    //                                 val[0], val[1], val[2]),
    //                               val[3], val[4])
    //                 }
    //             | primary_value call_op tCONSTANT tOP_ASGN command_rhs
    //                 {
    //                   result = @builder.op_assign(
    //                               @builder.call_method(
    //                                 val[0], val[1], val[2]),
    //                               val[3], val[4])
    //                 }
    //             | primary_value tCOLON2 tCONSTANT tOP_ASGN command_rhs
    //                 {
    //                   const  = @builder.const_op_assignable(
    //                               @builder.const_fetch(val[0], val[1], val[2]))
    //                   result = @builder.op_assign(const, val[3], val[4])
    //                 }
    //             | primary_value tCOLON2 tIDENTIFIER tOP_ASGN command_rhs
    //                 {
    //                   result = @builder.op_assign(
    //                               @builder.call_method(
    //                                 val[0], val[1], val[2]),
    //                               val[3], val[4])
    //                 }
    //             | backref tOP_ASGN command_rhs
    //                 {
    //                   @builder.op_assign(val[0], val[1], val[2])
    //                 }


    //  command_rhs: command_call =tOP_ASGN
    //             | command_call kRESCUE_MOD stmt
    //                 {
    //                   rescue_body = @builder.rescue_body(val[1],
    //                                     nil, nil, nil,
    //                                     nil, val[2])

    //                   result = @builder.begin_body(val[0], [ rescue_body ])
    //                 }
    //             | command_asgn

    //         expr: command_call
    //             | expr kAND expr
    //                 {
    //                   result = @builder.logical_op(:and, val[0], val[1], val[2])
    //                 }
    //             | expr kOR expr
    //                 {
    //                   result = @builder.logical_op(:or, val[0], val[1], val[2])
    //                 }
    //             | kNOT opt_nl expr
    //                 {
    //                   result = @builder.not_op(val[0], nil, val[2], nil)
    //                 }
    //             | tBANG command_call
    //                 {
    //                   result = @builder.not_op(val[0], nil, val[1], nil)
    //                 }
    //             | arg
    // TODO INCOMPLETE
    fn p_expr(&mut self) -> Option<Node> {
        if let Some(n_arg) = self.p_arg() { return Some(n_arg); }
        None
    }

    //       expr_value: expr

    //     command_call: command
    //                 | block_command

    //    block_command: block_call
    //                 | block_call dot_or_colon operation2 command_args
    //                     {
    //                       result = @builder.call_method(val[0], val[1], val[2],
    //                                   nil, val[3], nil)
    //                     }

    //  cmd_brace_block: tLBRACE_ARG brace_body tRCURLY
    //                     {
    //                       result = [ val[0], *val[1], val[2] ]
    //                     }

    //            fcall: operation

    //          command: fcall command_args =tLOWEST
    //                     {
    //                       result = @builder.call_method(nil, nil, val[0],
    //                                   nil, val[1], nil)
    //                     }
    //                 | fcall command_args cmd_brace_block
    //                     {
    //                       method_call = @builder.call_method(nil, nil, val[0],
    //                                         nil, val[1], nil)

    //                       begin_t, args, body, end_t = val[2]
    //                       result      = @builder.block(method_call,
    //                                       begin_t, args, body, end_t)
    //                     }
    //                 | primary_value call_op operation2 command_args =tLOWEST
    //                     {
    //                       result = @builder.call_method(val[0], val[1], val[2],
    //                                   nil, val[3], nil)
    //                     }
    //                 | primary_value call_op operation2 command_args cmd_brace_block
    //                     {
    //                       method_call = @builder.call_method(val[0], val[1], val[2],
    //                                         nil, val[3], nil)

    //                       begin_t, args, body, end_t = val[4]
    //                       result      = @builder.block(method_call,
    //                                       begin_t, args, body, end_t)
    //                     }
    //                 | primary_value tCOLON2 operation2 command_args =tLOWEST
    //                     {
    //                       result = @builder.call_method(val[0], val[1], val[2],
    //                                   nil, val[3], nil)
    //                     }
    //                 | primary_value tCOLON2 operation2 command_args cmd_brace_block
    //                     {
    //                       method_call = @builder.call_method(val[0], val[1], val[2],
    //                                         nil, val[3], nil)

    //                       begin_t, args, body, end_t = val[4]
    //                       result      = @builder.block(method_call,
    //                                       begin_t, args, body, end_t)
    //                     }
    //                 | kSUPER command_args
    //                     {
    //                       result = @builder.keyword_cmd(:super, val[0],
    //                                   nil, val[1], nil)
    //                     }
    //                 | kYIELD command_args
    //                     {
    //                       result = @builder.keyword_cmd(:yield, val[0],
    //                                   nil, val[1], nil)
    //                     }
    //                 | kRETURN call_args
    //                     {
    //                       result = @builder.keyword_cmd(:return, val[0],
    //                                   nil, val[1], nil)
    //                     }
    //                 | kBREAK call_args
    //                     {
    //                       result = @builder.keyword_cmd(:break, val[0],
    //                                   nil, val[1], nil)
    //                     }
    //                 | kNEXT call_args
    //                     {
    //                       result = @builder.keyword_cmd(:next, val[0],
    //                                   nil, val[1], nil)
    //                     }

    //         mlhs: mlhs_basic
    //                 {
    //                   result = @builder.multi_lhs(nil, val[0], nil)
    //                 }
    //             | tLPAREN mlhs_inner rparen
    //                 {
    //                   result = @builder.begin(val[0], val[1], val[2])
    //                 }

    //   mlhs_inner: mlhs_basic
    //                 {
    //                   result = @builder.multi_lhs(nil, val[0], nil)
    //                 }
    //             | tLPAREN mlhs_inner rparen
    //                 {
    //                   result = @builder.multi_lhs(val[0], val[1], val[2])
    //                 }

    //   mlhs_basic: mlhs_head
    //             | mlhs_head mlhs_item
    //                 {
    //                   result = val[0].
    //                               push(val[1])
    //                 }
    //             | mlhs_head tSTAR mlhs_node
    //                 {
    //                   result = val[0].
    //                               push(@builder.splat(val[1], val[2]))
    //                 }
    //             | mlhs_head tSTAR mlhs_node tCOMMA mlhs_post
    //                 {
    //                   result = val[0].
    //                               push(@builder.splat(val[1], val[2])).
    //                               concat(val[4])
    //                 }
    //             | mlhs_head tSTAR
    //                 {
    //                   result = val[0].
    //                               push(@builder.splat(val[1]))
    //                 }
    //             | mlhs_head tSTAR tCOMMA mlhs_post
    //                 {
    //                   result = val[0].
    //                               push(@builder.splat(val[1])).
    //                               concat(val[3])
    //                 }
    //             | tSTAR mlhs_node
    //                 {
    //                   result = [ @builder.splat(val[0], val[1]) ]
    //                 }
    //             | tSTAR mlhs_node tCOMMA mlhs_post
    //                 {
    //                   result = [ @builder.splat(val[0], val[1]),
    //                              *val[3] ]
    //                 }
    //             | tSTAR
    //                 {
    //                   result = [ @builder.splat(val[0]) ]
    //                 }
    //             | tSTAR tCOMMA mlhs_post
    //                 {
    //                   result = [ @builder.splat(val[0]),
    //                              *val[2] ]
    //                 }

    //    mlhs_item: mlhs_node
    //             | tLPAREN mlhs_inner rparen
    //                 {
    //                   result = @builder.begin(val[0], val[1], val[2])
    //                 }

    //    mlhs_head: mlhs_item tCOMMA
    //                 {
    //                   result = [ val[0] ]
    //                 }
    //             | mlhs_head mlhs_item tCOMMA
    //                 {
    //                   result = val[0] << val[1]
    //                 }

    //    mlhs_post: mlhs_item
    //                 {
    //                   result = [ val[0] ]
    //                 }
    //             | mlhs_post tCOMMA mlhs_item
    //                 {
    //                   result = val[0] << val[2]
    //                 }

    //    mlhs_node: user_variable
    //                 {
    //                   result = @builder.assignable(val[0])
    //                 }
    //             | keyword_variable
    //                 {
    //                   result = @builder.assignable(val[0])
    //                 }
    //             | primary_value tLBRACK2 opt_call_args rbracket
    //                 {
    //                   result = @builder.index_asgn(val[0], val[1], val[2], val[3])
    //                 }
    //             | primary_value call_op tIDENTIFIER
    //                 {
    //                   result = @builder.attr_asgn(val[0], val[1], val[2])
    //                 }
    //             | primary_value tCOLON2 tIDENTIFIER
    //                 {
    //                   result = @builder.attr_asgn(val[0], val[1], val[2])
    //                 }
    //             | primary_value call_op tCONSTANT
    //                 {
    //                   result = @builder.attr_asgn(val[0], val[1], val[2])
    //                 }
    //             | primary_value tCOLON2 tCONSTANT
    //                 {
    //                   result = @builder.assignable(
    //                               @builder.const_fetch(val[0], val[1], val[2]))
    //                 }
    //             | tCOLON3 tCONSTANT
    //                 {
    //                   result = @builder.assignable(
    //                               @builder.const_global(val[0], val[1]))
    //                 }
    //             | backref
    //                 {
    //                   result = @builder.assignable(val[0])
    //                 }

    //  lhs: user_variable
    //         {
    //           result = @builder.assignable(val[0])
    //         }
    //     | keyword_variable
    //         {
    //           result = @builder.assignable(val[0])
    //         }
    //     | primary_value tLBRACK2 opt_call_args rbracket
    //         {
    //           result = @builder.index_asgn(val[0], val[1], val[2], val[3])
    //         }
    //     | primary_value call_op tIDENTIFIER
    //         {
    //           result = @builder.attr_asgn(val[0], val[1], val[2])
    //         }
    //     | primary_value tCOLON2 tIDENTIFIER
    //         {
    //           result = @builder.attr_asgn(val[0], val[1], val[2])
    //         }
    //     | primary_value call_op tCONSTANT
    //         {
    //           result = @builder.attr_asgn(val[0], val[1], val[2])
    //         }
    //     | primary_value tCOLON2 tCONSTANT
    //         {
    //           result = @builder.assignable(
    //                       @builder.const_fetch(val[0], val[1], val[2]))
    //         }
    //     | tCOLON3 tCONSTANT
    //         {
    //           result = @builder.assignable(
    //                       @builder.const_global(val[0], val[1]))
    //         }
    //     | backref
    //         {
    //           result = @builder.assignable(val[0])
    //         }
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

    //        cname: tIDENTIFIER
    //                 {
    //                   diagnostic :error, :module_name_const, nil, val[0]
    //                 }
    //             | tCONSTANT

    //        cpath: tCOLON3 cname
    //                 {
    //                   result = @builder.const_global(val[0], val[1])
    //                 }
    //             | cname
    //                 {
    //                   result = @builder.const(val[0])
    //                 }
    //             | primary_value tCOLON2 cname
    //                 {
    //                   result = @builder.const_fetch(val[0], val[1], val[2])
    //                 }

    //        fname: tIDENTIFIER | tCONSTANT | tFID
    //             | op
    //             | reswords

    //         fsym: fname
    //                 {
    //                   result = @builder.symbol(val[0])
    //                 }
    //             | symbol

    //        fitem: fsym
    //             | dsym

    //   undef_list: fitem
    //                 {
    //                   result = [ val[0] ]
    //                 }
    //             | undef_list tCOMMA
    //                 {
    //                   @lexer.state = :expr_fname
    //                 }
    //                 fitem
    //                 {
    //                   result = val[0] << val[3]
    //                 }

    //           op:   tPIPE    | tCARET  | tAMPER2  | tCMP  | tEQ     | tEQQ
    //             |   tMATCH   | tNMATCH | tGT      | tGEQ  | tLT     | tLEQ
    //             |   tNEQ     | tLSHFT  | tRSHFT   | tPLUS | tMINUS  | tSTAR2
    //             |   tSTAR    | tDIVIDE | tPERCENT | tPOW  | tBANG   | tTILDE
    //             |   tUPLUS   | tUMINUS | tAREF    | tASET | tDSTAR  | tBACK_REF2

    //     reswords: k__LINE__ | k__FILE__ | k__ENCODING__ | klBEGIN | klEND
    //             | kALIAS    | kAND      | kBEGIN        | kBREAK  | kCASE
    //             | kCLASS    | kDEF      | kDEFINED      | kDO     | kELSE
    //             | kELSIF    | kEND      | kENSURE       | kFALSE  | kFOR
    //             | kIN       | kMODULE   | kNEXT         | kNIL    | kNOT
    //             | kOR       | kREDO     | kRESCUE       | kRETRY  | kRETURN
    //             | kSELF     | kSUPER    | kTHEN         | kTRUE   | kUNDEF
    //             | kWHEN     | kYIELD    | kIF           | kUNLESS | kWHILE
    //             | kUNTIL

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
        //     | var_lhs tOP_ASGN arg_rhs
        //         {
        //           result = @builder.op_assign(val[0], val[1], val[2])
        //         }
        //     | primary_value tLBRACK2 opt_call_args rbracket tOP_ASGN arg_rhs
        //         {
        //           result = @builder.op_assign(
        //                       @builder.index(
        //                         val[0], val[1], val[2], val[3]),
        //                       val[4], val[5])
        //         }
        //     | primary_value call_op tIDENTIFIER tOP_ASGN arg_rhs
        //         {
        //           result = @builder.op_assign(
        //                       @builder.call_method(
        //                         val[0], val[1], val[2]),
        //                       val[3], val[4])
        //         }
        //     | primary_value call_op tCONSTANT tOP_ASGN arg_rhs
        //         {
        //           result = @builder.op_assign(
        //                       @builder.call_method(
        //                         val[0], val[1], val[2]),
        //                       val[3], val[4])
        //         }
        //     | primary_value tCOLON2 tIDENTIFIER tOP_ASGN arg_rhs
        //         {
        //           result = @builder.op_assign(
        //                       @builder.call_method(
        //                         val[0], val[1], val[2]),
        //                       val[3], val[4])
        //         }
        //     | primary_value tCOLON2 tCONSTANT tOP_ASGN arg_rhs
        //         {
        //           const  = @builder.const_op_assignable(
        //                       @builder.const_fetch(val[0], val[1], val[2]))
        //           result = @builder.op_assign(const, val[3], val[4])
        //         }
        //     | tCOLON3 tCONSTANT tOP_ASGN arg_rhs
        //         {
        //           const  = @builder.const_op_assignable(
        //                       @builder.const_global(val[0], val[1]))
        //           result = @builder.op_assign(const, val[2], val[3])
        //         }
        //     | backref tOP_ASGN arg_rhs
        //         {
        //           result = @builder.op_assign(val[0], val[1], val[2])
        //         }
        //     | arg tDOT2 arg
        //         {
        //           result = @builder.range_inclusive(val[0], val[1], val[2])
        //         }
        //     | arg tDOT3 arg
        //         {
        //           result = @builder.range_exclusive(val[0], val[1], val[2])
        //         }
        //     | arg tPLUS arg
        //         {
        //           result = @builder.binary_op(val[0], val[1], val[2])
        //         }
        //     | arg tMINUS arg
        //         {
        //           result = @builder.binary_op(val[0], val[1], val[2])
        //         }
        //     | arg tSTAR2 arg
        //         {
        //           result = @builder.binary_op(val[0], val[1], val[2])
        //         }
        //     | arg tDIVIDE arg
        //         {
        //           result = @builder.binary_op(val[0], val[1], val[2])
        //         }
        //     | arg tPERCENT arg
        //         {
        //           result = @builder.binary_op(val[0], val[1], val[2])
        //         }
        //     | arg tPOW arg
        //         {
        //           result = @builder.binary_op(val[0], val[1], val[2])
        //         }
        //     | tUNARY_NUM simple_numeric tPOW arg
        //         {
        //           result = @builder.unary_op(val[0],
        //                       @builder.binary_op(
        //                         val[1], val[2], val[3]))
        //         }
        //     | tUPLUS arg
        //         {
        //           result = @builder.unary_op(val[0], val[1])
        //         }
        //     | tUMINUS arg
        //         {
        //           result = @builder.unary_op(val[0], val[1])
        //         }
        //     | arg tPIPE arg
        //         {
        //           result = @builder.binary_op(val[0], val[1], val[2])
        //         }
        //     | arg tCARET arg
        //         {
        //           result = @builder.binary_op(val[0], val[1], val[2])
        //         }
        //     | arg tAMPER2 arg
        //         {
        //           result = @builder.binary_op(val[0], val[1], val[2])
        //         }
        //     | arg tCMP arg
        //         {
        //           result = @builder.binary_op(val[0], val[1], val[2])
        //         }
        //     | rel_expr =tCMP
        //     | arg tEQ arg
        //         {
        //           result = @builder.binary_op(val[0], val[1], val[2])
        //         }
        //     | arg tEQQ arg
        //         {
        //           result = @builder.binary_op(val[0], val[1], val[2])
        //         }
        //     | arg tNEQ arg
        //         {
        //           result = @builder.binary_op(val[0], val[1], val[2])
        //         }
        //     | arg tMATCH arg
        //         {
        //           result = @builder.match_op(val[0], val[1], val[2])
        //         }
        //     | arg tNMATCH arg
        //         {
        //           result = @builder.binary_op(val[0], val[1], val[2])
        //         }
        //     | tBANG arg
        //         {
        //           result = @builder.not_op(val[0], nil, val[1], nil)
        //         }
        //     | tTILDE arg
        //         {
        //           result = @builder.unary_op(val[0], val[1])
        //         }
        //     | arg tLSHFT arg
        //         {
        //           result = @builder.binary_op(val[0], val[1], val[2])
        //         }
        //     | arg tRSHFT arg
        //         {
        //           result = @builder.binary_op(val[0], val[1], val[2])
        //         }
        //     | arg tANDOP arg
        //         {
        //           result = @builder.logical_op(:and, val[0], val[1], val[2])
        //         }
        //     | arg tOROP arg
        //         {
        //           result = @builder.logical_op(:or, val[0], val[1], val[2])
        //         }
        //     | kDEFINED opt_nl arg
        //         {
        //           result = @builder.keyword_cmd(:defined?, val[0], nil, [ val[2] ], nil)
        //         }
        //     | arg tEH arg opt_nl tCOLON arg
        //         {
        //           result = @builder.ternary(val[0], val[1],
        //                                     val[2], val[4], val[5])
        //         }
        //     | primary
        if let Some(n_primary) = self.p_primary() { return Some(n_primary); }

        None
    }

    //        relop: tGT | tLT | tGEQ | tLEQ

    //     rel_expr: arg relop arg =tGT
    //                 {
    //                   result = @builder.binary_op(val[0], val[1], val[2])
    //                 }
    //             | rel_expr relop arg =tGT
    //                 {
    //                   result = @builder.binary_op(val[0], val[1], val[2])
    //                 }

    //    arg_value: arg
    fn p_arg_value(&mut self) -> Option<Node> {
        println!("p_arg_value");

        if let Some(n_arg) = self.p_arg() { return Some(n_arg); }

        None
    }

    //    aref_args: none
    //             | args trailer
    //             | args tCOMMA assocs trailer
    //                 {
    //                   result = val[0] << @builder.associate(nil, val[2], nil)
    //                 }
    //             | assocs trailer
    //                 {
    //                   result = [ @builder.associate(nil, val[0], nil) ]
    //                 }
    // NOTE the rule `none` will be handled by parent rule
    // TODO WIP INCOMPLETE
    fn p_aref_args(&mut self) -> Option<Node> {
        println!("p_aref_args");

        if let Some(n_args) = self.p_args() {
            if let Some(n_trailer) = self.p_trailer() {
                return Some(Node::Dummy);
            }

            // trailer being none
            return Some(n_args);
        }

        None
    }

    //  arg_rhs: arg =tOP_ASGN
    //         | arg kRESCUE_MOD arg
    //             {
    //               rescue_body = @builder.rescue_body(val[1],
    //                                 nil, nil, nil,
    //                                 nil, val[2])
    // 
    //               result = @builder.begin_body(val[0], [ rescue_body ])
    //             }
    // TODO INCOMPLETE
    // TODO handle %prec
    fn p_arg_rhs(&mut self) -> Option<Node> {
        // TODO DUMMY
        if let Some(n_primary) = self.p_primary() { return Some(n_primary); }

        None
    }

    //       paren_args: tLPAREN2 opt_call_args rparen
    //                     {
    //                       result = val
    //                     }

    //   opt_paren_args: # nothing
    //                     {
    //                       result = [ nil, [], nil ]
    //                     }
    //                 | paren_args

    //    opt_call_args: # nothing
    //                     {
    //                       result = []
    //                     }
    //                 | call_args
    //                 | args tCOMMA
    //                 | args tCOMMA assocs tCOMMA
    //                     {
    //                       result = val[0] << @builder.associate(nil, val[2], nil)
    //                     }
    //                 | assocs tCOMMA
    //                     {
    //                       result = [ @builder.associate(nil, val[0], nil) ]
    //                     }

    //        call_args: command
    //                     {
    //                       result = [ val[0] ]
    //                     }
    //                 | args opt_block_arg
    //                     {
    //                       result = val[0].concat(val[1])
    //                     }
    //                 | assocs opt_block_arg
    //                     {
    //                       result = [ @builder.associate(nil, val[0], nil) ]
    //                       result.concat(val[1])
    //                     }
    //                 | args tCOMMA assocs opt_block_arg
    //                     {
    //                       assocs = @builder.associate(nil, val[2], nil)
    //                       result = val[0] << assocs
    //                       result.concat(val[3])
    //                     }
    //                 | block_arg
    //                     {
    //                       result =  [ val[0] ]
    //                     }

    //     command_args:   {
    //                       result = @lexer.cmdarg.dup
    //                       @lexer.cmdarg.push(true)
    //                     }
    //                   call_args
    //                     {
    //                       @lexer.cmdarg = val[0]

    //                       result = val[1]
    //                     }

    //        block_arg: tAMPER arg_value
    //                     {
    //                       result = @builder.block_pass(val[0], val[1])
    //                     }

    //    opt_block_arg: tCOMMA block_arg
    //                     {
    //                       result = [ val[1] ]
    //                     }
    //                 | # nothing
    //                     {
    //                       result = []
    //                     }


    // args: arg_value
    //         {
    //           result = [ val[0] ]
    //         }
    //     | tSTAR arg_value
    //         {
    //           result = [ @builder.splat(val[0], val[1]) ]
    //         }
    //     | args tCOMMA arg_value
    //         {
    //           result = val[0] << val[2]
    //         }
    //     | args tCOMMA tSTAR arg_value
    //         {
    //           result = val[0] << @builder.splat(val[2], val[3])
    //         }
    // TODO INCOMPLETE
    // 
    // NOTE transformed into non-recursive form
    // 
    // original grammar:
    // as: a | * a | as , a | as , * a 
    // 
    // transformed:
    // as:  a  [, a | , * a]
    //    | *a [, a | , * a] 
    // 
    fn p_args(&mut self) -> Option<Node> {
        println!("p_args");

        if let Some(n_arg_value) = self.p_arg_value() {
            let mut nodes = vec![n_arg_value];

            loop {
                if let Some(t_comma) = self.match_1_token(Token::T_COMMA) {
                    if let Some(n_arg_value) = self.p_arg_value() {
                        nodes.push(n_arg_value);
                        continue;
                    }
                    if let Some(t_star) = self.match_1_token(Token::T_STAR) {
                        if let Some(n_arg_value) = self.p_arg_value() {
                            panic!("n_arg_value t_star n_arg_value UNIMPL");
                            // TODO handle builder.splat
                            // nodes.push(n_arg_value);
                            // continue;
                        }
                    }
                    break;
                } else {
                    break;
                }
            }

            return Some( Node::Nodes( nodes ) );
        }

        if let Some(t_star) = self.match_1_token(Token::T_STAR) {
            if let Some(n_arg_value) = self.p_arg_value() {
                // TODO handle builder.splat
                // let nodes = vec![n_arg_value];

                panic!("p_args t_star n_arg_value UNIMPL");

                // loop {
                //     if let Some(t_comma) = self.match_1_token(Token::T_COMMA) {
                //         if let Some(n_arg_value) = self.p_arg_value() {
                //             nodes.push(n_arg_value);
                //             continue;
                //         }
                //         if let Some(t_star) = self.match_1_token(Token::T_STAR) {
                //             if let Some(n_arg_value) = self.p_arg_value() {
                //                 panic!("n_arg_value t_star n_arg_value UNIMPL");
                //                 // TODO handle builder.splat
                //                 // nodes.push(n_arg_value);
                //                 // continue;
                //             }
                //         }
                //         break;
                //     } else {
                //         break;
                //     }
                // }

                // return Some( Node::Nodes( nodes ) );
            }
        }


        None
    }


    // mrhs_arg: mrhs
    //             {
    //               result = @builder.array(nil, val[0], nil)
    //             }
    //         | arg_value

    //     mrhs: args tCOMMA arg_value
    //             {
    //               result = val[0] << val[2]
    //             }
    //         | args tCOMMA tSTAR arg_value
    //             {
    //               result = val[0] << @builder.splat(val[2], val[3])
    //             }
    //         | tSTAR arg_value
    //             {
    //               result = [ @builder.splat(val[0], val[1]) ]
    //             }

    // TODO INCOMPLETE
    fn p_primary(&mut self) -> Option<Node> {

        //  primary: literal
        if let Some(n_literal) = self.p_literal() { return Some(n_literal); }
        //         | strings
        if let Some(n_strings) = self.p_strings() { return Some(n_strings); }
        //         | xstring
        // if let Some(n_xstring) = self.p_xstring() { return Some(n_xstring); }
        //         | regexp
        //         | words
        if let Some(n_words) = self.p_words() { return Some(n_words); }
        //         | qwords
        if let Some(n_qwords) = self.p_qwords() { return Some(n_qwords); }
        //         | symbols
        //         | qsymbols
        //         | var_ref
        if let Some(n_var_ref) = self.p_var_ref() { return Some(n_var_ref); }
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
        // 
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
        // 
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
        // TODO WIP
        if let Some(t_lbrack) = self.match_1_token(Token::T_LBRACK) {
            // special rule for aref_args being `none`
            if let Some(t_rbrack) = self.match_1_token(Token::T_RBRACK) {
                return Some(Node::Array(vec![]));
            }

            if let Some(n_aref_args) = self.p_aref_args() {
                if let Some(t_rbrack) = self.match_1_token(Token::T_RBRACK) {
                    // TODO handle builder.array
                    if let Node::Nodes(nodes) = n_aref_args {
                        return Some(Node::Array(nodes));
                    } else { panic!("cant extract nodes from n_aref_args"); }
                }
            }
        }
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
        // 
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
        // 
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
        // 
        //               result = @builder.case(val[0], val[1],
        //                                      when_bodies, else_t, else_body,
        //                                      val[4])
        //             }
        //         | kCASE            opt_terms case_body kEND
        //             {
        //               *when_bodies, (else_t, else_body) = *val[2]
        // 
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
        // 
        //               lt_t, superclass = val[2]
        //               result = @builder.def_class(val[0], val[1],
        //                                           lt_t, superclass,
        //                                           val[4], val[5])
        // 
        //               @lexer.pop_cmdarg
        //               @static_env.unextend
        //             }
        //         | kCLASS tLSHFT expr term
        //             {
        //               result = @def_level
        //               @def_level = 0
        // 
        //               @static_env.extend_static
        //               @lexer.push_cmdarg
        //             }
        //             bodystmt kEND
        //             {
        //               result = @builder.def_sclass(val[0], val[1], val[2],
        //                                            val[5], val[6])
        // 
        //               @lexer.pop_cmdarg
        //               @static_env.unextend
        // 
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
        // 
        //               result = @builder.def_module(val[0], val[1],
        //                                            val[3], val[4])
        // 
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
        // 
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
        // 
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

        None
    }

    //    primary_value: primary

    //             then: term
    //                 | kTHEN
    //                 | term kTHEN
    //                     {
    //                       result = val[1]
    //                     }

    //               do: term
    //                 | kDO_COND

    //          if_tail: opt_else
    //                 | kELSIF expr_value then compstmt if_tail
    //                     {
    //                       else_t, else_ = val[4]
    //                       result = [ val[0],
    //                                  @builder.condition(val[0], val[1], val[2],
    //                                                     val[3], else_t,
    //                                                     else_,  nil),
    //                                ]
    //                     }

    //         opt_else: none
    //                 | kELSE compstmt
    //                     {
    //                       result = val
    //                     }

    //          for_var: lhs
    //                 | mlhs

    //           f_marg: f_norm_arg
    //                     {
    //                       result = @builder.arg(val[0])
    //                     }
    //                 | tLPAREN f_margs rparen
    //                     {
    //                       result = @builder.multi_lhs(val[0], val[1], val[2])
    //                     }

    //      f_marg_list: f_marg
    //                     {
    //                       result = [ val[0] ]
    //                     }
    //                 | f_marg_list tCOMMA f_marg
    //                     {
    //                       result = val[0] << val[2]
    //                     }

    //          f_margs: f_marg_list
    //                 | f_marg_list tCOMMA tSTAR f_norm_arg
    //                     {
    //                       result = val[0].
    //                                   push(@builder.restarg(val[2], val[3]))
    //                     }
    //                 | f_marg_list tCOMMA tSTAR f_norm_arg tCOMMA f_marg_list
    //                     {
    //                       result = val[0].
    //                                   push(@builder.restarg(val[2], val[3])).
    //                                   concat(val[5])
    //                     }
    //                 | f_marg_list tCOMMA tSTAR
    //                     {
    //                       result = val[0].
    //                                   push(@builder.restarg(val[2]))
    //                     }
    //                 | f_marg_list tCOMMA tSTAR            tCOMMA f_marg_list
    //                     {
    //                       result = val[0].
    //                                   push(@builder.restarg(val[2])).
    //                                   concat(val[4])
    //                     }
    //                 |                    tSTAR f_norm_arg
    //                     {
    //                       result = [ @builder.restarg(val[0], val[1]) ]
    //                     }
    //                 |                    tSTAR f_norm_arg tCOMMA f_marg_list
    //                     {
    //                       result = [ @builder.restarg(val[0], val[1]),
    //                                  *val[3] ]
    //                     }
    //                 |                    tSTAR
    //                     {
    //                       result = [ @builder.restarg(val[0]) ]
    //                     }
    //                 |                    tSTAR tCOMMA f_marg_list
    //                     {
    //                       result = [ @builder.restarg(val[0]),
    //                                  *val[2] ]
    //                     }

    //  block_args_tail: f_block_kwarg tCOMMA f_kwrest opt_f_block_arg
    //                     {
    //                       result = val[0].concat(val[2]).concat(val[3])
    //                     }
    //                 | f_block_kwarg opt_f_block_arg
    //                     {
    //                       result = val[0].concat(val[1])
    //                     }
    //                 | f_kwrest opt_f_block_arg
    //                     {
    //                       result = val[0].concat(val[1])
    //                     }
    //                 | f_block_arg
    //                     {
    //                       result = [ val[0] ]
    //                     }

    // opt_block_args_tail:
    //                   tCOMMA block_args_tail
    //                     {
    //                       result = val[1]
    //                     }
    //                 | # nothing
    //                     {
    //                       result = []
    //                     }

    //      block_param: f_arg tCOMMA f_block_optarg tCOMMA f_rest_arg              opt_block_args_tail
    //                     {
    //                       result = val[0].
    //                                   concat(val[2]).
    //                                   concat(val[4]).
    //                                   concat(val[5])
    //                     }
    //                 | f_arg tCOMMA f_block_optarg tCOMMA f_rest_arg tCOMMA f_arg opt_block_args_tail
    //                     {
    //                       result = val[0].
    //                                   concat(val[2]).
    //                                   concat(val[4]).
    //                                   concat(val[6]).
    //                                   concat(val[7])
    //                     }
    //                 | f_arg tCOMMA f_block_optarg                                opt_block_args_tail
    //                     {
    //                       result = val[0].
    //                                   concat(val[2]).
    //                                   concat(val[3])
    //                     }
    //                 | f_arg tCOMMA f_block_optarg tCOMMA                   f_arg opt_block_args_tail
    //                     {
    //                       result = val[0].
    //                                   concat(val[2]).
    //                                   concat(val[4]).
    //                                   concat(val[5])
    //                     }
    //                 | f_arg tCOMMA                       f_rest_arg              opt_block_args_tail
    //                     {
    //                       result = val[0].
    //                                   concat(val[2]).
    //                                   concat(val[3])
    //                     }
    //                 | f_arg tCOMMA
    //                 | f_arg tCOMMA                       f_rest_arg tCOMMA f_arg opt_block_args_tail
    //                     {
    //                       result = val[0].
    //                                   concat(val[2]).
    //                                   concat(val[4]).
    //                                   concat(val[5])
    //                     }
    //                 | f_arg                                                      opt_block_args_tail
    //                     {
    //                       if val[1].empty? && val[0].size == 1
    //                         result = [@builder.procarg0(val[0][0])]
    //                       else
    //                         result = val[0].concat(val[1])
    //                       end
    //                     }
    //                 | f_block_optarg tCOMMA              f_rest_arg              opt_block_args_tail
    //                     {
    //                       result = val[0].
    //                                   concat(val[2]).
    //                                   concat(val[3])
    //                     }
    //                 | f_block_optarg tCOMMA              f_rest_arg tCOMMA f_arg opt_block_args_tail
    //                     {
    //                       result = val[0].
    //                                   concat(val[2]).
    //                                   concat(val[4]).
    //                                   concat(val[5])
    //                     }
    //                 | f_block_optarg                                             opt_block_args_tail
    //                     {
    //                       result = val[0].
    //                                   concat(val[1])
    //                     }
    //                 | f_block_optarg tCOMMA                                f_arg opt_block_args_tail
    //                     {
    //                       result = val[0].
    //                                   concat(val[2]).
    //                                   concat(val[3])
    //                     }
    //                 |                                    f_rest_arg              opt_block_args_tail
    //                     {
    //                       result = val[0].
    //                                   concat(val[1])
    //                     }
    //                 |                                    f_rest_arg tCOMMA f_arg opt_block_args_tail
    //                     {
    //                       result = val[0].
    //                                   concat(val[2]).
    //                                   concat(val[3])
    //                     }
    //                 |                                                                block_args_tail

    //  opt_block_param: # nothing
    //                     {
    //                       result = @builder.args(nil, [], nil)
    //                     }
    //                 | block_param_def
    //                     {
    //                       @lexer.state = :expr_value
    //                     }

    //  block_param_def: tPIPE opt_bv_decl tPIPE
    //                     {
    //                       result = @builder.args(val[0], val[1], val[2])
    //                     }
    //                 | tOROP
    //                     {
    //                       result = @builder.args(val[0], [], val[0])
    //                     }
    //                 | tPIPE block_param opt_bv_decl tPIPE
    //                     {
    //                       result = @builder.args(val[0], val[1].concat(val[2]), val[3])
    //                     }

    //      opt_bv_decl: opt_nl
    //                     {
    //                       result = []
    //                     }
    //                 | opt_nl tSEMI bv_decls opt_nl
    //                     {
    //                       result = val[2]
    //                     }

    //         bv_decls: bvar
    //                     {
    //                       result = [ val[0] ]
    //                     }
    //                 | bv_decls tCOMMA bvar
    //                     {
    //                       result = val[0] << val[2]
    //                     }

    //             bvar: tIDENTIFIER
    //                     {
    //                       @static_env.declare val[0][0]
    //                       result = @builder.shadowarg(val[0])
    //                     }
    //                 | f_bad_arg

    //           lambda:   {
    //                       @static_env.extend_dynamic
    //                     }
    //                   f_larglist
    //                     {
    //                       result = @lexer.cmdarg.dup
    //                       @lexer.cmdarg.clear
    //                     }
    //                   lambda_body
    //                     {
    //                       @lexer.cmdarg = val[2]
    //                       @lexer.cmdarg.lexpop

    //                       result = [ val[1], val[3] ]

    //                       @static_env.unextend
    //                     }

    //      f_larglist: tLPAREN2 f_args opt_bv_decl tRPAREN
    //                     {
    //                       result = @builder.args(val[0], val[1].concat(val[2]), val[3])
    //                     }
    //                 | f_args
    //                     {
    //                       result = @builder.args(nil, val[0], nil)
    //                     }

    //      lambda_body: tLAMBEG compstmt tRCURLY
    //                     {
    //                       result = [ val[0], val[1], val[2] ]
    //                     }
    //                 | kDO_LAMBDA compstmt kEND
    //                     {
    //                       result = [ val[0], val[1], val[2] ]
    //                     }

    //         do_block: kDO_BLOCK do_body kEND
    //                     {
    //                       result = [ val[0], *val[1], val[2] ]
    //                     }

    //       block_call: command do_block
    //                     {
    //                       begin_t, block_args, body, end_t = val[1]
    //                       result      = @builder.block(val[0],
    //                                       begin_t, block_args, body, end_t)
    //                     }
    //                 | block_call dot_or_colon operation2 opt_paren_args
    //                     {
    //                       lparen_t, args, rparen_t = val[3]
    //                       result = @builder.call_method(val[0], val[1], val[2],
    //                                   lparen_t, args, rparen_t)
    //                     }
    //                 | block_call dot_or_colon operation2 opt_paren_args brace_block
    //                     {
    //                       lparen_t, args, rparen_t = val[3]
    //                       method_call = @builder.call_method(val[0], val[1], val[2],
    //                                       lparen_t, args, rparen_t)

    //                       begin_t, args, body, end_t = val[4]
    //                       result      = @builder.block(method_call,
    //                                       begin_t, args, body, end_t)
    //                     }
    //                 | block_call dot_or_colon operation2 command_args do_block
    //                     {
    //                       method_call = @builder.call_method(val[0], val[1], val[2],
    //                                       nil, val[3], nil)

    //                       begin_t, args, body, end_t = val[4]
    //                       result      = @builder.block(method_call,
    //                                       begin_t, args, body, end_t)
    //                     }

    //      method_call: fcall paren_args
    //                     {
    //                       lparen_t, args, rparen_t = val[1]
    //                       result = @builder.call_method(nil, nil, val[0],
    //                                   lparen_t, args, rparen_t)
    //                     }
    //                 | primary_value call_op operation2 opt_paren_args
    //                     {
    //                       lparen_t, args, rparen_t = val[3]
    //                       result = @builder.call_method(val[0], val[1], val[2],
    //                                   lparen_t, args, rparen_t)
    //                     }
    //                 | primary_value tCOLON2 operation2 paren_args
    //                     {
    //                       lparen_t, args, rparen_t = val[3]
    //                       result = @builder.call_method(val[0], val[1], val[2],
    //                                   lparen_t, args, rparen_t)
    //                     }
    //                 | primary_value tCOLON2 operation3
    //                     {
    //                       result = @builder.call_method(val[0], val[1], val[2])
    //                     }
    //                 | primary_value call_op paren_args
    //                     {
    //                       lparen_t, args, rparen_t = val[2]
    //                       result = @builder.call_method(val[0], val[1], nil,
    //                                   lparen_t, args, rparen_t)
    //                     }
    //                 | primary_value tCOLON2 paren_args
    //                     {
    //                       lparen_t, args, rparen_t = val[2]
    //                       result = @builder.call_method(val[0], val[1], nil,
    //                                   lparen_t, args, rparen_t)
    //                     }
    //                 | kSUPER paren_args
    //                     {
    //                       lparen_t, args, rparen_t = val[1]
    //                       result = @builder.keyword_cmd(:super, val[0],
    //                                   lparen_t, args, rparen_t)
    //                     }
    //                 | kSUPER
    //                     {
    //                       result = @builder.keyword_cmd(:zsuper, val[0])
    //                     }
    //                 | primary_value tLBRACK2 opt_call_args rbracket
    //                     {
    //                       result = @builder.index(val[0], val[1], val[2], val[3])
    //                     }

    //      brace_block: tLCURLY brace_body tRCURLY
    //                     {
    //                       result = [ val[0], *val[1], val[2] ]
    //                     }
    //                 | kDO do_body kEND
    //                     {
    //                       result = [ val[0], *val[1], val[2] ]
    //                     }

    //       brace_body:   {
    //                       @static_env.extend_dynamic
    //                     }
    //                     {
    //                       result = @lexer.cmdarg.dup
    //                       @lexer.cmdarg.clear
    //                     }
    //                     opt_block_param compstmt
    //                     {
    //                       result = [ val[2], val[3] ]

    //                       @static_env.unextend
    //                       @lexer.cmdarg = val[1]
    //                       @lexer.cmdarg.pop
    //                     }

    //          do_body:   {
    //                       @static_env.extend_dynamic
    //                     }
    //                     {
    //                       result = @lexer.cmdarg.dup
    //                       @lexer.cmdarg.clear
    //                     }
    //                     opt_block_param bodystmt
    //                     {
    //                       result = [ val[2], val[3] ]

    //                       @static_env.unextend
    //                       @lexer.cmdarg = val[1]
    //                     }

    //        case_body: kWHEN args then compstmt cases
    //                     {
    //                       result = [ @builder.when(val[0], val[1], val[2], val[3]),
    //                                  *val[4] ]
    //                     }

    //            cases: opt_else
    //                     {
    //                       result = [ val[0] ]
    //                     }
    //                 | case_body

    //       opt_rescue: kRESCUE exc_list exc_var then compstmt opt_rescue
    //                     {
    //                       assoc_t, exc_var = val[2]

    //                       if val[1]
    //                         exc_list = @builder.array(nil, val[1], nil)
    //                       end

    //                       result = [ @builder.rescue_body(val[0],
    //                                       exc_list, assoc_t, exc_var,
    //                                       val[3], val[4]),
    //                                  *val[5] ]
    //                     }
    //                 |
    //                     {
    //                       result = []
    //                     }

    //         exc_list: arg_value
    //                     {
    //                       result = [ val[0] ]
    //                     }
    //                 | mrhs
    //                 | none

    //          exc_var: tASSOC lhs
    //                     {
    //                       result = [ val[0], val[1] ]
    //                     }
    //                 | none

    //       opt_ensure: kENSURE compstmt
    //                     {
    //                       result = [ val[0], val[1] ]
    //                     }
    //                 | none

    //  literal: numeric
    //         | symbol
    //         | dsym
    fn p_literal(&mut self) -> Option<Node> {
        println!("PARSER p_literal");

        if let Some(n_numeric) = self.p_numeric() { return Some(n_numeric); }
        if let Some(n_symbol) = self.p_symbol() { return Some(n_symbol); }
        if let Some(n_dsym) = self.p_dsym() { return Some(n_dsym); }

        None
    }

    //  strings: string
    //             {
    //               result = @builder.string_compose(nil, val[0], nil)
    //             }
    fn p_strings(&mut self) -> Option<Node> {
        println!("PARSER p_strings");

        if let Some(n_string) = self.p_string() {
            return Some(node::string_compose(n_string));
        }

        None
    }

    //   string: string1
    //             {
    //               result = [ val[0] ]
    //             }
    //         | string string1
    //             {
    //               result = val[0] << val[1]
    //             }
    // NOTE transformed into non-recursive form
    fn p_string(&mut self) -> Option<Node> {
        println!("PARSER p_string");

        if let Some(n_string1) = self.p_string1() {
            let mut string1s = vec![n_string1];

            loop {
                if let Some(n_string1) = self.p_string1() {
                    string1s.push(n_string1);
                } else {
                    break;
                }
            }

            return Some(Node::Nodes(string1s));
        }

        None
    }

    // TODO INCOMPLETE
    fn p_string1(&mut self) -> Option<Node> {
        println!("PARSER p_string1");

        //  string1: tSTRING_BEG string_contents tSTRING_END
        //             {
        //               string = @builder.string_compose(val[0], val[1], val[2])
        //               result = @builder.dedent_string(string, @lexer.dedent_level)
        //             }
        if let Some(t_string_beg) = self.match_1_token(Token::T_STRING_BEG) {
            if let Some(n_string_contents) = self.p_string_contents() {
                if let Some(t_string_end) = self.match_1_token(Token::T_STRING_END) {
                    //   string = @builder.string_compose(val[0], val[1], val[2])
                    //   result = @builder.dedent_string(string, @lexer.dedent_level)
                    // TODO DUMMY
                    return Some(node::string_compose(n_string_contents));
                }
            }
        }

        //         | tSTRING
        //             {
        //               string = @builder.string(val[0])
        //               result = @builder.dedent_string(string, @lexer.dedent_level)
        //             }
        if let Some(Token::T_STRING(token_string)) = self.current_token() {
            self.consume_current_token();
            return Some( Node::Str( token_string ) );
        }

        //         | tCHARACTER
        //             {
        //               result = @builder.character(val[0])
        //             }
        // TODO

        None
    }

    //      xstring: tXSTRING_BEG xstring_contents tSTRING_END
    //                 {
    //                   string = @builder.xstring_compose(val[0], val[1], val[2])
    //                   result = @builder.dedent_string(string, @lexer.dedent_level)
    //                 }
    // fn p_xstring(&mut self) -> Option<Node> {
    //     if let Token::T_XSTRING_BEG = self.current_token() {
    //         self.consume_current_token();
    //         panic!("p_xstring UNIMPL");
    //     }
    // 
    //     None
    // }

    //       regexp: tREGEXP_BEG regexp_contents tSTRING_END tREGEXP_OPT
    //                 {
    //                   opts   = @builder.regexp_options(val[3])
    //                   result = @builder.regexp_compose(val[0], val[1], val[2], opts)
    //                 }

    //        words: tWORDS_BEG word_list tSTRING_END
    //                 {
    //                   result = @builder.words_compose(val[0], val[1], val[2])
    //                 }
    fn p_words(&mut self) -> Option<Node> {
        if let Some(t_words_beg) = self.match_1_token(Token::T_WORDS_BEG) {
            // handle word_list being none
            if let Some(t_string_end) = self.match_1_token(Token::T_STRING_END) {
                return Some(Node::Array(vec![]));
            }

            if let Some(n_word_list) = self.p_word_list() {
                if let Some(t_string_end) = self.match_1_token(Token::T_STRING_END) {
                    if let Node::Nodes(nodes) = n_word_list { return Some(Node::Array(nodes)); }
                }
            }
        }

        None
    }

    //    word_list: # nothing
    //                 {
    //                   result = []
    //                 }
    //             | word_list word tSPACE
    //                 {
    //                   result = val[0] << @builder.word(val[1])
    //                 }
    // NOTE transformed into non-recursive form
    fn p_word_list(&mut self) -> Option<Node> {
        if let Some(n_word) = self.p_word() {
            let mut n_words = vec![n_word];
            loop { if let Some(n_word) = self.p_word() { n_words.push(n_word); } else { break; } }
            return Some(Node::Nodes(n_words));
        }

        None
    }

    //         word: string_content
    //                 {
    //                   result = [ val[0] ]
    //                 }
    //             | word string_content
    //                 {
    //                   result = val[0] << val[1]
    //                 }
    // TODO NOTE transformed to non-recursive form
    fn p_word(&mut self) -> Option<Node> {
        if let Some(n_string_content) = self.p_string_content() {
            let mut n_words = vec![n_string_content];
            // TODO properly handle node children
            loop { if let Some(n_string_content) = self.p_string_content() { n_words.push(n_string_content); } else { break; } }
            return Some(Node::Nodes(n_words));
        }

        None
    }

    //      symbols: tSYMBOLS_BEG symbol_list tSTRING_END
    //                 {
    //                   result = @builder.symbols_compose(val[0], val[1], val[2])
    //                 }

    //  symbol_list: # nothing
    //                 {
    //                   result = []
    //                 }
    //             | symbol_list word tSPACE
    //                 {
    //                   result = val[0] << @builder.word(val[1])
    //                 }

    //       qwords: tQWORDS_BEG qword_list tSTRING_END
    //                 {
    //                   result = @builder.words_compose(val[0], val[1], val[2])
    //                 }
    fn p_qwords(&mut self) -> Option<Node> {
        if let Some(_t_qwords_beg) = self.match_1_token(Token::T_QWORDS_BEG) {
            // handle qword_list being `none`
            if let Some(_t_string_end) = self.match_1_token(Token::T_STRING_END) {
                // TODO builder.words_compose
                return Some(Node::Array(vec![]));
            }

            if let Some(qword_list) = self.p_qword_list() {
                if let Some(_t_string_end) = self.match_1_token(Token::T_STRING_END) {
                    // TODO builder.words_compose
                    return Some(Node::Array(extract_nodes(qword_list)));
                }
            }
        }

        None
    }

    //     qsymbols: tQSYMBOLS_BEG qsym_list tSTRING_END
    //                 {
    //                   result = @builder.symbols_compose(val[0], val[1], val[2])
    //                 }

    //   qword_list: # nothing
    //                 {
    //                   result = []
    //                 }
    //             | qword_list tSTRING_CONTENT tSPACE
    //                 {
    //                   result = val[0] << @builder.string_internal(val[1])
    //                 }
    // NOTE transformed into non-recursive form
    fn p_qword_list(&mut self) -> Option<Node> {
        if let Some(Token::T_STRING_CONTENT(str_content)) = self.current_token() {
            self.consume_current_token();
            if let Some(_t_space) = self.match_1_token(Token::T_SPACE) {
                let mut nodes = vec![Node::Str(str_content)];

                loop {
                    if let Some(Token::T_STRING_CONTENT(str_content)) = self.current_token() {
                        self.consume_current_token();
                        if let Some(_t_space) = self.match_1_token(Token::T_SPACE) {
                            nodes.push(Node::Str(str_content));
                            continue;
                        }
                    }
                    break;
                }

                // TODO builder.string_internal
                return Some(Node::Nodes(nodes));
            }
        }

        None
    }

    //    qsym_list: # nothing
    //                 {
    //                   result = []
    //                 }
    //             | qsym_list tSTRING_CONTENT tSPACE
    //                 {
    //                   result = val[0] << @builder.symbol_internal(val[1])
    //                 }

    //  string_contents: # nothing
    //                     {
    //                       result = []
    //                     }
    //                 | string_contents string_content
    //                     {
    //                       result = val[0] << val[1]
    //                     }
    // NOTE transformed to non-recursive
    fn p_string_contents(&mut self) -> Option<Node> {
        println!("PARSER p_string_contents");

        if let Some(n_string_content) = self.p_string_content() {
            let mut string_contents = vec![n_string_content];

            loop {
                if let Some(n_string_content)  = self.p_string_content() {
                    string_contents.push(n_string_content);
                } else {
                    break;
                }
            }

            return Some(Node::Nodes(string_contents));
        }

        None
    }

    // xstring_contents: # nothing
    //                     {
    //                     result = []
    //                     }
    //                 | xstring_contents string_content
    //                     {
    //                     result = val[0] << val[1]
    //                     }
    // TODO INCOMPLETE DUMMY
    fn p_xstring_contents(&mut self) -> Option<Node> {
        println!("PARSER p_xstring_contents");

        // NOTE transformed to non-recursive
        if let Some(n_string_content) = self.p_string_content() {
            // TODO handle list

            return Some(n_string_content);
        }

        None
    }

    // regexp_contents: # nothing
    //                     {
    //                       result = []
    //                     }
    //                 | regexp_contents string_content
    //                     {
    //                       result = val[0] << val[1]
    //                     }

    // TODO INCOMPLETE
    fn p_string_content(&mut self) -> Option<Node> {
        println!("PARSER p_string_content");

        //   string_content: tSTRING_CONTENT
        //                     {
        //                       result = @builder.string_internal(val[0])
        //                     }
        if let Some(Token::T_STRING_CONTENT(t_string_content_value)) = self.current_token() {
            self.consume_current_token();
            return Some(Node::Str(t_string_content_value));
        }

        //                 | tSTRING_DVAR string_dvar
        //                     {
        //                       result = val[1]
        //                     }
        // TODO

        //                 | tSTRING_DBEG
        //                     {
        //                       @lexer.cond.push(false)
        //                       @lexer.cmdarg.push(false)
        //                     }
        //                     compstmt tSTRING_DEND
        //                     {
        //                       @lexer.cond.lexpop
        //                       @lexer.cmdarg.lexpop
        // 
        //                       result = @builder.begin(val[0], val[2], val[3])
        //                     }
        // TODO NOTE embedded action
        if let Some(t_string_dbeg) = self.match_1_token(Token::T_STRING_DBEG) {
            self.lexer.cond.push(false);
            self.lexer.cmdarg.push(false);
            if let Some(n_compstmt) = self.p_compstmt() {
                if let Some(t_string_dend) = self.match_1_token(Token::T_STRING_DEND) {
                    self.lexer.cond.lexpop();
                    self.lexer.cmdarg.lexpop();

                    // TODO builder.begin
                    panic!("WIP");
                }
            }
        }

        None
    }

    //  string_dvar: tGVAR
    //                 {
    //                   result = @builder.gvar(val[0])
    //                 }
    //             | tIVAR
    //                 {
    //                   result = @builder.ivar(val[0])
    //                 }
    //             | tCVAR
    //                 {
    //                   result = @builder.cvar(val[0])
    //                 }
    //             | backref

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
        if let Some(Token::T_UNARY_NUM(_)) = self.current_token() {
            let t_unary_num = self.consume_current_token();
            if let Some(n_simple_numeric) = self.p_simple_numeric() {
                return Some(node::unary_num(t_unary_num, n_simple_numeric));
            }
        }

        None
    }

    //   symbol: tSYMBOL
    //             {
    //               @lexer.state = :expr_endarg
    //               result = @builder.symbol(val[0])
    //             }
    fn p_symbol(&mut self) -> Option<Node> {
        println!("PARSER p_symbol");

        if let Some(Token::T_SYMBOL(symbol_string)) = self.current_token() {
            let _t_symbol = self.consume_current_token();

            self.lexer.set_state(state!("expr_endarg"));

            return Some(Node::Sym(symbol_string));
        }

        None
    }

    // dsym: tSYMBEG xstring_contents tSTRING_END
    //         {
    //           @lexer.state = :expr_endarg
    //           result = @builder.symbol_compose(val[0], val[1], val[2])
    //         }
    fn p_dsym(&mut self) -> Option<Node> {
        println!("PARSER p_dsym");
        if let Some(t_symbeg) = self.match_1_token(Token::T_SYMBEG) {
            if let Some(n_xstring_contents) = self.p_xstring_contents() {
                if let Some(t_string_end) = self.match_1_token(Token::T_STRING_END) {
                    self.lexer.set_state(state!("expr_endarg"));
                    // TODO DUMMY
                    // return Some(node::symbol_compose(t_symbeg, n_xstring_contents, t_string_end));
                    if let Node::Str(str_value) = n_xstring_contents { return Some(Node::Sym(str_value)); }
                }
            }
        }

        None
    }


    //  numeric: simple_numeric
    //             {
    //               result = val[0]
    //             }
    //         | tUNARY_NUM simple_numeric =tLOWEST
    //             {
    //               if @builder.respond_to? :negate
    //                 # AST builder interface compatibility
    //                 result = @builder.negate(val[0], val[1])
    //               else
    //                 result = @builder.unary_num(val[0], val[1])
    //               end
    //             }


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
            Some(Token::T_INTEGER(i)) => {
                self.lexer.set_state(state!("expr_end"));

                self.consume_current_token();

                return Some(Node::Int(i));
            },
            _ => { return None; }
        }
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

        if let Some(Token::T_IDENTIFIER(t_id_value)) = current_token {
            self.consume_current_token();
            return Some(Node::Ident(t_id_value));
        }

        None
    }

    // TODO INCOMPLETE
    fn p_keyword_variable(&mut self) -> Option<Node> {
        println!("p_keyword_variable");

        // keyword_variable: kNIL
        //                     {
        //                       result = @builder.nil(val[0])
        //                     }
        if let Some(_) = self.match_1_token(Token::K_NIL) { return Some(Node::Nil); }

        //                 | kSELF
        //                     {
        //                       result = @builder.self(val[0])
        //                     }
        if let Some(_) = self.match_1_token(Token::K_SELF) { return Some(Node::NSelf); }

        //                 | kTRUE
        //                     {
        //                       result = @builder.true(val[0])
        //                     }
        if let Some(_) = self.match_1_token(Token::K_TRUE) { return Some(Node::True); }

        //                 | kFALSE
        //                     {
        //                       result = @builder.false(val[0])
        //                     }
        if let Some(_) = self.match_1_token(Token::K_FALSE) { return Some(Node::False); }

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

    //          var_lhs: user_variable
    //                     {
    //                       result = @builder.assignable(val[0])
    //                     }
    //                 | keyword_variable
    //                     {
    //                       result = @builder.assignable(val[0])
    //                     }

    //          backref: tNTH_REF
    //                     {
    //                       result = @builder.nth_ref(val[0])
    //                     }
    //                 | tBACK_REF
    //                     {
    //                       result = @builder.back_ref(val[0])
    //                     }

    //       superclass: tLT
    //                     {
    //                       @lexer.state = :expr_value
    //                     }
    //                     expr_value term
    //                     {
    //                       result = [ val[0], val[2] ]
    //                     }
    //                 | # nothing
    //                     {
    //                       result = nil
    //                     }

    //        f_arglist: tLPAREN2 f_args rparen
    //                     {
    //                       result = @builder.args(val[0], val[1], val[2])

    //                       @lexer.state = :expr_value
    //                     }
    //                 |   {
    //                       result = @lexer.in_kwarg
    //                       @lexer.in_kwarg = true
    //                     }
    //                   f_args term
    //                     {
    //                       @lexer.in_kwarg = val[0]
    //                       result = @builder.args(nil, val[1], nil)
    //                     }

    //        args_tail: f_kwarg tCOMMA f_kwrest opt_f_block_arg
    //                     {
    //                       result = val[0].concat(val[2]).concat(val[3])
    //                     }
    //                 | f_kwarg opt_f_block_arg
    //                     {
    //                       result = val[0].concat(val[1])
    //                     }
    //                 | f_kwrest opt_f_block_arg
    //                     {
    //                       result = val[0].concat(val[1])
    //                     }
    //                 | f_block_arg
    //                     {
    //                       result = [ val[0] ]
    //                     }

    //    opt_args_tail: tCOMMA args_tail
    //                     {
    //                       result = val[1]
    //                     }
    //                 | # nothing
    //                     {
    //                       result = []
    //                     }

    //           f_args: f_arg tCOMMA f_optarg tCOMMA f_rest_arg              opt_args_tail
    //                     {
    //                       result = val[0].
    //                                   concat(val[2]).
    //                                   concat(val[4]).
    //                                   concat(val[5])
    //                     }
    //                 | f_arg tCOMMA f_optarg tCOMMA f_rest_arg tCOMMA f_arg opt_args_tail
    //                     {
    //                       result = val[0].
    //                                   concat(val[2]).
    //                                   concat(val[4]).
    //                                   concat(val[6]).
    //                                   concat(val[7])
    //                     }
    //                 | f_arg tCOMMA f_optarg                                opt_args_tail
    //                     {
    //                       result = val[0].
    //                                   concat(val[2]).
    //                                   concat(val[3])
    //                     }
    //                 | f_arg tCOMMA f_optarg tCOMMA                   f_arg opt_args_tail
    //                     {
    //                       result = val[0].
    //                                   concat(val[2]).
    //                                   concat(val[4]).
    //                                   concat(val[5])
    //                     }
    //                 | f_arg tCOMMA                 f_rest_arg              opt_args_tail
    //                     {
    //                       result = val[0].
    //                                   concat(val[2]).
    //                                   concat(val[3])
    //                     }
    //                 | f_arg tCOMMA                 f_rest_arg tCOMMA f_arg opt_args_tail
    //                     {
    //                       result = val[0].
    //                                   concat(val[2]).
    //                                   concat(val[4]).
    //                                   concat(val[5])
    //                     }
    //                 | f_arg                                                opt_args_tail
    //                     {
    //                       result = val[0].
    //                                   concat(val[1])
    //                     }
    //                 |              f_optarg tCOMMA f_rest_arg              opt_args_tail
    //                     {
    //                       result = val[0].
    //                                   concat(val[2]).
    //                                   concat(val[3])
    //                     }
    //                 |              f_optarg tCOMMA f_rest_arg tCOMMA f_arg opt_args_tail
    //                     {
    //                       result = val[0].
    //                                   concat(val[2]).
    //                                   concat(val[4]).
    //                                   concat(val[5])
    //                     }
    //                 |              f_optarg                                opt_args_tail
    //                     {
    //                       result = val[0].
    //                                   concat(val[1])
    //                     }
    //                 |              f_optarg tCOMMA                   f_arg opt_args_tail
    //                     {
    //                       result = val[0].
    //                                   concat(val[2]).
    //                                   concat(val[3])
    //                     }
    //                 |                              f_rest_arg              opt_args_tail
    //                     {
    //                       result = val[0].
    //                                   concat(val[1])
    //                     }
    //                 |                              f_rest_arg tCOMMA f_arg opt_args_tail
    //                     {
    //                       result = val[0].
    //                                   concat(val[2]).
    //                                   concat(val[3])
    //                     }
    //                 |                                                          args_tail
    //                     {
    //                       result = val[0]
    //                     }
    //                 | # nothing
    //                     {
    //                       result = []
    //                     }

    //        f_bad_arg: tCONSTANT
    //                     {
    //                       diagnostic :error, :argument_const, nil, val[0]
    //                     }
    //                 | tIVAR
    //                     {
    //                       diagnostic :error, :argument_ivar, nil, val[0]
    //                     }
    //                 | tGVAR
    //                     {
    //                       diagnostic :error, :argument_gvar, nil, val[0]
    //                     }
    //                 | tCVAR
    //                     {
    //                       diagnostic :error, :argument_cvar, nil, val[0]
    //                     }

    //       f_norm_arg: f_bad_arg
    //                 | tIDENTIFIER
    //                     {
    //                       @static_env.declare val[0][0]

    //                       result = val[0]
    //                     }

    //       f_arg_asgn: f_norm_arg
    //                     {
    //                       result = val[0]
    //                     }

    //       f_arg_item: f_arg_asgn
    //                     {
    //                       result = @builder.arg(val[0])
    //                     }
    //                 | tLPAREN f_margs rparen
    //                     {
    //                       result = @builder.multi_lhs(val[0], val[1], val[2])
    //                     }

    //            f_arg: f_arg_item
    //                     {
    //                       result = [ val[0] ]
    //                     }
    //                 | f_arg tCOMMA f_arg_item
    //                     {
    //                       result = val[0] << val[2]
    //                     }

    //          f_label: tLABEL
    //                     {
    //                       check_kwarg_name(val[0])

    //                       @static_env.declare val[0][0]

    //                       result = val[0]
    //                     }

    //             f_kw: f_label arg_value
    //                     {
    //                       result = @builder.kwoptarg(val[0], val[1])
    //                     }
    //                 | f_label
    //                     {
    //                       result = @builder.kwarg(val[0])
    //                     }

    //       f_block_kw: f_label primary_value
    //                     {
    //                       result = @builder.kwoptarg(val[0], val[1])
    //                     }
    //                 | f_label
    //                     {
    //                       result = @builder.kwarg(val[0])
    //                     }

    //    f_block_kwarg: f_block_kw
    //                     {
    //                       result = [ val[0] ]
    //                     }
    //                 | f_block_kwarg tCOMMA f_block_kw
    //                     {
    //                       result = val[0] << val[2]
    //                     }

    //          f_kwarg: f_kw
    //                     {
    //                       result = [ val[0] ]
    //                     }
    //                 | f_kwarg tCOMMA f_kw
    //                     {
    //                       result = val[0] << val[2]
    //                     }

    //      kwrest_mark: tPOW | tDSTAR

    //         f_kwrest: kwrest_mark tIDENTIFIER
    //                     {
    //                       @static_env.declare val[1][0]

    //                       result = [ @builder.kwrestarg(val[0], val[1]) ]
    //                     }
    //                 | kwrest_mark
    //                     {
    //                       result = [ @builder.kwrestarg(val[0]) ]
    //                     }

    //            f_opt: f_arg_asgn tEQL arg_value
    //                     {
    //                       result = @builder.optarg(val[0], val[1], val[2])
    //                     }

    //      f_block_opt: f_arg_asgn tEQL primary_value
    //                     {
    //                       result = @builder.optarg(val[0], val[1], val[2])
    //                     }

    //   f_block_optarg: f_block_opt
    //                     {
    //                       result = [ val[0] ]
    //                     }
    //                 | f_block_optarg tCOMMA f_block_opt
    //                     {
    //                       result = val[0] << val[2]
    //                     }

    //         f_optarg: f_opt
    //                     {
    //                       result = [ val[0] ]
    //                     }
    //                 | f_optarg tCOMMA f_opt
    //                     {
    //                       result = val[0] << val[2]
    //                     }

    //     restarg_mark: tSTAR2 | tSTAR

    //       f_rest_arg: restarg_mark tIDENTIFIER
    //                     {
    //                       @static_env.declare val[1][0]

    //                       result = [ @builder.restarg(val[0], val[1]) ]
    //                     }
    //                 | restarg_mark
    //                     {
    //                       result = [ @builder.restarg(val[0]) ]
    //                     }

    //      blkarg_mark: tAMPER2 | tAMPER

    //      f_block_arg: blkarg_mark tIDENTIFIER
    //                     {
    //                       @static_env.declare val[1][0]

    //                       result = @builder.blockarg(val[0], val[1])
    //                     }

    //  opt_f_block_arg: tCOMMA f_block_arg
    //                     {
    //                       result = [ val[1] ]
    //                     }
    //                 |
    //                     {
    //                       result = []
    //                     }

    //        singleton: var_ref
    //                 | tLPAREN2 expr rparen
    //                     {
    //                       result = val[1]
    //                     }

    //       assoc_list: # nothing
    //                     {
    //                       result = []
    //                     }
    //                 | assocs trailer

    //           assocs: assoc
    //                     {
    //                       result = [ val[0] ]
    //                     }
    //                 | assocs tCOMMA assoc
    //                     {
    //                       result = val[0] << val[2]
    //                     }
    // NOTE transformed into non-recursive form
    fn p_assocs(&mut self) -> Option<Node> {
        println!("p_assocs");
        panic!("p_assocs UNIMPL");

        // if let Some(n_assoc) = self.p_assoc() {
        // }

        None
    }

    //            assoc: arg_value tASSOC arg_value
    //                     {
    //                       result = @builder.pair(val[0], val[1], val[2])
    //                     }
    //                 | tLABEL arg_value
    //                     {
    //                       result = @builder.pair_keyword(val[0], val[1])
    //                     }
    //                 | tSTRING_BEG string_contents tLABEL_END arg_value
    //                     {
    //                       result = @builder.pair_quoted(val[0], val[1], val[2], val[3])
    //                     }
    //                 | tDSTAR arg_value
    //                     {
    //                       result = @builder.kwsplat(val[0], val[1])
    //                     }
    // TODO INCOMPLETE
    fn p_assoc(&mut self) -> Option<Node> {
        println!("p_assoc");
        panic!("p_assoc UNIMPL");
        None
    }

    //        operation: tIDENTIFIER | tCONSTANT | tFID
    //       operation2: tIDENTIFIER | tCONSTANT | tFID | op
    //       operation3: tIDENTIFIER | tFID | op
    //     dot_or_colon: call_op | tCOLON2
    //          call_op: tDOT
    //                     {
    //                       result = [:dot, val[0][1]]
    //                     }
    //                 | tANDDOT
    //                     {
    //                       result = [:anddot, val[0][1]]
    //                     }
    //        opt_terms:  | terms
    //           opt_nl:  | tNL
    //           rparen: opt_nl tRPAREN
    //                     {
    //                       result = val[1]
    //                     }
    //         rbracket: opt_nl tRBRACK
    //                     {
    //                       result = val[1]
    //                     }
    // 
    //          trailer:  | tNL | tCOMMA
    // TODO handle option none
    fn p_trailer(&mut self) -> Option<Node> {
        if let Some(t_nl) = self.match_1_token(Token::T_NL) { return Some(Node::Dummy); }
        if let Some(t_tomma) = self.match_1_token(Token::T_COMMA) { return Some(Node::Dummy); }
        None
    }

    //             term: tSEMI
    //                   {
    //                     yyerrok
    //                   }
    //                 | tNL

    //            terms: term
    //                 | terms tSEMI
}
