// enum node_type {
//     NODE_SCOPE,
//     NODE_BLOCK,
//     NODE_IF,
//     NODE_UNLESS,
//     NODE_CASE,
//     NODE_CASE2,
//     NODE_WHEN,
//     NODE_WHILE,
//     NODE_UNTIL,
//     NODE_ITER,
//     NODE_FOR,
//     NODE_FOR_MASGN,
//     NODE_BREAK,
//     NODE_NEXT,
//     NODE_REDO,
//     NODE_RETRY,
//     NODE_BEGIN,
//     NODE_RESCUE,
//     NODE_RESBODY,
//     NODE_ENSURE,
//     NODE_AND,
//     NODE_OR,
//     NODE_MASGN,
//     NODE_LASGN,
//     NODE_DASGN,
//     NODE_DASGN_CURR,
//     NODE_GASGN,
//     NODE_IASGN,
//     NODE_CDECL,
//     NODE_CVASGN,
//     NODE_OP_ASGN1,
//     NODE_OP_ASGN2,
//     NODE_OP_ASGN_AND,
//     NODE_OP_ASGN_OR,
//     NODE_OP_CDECL,
//     NODE_CALL,
//     NODE_OPCALL,
//     NODE_FCALL,
//     NODE_VCALL,
//     NODE_QCALL,
//     NODE_SUPER,
//     NODE_ZSUPER,
//     NODE_ARRAY,
//     NODE_ZARRAY,
//     NODE_VALUES,
//     NODE_HASH,
//     NODE_RETURN,
//     NODE_YIELD,
//     NODE_LVAR,
//     NODE_DVAR,
//     NODE_GVAR,
//     NODE_IVAR,
//     NODE_CONST,
//     NODE_CVAR,
//     NODE_NTH_REF,
//     NODE_BACK_REF,
//     NODE_MATCH,
//     NODE_MATCH2,
//     NODE_MATCH3,
//     NODE_LIT,
//     NODE_STR,
//     NODE_DSTR,
//     NODE_XSTR,
//     NODE_DXSTR,
//     NODE_EVSTR,
//     NODE_DREGX,
//     NODE_ONCE,
//     NODE_ARGS,
//     NODE_ARGS_AUX,
//     NODE_OPT_ARG,
//     NODE_KW_ARG,
//     NODE_POSTARG,
//     NODE_ARGSCAT,
//     NODE_ARGSPUSH,
//     NODE_SPLAT,
//     NODE_BLOCK_PASS,
//     NODE_DEFN,
//     NODE_DEFS,
//     NODE_ALIAS,
//     NODE_VALIAS,
//     NODE_UNDEF,
//     NODE_CLASS,
//     NODE_MODULE,
//     NODE_SCLASS,
//     NODE_COLON2,
//     NODE_COLON3,
//     NODE_DOT2,
//     NODE_DOT3,
//     NODE_FLIP2,
//     NODE_FLIP3,
//     NODE_SELF,
//     NODE_NIL,
//     NODE_TRUE,
//     NODE_FALSE,
//     NODE_ERRINFO,
//     NODE_DEFINED,
//     NODE_POSTEXE,
//     NODE_DSYM,
//     NODE_ATTRASGN,
//     NODE_LAMBDA,
//     NODE_LAST
// };

// TODO refine order, maybe via ruby-parser/AST_FORMAT
#[derive(Debug, PartialEq, Clone)]
pub enum Node {
    Nil,

    True,
    False,

    Int(isize),

    Str(String),
    DStr(Nodes),
    XStr(Nodes),

    Sym(String),
    DSym(Nodes),

    Array(Nodes),

    Pair{ key: NNode, value: NNode },
    Hash(Nodes), // NOTE Hash(Vec<Node::Pair>) 

    NSelf,

    LVar(String),
    IVar(String),
    GVar(String),
    CVar(String),

    Const { scope: NSNode, name: String },
    //            ^ CBase/Lvar
    //            ^ None means unscoped
    CBase, // :: in ::Foo

    // Const -> CAsgn
    CAsgn { scope: NSNode, name: String },

    Ident(String),

    // assignable
    // TODO maybe rename to LVAsgn?
    LVasgn(String, Nodes),
    IVasgn(String, Nodes),
    CVasgn(String, Nodes),
    GVasgn(String, Nodes),

    Begin(Nodes),

    Send { receiver: NSNode, selector: String, args: Nodes },
    // https://github.com/whitequark/parser/blob/master/doc/AST_FORMAT.md#send
    // NOTE
    //     receiver being None means sending to self
    // TODO note about selector and such
    CSend { receiver: NSNode, selector: String, args: Nodes },

    MLhs(Nodes),

    Module { name: NNode, body: NSNode },
    Class { name: NNode, superclass: NSNode, body: NSNode },
    // node->nd_cpath, node->nd_super, node->nd_body

    If { condition: NNode, then_body: NSNode, else_body: NSNode },
    // node->nd_cond, node->nd_body, node->nd_else);

    Arg(String),
    Args(Nodes),

    Def { name: String, args: NNode, body: NSNode },

    IRange { start: NSNode, end: NSNode }, // I as inclusive
    ERange { start: NSNode, end: NSNode }, // E as exclusive
}

pub type SomeNode = Option<Node>;
type NNode = Box<Node>; // NestedNode
type NSNode = Box<SomeNode>; // NestedSomeNode

pub type Nodes = Vec<Node>;

// TODO generate macros via procedure and macro, like in strum
// TODO macro to wrap value in Some, unless it's None
// TODO macros like hash should use pattern like { "a" => "b" }
#[macro_export] macro_rules! n_str { ($string:expr) => { Node::Str(String::from($string)) }; }
#[macro_export] macro_rules! n_sym { ($string:expr) => { Node::Sym(String::from($string)) }; }
#[macro_export] macro_rules! n_lvar { ($string:expr) => { Node::LVar(String::from($string)) }; }
#[macro_export] macro_rules! n_ivar { ($string:expr) => { Node::IVar(String::from($string)) }; }
#[macro_export] macro_rules! n_cvar { ($string:expr) => { Node::CVar(String::from($string)) }; }
#[macro_export] macro_rules! n_gvar { ($string:expr) => { Node::GVar(String::from($string)) }; }
#[macro_export] macro_rules! n_begin { ( $( $x:expr ),* ) => { { Node::Begin(vec![ $($x),* ]) } }; }
#[macro_export] macro_rules! n_dstr { ( $( $x:expr ),* ) => { { Node::DStr(vec![ $($x),* ]) } }; }
#[macro_export] macro_rules! n_array { ( $( $x:expr ),* ) => { { Node::Array(vec![ $($x),* ]) } }; }
#[macro_export] macro_rules! n_hash { ( $( $x:expr ),* ) => { { Node::Hash(vec![ $($x),* ]) } }; }
#[macro_export] macro_rules! n_pair { ($key:expr, $value:expr) => { Node::Pair { key: Box::new($key), value: Box::new($value) }; } }
#[macro_export] macro_rules! n_send { ($receiver:expr, $selector:expr, $args:expr) => { Node::Send { receiver: Box::new($receiver), selector: String::from($selector), args: $args } }; }
#[macro_export] macro_rules! n_csend { ($receiver:expr, $selector:expr, $args:expr) => { Node::CSend { receiver: Box::new($receiver), selector: String::from($selector), args: $args } }; }
#[macro_export] macro_rules! n_int { ($v:expr) => { Node::Int($v) }; }
#[macro_export] macro_rules! n_cbase { () => { Node::CBase }; }
#[macro_export] macro_rules! n_const { ($scope:expr, $name:expr) => { Node::Const { scope: Box::new($scope), name: String::from($name) } }; }
#[macro_export] macro_rules! n_module { ($name:expr, $body:expr) => { Node::Module { name: Box::new($name), body: Box::new($body) } }; }
#[macro_export] macro_rules! n_class { ($name:expr, $superclass:expr, $body:expr) => { Node::Class { name: Box::new($name), superclass: Box::new($superclass), body: Box::new($body) } }; }
#[macro_export] macro_rules! n_if { ($condition:expr, $then_body:expr, $else_body:expr) => { Node::If { condition: Box::new($condition), then_body: Box::new($then_body), else_body: Box::new($else_body) } }; }
#[macro_export] macro_rules! n_args { ( $( $x:expr ),* ) => { { Node::Args(vec![ $($x),* ]) } }; }
#[macro_export] macro_rules! n_arg { ($string:expr) => { Node::Arg(String::from($string)) }; }
#[macro_export] macro_rules! n_def { ($name:expr, $args:expr, $body:expr) => { Node::Def { name: String::from($name), args: Box::new($args), body: Box::new($body) } }; }
#[macro_export] macro_rules! n_irange { ($start:expr, $end:expr) => { Node::IRange { start: Box::new($start), end: Box::new($end) }; } }
#[macro_export] macro_rules! n_erange { ($start:expr, $end:expr) => { Node::ERange { start: Box::new($start), end: Box::new($end) }; } }


// TODO use a procedure derive for this
impl Node {
    pub fn push_children(&mut self, node: Node) {
        match self {
            Node::LVasgn(name, nodes) | Node::IVasgn(name, nodes) | Node::CVasgn(name, nodes) | Node::GVasgn(name, nodes) => {
                nodes.push(node);
            },
            _ => { unreachable!(); }
        }
    }
}
