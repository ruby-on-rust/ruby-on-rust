// note about extracting values(token/node) in production
// 
// example 1
// 
// simple_numeric
//     :
//     tINTEGER {
//         || -> Node;
// 
//         let $$;
//         if let SV::_0(token) = $1 {
//             if let box InteriorToken::T_INTEGER(value) = token.interior_token {
//                 <REMOVE THIS LET>$$ = Node::Int(value);
//             } else { unreachable!(); }
//         } else { unreachable!(); }
//     }
// ;
// 
// `|| -> Node` means `$1` is unwrapped, so have to do the matching manually
// 
// TODO and why we don't want to unwrap it?
// 
// TODO <REMOVE THIS LET> is another issue here
// 
// 
// example 2
// 
// var_ref
//     : keyword_variable {
//         |$1:Node| -> Node;
// 
//         $$ = node::accessible($1);
//     }
// ;
// 
// `|$1:Node|` means a `pop` and an `unwrap`, so `$1` is already a `Node` unwrapped from `SV`
// 
// TODO make macros
// 

// TODO update
// this file is based on https://github.com/whitequark/parser/blob/2a73841d6da04a5ab9bd270561165fd766722d43/lib/parser/ruby25.y

// TODO
// check out this about transforming token names https://github.com/tenderlove/racc/blob/master/rdoc/en/grammar.en.rdoc#converting-token-symbols

// TODO fake embedded actions
// https://github.com/DmitrySoshnikov/syntax/issues/65

// TODO `error` in yacc/bison/racc is `a terminal symbol reserved for error recovery`, see http://dinosaur.compilertools.net/bison/bison_9.html#SEC81
// figure out what's the corresponsing word in syntax

%right    tBANG tTILDE tUPLUS
%right    tPOW
%right    tUNARY_NUM tUMINUS
%left     tSTAR2 tDIVIDE tPERCENT
%left     tPLUS tMINUS
%left     tLSHFT tRSHFT
%left     tAMPER2
%left     tPIPE tCARET
%left     tGT tGEQ tLT tLEQ
%nonassoc tCMP tEQ tEQQ tNEQ tMATCH tNMATCH
%left     tANDOP
%left     tOROP
%nonassoc tDOT2 tDOT3
%right    tEH tCOLON
%left     kRESCUE_MOD
%right    tEQL tOP_ASGN
%nonassoc kDEFINED
%right    kNOT
%left     kOR kAND
%nonassoc kIF_MOD kUNLESS_MOD kWHILE_MOD kUNTIL_MOD
%nonassoc tLBRACE_ARG
%nonassoc tLOWEST

%{

use token::token::Token as InteriorToken;
use parser::token::Token;
use parser::tokenizer::Tokenizer;
use ast::node;
use ast::node::{ Node, Nodes };

pub type TResult = Node;

macro_rules! wip { () => { panic!("WIP"); }; }

%}

%%

program: top_compstmt;

top_compstmt
    : top_stmts opt_terms {
        |$1: Nodes| -> Node;

        $$ = node::compstmt($1);
    }
;

top_stmts
    // nothing
    : {
        || -> Nodes; $$ = vec![];
    }
    | top_stmt {
        |$1: Node| -> Nodes; $$ = vec![$1];
    }
    | top_stmts terms top_stmt {
        |$1: Nodes; $3: Node| -> Nodes;

        $1.push($3);
        $$ = $1;
    }
    // | error top_stmt {
    //     |$2:Node| -> Nodes; $$ = vec![$2];
    // }
;

top_stmt
    : stmt
    | klBEGIN tLCURLY top_compstmt tRCURLY {
        // result = @builder.preexe(val[0], val[1], val[2], val[3])
        |$3:Node| -> Node;
        $$ = node::preexe($3);
    }
;

bodystmt
    : compstmt opt_rescue opt_else opt_ensure {
        || -> Node;
        //                       rescue_bodies     = val[1]
        //                       else_t,   else_   = val[2]
        //                       ensure_t, ensure_ = val[3]
        // 
        //                       if rescue_bodies.empty? && !else_.nil?
        //                         diagnostic :warning, :useless_else, nil, else_t
        //                       end
        // 
        //                       result = @builder.begin_body(val[0],
        //                                   rescue_bodies,
        //                                   else_t,   else_,
        //                                   ensure_t, ensure_)
        $$ = Node::DUMMY;
    }
;

compstmt: stmts opt_terms {
    // result = @builder.compstmt(val[0])
    |$1:Nodes| -> Node;
    $$ = node::compstmt($1);
};

stmts
    : {
        || -> Nodes; $$ = vec![];
    }
    | stmt_or_begin {
        |$1:Node| -> Nodes; $$ = vec![$1];
    }
    | stmts terms stmt_or_begin {
        // result = val[0] << val[2]
        wip!();
    }
    // | error stmt {
    //     // result = [ val[1] ]
    //     wip!();
    // }
;

stmt_or_begin
    : stmt
    | klBEGIN tLCURLY top_compstmt tRCURLY {
        // diagnostic :error, :begin_in_method, nil, val[0]
        wip!();
    }
;

fake_embedded_action__stmt__1: {
    // @lexer.state = :expr_fname
    wip!();
};

stmt
    : kALIAS fitem fake_embedded_action__stmt__1 fitem {
        // result = @builder.alias(val[0], val[1], val[3])
        wip!();
    }
    | kALIAS tGVAR tGVAR {
        // result = @builder.alias(val[0],
        //             @builder.gvar(val[1]),
        //             @builder.gvar(val[2]))
        wip!();
    }
    | kALIAS tGVAR tBACK_REF {
        // result = @builder.alias(val[0],
        //             @builder.gvar(val[1]),
        //             @builder.back_ref(val[2]))
        wip!();
    }
    | kALIAS tGVAR tNTH_REF {
        // diagnostic :error, :nth_ref_alias, nil, val[2]
        wip!();
    }
    | kUNDEF undef_list {
        // result = @builder.undef_method(val[0], val[1])
        wip!();
    }
    | stmt kIF_MOD expr_value {
        // result = @builder.condition_mod(val[0], nil,
        //                                 val[1], val[2])
        wip!();
    }
    | stmt kUNLESS_MOD expr_value {
        // result = @builder.condition_mod(nil, val[0],
        //                                 val[1], val[2])
        wip!();
    }
    | stmt kWHILE_MOD expr_value {
        // result = @builder.loop_mod(:while, val[0], val[1], val[2])
        wip!();
    }
    | stmt kUNTIL_MOD expr_value {
        // result = @builder.loop_mod(:until, val[0], val[1], val[2])
        wip!();
    }
    | stmt kRESCUE_MOD stmt {
        // rescue_body = @builder.rescue_body(val[1],
        //                 nil, nil, nil,
        //                 nil, val[2])

        // result = @builder.begin_body(val[0], [ rescue_body ])
        wip!();
    }
    | klEND tLCURLY compstmt tRCURLY {
        // result = @builder.postexe(val[0], val[1], val[2], val[3])
        wip!();
    }
    | command_asgn
    | mlhs tEQL command_call {
        // result = @builder.multi_assign(val[0], val[1], val[2])
        wip!();
    }
    | lhs tEQL mrhs {
        // result = @builder.assign(val[0], val[1],
        //             @builder.array(nil, val[2], nil))
        wip!();
    }
    | mlhs tEQL mrhs_arg {
        // result = @builder.multi_assign(val[0], val[1], val[2])
        wip!();
    }
    | expr
;

command_asgn: lhs tEQL command_rhs
                {
                    // result = @builder.assign(val[0], val[1], val[2])
                    wip!();
                }
            | var_lhs tOP_ASGN command_rhs
                {
                    // result = @builder.op_assign(val[0], val[1], val[2])
                    wip!();
                }
            | primary_value tLBRACK2 opt_call_args rbracket tOP_ASGN command_rhs
                {
                    // result = @builder.op_assign(
                    //             @builder.index(
                    //             val[0], val[1], val[2], val[3]),
                    //             val[4], val[5])
                    wip!();
                }
            | primary_value call_op tIDENTIFIER tOP_ASGN command_rhs
                {
                    // result = @builder.op_assign(
                    //             @builder.call_method(
                    //             val[0], val[1], val[2]),
                    //             val[3], val[4])
                    wip!();
                }
            | primary_value call_op tCONSTANT tOP_ASGN command_rhs
                {
                    // result = @builder.op_assign(
                    //             @builder.call_method(
                    //             val[0], val[1], val[2]),
                    //             val[3], val[4])
                    wip!();
                }
            | primary_value tCOLON2 tCONSTANT tOP_ASGN command_rhs
                {
                    // const  = @builder.const_op_assignable(
                    //             @builder.const_fetch(val[0], val[1], val[2]))
                    // result = @builder.op_assign(const, val[3], val[4])
                    wip!();
                }
            | primary_value tCOLON2 tIDENTIFIER tOP_ASGN command_rhs
                {
                    // result = @builder.op_assign(
                    //             @builder.call_method(
                    //             val[0], val[1], val[2]),
                    //             val[3], val[4])
                    wip!();
                }
            | backref tOP_ASGN command_rhs
                {
                    // @builder.op_assign(val[0], val[1], val[2])
                    wip!();
                }
;

     command_rhs: command_call %prec tOP_ASGN
                | command_call kRESCUE_MOD stmt
                    {
                    //   rescue_body = @builder.rescue_body(val[1],
                    //                     nil, nil, nil,
                    //                     nil, val[2])

                    //   result = @builder.begin_body(val[0], [ rescue_body ])
                        wip!();
                    }
                | command_asgn
;

            expr: command_call
                | expr kAND expr
                    {
                    //   result = @builder.logical_op(:and, val[0], val[1], val[2])
                    wip!();
                    }
                | expr kOR expr
                    {
                    //   result = @builder.logical_op(:or, val[0], val[1], val[2])
                    wip!();
                    }
                | kNOT opt_nl expr
                    {
                    //   result = @builder.not_op(val[0], nil, val[2], nil)
                    wip!();
                    }
                | tBANG command_call
                    {
                    //   result = @builder.not_op(val[0], nil, val[1], nil)
                    wip!();
                    }
                | arg
;

expr_value: expr;

command_call
    : command
    | block_command
;

block_command
    : block_call
    | block_call dot_or_colon operation2 command_args {
        // result = @builder.call_method(val[0], val[1], val[2],
        //             nil, val[3], nil)
        wip!();
    }
;

 cmd_brace_block: tLBRACE_ARG brace_body tRCURLY
                    {
                    //   result = [ val[0], *val[1], val[2] ]
                    wip!();
                    }
;

fcall: operation;

         command: fcall command_args %prec tLOWEST
                    {
                    //   result = @builder.call_method(nil, nil, val[0],
                    //               nil, val[1], nil)
                    wip!();
                    }
                | fcall command_args cmd_brace_block
                    {
                    //   method_call = @builder.call_method(nil, nil, val[0],
                    //                     nil, val[1], nil)

                    //   begin_t, args, body, end_t = val[2]
                    //   result      = @builder.block(method_call,
                    //                   begin_t, args, body, end_t)
                    wip!();
                    }
                | primary_value call_op operation2 command_args %prec tLOWEST
                    {
                    //   result = @builder.call_method(val[0], val[1], val[2],
                    //               nil, val[3], nil)
                    wip!();
                    }
                | primary_value call_op operation2 command_args cmd_brace_block
                    {
                    //   method_call = @builder.call_method(val[0], val[1], val[2],
                    //                     nil, val[3], nil)

                    //   begin_t, args, body, end_t = val[4]
                    //   result      = @builder.block(method_call,
                    //                   begin_t, args, body, end_t)
                    wip!();
                    }
                | primary_value tCOLON2 operation2 command_args %prec tLOWEST
                    {
                    //   result = @builder.call_method(val[0], val[1], val[2],
                    //               nil, val[3], nil)
                    wip!();
                    }
                | primary_value tCOLON2 operation2 command_args cmd_brace_block
                    {
                    //   method_call = @builder.call_method(val[0], val[1], val[2],
                    //                     nil, val[3], nil)

                    //   begin_t, args, body, end_t = val[4]
                    //   result      = @builder.block(method_call,
                    //                   begin_t, args, body, end_t)
                    wip!();
                    }
                | kSUPER command_args
                    {
                    //   result = @builder.keyword_cmd(:super, val[0],
                    //               nil, val[1], nil)
                    wip!();
                    }
                | kYIELD command_args
                    {
                    //   result = @builder.keyword_cmd(:yield, val[0],
                    //               nil, val[1], nil)
                    wip!();
                    }
                | kRETURN call_args
                    {
                    //   result = @builder.keyword_cmd(:return, val[0],
                    //               nil, val[1], nil)
                    wip!();
                    }
                | kBREAK call_args
                    {
                    //   result = @builder.keyword_cmd(:break, val[0],
                    //               nil, val[1], nil)
                    wip!();
                    }
                | kNEXT call_args
                    {
                    //   result = @builder.keyword_cmd(:next, val[0],
                    //               nil, val[1], nil)
                    wip!();
                    }
;

            mlhs: mlhs_basic
                    {
                    //   result = @builder.multi_lhs(nil, val[0], nil)
                    wip!();
                    }
                | tLPAREN mlhs_inner rparen
                    {
                    //   result = @builder.begin(val[0], val[1], val[2])
                    wip!();
                    }
;

      mlhs_inner: mlhs_basic
                    {
                    //   result = @builder.multi_lhs(nil, val[0], nil)
                    wip!();
                    }
                | tLPAREN mlhs_inner rparen
                    {
                    //   result = @builder.multi_lhs(val[0], val[1], val[2])
                    wip!();
                    }
;

      mlhs_basic: mlhs_head
                | mlhs_head mlhs_item
                    {
                    //   result = val[0].
                    //               push(val[1])
                    wip!();
                    }
                | mlhs_head tSTAR mlhs_node
                    {
                    //   result = val[0].
                    //               push(@builder.splat(val[1], val[2]))
                    wip!();
                    }
                | mlhs_head tSTAR mlhs_node tCOMMA mlhs_post
                    {
                    //   result = val[0].
                    //               push(@builder.splat(val[1], val[2])).
                    //               concat(val[4])
                    wip!();
                    }
                | mlhs_head tSTAR
                    {
                    //   result = val[0].
                    //               push(@builder.splat(val[1]))
                    wip!();
                    }
                | mlhs_head tSTAR tCOMMA mlhs_post
                    {
                    //   result = val[0].
                    //               push(@builder.splat(val[1])).
                    //               concat(val[3])
                    wip!();
                    }
                | tSTAR mlhs_node
                    {
                    //   result = [ @builder.splat(val[0], val[1]) ]
                    wip!();
                    }
                | tSTAR mlhs_node tCOMMA mlhs_post
                    {
                    //   result = [ @builder.splat(val[0], val[1]),
                    //              *val[3] ]
                    wip!();
                    }
                | tSTAR
                    {
                    //   result = [ @builder.splat(val[0]) ]
                    wip!();
                    }
                | tSTAR tCOMMA mlhs_post
                    {
                    //   result = [ @builder.splat(val[0]),
                    //              *val[2] ]
                    wip!();
                    }
;

       mlhs_item: mlhs_node
                | tLPAREN mlhs_inner rparen
                    {
                    //   result = @builder.begin(val[0], val[1], val[2])
                    wip!();
                    }
;

       mlhs_head: mlhs_item tCOMMA
                    {
                    //   result = [ val[0] ]
                    wip!();
                    }
                | mlhs_head mlhs_item tCOMMA
                    {
                    //   result = val[0] << val[1]
                    wip!();
                    }
;

       mlhs_post: mlhs_item
                    {
                    //   result = [ val[0] ]
                    wip!();
                    }
                | mlhs_post tCOMMA mlhs_item
                    {
                    //   result = val[0] << val[2]
                    wip!();
                    }
;

       mlhs_node: user_variable
                    {
                    //   result = @builder.assignable(val[0])
                    wip!();
                    }
                | keyword_variable
                    {
                    //   result = @builder.assignable(val[0])
                    wip!();
                    }
                | primary_value tLBRACK2 opt_call_args rbracket
                    {
                    //   result = @builder.index_asgn(val[0], val[1], val[2], val[3])
                    wip!();
                    }
                | primary_value call_op tIDENTIFIER
                    {
                    //   result = @builder.attr_asgn(val[0], val[1], val[2])
                    wip!();
                    }
                | primary_value tCOLON2 tIDENTIFIER
                    {
                    //   result = @builder.attr_asgn(val[0], val[1], val[2])
                    wip!();
                    }
                | primary_value call_op tCONSTANT
                    {
                    //   result = @builder.attr_asgn(val[0], val[1], val[2])
                    wip!();
                    }
                | primary_value tCOLON2 tCONSTANT
                    {
                    //   result = @builder.assignable(
                    //               @builder.const_fetch(val[0], val[1], val[2]))
                    wip!();
                    }
                | tCOLON3 tCONSTANT
                    {
                    //   result = @builder.assignable(
                    //               @builder.const_global(val[0], val[1]))
                    wip!();
                    }
                | backref
                    {
                    //   result = @builder.assignable(val[0])
                    wip!();
                    }
;

lhs
    : user_variable {
        |$1:Node| -> Node;
        $$ = node::assignable($1);
    }
    | keyword_variable
        {
            // result = @builder.assignable(val[0])
            wip!();
        }
    | primary_value tLBRACK2 opt_call_args rbracket
        {
            // result = @builder.index_asgn(val[0], val[1], val[2], val[3])
            wip!();
        }
    | primary_value call_op tIDENTIFIER
        {
            // result = @builder.attr_asgn(val[0], val[1], val[2])
            wip!();
        }
    | primary_value tCOLON2 tIDENTIFIER
        {
            // result = @builder.attr_asgn(val[0], val[1], val[2])
            wip!();
        }
    | primary_value call_op tCONSTANT
        {
            // result = @builder.attr_asgn(val[0], val[1], val[2])
            wip!();
        }
    | primary_value tCOLON2 tCONSTANT
        {
            // result = @builder.assignable(
            //             @builder.const_fetch(val[0], val[1], val[2]))
            wip!();
        }
    | tCOLON3 tCONSTANT
        {
            // result = @builder.assignable(
            //             @builder.const_global(val[0], val[1]))
            wip!();
        }
    | backref
        {
            // result = @builder.assignable(val[0])
            wip!();
        }
;

           cname: tIDENTIFIER
                    {
                    //   diagnostic :error, :module_name_const, nil, val[0]
                    wip!();
                    }
                | tCONSTANT
;

           cpath: tCOLON3 cname
                    {
                    //   result = @builder.const_global(val[0], val[1])
                    wip!();
                    }
                | cname
                    {
                    //   result = @builder.const(val[0])
                    wip!();
                    }
                | primary_value tCOLON2 cname
                    {
                    //   result = @builder.const_fetch(val[0], val[1], val[2])
                    wip!();
                    }
;

           fname: tIDENTIFIER | tCONSTANT | tFID
                | op
                | reswords
;

            fsym: fname
                    {
                    //   result = @builder.symbol(val[0])
                    wip!();
                    }
                | symbol
;

           fitem: fsym
                | dsym
;

      undef_list: fitem
                    {
                    //   result = [ val[0] ]
                    wip!();
                    }
                | undef_list tCOMMA fake_embedded_action_undef_list fitem
                    {
                    //   result = val[0] << val[3]
                    wip!();
                    }
;

fake_embedded_action_undef_list :{
    //   @lexer.state = :expr_fname
    wip!();
};

              op:   tPIPE    | tCARET  | tAMPER2  | tCMP  | tEQ     | tEQQ
                |   tMATCH   | tNMATCH | tGT      | tGEQ  | tLT     | tLEQ
                |   tNEQ     | tLSHFT  | tRSHFT   | tPLUS | tMINUS  | tSTAR2
                |   tSTAR    | tDIVIDE | tPERCENT | tPOW  | tBANG   | tTILDE
                |   tUPLUS   | tUMINUS | tAREF    | tASET | tDSTAR  | tBACK_REF2
;

        reswords: k__LINE__ | k__FILE__ | k__ENCODING__ | klBEGIN | klEND
                | kALIAS    | kAND      | kBEGIN        | kBREAK  | kCASE
                | kCLASS    | kDEF      | kDEFINED      | kDO     | kELSE
                | kELSIF    | kEND      | kENSURE       | kFALSE  | kFOR
                | kIN       | kMODULE   | kNEXT         | kNIL    | kNOT
                | kOR       | kREDO     | kRESCUE       | kRETRY  | kRETURN
                | kSELF     | kSUPER    | kTHEN         | kTRUE   | kUNDEF
                | kWHEN     | kYIELD    | kIF           | kUNLESS | kWHILE
                | kUNTIL
;

//              arg: lhs tEQL arg_rhs
//                     {
//                       result = @builder.assign(val[0], val[1], val[2])
//                     }
arg
    : lhs tEQL arg_rhs {
        |$1: Node; $2: Token, $3: Node| -> Node;

        $$ = node::assign($1, *$2.interior_token, $3)
    }
    | var_lhs tOP_ASGN arg_rhs
        {
            // result = @builder.op_assign(val[0], val[1], val[2])
                    wip!();
        }
    | primary_value tLBRACK2 opt_call_args rbracket tOP_ASGN arg_rhs
        {
            // result = @builder.op_assign(
            //             @builder.index(
            //             val[0], val[1], val[2], val[3]),
            //             val[4], val[5])
                    wip!();
        }
    | primary_value call_op tIDENTIFIER tOP_ASGN arg_rhs
        {
            // result = @builder.op_assign(
            //             @builder.call_method(
            //             val[0], val[1], val[2]),
            //             val[3], val[4])
                    wip!();
        }
    | primary_value call_op tCONSTANT tOP_ASGN arg_rhs
        {
            // result = @builder.op_assign(
            //             @builder.call_method(
            //             val[0], val[1], val[2]),
            //             val[3], val[4])
                    wip!();
        }
    | primary_value tCOLON2 tIDENTIFIER tOP_ASGN arg_rhs
        {
            // result = @builder.op_assign(
            //             @builder.call_method(
            //             val[0], val[1], val[2]),
            //             val[3], val[4])
                    wip!();
        }
    | primary_value tCOLON2 tCONSTANT tOP_ASGN arg_rhs
        {
            // const  = @builder.const_op_assignable(
            //             @builder.const_fetch(val[0], val[1], val[2]))
            // result = @builder.op_assign(const, val[3], val[4])
                    wip!();
        }
    | tCOLON3 tCONSTANT tOP_ASGN arg_rhs
        {
            // const  = @builder.const_op_assignable(
            //             @builder.const_global(val[0], val[1]))
            // result = @builder.op_assign(const, val[2], val[3])
                    wip!();
        }
    | backref tOP_ASGN arg_rhs
        {
            // result = @builder.op_assign(val[0], val[1], val[2])
                    wip!();
        }
    | arg tDOT2 arg
        {
            // result = @builder.range_inclusive(val[0], val[1], val[2])
                    wip!();
        }
    | arg tDOT3 arg
        {
            // result = @builder.range_exclusive(val[0], val[1], val[2])
                    wip!();
        }
    | arg tPLUS arg
        {
            // result = @builder.binary_op(val[0], val[1], val[2])
                    wip!();
        }
    | arg tMINUS arg
        {
            // result = @builder.binary_op(val[0], val[1], val[2])
                    wip!();
        }
    | arg tSTAR2 arg
        {
            // result = @builder.binary_op(val[0], val[1], val[2])
                    wip!();
        }
    | arg tDIVIDE arg
        {
            // result = @builder.binary_op(val[0], val[1], val[2])
                    wip!();
        }
    | arg tPERCENT arg
        {
            // result = @builder.binary_op(val[0], val[1], val[2])
                    wip!();
        }
    | arg tPOW arg
        {
            // result = @builder.binary_op(val[0], val[1], val[2])
                    wip!();
        }
    | tUNARY_NUM simple_numeric tPOW arg
        {
            // result = @builder.unary_op(val[0],
            //             @builder.binary_op(
            //             val[1], val[2], val[3]))
                    wip!();
        }
    | tUPLUS arg
        {
            // result = @builder.unary_op(val[0], val[1])
                    wip!();
        }
    | tUMINUS arg
        {
            // result = @builder.unary_op(val[0], val[1])
                    wip!();
        }
    | arg tPIPE arg
        {
            // result = @builder.binary_op(val[0], val[1], val[2])
                    wip!();
        }
    | arg tCARET arg
        {
            // result = @builder.binary_op(val[0], val[1], val[2])
                    wip!();
        }
    | arg tAMPER2 arg
        {
            // result = @builder.binary_op(val[0], val[1], val[2])
                    wip!();
        }
    | arg tCMP arg
        {
            // result = @builder.binary_op(val[0], val[1], val[2])
                    wip!();
        }
    | rel_expr %prec tCMP
    | arg tEQ arg
        {
            // result = @builder.binary_op(val[0], val[1], val[2])
                    wip!();
        }
    | arg tEQQ arg
        {
            // result = @builder.binary_op(val[0], val[1], val[2])
                    wip!();
        }
    | arg tNEQ arg
        {
            // result = @builder.binary_op(val[0], val[1], val[2])
                    wip!();
        }
    | arg tMATCH arg
        {
            // result = @builder.match_op(val[0], val[1], val[2])
                    wip!();
        }
    | arg tNMATCH arg
        {
            // result = @builder.binary_op(val[0], val[1], val[2])
                    wip!();
        }
    | tBANG arg
        {
            // result = @builder.not_op(val[0], nil, val[1], nil)
                    wip!();
        }
    | tTILDE arg
        {
            // result = @builder.unary_op(val[0], val[1])
                    wip!();
        }
    | arg tLSHFT arg
        {
            // result = @builder.binary_op(val[0], val[1], val[2])
                    wip!();
        }
    | arg tRSHFT arg
        {
            // result = @builder.binary_op(val[0], val[1], val[2])
                    wip!();
        }
    | arg tANDOP arg
        {
            // result = @builder.logical_op(:and, val[0], val[1], val[2])
                    wip!();
        }
    | arg tOROP arg
        {
            // result = @builder.logical_op(:or, val[0], val[1], val[2])
                    wip!();
        }
    | kDEFINED opt_nl arg
        {
            // result = @builder.keyword_cmd(:defined?, val[0], nil, [ val[2] ], nil)
                    wip!();
        }
    | arg tEH arg opt_nl tCOLON arg
        {
            // result = @builder.ternary(val[0], val[1],
            //                         val[2], val[4], val[5])
                    wip!();
        }
    | primary
;

           relop: tGT | tLT | tGEQ | tLEQ
;

        rel_expr: arg relop arg %prec tGT
                    {
                    //   result = @builder.binary_op(val[0], val[1], val[2])
                    wip!();
                    }
                | rel_expr relop arg %prec tGT
                    {
                    //   result = @builder.binary_op(val[0], val[1], val[2])
                    wip!();
                    }
;

arg_value: arg;

aref_args
    : {
        || -> Nodes; $$ = vec![];
    }
    | args trailer { $$ = $1; }
    | args tCOMMA assocs trailer
        {
            // result = val[0] << @builder.associate(nil, val[2], nil)
            wip!();
        }
    | assocs trailer
        {
            // result = [ @builder.associate(nil, val[0], nil) ]
            wip!();
        }
;

arg_rhs
    : arg %prec tOP_ASGN
    | arg kRESCUE_MOD arg
        {
            // rescue_body = @builder.rescue_body(val[1],
            //                 nil, nil, nil,
            //                 nil, val[2])

            // result = @builder.begin_body(val[0], [ rescue_body ])
            wip!();
        }
;

      paren_args: tLPAREN2 opt_call_args rparen
                    {
                    //   result = val
            wip!();
                    }
;

  opt_paren_args:
                    {
                    //   result = [ nil, [], nil ]
            wip!();
                    }
                | paren_args
;

   opt_call_args:
                    {
                    //   result = []
                    wip!();
                    }
                | call_args
                | args tCOMMA
                | args tCOMMA assocs tCOMMA
                    {
                    //   result = val[0] << @builder.associate(nil, val[2], nil)
                    wip!();
                    }
                | assocs tCOMMA
                    {
                    //   result = [ @builder.associate(nil, val[0], nil) ]
                    wip!();
                    }
;

       call_args: command
                    {
                    //   result = [ val[0] ]
            wip!();
                    }
                | args opt_block_arg
                    {
                    //   result = val[0].concat(val[1])
            wip!();
                    }
                | assocs opt_block_arg
                    {
                    //   result = [ @builder.associate(nil, val[0], nil) ]
                    //   result.concat(val[1])
            wip!();
                    }
                | args tCOMMA assocs opt_block_arg
                    {
                    //   assocs = @builder.associate(nil, val[2], nil)
                    //   result = val[0] << assocs
                    //   result.concat(val[3])
            wip!();
                    }
                | block_arg
                    {
                    //   result =  [ val[0] ]
            wip!();
                    }
;

    command_args: fake_embedded_action_command_args call_args {
                    //   @lexer.cmdarg = val[0]

                    //   result = val[1]
                    wip!();
                    }
;

fake_embedded_action_command_args: {
    //   result = @lexer.cmdarg.dup
    //   @lexer.cmdarg.push(true)
    wip!();
};

       block_arg: tAMPER arg_value
                    {
                    //   result = @builder.block_pass(val[0], val[1])
            wip!();
                    }
;

   opt_block_arg: tCOMMA block_arg
                    {
                    //   result = [ val[1] ]
            wip!();
                    }
                |
                    {
                    //   result = []
            wip!();
                    }
;

args
    : arg_value {
        |$1:Node| -> Nodes; $$ = vec![$1];
    }
                | tSTAR arg_value
                    {
                    //   result = [ @builder.splat(val[0], val[1]) ]
                    wip!();
                    }
                | args tCOMMA arg_value
                    {
                        |$1:Nodes, $2:Token, $3:Node| -> Nodes;

                        $1.push($3); $$ = $1;
                    }
                | args tCOMMA tSTAR arg_value
                    {
                    //   result = val[0] << @builder.splat(val[2], val[3])
                    wip!();
                    }
;

        mrhs_arg: mrhs
                    {
                    //   result = @builder.array(nil, val[0], nil)
                    wip!();
                    }
                | arg_value
;

            mrhs: args tCOMMA arg_value
                    {
                    //   result = val[0] << val[2]
                    wip!();
                    }
                | args tCOMMA tSTAR arg_value
                    {
                    //   result = val[0] << @builder.splat(val[2], val[3])
                    wip!();
                    }
                | tSTAR arg_value
                    {
                    //   result = [ @builder.splat(val[0], val[1]) ]
                    wip!();
                    }
;

fake_embedded_action_primary_kBEGIN: {
    // result = @lexer.cmdarg.dup
    // @lexer.cmdarg.clear
    wip!();
};

fake_embedded_action_primary_tLPAREN_ARG: {
    // result = @lexer.cmdarg.dup
    // @lexer.cmdarg.clear
    wip!();
};

fake_embedded_action_primary_tLPAREN_ARG_stmt: {
    // @lexer.state = :expr_endarg
    wip!();
};

fake_embedded_action_primary_tLPAREN_ARG_2: {
    // @lexer.state = :expr_endarg
    wip!();
};

fake_embedded_action_primary_kWHILE_1: {
    //   @lexer.cond.push(true)
    wip!();
};

fake_embedded_action_primary_kWHILE_2: {
    //   @lexer.cond.pop
    wip!();
};

fake_embedded_action_primary_kUNTIL_1: {
    //   @lexer.cond.push(true)
    wip!();
};

fake_embedded_action_primary_kUNTIL_2: {
    //   @lexer.cond.pop
    wip!();
};

fake_embedded_action__primary__kFOR_1: {
    //   @lexer.cond.push(true)
    wip!();
};

fake_embedded_action__primary__kFOR_2: {
    //   @lexer.cond.pop
    wip!();
};

fake_embedded_action__primary__kCLASS_1: {
    //   @static_env.extend_static
    //   @lexer.push_cmdarg
    wip!();
};

fake_embedded_action__primary__kCLASS_2: {
    //   result = @def_level
    //   @def_level = 0

    //   @static_env.extend_static
    //   @lexer.push_cmdarg
    wip!();
};

fake_embedded_action__primary__kMODULE_1: {
    //   @static_env.extend_static
    //   @lexer.push_cmdarg
    wip!();
};

fake_embedded_action__primary__kDEF_1: {
    //   @def_level += 1
    //   @static_env.extend_static
    //   @lexer.push_cmdarg
    wip!();
};

fake_embedded_action__primary__kDEF_2: {
    //   @lexer.state = :expr_fname
    wip!();
};

fake_embedded_action__primary__kDEF_3: {
    //   @def_level += 1
    //   @static_env.extend_static
    //   @lexer.push_cmdarg
    wip!();
};

primary
    : literal
    | strings
    | xstring
    | regexp
    | words
    | qwords
    | symbols
    | qsymbols
    | var_ref
    | backref
    | tFID
        {
            // result = @builder.call_method(nil, nil, val[0])
            wip!();
        }
    | kBEGIN fake_embedded_action_primary_kBEGIN bodystmt kEND
        {
            // @lexer.cmdarg = val[1]

            // result = @builder.begin_keyword(val[0], val[2], val[3])
            wip!();
        }
    | tLPAREN_ARG fake_embedded_action_primary_tLPAREN_ARG stmt fake_embedded_action_primary_tLPAREN_ARG_stmt rparen
        {
            // @lexer.cmdarg = val[1]

            // result = @builder.begin(val[0], val[2], val[4])
            wip!();
        }
    | tLPAREN_ARG fake_embedded_action_primary_tLPAREN_ARG_2 opt_nl tRPAREN
        {
            // result = @builder.begin(val[0], nil, val[3])
            wip!();
        }
    | tLPAREN compstmt tRPAREN
        {
            // result = @builder.begin(val[0], val[1], val[2])
            wip!();
        }
    | primary_value tCOLON2 tCONSTANT {
        |$1:Node; $2:Token, $3:Token| -> Node;

        $$ = node::const_fetch($1, *$2.interior_token, *$3.interior_token);
    }
    | tCOLON3 tCONSTANT {
        |$1:Token, $2:Token| -> Node;

        $$ = node::const_global(*$1.interior_token, *$2.interior_token);
    }
    | tLBRACK aref_args tRBRACK {
        |$1:Token; $2:Nodes; $3:Token| -> Node;

        $$ = node::array($2);
    }
    | tLBRACE assoc_list tRCURLY {
        |$1:Token; $2:Nodes; $3:Token| -> Node;

        $$ = node::associate($2);
    }
                | kRETURN
                    {
                    //   result = @builder.keyword_cmd(:return, val[0])
                    wip!();
                    }
                | kYIELD tLPAREN2 call_args rparen
                    {
                    //   result = @builder.keyword_cmd(:yield, val[0], val[1], val[2], val[3])
                    wip!();
                    }
                | kYIELD tLPAREN2 rparen
                    {
                    //   result = @builder.keyword_cmd(:yield, val[0], val[1], [], val[2])
                    wip!();
                    }
                | kYIELD
                    {
                    //   result = @builder.keyword_cmd(:yield, val[0])
                    wip!();
                    }
                | kDEFINED opt_nl tLPAREN2 expr rparen
                    {
                    //   result = @builder.keyword_cmd(:defined?, val[0],
                    //                                 val[2], [ val[3] ], val[4])
                    wip!();
                    }
                | kNOT tLPAREN2 expr rparen
                    {
                    //   result = @builder.not_op(val[0], val[1], val[2], val[3])
                    wip!();
                    }
                | kNOT tLPAREN2 rparen
                    {
                    //   result = @builder.not_op(val[0], val[1], nil, val[2])
                    wip!();
                    }
                | fcall brace_block
                    {
                    //   method_call = @builder.call_method(nil, nil, val[0])

                    //   begin_t, args, body, end_t = val[1]
                    //   result      = @builder.block(method_call,
                    //                   begin_t, args, body, end_t)
                    wip!();
                    }
                | method_call
                | method_call brace_block
                    {
                    //   begin_t, args, body, end_t = val[1]
                    //   result      = @builder.block(val[0],
                    //                   begin_t, args, body, end_t)
                    wip!();
                    }
                | tLAMBDA lambda
                    {
                    //   lambda_call = @builder.call_lambda(val[0])

                    //   args, (begin_t, body, end_t) = val[1]
                    //   result      = @builder.block(lambda_call,
                    //                   begin_t, args, body, end_t)
                    wip!();
                    }
                | kIF expr_value then compstmt if_tail kEND
                    {
                    //   else_t, else_ = val[4]
                    //   result = @builder.condition(val[0], val[1], val[2],
                    //                               val[3], else_t,
                    //                               else_,  val[5])
                    wip!();
                    }
                | kUNLESS expr_value then compstmt opt_else kEND
                    {
                    //   else_t, else_ = val[4]
                    //   result = @builder.condition(val[0], val[1], val[2],
                    //                               else_,  else_t,
                    //                               val[3], val[5])
                    wip!();
                    }
                | kWHILE fake_embedded_action_primary_kWHILE_1 expr_value do fake_embedded_action_primary_kWHILE_2 compstmt kEND
                    {
                        //   result = @builder.loop(:while, val[0], val[2], val[3],
                        //                          val[5], val[6])
                        wip!();
                    }
                | kUNTIL fake_embedded_action_primary_kUNTIL_1 expr_value do fake_embedded_action_primary_kUNTIL_2 compstmt kEND
                    {
                    //   result = @builder.loop(:until, val[0], val[2], val[3],
                    //                          val[5], val[6])
                    wip!();
                    }
                | kCASE expr_value opt_terms case_body kEND
                    {
                    //   *when_bodies, (else_t, else_body) = *val[3]

                    //   result = @builder.case(val[0], val[1],
                    //                          when_bodies, else_t, else_body,
                    //                          val[4])
                    wip!();
                    }
                | kCASE            opt_terms case_body kEND
                    {
                    //   *when_bodies, (else_t, else_body) = *val[2]

                    //   result = @builder.case(val[0], nil,
                    //                          when_bodies, else_t, else_body,
                    //                          val[3])
                    wip!();
                    }
                | kFOR for_var kIN fake_embedded_action__primary__kFOR_1 expr_value do fake_embedded_action__primary__kFOR_2 compstmt kEND
                    {
                    //   result = @builder.for(val[0], val[1],
                    //                         val[2], val[4],
                    //                         val[5], val[7], val[8])
                    wip!();
                    }
                | kCLASS cpath superclass fake_embedded_action__primary__kCLASS_1 bodystmt kEND
                    {
                    //   if in_def?
                    //     diagnostic :error, :class_in_def, nil, val[0]
                    //   end

                    //   lt_t, superclass = val[2]
                    //   result = @builder.def_class(val[0], val[1],
                    //                               lt_t, superclass,
                    //                               val[4], val[5])

                    //   @lexer.pop_cmdarg
                    //   @static_env.unextend
                    wip!();
                    }
                | kCLASS tLSHFT expr term fake_embedded_action__primary__kCLASS_2 bodystmt kEND
                    {
                    //   result = @builder.def_sclass(val[0], val[1], val[2],
                    //                                val[5], val[6])

                    //   @lexer.pop_cmdarg
                    //   @static_env.unextend

                    //   @def_level = val[4]
                    wip!();
                    }
                | kMODULE cpath fake_embedded_action__primary__kMODULE_1 bodystmt kEND
                    {
                    //   if in_def?
                    //     diagnostic :error, :module_in_def, nil, val[0]
                    //   end

                    //   result = @builder.def_module(val[0], val[1],
                    //                                val[3], val[4])

                    //   @lexer.pop_cmdarg
                    //   @static_env.unextend
                    wip!();
                    }
                | kDEF fname fake_embedded_action__primary__kDEF_1 f_arglist bodystmt kEND
                    {
                    //   result = @builder.def_method(val[0], val[1],
                    //               val[3], val[4], val[5])

                    //   @lexer.pop_cmdarg
                    //   @static_env.unextend
                    //   @def_level -= 1
                    wip!();
                    }
                | kDEF singleton dot_or_colon fake_embedded_action__primary__kDEF_2 fname fake_embedded_action__primary__kDEF_3 f_arglist bodystmt kEND
                    {
                    //   result = @builder.def_singleton(val[0], val[1], val[2],
                    //               val[4], val[6], val[7], val[8])

                    //   @lexer.pop_cmdarg
                    //   @static_env.unextend
                    //   @def_level -= 1
                    wip!();
                    }
                | kBREAK
                    {
                    //   result = @builder.keyword_cmd(:break, val[0])
                    wip!();
                    }
                | kNEXT
                    {
                    //   result = @builder.keyword_cmd(:next, val[0])
                    wip!();
                    }
                | kREDO
                    {
                    //   result = @builder.keyword_cmd(:redo, val[0])
                    wip!();
                    }
                | kRETRY
                    {
                    //   result = @builder.keyword_cmd(:retry, val[0])
                    wip!();
                    }
;

primary_value: primary;

            then: term
                | kTHEN
                | term kTHEN
                    {
                    //   result = val[1]
                    wip!();
                    }
;

              do: term
                | kDO_COND
;

         if_tail: opt_else
                | kELSIF expr_value then compstmt if_tail
                    {
                    //   else_t, else_ = val[4]
                    //   result = [ val[0],
                    //              @builder.condition(val[0], val[1], val[2],
                    //                                 val[3], else_t,
                    //                                 else_,  nil),
                    //            ]
                    wip!();
                    }
;

        opt_else: none
                | kELSE compstmt
                    {
                    //   result = val
                    wip!();
                    }
;

         for_var: lhs
                | mlhs
;

          f_marg: f_norm_arg
                    {
                    //   result = @builder.arg(val[0])
                    wip!();
                    }
                | tLPAREN f_margs rparen
                    {
                    //   result = @builder.multi_lhs(val[0], val[1], val[2])
                    wip!();
                    }
;

     f_marg_list: f_marg
                    {
                    //   result = [ val[0] ]
                    wip!();
                    }
                | f_marg_list tCOMMA f_marg
                    {
                    //   result = val[0] << val[2]
                    wip!();
                    }
;

         f_margs: f_marg_list
                | f_marg_list tCOMMA tSTAR f_norm_arg
                    {
                    //   result = val[0].
                    //               push(@builder.restarg(val[2], val[3]))
                    wip!();
                    }
                | f_marg_list tCOMMA tSTAR f_norm_arg tCOMMA f_marg_list
                    {
                    //   result = val[0].
                    //               push(@builder.restarg(val[2], val[3])).
                    //               concat(val[5])
                    wip!();
                    }
                | f_marg_list tCOMMA tSTAR
                    {
                    //   result = val[0].
                    //               push(@builder.restarg(val[2]))
                    wip!();
                    }
                | f_marg_list tCOMMA tSTAR            tCOMMA f_marg_list
                    {
                    //   result = val[0].
                    //               push(@builder.restarg(val[2])).
                    //               concat(val[4])
                    wip!();
                    }
                |                    tSTAR f_norm_arg
                    {
                    //   result = [ @builder.restarg(val[0], val[1]) ]
                    wip!();
                    }
                |                    tSTAR f_norm_arg tCOMMA f_marg_list
                    {
                    //   result = [ @builder.restarg(val[0], val[1]),
                    //              *val[3] ]
                    wip!();
                    }
                |                    tSTAR
                    {
                    //   result = [ @builder.restarg(val[0]) ]
                    wip!();
                    }
                |                    tSTAR tCOMMA f_marg_list
                    {
                    //   result = [ @builder.restarg(val[0]),
                    //              *val[2] ]
                    wip!();
                    }
;

 block_args_tail: f_block_kwarg tCOMMA f_kwrest opt_f_block_arg
                    {
                    //   result = val[0].concat(val[2]).concat(val[3])
                    wip!();
                    }
                | f_block_kwarg opt_f_block_arg
                    {
                    //   result = val[0].concat(val[1])
                    wip!();
                    }
                | f_kwrest opt_f_block_arg
                    {
                    //   result = val[0].concat(val[1])
                    wip!();
                    }
                | f_block_arg
                    {
                    //   result = [ val[0] ]
                    wip!();
                    }
;

opt_block_args_tail:
                  tCOMMA block_args_tail
                    {
                    //   result = val[1]
                    wip!();
                    }
                |
                    {
                    //   result = []
                    wip!();
                    }
;

     block_param: f_arg tCOMMA f_block_optarg tCOMMA f_rest_arg              opt_block_args_tail
                    {
                    //   result = val[0].
                    //               concat(val[2]).
                    //               concat(val[4]).
                    //               concat(val[5])
                    wip!();
                    }
                | f_arg tCOMMA f_block_optarg tCOMMA f_rest_arg tCOMMA f_arg opt_block_args_tail
                    {
                    //   result = val[0].
                    //               concat(val[2]).
                    //               concat(val[4]).
                    //               concat(val[6]).
                    //               concat(val[7])
                    wip!();
                    }
                | f_arg tCOMMA f_block_optarg                                opt_block_args_tail
                    {
                    //   result = val[0].
                    //               concat(val[2]).
                    //               concat(val[3])
                    wip!();
                    }
                | f_arg tCOMMA f_block_optarg tCOMMA                   f_arg opt_block_args_tail
                    {
                    //   result = val[0].
                    //               concat(val[2]).
                    //               concat(val[4]).
                    //               concat(val[5])
                    wip!();
                    }
                | f_arg tCOMMA                       f_rest_arg              opt_block_args_tail
                    {
                    //   result = val[0].
                    //               concat(val[2]).
                    //               concat(val[3])
                    wip!();
                    }
                | f_arg tCOMMA
                | f_arg tCOMMA                       f_rest_arg tCOMMA f_arg opt_block_args_tail
                    {
                    //   result = val[0].
                    //               concat(val[2]).
                    //               concat(val[4]).
                    //               concat(val[5])
                    wip!();
                    }
                | f_arg                                                      opt_block_args_tail
                    {
                    //   if val[1].empty? && val[0].size == 1
                    //     result = [@builder.procarg0(val[0][0])]
                    //   else
                    //     result = val[0].concat(val[1])
                    //   end
                    wip!();
                    }
                | f_block_optarg tCOMMA              f_rest_arg              opt_block_args_tail
                    {
                    //   result = val[0].
                    //               concat(val[2]).
                    //               concat(val[3])
                    wip!();
                    }
                | f_block_optarg tCOMMA              f_rest_arg tCOMMA f_arg opt_block_args_tail
                    {
                    //   result = val[0].
                    //               concat(val[2]).
                    //               concat(val[4]).
                    //               concat(val[5])
                    wip!();
                    }
                | f_block_optarg                                             opt_block_args_tail
                    {
                    //   result = val[0].
                    //               concat(val[1])
                    wip!();
                    }
                | f_block_optarg tCOMMA                                f_arg opt_block_args_tail
                    {
                    //   result = val[0].
                    //               concat(val[2]).
                    //               concat(val[3])
                    }
                |                                    f_rest_arg              opt_block_args_tail
                    {
                    //   result = val[0].
                    //               concat(val[1])
                    wip!();
                    }
                |                                    f_rest_arg tCOMMA f_arg opt_block_args_tail
                    {
                    //   result = val[0].
                    //               concat(val[2]).
                    //               concat(val[3])
                    wip!();
                    }
                |                                                                block_args_tail
;

 opt_block_param:
                    {
                    //   result = @builder.args(nil, [], nil)
                    wip!();
                    }
                | block_param_def
                    {
                    //   @lexer.state = :expr_value
                    wip!();
                    }
;

 block_param_def: tPIPE opt_bv_decl tPIPE
                    {
                    //   result = @builder.args(val[0], val[1], val[2])
                    wip!();
                    }
                | tOROP
                    {
                    //   result = @builder.args(val[0], [], val[0])
                    wip!();
                    }
                | tPIPE block_param opt_bv_decl tPIPE
                    {
                    //   result = @builder.args(val[0], val[1].concat(val[2]), val[3])
                    wip!();
                    }
;

     opt_bv_decl: opt_nl
                    {
                    //   result = []
                    wip!();
                    }
                | opt_nl tSEMI bv_decls opt_nl
                    {
                    //   result = val[2]
                    wip!();
                    }
;

        bv_decls: bvar
                    {
                    //   result = [ val[0] ]
                    wip!();
                    }
                | bv_decls tCOMMA bvar
                    {
                    //   result = val[0] << val[2]
                    wip!();
                    }
;

            bvar: tIDENTIFIER
                    {
                    //   @static_env.declare val[0][0]
                    //   result = @builder.shadowarg(val[0])
                    wip!();
                    }
                | f_bad_arg
;

fake_embedded_action_lambda_1: {
    //   @static_env.extend_dynamic
    wip!();
};

fake_embedded_action_lambda_2: {
    //   result = @lexer.cmdarg.dup
    //   @lexer.cmdarg.clear
    wip!();
};

          lambda: fake_embedded_action_lambda_1 f_larglist fake_embedded_action_lambda_2 lambda_body
                    {
                    //   @lexer.cmdarg = val[2]
                    //   @lexer.cmdarg.lexpop

                    //   result = [ val[1], val[3] ]

                    //   @static_env.unextend
                    wip!();
                    }
;

     f_larglist: tLPAREN2 f_args opt_bv_decl tRPAREN
                    {
                    //   result = @builder.args(val[0], val[1].concat(val[2]), val[3])
                    wip!();
                    }
                | f_args
                    {
                    //   result = @builder.args(nil, val[0], nil)
                    wip!();
                    }
;

     lambda_body: tLAMBEG compstmt tRCURLY
                    {
                    //   result = [ val[0], val[1], val[2] ]
                    wip!();
                    }
                | kDO_LAMBDA compstmt kEND
                    {
                    //   result = [ val[0], val[1], val[2] ]
                    wip!();
                    }
;

        do_block: kDO_BLOCK do_body kEND
                    {
                    //   result = [ val[0], *val[1], val[2] ]
                    wip!();
                    }
;

      block_call: command do_block
                    {
                    //   begin_t, block_args, body, end_t = val[1]
                    //   result      = @builder.block(val[0],
                    //                   begin_t, block_args, body, end_t)
                    wip!();
                    }
                | block_call dot_or_colon operation2 opt_paren_args
                    {
                    //   lparen_t, args, rparen_t = val[3]
                    //   result = @builder.call_method(val[0], val[1], val[2],
                    //               lparen_t, args, rparen_t)
                    wip!();
                    }
                | block_call dot_or_colon operation2 opt_paren_args brace_block
                    {
                    //   lparen_t, args, rparen_t = val[3]
                    //   method_call = @builder.call_method(val[0], val[1], val[2],
                    //                   lparen_t, args, rparen_t)

                    //   begin_t, args, body, end_t = val[4]
                    //   result      = @builder.block(method_call,
                    //                   begin_t, args, body, end_t)
                    wip!();
                    }
                | block_call dot_or_colon operation2 command_args do_block
                    {
                    //   method_call = @builder.call_method(val[0], val[1], val[2],
                    //                   nil, val[3], nil)

                    //   begin_t, args, body, end_t = val[4]
                    //   result      = @builder.block(method_call,
                    //                   begin_t, args, body, end_t)
                    wip!();
                    }
;

     method_call: fcall paren_args
                    {
                    //   lparen_t, args, rparen_t = val[1]
                    //   result = @builder.call_method(nil, nil, val[0],
                    //               lparen_t, args, rparen_t)
                    wip!();
                    }
                | primary_value call_op operation2 opt_paren_args
                    {
                    //   lparen_t, args, rparen_t = val[3]
                    //   result = @builder.call_method(val[0], val[1], val[2],
                    //               lparen_t, args, rparen_t)
                    wip!();
                    }
                | primary_value tCOLON2 operation2 paren_args
                    {
                    //   lparen_t, args, rparen_t = val[3]
                    //   result = @builder.call_method(val[0], val[1], val[2],
                    //               lparen_t, args, rparen_t)
                    wip!();
                    }
                | primary_value tCOLON2 operation3
                    {
                    //   result = @builder.call_method(val[0], val[1], val[2])
                    wip!();
                    }
                | primary_value call_op paren_args
                    {
                    //   lparen_t, args, rparen_t = val[2]
                    //   result = @builder.call_method(val[0], val[1], nil,
                    //               lparen_t, args, rparen_t)
                    wip!();
                    }
                | primary_value tCOLON2 paren_args
                    {
                    //   lparen_t, args, rparen_t = val[2]
                    //   result = @builder.call_method(val[0], val[1], nil,
                    //               lparen_t, args, rparen_t)
                    wip!();
                    }
                | kSUPER paren_args
                    {
                    //   lparen_t, args, rparen_t = val[1]
                    //   result = @builder.keyword_cmd(:super, val[0],
                    //               lparen_t, args, rparen_t)
                    wip!();
                    }
                | kSUPER
                    {
                    //   result = @builder.keyword_cmd(:zsuper, val[0])
                    wip!();
                    }
                | primary_value tLBRACK2 opt_call_args rbracket
                    {
                    //   result = @builder.index(val[0], val[1], val[2], val[3])
                    wip!();
                    }
;

     brace_block: tLCURLY brace_body tRCURLY
                    {
                    //   result = [ val[0], *val[1], val[2] ]
                    wip!();
                    }
                | kDO do_body kEND
                    {
                    //   result = [ val[0], *val[1], val[2] ]
                    wip!();
                    }
;

fake_embedded_action_brace_body_1: {
    //   @static_env.extend_dynamic
    wip!();
};
fake_embedded_action_brace_body_2: {
    // result = @lexer.cmdarg.dup
    // @lexer.cmdarg.clear
    wip!();
};

      brace_body: fake_embedded_action_brace_body_1 fake_embedded_action_brace_body_2 opt_block_param compstmt
                    {
                    //   result = [ val[2], val[3] ]

                    //   @static_env.unextend
                    //   @lexer.cmdarg = val[1]
                    //   @lexer.cmdarg.pop
                    wip!();
                    }
;

fake_embedded_action_do_body_1: {
    //   @static_env.extend_dynamic
    wip!();
};
fake_embedded_action_do_body_2: {
    //   result = @lexer.cmdarg.dup
    //   @lexer.cmdarg.clear
    wip!();
};

         do_body: fake_embedded_action_do_body_1 fake_embedded_action_do_body_2 opt_block_param bodystmt
                    {
                    //   result = [ val[2], val[3] ]

                    //   @static_env.unextend
                    //   @lexer.cmdarg = val[1]
                    wip!();
                    }
;

       case_body: kWHEN args then compstmt cases
                    {
                    //   result = [ @builder.when(val[0], val[1], val[2], val[3]),
                    //              *val[4] ]
                    wip!();
                    }
;

           cases: opt_else
                    {
                    //   result = [ val[0] ]
                    wip!();
                    }
                | case_body
;

      opt_rescue: kRESCUE exc_list exc_var then compstmt opt_rescue
                    {
                    //   assoc_t, exc_var = val[2]

                    //   if val[1]
                    //     exc_list = @builder.array(nil, val[1], nil)
                    //   end

                    //   result = [ @builder.rescue_body(val[0],
                    //                   exc_list, assoc_t, exc_var,
                    //                   val[3], val[4]),
                    //              *val[5] ]
                    wip!();
                    }
                |
                    {
                    //   result = []
                    wip!();
                    }
;

        exc_list: arg_value
                    {
                    //   result = [ val[0] ]
                    wip!();
                    }
                | mrhs
                | none
;

         exc_var: tASSOC lhs
                    {
                    //   result = [ val[0], val[1] ]
                    wip!();
                    }
                | none
;

      opt_ensure: kENSURE compstmt
                    {
                    //   result = [ val[0], val[1] ]
                    wip!();
                    }
                | none
;

literal
    : numeric
    | symbol
    | dsym
;

strings
    : string {
        // result = @builder.string_compose(nil, val[0], nil)
        |$1:Nodes| -> Node;

        $$ = node::string_compose($1);
    }
;

string
    :string1 {
        |$1:Node| -> Nodes; $$ = vec![$1];
    }
    | string string1 {
        // result = val[0] << val[1]
        wip!();
    }
;

string1
    : tSTRING_BEG string_contents tSTRING_END {
        // string = @builder.string_compose(val[0], val[1], val[2])
        // result = @builder.dedent_string(string, @lexer.dedent_level)
        |$1:Token, $2:Nodes, $3:Token| -> Node;

        $$ = node::string_compose($2);
        // TODO dedent_string
    }
    | tSTRING {
        // string = @builder.string(val[0])
        // result = @builder.dedent_string(string, @lexer.dedent_level)

        |$1:Token| -> Node;

        let $$;
        if let InteriorToken::T_STRING(string_value) = *$1.interior_token {
            <REMOVE THIS LET>$$ = Node::Str(string_value);
        } else { unreachable!(); }
        // TODO builder.dedent_string
    }
    | tCHARACTER
        {
            // result = @builder.character(val[0])
            wip!();
        }
;

         xstring: tXSTRING_BEG xstring_contents tSTRING_END
                    {
                    //   string = @builder.xstring_compose(val[0], val[1], val[2])
                    //   result = @builder.dedent_string(string, @lexer.dedent_level)
                    wip!();
                    }
;

          regexp: tREGEXP_BEG regexp_contents tSTRING_END tREGEXP_OPT
                    {
                    //   opts   = @builder.regexp_options(val[3])
                    //   result = @builder.regexp_compose(val[0], val[1], val[2], opts)
                    wip!();
                    }
;

words
    : tWORDS_BEG word_list tSTRING_END {
        // result = @builder.words_compose(val[0], val[1], val[2])
        |$2:Nodes| -> Node;

        $$ = node::words_compose($2);
    }
;

word_list
    : {
        || -> Nodes; $$ = vec![];
    }
    | word_list word tSPACE {
        // result = val[0] << @builder.word(val[1])
        |$1:Nodes, $2:Node, $3:Token| -> Nodes;

        $1.push($2);
        $$ = $1;
    }
;

word
    : string_content {
        |$1:Node| -> Nodes; $$ = vec![$1];
    }
    | word string_content {
        |$1:Nodes, $2:Node| -> Nodes;
        $1.push($2); $$ = $1;
    }
;

         symbols: tSYMBOLS_BEG symbol_list tSTRING_END
                    {
                    //   result = @builder.symbols_compose(val[0], val[1], val[2])
                    wip!();
                    }
;

     symbol_list:
                    {
                    //   result = []
                    wip!();
                    }
                | symbol_list word tSPACE
                    {
                    //   result = val[0] << @builder.word(val[1])
                    wip!();
                    }
;

//           qwords: tQWORDS_BEG qword_list tSTRING_END
//                     {
//                       result = @builder.words_compose(val[0], val[1], val[2])
//                     }
qwords: tQWORDS_BEG qword_list tSTRING_END {
    |$2: Nodes| -> Node;

    $$ = node::words_compose($2);
};

        qsymbols: tQSYMBOLS_BEG qsym_list tSTRING_END
                    {
                    //   result = @builder.symbols_compose(val[0], val[1], val[2])
                    wip!();
                    }
;

qword_list
    : {
        || -> Nodes; $$ = vec![];
    }
    | qword_list tSTRING_CONTENT tSPACE {
        |$1:Nodes, $2:Token, $3:Token| -> Nodes;

        $1.push(node::string_internal(*$2.interior_token));
        $$ = $1;
    }
;

       qsym_list:
                    {
                    //   result = []
                    wip!();
                    }
                | qsym_list tSTRING_CONTENT tSPACE
                    {
                    //   result = val[0] << @builder.symbol_internal(val[1])
                    wip!();
                    }
;

string_contents
    : {
        || -> Nodes; $$ = vec![];
    }
    | string_contents string_content {
        |$1:Nodes, $2:Node| -> Nodes;

        $1.push($2);
        $$ = $1;
    }
;

xstring_contents
    : {
        || -> Nodes; $$ = vec![];
    }
    | xstring_contents string_content {
        |$1:Nodes, $2:Node| -> Nodes;
        $1.push($2);
        $$ = $1;
    }
;

regexp_contents:
                    {
                    //   result = []
                    wip!();
                    }
                | regexp_contents string_content
                    {
                    //   result = val[0] << val[1]
                    wip!();
                    }
;

fake_embedded_action__string_content__tSTRING_DBEG: {
    // @lexer.cond.push(false)
    // @lexer.cmdarg.push(false)
    wip!();
};

string_content
    : tSTRING_CONTENT {
        //                       result = @builder.string_internal(val[0])
        |$1:Token| -> Node;

        let $$;
        if let InteriorToken::T_STRING_CONTENT(string_value) = *$1.interior_token {
            <REMOVE THIS LET>$$ = Node::Str(string_value);
        } else { unreachable!(); } 
    }
    | tSTRING_DVAR string_dvar
        {
            // result = val[1]
            wip!();
        }
    | tSTRING_DBEG fake_embedded_action__string_content__tSTRING_DBEG compstmt tSTRING_DEND
        {
            // @lexer.cond.lexpop
            // @lexer.cmdarg.lexpop

            // result = @builder.begin(val[0], val[2], val[3])
            wip!();
        }
;

     string_dvar: tGVAR
                    {
                    //   result = @builder.gvar(val[0])
                    wip!();
                    }
                | tIVAR
                    {
                    //   result = @builder.ivar(val[0])
                    wip!();
                    }
                | tCVAR
                    {
                    //   result = @builder.cvar(val[0])
                    wip!();
                    }
                | backref
;

symbol
    : tSYMBOL {
        //                       @lexer.state = :expr_endarg
        //                       result = @builder.symbol(val[0])
        |$1:Token| -> Node;

        // TODO lexer.state
        $$ = node::symbol(*$1.interior_token);
    }
;

dsym
    : tSYMBEG xstring_contents tSTRING_END {
        //                       @lexer.state = :expr_endarg
        //                       result = @builder.symbol_compose(val[0], val[1], val[2])
        |$1:Token, $2:Nodes, $3:Token| -> Node;

        // TODO lexer.state
        $$ = node::symbol_compose($2);
    }
;

         numeric: simple_numeric
                    {
                    //   result = val[0]
                    wip!();
                    }
                | tUNARY_NUM simple_numeric %prec tLOWEST
                    {
                    //   if @builder.respond_to? :negate
                    //     # AST builder interface compatibility
                    //     result = @builder.negate(val[0], val[1])
                    //   else
                    //     result = @builder.unary_num(val[0], val[1])
                    //   end
                    wip!();
                    }
;

simple_numeric
    : tINTEGER {
        // TODO
        //                       @lexer.state = :expr_endarg
        //                       result = @builder.integer(val[0])
        || -> Node;

        let $$;
        if let SV::_0(token) = $1 {
            if let InteriorToken::T_INTEGER(value) = *token.interior_token {
                <REMOVE THIS LET>$$ = Node::Int(value);
            } else { unreachable!(); }
        } else { unreachable!(); }
    }
    | tFLOAT
        {
            // @lexer.state = :expr_endarg
            // result = @builder.float(val[0])
                    wip!();
        }
    | tRATIONAL
        {
            // @lexer.state = :expr_endarg
            // result = @builder.rational(val[0])
                    wip!();
        }
    | tIMAGINARY
        {
            // @lexer.state = :expr_endarg
            // result = @builder.complex(val[0])
                    wip!();
        }
;

user_variable
    : tIDENTIFIER {
        |$1:Token| -> Node;

        $$ = node::ident(*$1.interior_token);
    }
    | tIVAR {
        |$1:Token| -> Node;

        $$ = node::ivar(*$1.interior_token);
    }
    | tGVAR {
        |$1:Token| -> Node;

        $$ = node::gvar(*$1.interior_token);
    }
    | tCONSTANT {
        |$1:Token| -> Node;

        $$ = node::build_const(*$1.interior_token);
    }
    | tCVAR {
        |$1:Token| -> Node;

        $$ = node::cvar(*$1.interior_token);
    }
;

// keyword_variable: kNIL
//                     {
//                       result = @builder.nil(val[0])
//                     }
// TODO
keyword_variable
    // TODO builder.nil
    : kNIL { || -> Node; $$ = Node::Nil; }
//                 | kSELF
//                     {
//                       result = @builder.self(val[0])
//                     }
    // TODO builder.self
    | kSELF { || -> Node; $$ = Node::NSelf; }
//                 | kTRUE
//                     {
//                       result = @builder.true(val[0])
//                     }
    // TODO builder.true
    | kTRUE { || -> Node; $$ = Node::True; }
//                 | kFALSE
//                     {
//                       result = @builder.false(val[0])
//                     }
    // TODO builder.false
    | kFALSE { || -> Node; $$ = Node::False; }

                | k__FILE__
                    {
                    //   result = @builder.__FILE__(val[0])
                    wip!();
                    }
                | k__LINE__
                    {
                    //   result = @builder.__LINE__(val[0])
                    wip!();
                    }
                | k__ENCODING__
                    {
                    //   result = @builder.__ENCODING__(val[0])
                    wip!();
                    }
;

var_ref
    : user_variable {
        |$1:Node| -> Node;
        $$ = node::accessible($1);
    }
    | keyword_variable {
        |$1:Node| -> Node;
        $$ = node::accessible($1);
    }
;

         var_lhs: user_variable
                    {
                    //   result = @builder.assignable(val[0])
                    wip!();
                    }
                | keyword_variable
                    {
                    //   result = @builder.assignable(val[0])
                    wip!();
                    }
;

         backref: tNTH_REF
                    {
                    //   result = @builder.nth_ref(val[0])
                    wip!();
                    }
                | tBACK_REF
                    {
                    //   result = @builder.back_ref(val[0])
                    wip!();
                    }
;

fake_embedded_action__superclass__tLT: {
    //   @lexer.state = :expr_value
    wip!();
};

      superclass: tLT expr_value term
                    {
                    //   result = [ val[0], val[2] ]
                    wip!();
                    }
                |
                    {
                    //   result = nil
                    wip!();
                    }
;

fake_embedded_action__f_arglist__episolon: {
    //   result = @lexer.in_kwarg
    //   @lexer.in_kwarg = true
    wip!();
};

       f_arglist: tLPAREN2 f_args rparen
                    {
                    //   result = @builder.args(val[0], val[1], val[2])

                    //   @lexer.state = :expr_value
                    wip!();
                    }
                | fake_embedded_action__f_arglist__episolon f_args term
                    {
                    //   @lexer.in_kwarg = val[0]
                    //   result = @builder.args(nil, val[1], nil)
                    wip!();
                    }
;

       args_tail: f_kwarg tCOMMA f_kwrest opt_f_block_arg
                    {
                    //   result = val[0].concat(val[2]).concat(val[3])
                    wip!();
                    }
                | f_kwarg opt_f_block_arg
                    {
                    //   result = val[0].concat(val[1])
                    wip!();
                    }
                | f_kwrest opt_f_block_arg
                    {
                    //   result = val[0].concat(val[1])
                    wip!();
                    }
                | f_block_arg
                    {
                    //   result = [ val[0] ]
                    wip!();
                    }
;

   opt_args_tail: tCOMMA args_tail
                    {
                    //   result = val[1]
                    wip!();
                    }
                |
                    {
                    //   result = []
                    wip!();
                    }
;

          f_args: f_arg tCOMMA f_optarg tCOMMA f_rest_arg              opt_args_tail
                    {
                    //   result = val[0].
                    //               concat(val[2]).
                    //               concat(val[4]).
                    //               concat(val[5])
                    wip!();
                    }
                | f_arg tCOMMA f_optarg tCOMMA f_rest_arg tCOMMA f_arg opt_args_tail
                    {
                    //   result = val[0].
                    //               concat(val[2]).
                    //               concat(val[4]).
                    //               concat(val[6]).
                    //               concat(val[7])
                    wip!();
                    }
                | f_arg tCOMMA f_optarg                                opt_args_tail
                    {
                    //   result = val[0].
                    //               concat(val[2]).
                    //               concat(val[3])
                    wip!();
                    }
                | f_arg tCOMMA f_optarg tCOMMA                   f_arg opt_args_tail
                    {
                    //   result = val[0].
                    //               concat(val[2]).
                    //               concat(val[4]).
                    //               concat(val[5])
                    wip!();
                    }
                | f_arg tCOMMA                 f_rest_arg              opt_args_tail
                    {
                    //   result = val[0].
                    //               concat(val[2]).
                    //               concat(val[3])
                    wip!();
                    }
                | f_arg tCOMMA                 f_rest_arg tCOMMA f_arg opt_args_tail
                    {
                    //   result = val[0].
                    //               concat(val[2]).
                    //               concat(val[4]).
                    //               concat(val[5])
                    wip!();
                    }
                | f_arg                                                opt_args_tail
                    {
                    //   result = val[0].
                    //               concat(val[1])
                    wip!();
                    }
                |              f_optarg tCOMMA f_rest_arg              opt_args_tail
                    {
                    //   result = val[0].
                    //               concat(val[2]).
                    //               concat(val[3])
                    wip!();
                    }
                |              f_optarg tCOMMA f_rest_arg tCOMMA f_arg opt_args_tail
                    {
                    //   result = val[0].
                    //               concat(val[2]).
                    //               concat(val[4]).
                    //               concat(val[5])
                    wip!();
                    }
                |              f_optarg                                opt_args_tail
                    {
                    //   result = val[0].
                    //               concat(val[1])
                    wip!();
                    }
                |              f_optarg tCOMMA                   f_arg opt_args_tail
                    {
                    //   result = val[0].
                    //               concat(val[2]).
                    //               concat(val[3])
                    wip!();
                    }
                |                              f_rest_arg              opt_args_tail
                    {
                    //   result = val[0].
                    //               concat(val[1])
                    wip!();
                    }
                |                              f_rest_arg tCOMMA f_arg opt_args_tail
                    {
                    //   result = val[0].
                    //               concat(val[2]).
                    //               concat(val[3])
                    wip!();
                    }
                |                                                          args_tail
                    {
                    //   result = val[0]
                    wip!();
                    }
                |
                    {
                    //   result = []
                    wip!();
                    }
;

       f_bad_arg: tCONSTANT
                    {
                    //   diagnostic :error, :argument_const, nil, val[0]
                    wip!();
                    }
                | tIVAR
                    {
                    //   diagnostic :error, :argument_ivar, nil, val[0]
                    wip!();
                    }
                | tGVAR
                    {
                    //   diagnostic :error, :argument_gvar, nil, val[0]
                    wip!();
                    }
                | tCVAR
                    {
                    //   diagnostic :error, :argument_cvar, nil, val[0]
                    wip!();
                    }
;

      f_norm_arg: f_bad_arg
                | tIDENTIFIER
                    {
                    //   @static_env.declare val[0][0]

                    //   result = val[0]
                    wip!();
                    }
;

      f_arg_asgn: f_norm_arg
                    {
                    //   result = val[0]
                    wip!();
                    }
;

      f_arg_item: f_arg_asgn
                    {
                    //   result = @builder.arg(val[0])
                    wip!();
                    }
                | tLPAREN f_margs rparen
                    {
                    //   result = @builder.multi_lhs(val[0], val[1], val[2])
                    wip!();
                    }
;

           f_arg: f_arg_item
                    {
                    //   result = [ val[0] ]
                    wip!();
                    }
                | f_arg tCOMMA f_arg_item
                    {
                    //   result = val[0] << val[2]
                    wip!();
                    }
;

         f_label: tLABEL
                    {
                    //   check_kwarg_name(val[0])

                    //   @static_env.declare val[0][0]

                    //   result = val[0]
                    wip!();
                    }
;

            f_kw: f_label arg_value
                    {
                    //   result = @builder.kwoptarg(val[0], val[1])
                    wip!();
                    }
                | f_label
                    {
                    //   result = @builder.kwarg(val[0])
                    wip!();
                    }
;

      f_block_kw: f_label primary_value
                    {
                    //   result = @builder.kwoptarg(val[0], val[1])
                    wip!();
                    }
                | f_label
                    {
                    //   result = @builder.kwarg(val[0])
                    wip!();
                    }
;

   f_block_kwarg: f_block_kw
                    {
                    //   result = [ val[0] ]
                    wip!();
                    }
                | f_block_kwarg tCOMMA f_block_kw
                    {
                    //   result = val[0] << val[2]
                    wip!();
                    }
;

         f_kwarg: f_kw
                    {
                    //   result = [ val[0] ]
                    wip!();
                    }
                | f_kwarg tCOMMA f_kw
                    {
                    //   result = val[0] << val[2]
                    wip!();
                    }
;

     kwrest_mark: tPOW | tDSTAR;

        f_kwrest: kwrest_mark tIDENTIFIER
                    {
                    //   @static_env.declare val[1][0]

                    //   result = [ @builder.kwrestarg(val[0], val[1]) ]
                    wip!();
                    }
                | kwrest_mark
                    {
                    //   result = [ @builder.kwrestarg(val[0]) ]
                    wip!();
                    }
;

           f_opt: f_arg_asgn tEQL arg_value
                    {
                    //   result = @builder.optarg(val[0], val[1], val[2])
                    wip!();
                    }
;

     f_block_opt: f_arg_asgn tEQL primary_value
                    {
                    //   result = @builder.optarg(val[0], val[1], val[2])
                    wip!();
                    }
;

  f_block_optarg: f_block_opt
                    {
                    //   result = [ val[0] ]
                    wip!();
                    }
                | f_block_optarg tCOMMA f_block_opt
                    {
                    //   result = val[0] << val[2]
                    wip!();
                    }
;

        f_optarg: f_opt
                    {
                    //   result = [ val[0] ]
                    wip!();
                    }
                | f_optarg tCOMMA f_opt
                    {
                    //   result = val[0] << val[2]
                    wip!();
                    }
;

    restarg_mark: tSTAR2 | tSTAR;

      f_rest_arg: restarg_mark tIDENTIFIER
                    {
                    //   @static_env.declare val[1][0]

                    //   result = [ @builder.restarg(val[0], val[1]) ]
                    wip!();
                    }
                | restarg_mark
                    {
                    //   result = [ @builder.restarg(val[0]) ]
                    wip!();
                    }
;

     blkarg_mark: tAMPER2 | tAMPER;

     f_block_arg: blkarg_mark tIDENTIFIER
                    {
                    //   @static_env.declare val[1][0]

                    //   result = @builder.blockarg(val[0], val[1])
                    wip!();
                    };

 opt_f_block_arg: tCOMMA f_block_arg
                    {
                        || -> Nodes; $$ = vec![$2];
                    }
                |
                    {
                        || -> Nodes; $$ = vec![];
                    }
;

       singleton: var_ref
                | tLPAREN2 expr rparen
                    {
                        |$2:Node| -> Nodes; $$ = vec![$2];
                    }
;

assoc_list
    : {
        || -> Nodes; $$ = vec![];
    }
    | assocs trailer { $$ = $1; } // TODO i thought `$$ = $1;` is the default one, yet the generator does not yield it.
;

assocs
    : assoc {
        |$1:Node| -> Nodes;

        $$ = vec![$1];
    }
    | assocs tCOMMA assoc {
        |$1: Nodes; $2: Token; $3: Node| -> Nodes;

        $1.push($3);
        $$ = $1;
    }
;

           assoc: arg_value tASSOC arg_value
                    {
                    //   result = @builder.pair(val[0], val[1], val[2])
                        |$1: Node; $2: Token; $3: Node| -> Node;
                        $$ = node::pair($1, *$2.interior_token, $3);
                    }
                | tLABEL arg_value
                    {
                    //   result = @builder.pair_keyword(val[0], val[1])
                        |$1: Token; $2: Node| -> Node;
                        $$ = node::pair_keyword(*$1.interior_token, $2);
                    }
                | tSTRING_BEG string_contents tLABEL_END arg_value
                    {
                    //   result = @builder.pair_quoted(val[0], val[1], val[2], val[3])
                    wip!();
                    }
                | tDSTAR arg_value
                    {
                    //   result = @builder.kwsplat(val[0], val[1])
                    wip!();
                    }
;

       operation: tIDENTIFIER | tCONSTANT | tFID;
      operation2: tIDENTIFIER | tCONSTANT | tFID | op;
      operation3: tIDENTIFIER | tFID | op;
    dot_or_colon: call_op | tCOLON2;
         call_op: tDOT
                    {
                    //   result = [:dot, val[0][1]]
                    wip!();
                    }
                | tANDDOT
                    {
                    //   result = [:anddot, val[0][1]]
                    wip!();
                    }
;

opt_terms:  | terms ;

          opt_nl:  | tNL;

          rparen: opt_nl tRPAREN
                    {
                    //   result = val[1]
                    wip!();
                    };

        rbracket: opt_nl tRBRACK
                    {
                    //   result = val[1]
                    wip!();
                    }
;

trailer:  | tNL | tCOMMA ;

term
    : tSEMI {
        // yyerrok
        // TODO
        $$ = $1;
    }
    | tNL
;

terms
    : term
    | terms tSEMI
;

            none:
                  {
                    // result = nil
                    wip!();
                  }
;
