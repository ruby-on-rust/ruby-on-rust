// 7336721746da5d4b8bdbe147727594b1dc6824d1

use crate::{
    token::token::Token,
    lexer::dedenter::Dedenter,
    parser::static_env::StaticEnv,
};

macro_rules! wip { () => { panic!("WIP"); }; }

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
    // TODO CLEANUP
    // for migrating rules in grammar
    DUMMY,

    // for rules which may returns a result being `nil`, and the rule is acutally applied so we cannot return a None, i guess.
    // TODO still not sure about this.
    Null,

    // 
    // primitive values
    // 
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

    Pair { key: Box<Node>, value: Box<Node> },
    Hash(Nodes), // NOTE Hash(Vec<Node::Pair>) 

    NSelf,

    LVar(String),
    IVar(String),
    GVar(String),
    CVar(String),

    Const { scope: Option<Box<Node>>, name: String },
    //                        ^ CBase/Lvar
    //             ^ None means unscoped
    CBase, // :: in ::Foo

    // Const -> CAsgn
    CAsgn { scope: Option<Box<Node>>, name: String },

    Ident(String),

    // assignable
    // TODO maybe rename to LVAsgn?
    LVasgn(String, Nodes),
    IVasgn(String, Nodes),
    CVasgn(String, Nodes),
    GVasgn(String, Nodes),

    Begin(Nodes),

    Arg(String),

    Send { receiver: Option<Box<Node>>, selector: String, args: Nodes },
    // https://github.com/whitequark/parser/blob/master/doc/AST_FORMAT.md#send
    // NOTE
    //     receiver being None means sending to self
    // TODO note about selector and such
    CSend { receiver: Option<Box<Node>>, selector: String, args: Nodes },

    MLhs(Nodes),

    Module { name: Box<Node>, body: Box<Option<Node>> },
    Class { name: Box<Node>, superclass: Box<Option<Node>>, body: Box<Option<Node>> },
    // node->nd_cpath, node->nd_super, node->nd_body

    If { condition: Box<Node>, then_body: Box<Option<Node>>, else_body: Box<Option<Node>> },
    // node->nd_cond, node->nd_body, node->nd_else);
}

pub type Nodes = Vec<Node>;

// TODO generate macros via procedure and macro, like in strum
#[macro_export] macro_rules! n_str { ($string:expr) => { Node::Str(String::from($string)) }; }
#[macro_export] macro_rules! n_sym { ($string:expr) => { Node::Sym(String::from($string)) }; }
#[macro_export] macro_rules! n_lvar { ($string:expr) => { Node::LVar(String::from($string)) }; }
#[macro_export] macro_rules! n_ivar { ($string:expr) => { Node::IVar(String::from($string)) }; }
#[macro_export] macro_rules! n_cvar { ($string:expr) => { Node::CVar(String::from($string)) }; }
#[macro_export] macro_rules! n_gvar { ($string:expr) => { Node::GVar(String::from($string)) }; }
#[macro_export] macro_rules! n_begin { ( $( $x:expr ),* ) => { { Node::Begin(vec![ $($x),* ]) } }; }
#[macro_export] macro_rules! n_dstr { ( $( $x:expr ),* ) => { { Node::DStr(vec![ $($x),* ]) } }; }
#[macro_export] macro_rules! n_hash { ( $( $x:expr ),* ) => { { Node::Hash(vec![ $($x),* ]) } }; }
#[macro_export] macro_rules! n_pair { ($key:expr, $value:expr) => { Node::Pair { key: Box::new($key), value: Box::new($value) }; } }
#[macro_export] macro_rules! n_send { ($receiver:expr, $selector:expr, $args:expr) => { Node::Send { receiver: $receiver, selector: String::from($selector), args: $args } }; }
#[macro_export] macro_rules! n_csend { ($receiver:expr, $selector:expr, $args:expr) => { Node::CSend { receiver: $receiver, selector: String::from($selector), args: $args } }; }
#[macro_export] macro_rules! n_int { ($v:expr) => { Node::Int($v) }; }
#[macro_export] macro_rules! n_const { ($scope:expr, $name:expr) => { Node::Const { scope: $scope, name: $name } }; }
#[macro_export] macro_rules! n_module { ($name:expr, $body:expr) => { Node::Module { name: Box::new($name), body: Box::new($body) } }; }
#[macro_export] macro_rules! n_class { ($name:expr, $superclass:expr, $body:expr) => { Node::Class { name: Box::new($name), superclass: Box::new($superclass), body: Box::new($body) } }; }
#[macro_export] macro_rules! n_if { ($condition:expr, $then_body:expr, $else_body:expr) => { Node::If { condition: Box::new($condition), then_body: Box::new($then_body), else_body: Box::new($else_body) } }; }

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

// #
// # Literals
// #

// # Singletons

// def nil(nil_t)
//   n0(:nil,
//     token_map(nil_t))
// end

// def true(true_t)
//   n0(:true,
//     token_map(true_t))
// end

// def false(false_t)
//   n0(:false,
//     token_map(false_t))
// end

// # Numerics

// def integer(integer_t)
//   numeric(:int, integer_t)
// end
pub fn integer(integer_t: Token) -> Node {
    numeric(integer_t)
}

// def float(float_t)
//   numeric(:float, float_t)
// end

// def rational(rational_t)
//   numeric(:rational, rational_t)
// end

// def complex(complex_t)
//   numeric(:complex, complex_t)
// end

// def numeric(kind, token)
//   n(kind, [ value(token) ],
//     Source::Map::Operator.new(nil, loc(token)))
// end
pub fn numeric(token: Token) -> Node {
    match token {
        Token::T_INTEGER(value) => Node::Int(value),
        _ => { panic!("unknown numeric token") }
    }
}

// def unary_num(unary_t, numeric)
//   value, = *numeric
//   operator_loc = loc(unary_t)
// 
//   case value(unary_t)
//   when '+'
//     value = +value
//   when '-'
//     value = -value
//   end
// 
//   numeric.updated(nil, [ value ],
//     :location =>
//       Source::Map::Operator.new(
//         operator_loc,
//         operator_loc.join(numeric.loc.expression)))
// end
// TODO INCOMPLETE
// DUMMY ALWAYS Node::Int
pub fn unary_num(t_unary: Token, n_simple_numeric: Node) -> Node {
    let mut numeric_value = if let Node::Int(int_value) = n_simple_numeric { int_value } else { panic!(); };

    if let Token::T_UNARY_NUM(polarty) = t_unary {
        match polarty.as_ref() {
            "+" => (),
            "-" => { numeric_value = 0 - numeric_value; },
            _ => { panic!(); }
        }
    } else { panic!(); }

    return Node::Int(numeric_value);
}

// def __LINE__(__LINE__t)
//   n0(:__LINE__,
//     token_map(__LINE__t))
// end

// # Strings

// def string(string_t)
//   n(:str, [ string_value(string_t) ],
//     delimited_string_map(string_t))
// end
pub fn string(string_t: Token) -> Node {
    Node::Str(string_value(string_t))
}

// def string_internal(string_t)
//   n(:str, [ string_value(string_t) ],
//     unquoted_map(string_t))
// end
// 
// string_t: Token::T_STRING_CONTENT
pub fn string_internal(string_t: Token) -> Node {
    Node::Str(string_value(string_t))
}

// def string_compose(begin_t, parts, end_t)
//   if collapse_string_parts?(parts)
//     if begin_t.nil? && end_t.nil?
//       parts.first
//     else
//       n(:str, parts.first.children,
//         string_map(begin_t, parts, end_t))
//     end
//   else
//     n(:dstr, [ *parts ],
//       string_map(begin_t, parts, end_t))
//   end
// end
// TODO note
// TODO INCOMPLETE DUMMY
pub fn string_compose(begin_t: Option<Token>, parts: Nodes, end_t: Option<Token>) -> Node {
    if is_collapse_string_parts(&parts) {
        // TODO DUMMY
        return parts.get(0).unwrap().clone()
    } else {
        return Node::DStr(parts)
    }
}

// def character(char_t)
//   n(:str, [ string_value(char_t) ],
//     prefix_string_map(char_t))
// end
pub fn character(char_t: Token) -> Node {
    wip!();
}

// def __FILE__(__FILE__t)
//   n0(:__FILE__,
//     token_map(__FILE__t))
// end

// # Symbols

// def symbol(symbol_t)
//   n(:sym, [ string_value(symbol_t).to_sym ],
//     prefix_string_map(symbol_t))
// end
// TODO INCOMPLETE
pub fn symbol(symbol_t: Token) -> Node {
    Node::Sym(string_value(symbol_t))
}

// def symbol_internal(symbol_t)
//   n(:sym, [ string_value(symbol_t).to_sym ],
//     unquoted_map(symbol_t))
// end
pub fn symbol_internal(symbol_t: Token) -> Node {
    wip!();
}

// def symbol_compose(begin_t, parts, end_t)
//   if collapse_string_parts?(parts)
//     str = parts.first
// 
//     n(:sym, [ str.children.first.to_sym ],
//       collection_map(begin_t, str.loc.expression, end_t))
//   elsif @parser.version == 18 && parts.empty?
//     diagnostic :error, :empty_symbol, nil, loc(begin_t).join(loc(end_t))
//   else
//     n(:dsym, [ *parts ],
//       collection_map(begin_t, parts, end_t))
//   end
// end
// TODO note
pub fn symbol_compose(begin_t: Token, parts: Nodes, end_t: Token) -> Node {
    // parts: Nodes(Vec<Node::Str>)

    if is_collapse_string_parts(&parts) {
        let n_str = parts.get(0).unwrap();

        // TODO DUMMY collection_map
        if let Node::Str(str_value) = n_str {
            return Node::Sym(str_value.to_string());
        } else { unreachable!(); }
    } else {
        // NOTE ignored ruby18
        // TODO DUMMY collection_map
        return Node::DSym(parts);
    }
}

// # Executable strings

// def xstring_compose(begin_t, parts, end_t)
//   n(:xstr, [ *parts ],
//     string_map(begin_t, parts, end_t))
// end
pub fn xstring_compose(begin_t: Token, parts: Nodes, end_t: Token) -> Node {
    wip!();
}

// # Indented (interpolated, noninterpolated, executable) strings

pub fn dedent_string(node: Node, dedent_level: Option<isize>) -> Node {
    //   if !dedent_level.nil?
    //     dedenter = Lexer::Dedenter.new(dedent_level)
    // 
    //     if node.type == :str
    //       str = node.children.first
    //       dedenter.dedent(str)
    //     elsif node.type == :dstr || node.type == :xstr
    //       node.children.each do |str_node|
    //         if str_node.type == :str
    //           str = str_node.children.first
    //           dedenter.dedent(str)
    //         else
    //           dedenter.interrupt
    //         end
    //       end
    //     end
    //   end
    // 
    //   node

    if let Some(dedent_level) = dedent_level {
        let mut dedenter = Dedenter::new(dedent_level);
        match node {
            Node::Str(ref string) => {
                dedenter.dedent(string);
            },
            Node::DStr(nodes) | Node::XStr(nodes) => {
                // dedenter.dedent(nodes[0].expect("node:dedent_string empty nodes"));
                wip!();
            },
            _ => { panic!("node:dedent_string: unknown how to handle node {:?}", node); }
        }
    }

    node
}

// # Regular expressions

// def regexp_options(regopt_t)
//   options = value(regopt_t).
//     each_char.sort.uniq.
//     map(&:to_sym)
// 
//   n(:regopt, options,
//     token_map(regopt_t))
// end

// def regexp_compose(begin_t, parts, end_t, options)
//   begin
//     static_regexp(parts, options)
//   rescue RegexpError => e
//     diagnostic :error, :invalid_regexp, { :message => e.message },
//                loc(begin_t).join(loc(end_t))
//   end
// 
//   n(:regexp, (parts << options),
//     regexp_map(begin_t, end_t, options))
// end

// # Arrays

// def array(begin_t, elements, end_t)
//   n(:array, elements,
//     collection_map(begin_t, elements, end_t))
// end
// TODO INCOMPLETE
pub fn array(begin_t: Option<Token>, elements: Nodes, end_t: Option<Token>) -> Node {
    Node::Array(elements)
}

// def splat(star_t, arg=nil)
//   if arg.nil?
//     n0(:splat,
//       unary_op_map(star_t))
//   else
//     n(:splat, [ arg ],
//       unary_op_map(star_t, arg))
//   end
// end
pub fn splat(star_t: Token, arg: Option<Node>) -> Node {
    wip!();
    // if let Some(n_arg) = arg {
    // } else {
    //     wip!();
    // }
}

// def word(parts)
//   if collapse_string_parts?(parts)
//     parts.first
//   else
//     n(:dstr, [ *parts ],
//       collection_map(nil, parts, nil))
//   end
// end
pub fn word(parts: Nodes) -> Node {
    wip!();
}

// def words_compose(begin_t, parts, end_t)
//   n(:array, [ *parts ],
//     collection_map(begin_t, parts, end_t))
// end
pub fn words_compose(begin_t: Token, parts: Nodes, end_t: Token) -> Node {
    return Node::Array(parts);
}

// def symbols_compose(begin_t, parts, end_t)
//   parts = parts.map do |part|
//     case part.type
//     when :str
//       value, = *part
//       part.updated(:sym, [ value.to_sym ])
//     when :dstr
//       part.updated(:dsym)
//     else
//       part
//     end
//   end
// 
//   n(:array, [ *parts ],
//     collection_map(begin_t, parts, end_t))
// end
pub fn symbols_compose(begin_t: Token, parts: Nodes, end_t: Token) -> Node {
    wip!();
}

// # Hashes

// def pair(key, assoc_t, value)
//   n(:pair, [ key, value ],
//     binary_op_map(key, assoc_t, value))
// end
pub fn pair(key: Node, t_assoc: Token, value: Node) -> Node {
    // TODO binary_op_map
    Node::Pair { key: Box::new(key), value: Box::new(value) }
}

// def pair_list_18(list)
//   if list.size % 2 != 0
//     diagnostic :error, :odd_hash, nil, list.last.loc.expression
//   else
//     list.
//       each_slice(2).map do |key, value|
//         n(:pair, [ key, value ],
//           binary_op_map(key, nil, value))
//       end
//   end
// end

// def pair_keyword(key_t, value)
//   key_map, pair_map = pair_keyword_map(key_t, value)
// 
//   key = n(:sym, [ value(key_t).to_sym ], key_map)
// 
//   n(:pair, [ key, value ], pair_map)
// end
pub fn pair_keyword(key_t: Token, value: Node) -> Node {
    // key_t: Tokne::Node::T_LABEL

    // TODO INCOMPLETE pair_keyword_map

    // TODO macro like value! to extract value, instead of `if let`
    if let Token::T_LABEL(key_name) = key_t {
        let key = Node::Sym(key_name);
        return Node::Pair { key: Box::new(key), value: Box::new(value) };
    }

    unreachable!();
}

// def pair_quoted(begin_t, parts, end_t, value)
//   end_t, pair_map = pair_quoted_map(begin_t, end_t, value)
// 
//   key = symbol_compose(begin_t, parts, end_t)
// 
//   n(:pair, [ key, value ], pair_map)
// end
pub fn pair_quoted(begin_t: Token, parts: Nodes, end_t: Token, value: Node) -> Node {
    let key = symbol_compose(begin_t, parts, end_t);
    n_pair!(key, value)
}

// def kwsplat(dstar_t, arg)
//   n(:kwsplat, [ arg ],
//     unary_op_map(dstar_t, arg))
// end
pub fn kwsplat(dstar_t: Token, arg: Node) -> Node {
    wip!();
}

// def associate(begin_t, pairs, end_t)
//   n(:hash, [ *pairs ],
//     collection_map(begin_t, pairs, end_t))
// end
pub fn associate(begin_t: Option<Token>, pairs: Nodes, end_t: Option<Token>) -> Node {
    // TODO map
    return Node::Hash(pairs);
}

// # Ranges

// def range_inclusive(lhs, dot2_t, rhs)
//   n(:irange, [ lhs, rhs ],
//     range_map(lhs, dot2_t, rhs))
// end
pub fn range_inclusive(lhs: Node, dot2_t: Token, rhs: Option<Node>) -> Node {
    wip!();
}

// def range_exclusive(lhs, dot3_t, rhs)
//   n(:erange, [ lhs, rhs ],
//     range_map(lhs, dot3_t, rhs))
// end
pub fn range_exclusive(lhs: Node, dot3_t: Token, rhs: Option<Node>) -> Node {
    wip!();
}

// #
// # Access
// #

// def self(token)
//   n0(:self,
//     token_map(token))
// end
pub fn build_self(token: Token) -> Node {
    Node::NSelf
}

// def ident(token)
//   n(:ident, [ value(token).to_sym ],
//     variable_map(token))
// end
pub fn ident(token: Token) -> Node {
    // token: Token::T_IDENTIFIER

    if let Token::T_IDENTIFIER(id) = token {
        // TODO variable_map
        return Node::Ident(id);
    }
    unreachable!();
}

// def ivar(token)
//   n(:ivar, [ value(token).to_sym ],
//     variable_map(token))
// end
pub fn ivar(token: Token) -> Node {
    // token: Token::T_IVAR

    if let Token::T_IVAR(var_str) = token {
        // TODO variable_map
        return Node::IVar(var_str);
    }
    unreachable!();
}

// def gvar(token)
//   n(:gvar, [ value(token).to_sym ],
//     variable_map(token))
// end
pub fn gvar(token: Token) -> Node {
    // token: Token::T_GVAR

    if let Token::T_GVAR(var_str) = token {
        // TODO variable_map
        return Node::GVar(var_str);
    }
    unreachable!();
}

// def cvar(token)
//   n(:cvar, [ value(token).to_sym ],
//     variable_map(token))
// end
pub fn cvar(token: Token) -> Node {
    // token: Token::T_CVAR

    if let Token::T_CVAR(var_str) = token {
        // TODO variable_map
        return Node::CVar(var_str);
    }
    unreachable!();
}

// def back_ref(token)
//   n(:back_ref, [ value(token).to_sym ],
//     token_map(token))
// end
pub fn back_ref(token: Token) -> Node {
    wip!();
}

// def nth_ref(token)
//   n(:nth_ref, [ value(token) ],
//     token_map(token))
// end
pub fn nth_ref(token: Token) -> Node {
    wip!();
}

// TODO INCOMPLETE
pub fn accessible(node: Node, static_env: &StaticEnv) -> Node {
    println!("node:accessible, node: {:?}", node);

    match node {
        //   when :__FILE__
        //     if @emit_file_line_as_literals
        //       n(:str, [ node.loc.expression.source_buffer.name ],
        //         node.loc.dup)
        //     else
        //       node
        //     end
        // 
        //   when :__LINE__
        //     if @emit_file_line_as_literals
        //       n(:int, [ node.loc.expression.line ],
        //         node.loc.dup)
        //     else
        //       node
        //     end
        // 
        //   when :__ENCODING__
        //     n(:const, [ n(:const, [ nil, :Encoding], nil), :UTF_8 ],
        //       node.loc.dup)
        // 
        //   when :ident
        //     name, = *node
        // 
        //     if @parser.static_env.declared?(name)
        //       node.updated(:lvar)
        //     else
        //       name, = *node
        //       n(:send, [ nil, name ],
        //         var_send_map(node))
        //     end
        Node::Ident(n_value) => {
            if static_env.has_declared(&n_value) {
                Node::LVar(n_value)
            } else {
                n_send!(None, n_value, vec![])
            }
        }
        _ => { node }
    }
}

// def const(name_t)
//   n(:const, [ nil, value(name_t).to_sym ],
//     constant_map(nil, nil, name_t))
// end
// NOTE unscoped (scope being None) const
pub fn build_const(name_t: Token) -> Node {
    if let Token::T_CONSTANT(const_name) = name_t {
        return Node::Const {
            scope: None,
            name: const_name
        }
    }
    unreachable!();
}

// def const_global(t_colon3, name_t)
//   cbase = n0(:cbase, token_map(t_colon3))
// 
//   n(:const, [ cbase, value(name_t).to_sym ],
//     constant_map(cbase, t_colon3, name_t))
// end
// NOTE top level const like ::Foo
pub fn const_global(t_colon3: Token, name_t: Token) -> Node {
    if let Token::T_CONSTANT(const_name) = name_t {
        return Node::Const {
            scope: Some(Box::new(Node::CBase)),
            name: const_name
        }
    }
    unreachable!();
}

// def const_fetch(scope, t_colon2, name_t)
//   n(:const, [ scope, value(name_t).to_sym ],
//     constant_map(scope, t_colon2, name_t))
// end
pub fn const_fetch(scope: Node, t_colon2: Token, name_t: Token) -> Node {
    if let Token::T_CONSTANT(name_str) = name_t {
        return Node::Const {
            scope: Some(Box::new(scope)),
            name: name_str
        };
    }
    unreachable!();
}

// def __ENCODING__(__ENCODING__t)
//   n0(:__ENCODING__,
//     token_map(__ENCODING__t))
// end

// #
// # Assignment
// #

// def assignable(node)
//   case node.type
//   when :cvar
//     node.updated(:cvasgn)
// 
//   when :ivar
//     node.updated(:ivasgn)
// 
//   when :gvar
//     node.updated(:gvasgn)
// 
//   when :const
//     if @parser.in_def?
//       diagnostic :error, :dynamic_const, nil, node.loc.expression
//     end
// 
//     node.updated(:casgn)
// 
//   when :ident
//     name, = *node
//     @parser.static_env.declare(name)
// 
//     node.updated(:lVasgn)
// 
//   when :nil, :self, :true, :false,
//        :__FILE__, :__LINE__, :__ENCODING__
//     diagnostic :error, :invalid_assignment, nil, node.loc.expression
// 
//   when :back_ref, :nth_ref
//     diagnostic :error, :backref_assignment, nil, node.loc.expression
//   end
// end
// TODO INCOMPLETE
pub fn assignable(node: Node, static_env: &mut StaticEnv) -> Node {
    match node {
        Node::CVar(cvar_str) => {
            return Node::CVasgn(cvar_str, vec![]);
        },
        Node::IVar(ivar_str) => {
            return Node::IVasgn(ivar_str, vec![]);
        },
        Node::GVar(gvar_str) => {
            return Node::GVasgn(gvar_str, vec![]);
        },

        //   when :const
        //     unless @parser.context.dynamic_const_definition_allowed?
        //       diagnostic :error, :dynamic_const, nil, node.loc.expression
        //     end
        // 
        //     node.updated(:casgn)
        Node::Const { scope: scope, name: name } => {
            // TODO handle `in_def?`
            return Node::CAsgn { scope, name };
        },

        //   when :ident
        //     name, = *node
        //     @parser.static_env.declare(name)
        // 
        //     node.updated(:lVasgn)
        Node::Ident(ident) => {
            static_env.declare(ident.clone());
            return Node::LVasgn(ident, vec![]);
        },

        _ => { panic!("node::assignable: UNIMPL branch"); }
    }
}

// def const_op_assignable(node)
//   node.updated(:casgn)
// end
pub fn const_op_assignable(node: Node) -> Node {
    wip!();
}

// def assign(lhs, eql_t, rhs)
//   (lhs << rhs).updated(nil, nil,
//     :location => lhs.loc.
//       with_operator(loc(eql_t)).
//       with_expression(join_exprs(lhs, rhs)))
// end
// TODO INCOMPLETE
pub fn assign(mut lhs_node: Node, token: Token, rhs_node: Node) -> Node {
    match lhs_node {
        Node::LVasgn(_, _) | Node::IVasgn(_, _) | Node::CVasgn(_, _) | Node::GVasgn(_, _) => {
            lhs_node.push_children(rhs_node);
            return lhs_node;
        },
        _ => { unreachable!(); }
    }
}

// def op_assign(lhs, op_t, rhs)
//   case lhs.type
//   when :gvasgn, :ivasgn, :lVasgn, :cvasgn, :casgn, :send, :csend, :index
//     operator   = value(op_t)[0..-1].to_sym
//     source_map = lhs.loc.
//                     with_operator(loc(op_t)).
//                     with_expression(join_exprs(lhs, rhs))
// 
//     if lhs.type  == :index
//       lhs = lhs.updated(:indexasgn)
//     end
// 
//     case operator
//     when :'&&'
//       n(:and_asgn, [ lhs, rhs ], source_map)
//     when :'||'
//       n(:or_asgn, [ lhs, rhs ], source_map)
//     else
//       n(:op_asgn, [ lhs, operator, rhs ], source_map)
//     end
// 
//   when :back_ref, :nth_ref
//     diagnostic :error, :backref_assignment, nil, lhs.loc.expression
//   end
// end
pub fn op_assign(lhs: Node, op_t: Token, rhs: Node) -> Node {
    wip!();
}

// def multi_lhs(begin_t, items, end_t)
//   n(:mlhs, [ *items ],
//     collection_map(begin_t, items, end_t))
// end
pub fn multi_lhs(begin_t: Option<Token>, items: Nodes, end_t: Option<Token>) -> Node {
    wip!();
}

// def multi_assign(lhs, eql_t, rhs)
//   n(:masgn, [ lhs, rhs ],
//     binary_op_map(lhs, eql_t, rhs))
// end
pub fn multi_assign(lhs: Node, eql_t: Token, rhs: Node) -> Node {
    wip!();
}

// #
// # Class and module definition
// #

// def def_class(class_t, name,
//               lt_t, superclass,
//               body, end_t)
//   n(:class, [ name, superclass, body ],
//     module_definition_map(class_t, name, lt_t, end_t))
// end
pub fn def_class(class_t: Token, name: Node, lt_t: Token, superclass: Option<Node>, body: Option<Node>, end_t: Token) -> Node {
    n_class!(name, superclass, body)
}

// def def_sclass(class_t, lshft_t, expr,
//                body, end_t)
//   n(:sclass, [ expr, body ],
//     module_definition_map(class_t, nil, lshft_t, end_t))
// end

// def def_module(module_t, name,
//                body, end_t)
//   n(:module, [ name, body ],
//     module_definition_map(module_t, name, nil, end_t))
// end
pub fn def_module(module_t: Token, name: Node, body: Option<Node>, end_t: Token) -> Node {
    n_module!(name, body)
}

// #
// # Method (un)definition
// #

// def def_method(def_t, name_t, args,
//                body, end_t)
//   n(:def, [ value(name_t).to_sym, args, body ],
//     definition_map(def_t, nil, name_t, end_t))
// end

// def def_singleton(def_t, definee, dot_t,
//                   name_t, args,
//                   body, end_t)
//   case definee.type
//   when :int, :str, :dstr, :sym, :dsym,
//        :regexp, :array, :hash
// 
//     diagnostic :error, :singleton_literal, nil, definee.loc.expression
// 
//   else
//     n(:defs, [ definee, value(name_t).to_sym, args, body ],
//       definition_map(def_t, dot_t, name_t, end_t))
//   end
// end

// def undef_method(undef_t, names)
//   n(:undef, [ *names ],
//     keyword_map(undef_t, nil, names, nil))
// end
pub fn undef_method(undef_t: Token, names: Nodes) -> Node {
    wip!();
}

// def alias(alias_t, to, from)
//   n(:alias, [ to, from ],
//     keyword_map(alias_t, nil, [to, from], nil))
// end
pub fn alias(alias_t: Token, to: Node, from: Node) -> Node {
    wip!();
}

// #
// # Formal arguments
// #

// def args(begin_t, args, end_t, check_args=true)
//   args = check_duplicate_args(args) if check_args
//   n(:args, args,
//     collection_map(begin_t, args, end_t))
// end
pub fn args(begin_t: Option<Token>, args: Nodes, name_t: Option<Token>) -> Node {
    wip!();
}

// def arg(name_t)
//   n(:arg, [ value(name_t).to_sym ],
//     variable_map(name_t))
// end
pub fn arg(name_t: Token) -> Node {
    // Node::Arg()
    wip!();
}

// def optarg(name_t, eql_t, value)
//   n(:optarg, [ value(name_t).to_sym, value ],
//     variable_map(name_t).
//       with_operator(loc(eql_t)).
//       with_expression(loc(name_t).join(value.loc.expression)))
// end
pub fn optarg(name_t: Token, eql_t: Token, value: Node) -> Node {
    wip!();
}

// def restarg(star_t, name_t=nil)
//   if name_t
//     n(:restarg, [ value(name_t).to_sym ],
//       arg_prefix_map(star_t, name_t))
//   else
//     n0(:restarg,
//       arg_prefix_map(star_t))
//   end
// end
pub fn restarg(star_t: Token, name_t: Option<Token>) -> Node {
    wip!();
}

// def kwarg(name_t)
//   n(:kwarg, [ value(name_t).to_sym ],
//     kwarg_map(name_t))
// end
pub fn kwarg(name_t: Token) -> Node {
    wip!();
}

// def kwoptarg(name_t, value)
//   n(:kwoptarg, [ value(name_t).to_sym, value ],
//     kwarg_map(name_t, value))
// end
pub fn kwoptarg(name_t: Token, value: Node) -> Node {
    wip!();
}

// def kwrestarg(dstar_t, name_t=nil)
//   if name_t
//     n(:kwrestarg, [ value(name_t).to_sym ],
//       arg_prefix_map(dstar_t, name_t))
//   else
//     n0(:kwrestarg,
//       arg_prefix_map(dstar_t))
//   end
// end
pub fn kwrestarg(dstar_t: Token, name_t: Option<Token>) -> Node {
    wip!();
}

// def shadowarg(name_t)
//   n(:shadowarg, [ value(name_t).to_sym ],
//     variable_map(name_t))
// end
pub fn shadowarg(name_t: Token) -> Node {
    wip!();
}

// def blockarg(amper_t, name_t)
//   n(:blockarg, [ value(name_t).to_sym ],
//     arg_prefix_map(amper_t, name_t))
// end
pub fn blockarg(amper_t: Token, name_t: Token) -> Node {
    wip!();
}

// def procarg0(arg)
//   if self.class.emit_procarg0
//     arg.updated(:procarg0)
//   else
//     arg
//   end
// end

// # Ruby 1.8 block arguments

// def arg_expr(expr)
//   if expr.type == :lVasgn
//     expr.updated(:arg)
//   else
//     n(:arg_expr, [ expr ],
//       expr.loc.dup)
//   end
// end

// def restarg_expr(star_t, expr=nil)
//   if expr.nil?
//     n0(:restarg, token_map(star_t))
//   elsif expr.type == :lVasgn
//     expr.updated(:restarg)
//   else
//     n(:restarg_expr, [ expr ],
//       expr.loc.dup)
//   end
// end

// def blockarg_expr(amper_t, expr)
//   if expr.type == :lVasgn
//     expr.updated(:blockarg)
//   else
//     n(:blockarg_expr, [ expr ],
//       expr.loc.dup)
//   end
// end

// # MacRuby Objective-C arguments

// def objc_kwarg(kwname_t, assoc_t, name_t)
//   kwname_l = loc(kwname_t)
//   if assoc_t.nil? # a: b, not a => b
//     kwname_l   = kwname_l.resize(kwname_l.size - 1)
//     operator_l = kwname_l.end.resize(1)
//   else
//     operator_l = loc(assoc_t)
//   end

//   n(:objc_kwarg, [ value(kwname_t).to_sym, value(name_t).to_sym ],
//     Source::Map::ObjcKwarg.new(kwname_l, operator_l, loc(name_t),
//                                kwname_l.join(loc(name_t))))
// end

// def objc_restarg(star_t, name=nil)
//   if name.nil?
//     n0(:restarg, arg_prefix_map(star_t))
//   elsif name.type == :arg # regular restarg
//     name.updated(:restarg, nil,
//       { :location => name.loc.with_operator(loc(star_t)) })
//   else # restarg with objc_kwarg inside
//     n(:objc_restarg, [ name ],
//       unary_op_map(star_t, name))
//   end
// end

// #
// # Method calls
// #

// def call_type_for_dot(dot_t)
//   if !dot_t.nil? && value(dot_t) == :anddot
//     :csend
//   else
//     # This case is a bit tricky. ruby23.y returns the token tDOT with
//     # the value :dot, and the token :tANDDOT with the value :anddot.
//     #
//     # But, ruby{18..22}.y (which unconditionally expect tDOT) just
//     # return "." there, since they are to be kept close to the corresponding
//     # Ruby MRI grammars.
//     #
//     # Thankfully, we don't have to care.
//     :send
//   end
// end
// 
// TODO REFINE THIS METHOD
// returns a "csend" or "send"
fn call_type_for_dot(dot_t: Option<Token>) -> &'static str {
    if let Some(dot_t) = dot_t {
        if dot_t.to_string() == "Token::T_ANDDOT" {
            return "csend"
        }
    }
    "send"
}

// def call_method(receiver, dot_t, selector_t,
//                 lparen_t=nil, args=[], rparen_t=nil)
//   type = call_type_for_dot(dot_t)
//   if selector_t.nil?
//     n(type, [ receiver, :call, *args ],
//       send_map(receiver, dot_t, nil, lparen_t, args, rparen_t))
//   else
//     n(type, [ receiver, value(selector_t).to_sym, *args ],
//       send_map(receiver, dot_t, selector_t, lparen_t, args, rparen_t))
//   end
// end
pub fn call_method(receiver: Option<Node>, dot_t: Option<Token>, selector_t: Option<Token>, lparen_t: Option<Token>, args: Nodes, rparen_t: Option<Token>) -> Node {
    println!("node:call_method, receiver: {:?}, selector: {:?}", receiver, selector_t);

    let r#type = call_type_for_dot(dot_t);

    // unwrap from Option, wrap again with Option<Box<>>>
    let receiver = match receiver {
        Some(node) => Some(Box::new(node)),
        None => None
    };

    if let Some(selector_t) = selector_t {
        // TODO refine this after we make every token has a value
        let selector_t_value = match selector_t {
            Token::T_FID(v) | Token::T_IDENTIFIER(v) | Token::T_CONSTANT(v) => v,
            _ => { panic!("unknown how to handle token {:?}", selector_t) }
        };

        match r#type {
            // TODO what is this ":call"? when selector is empty
            "csend" => n_csend!(receiver, selector_t_value, args),
            "send" => n_send!(receiver, selector_t_value, args),
            _ => { panic!("invalid type"); }
        }
    } else {
        match r#type {
            // TODO what is this ":call"? when selector is empty
            "csend" => n_csend!(receiver, "call", args),
            "send" => n_send!(receiver, "call", args),
            _ => { panic!("invalid type"); }
        }
    }
}

// def call_lambda(lambda_t)
//   if self.class.emit_lambda
//     n0(:lambda, expr_map(loc(lambda_t)))
//   else
//     n(:send, [ nil, :lambda ],
//       send_map(nil, nil, lambda_t))
//   end
// end
pub fn call_lambda(lambda_t: Token) -> Node {
    wip!();
}

// def block(method_call, begin_t, args, body, end_t)
//   _receiver, _selector, *call_args = *method_call
// 
//   if method_call.type == :yield
//     diagnostic :error, :block_given_to_yield, nil, method_call.loc.keyword, [loc(begin_t)]
//   end
// 
//   last_arg = call_args.last
//   if last_arg && last_arg.type == :block_pass
//     diagnostic :error, :block_and_blockarg, nil, last_arg.loc.expression, [loc(begin_t)]
//   end
// 
//   if [:send, :csend, :index, :super, :zsuper, :lambda].include?(method_call.type)
//     n(:block, [ method_call, args, body ],
//       block_map(method_call.loc.expression, begin_t, end_t))
//   else
//     # Code like "return foo 1 do end" is reduced in a weird sequence.
//     # Here, method_call is actually (return).
//     actual_send, = *method_call
//     block =
//       n(:block, [ actual_send, args, body ],
//         block_map(actual_send.loc.expression, begin_t, end_t))
// 
//     n(method_call.type, [ block ],
//       method_call.loc.with_expression(join_exprs(method_call, block)))
//   end
// end
pub fn block(method_call: Node, begin_t: Token, args: Node, body: Node, end_t: Token) -> Node {
    wip!();
}

// def block_pass(amper_t, arg)
//   n(:block_pass, [ arg ],
//     unary_op_map(amper_t, arg))
// end
pub fn block_pass(amper_t: Token, arg: Node) -> Node {
    wip!();
}

// def objc_varargs(pair, rest_of_varargs)
//   value, first_vararg = *pair
//   vararg_array = array(nil, [ first_vararg, *rest_of_varargs ], nil).
//     updated(:objc_varargs)
//   pair.updated(nil, [ value, vararg_array ],
//     { :location => pair.loc.with_expression(
//           pair.loc.expression.join(vararg_array.loc.expression)) })
// end

// def attr_asgn(receiver, dot_t, selector_t)
//   method_name = (value(selector_t) + '=').to_sym
//   type = call_type_for_dot(dot_t)
// 
//   # Incomplete method call.
//   n(type, [ receiver, method_name ],
//     send_map(receiver, dot_t, selector_t))
// end
pub fn attr_asgn(receiver: Node, dot_t: Token, selector_t: Token) -> Node {
    wip!();
}

// def index(receiver, lbrack_t, indexes, rbrack_t)
//     if self.class.emit_index
//     n(:index, [ receiver, *indexes ],
//         index_map(receiver, lbrack_t, rbrack_t))
//     else
//     n(:send, [ receiver, :[], *indexes ],
//         send_index_map(receiver, lbrack_t, rbrack_t))
//     end
// end
pub fn index(receiver: Node, lbrack_t: Token, indexes: Nodes, rbrack_t: Token) -> Node {
    wip!();
}

// def index_asgn(receiver, lbrack_t, indexes, rbrack_t)
//     if self.class.emit_index
//     n(:indexasgn, [ receiver, *indexes ],
//         index_map(receiver, lbrack_t, rbrack_t))
//     else
//     # Incomplete method call.
//     n(:send, [ receiver, :[]=, *indexes ],
//         send_index_map(receiver, lbrack_t, rbrack_t))
//     end
// end
pub fn index_asgn(receiver: Node, lbrack_t: Token, indexes: Nodes, rbrack_t: Token) -> Node {
    wip!();
}

// def binary_op(receiver, operator_t, arg)
//   source_map = send_binary_op_map(receiver, operator_t, arg)
// 
//   if @parser.version == 18
//     operator = value(operator_t)
// 
//     if operator == '!='
//       method_call = n(:send, [ receiver, :==, arg ], source_map)
//     elsif operator == '!~'
//       method_call = n(:send, [ receiver, :=~, arg ], source_map)
//     end
// 
//     if %w(!= !~).include?(operator)
//       return n(:not, [ method_call ],
//                expr_map(source_map.expression))
//     end
//   end
// 
//   n(:send, [ receiver, value(operator_t).to_sym, arg ],
//     source_map)
// end
pub fn binary_op(receiver: Node, operator_t: Token, arg: Node) -> Node {
    // TODO after we give every token a value
    let token_value = match operator_t {
        T_PLUS => "+",
        _ => {wip!();}
    };

    n_send!(Some(Box::new(receiver)), token_value, vec![arg])
}

// def match_op(receiver, match_t, arg)
//   source_map = send_binary_op_map(receiver, match_t, arg)
// 
//   if (regexp = static_regexp_node(receiver))
//     regexp.names.each do |name|
//       @parser.static_env.declare(name)
//     end
// 
//     n(:match_with_lVasgn, [ receiver, arg ],
//       source_map)
//   else
//     n(:send, [ receiver, :=~, arg ],
//       source_map)
//   end
// end
pub fn match_op(receiver: Node, match_t: Token, arg: Node) -> Node {
    wip!();
}

// def unary_op(op_t, receiver)
//   case value(op_t)
//   when '+', '-'
//     method = value(op_t) + '@'
//   else
//     method = value(op_t)
//   end
// 
//   n(:send, [ receiver, method.to_sym ],
//     send_unary_op_map(op_t, receiver))
// end
pub fn unary_op(op_t: Token, receiver: Node) -> Node {
    wip!();
}

// def not_op(not_t, begin_t=nil, receiver=nil, end_t=nil)
//   if @parser.version == 18
//     n(:not, [ check_condition(receiver) ],
//       unary_op_map(not_t, receiver))
//   else
//     if receiver.nil?
//       nil_node = n0(:begin, collection_map(begin_t, nil, end_t))
// 
//       n(:send, [
//         nil_node, :'!'
//       ], send_unary_op_map(not_t, nil_node))
//     else
//       n(:send, [ check_condition(receiver), :'!' ],
//         send_map(nil, nil, not_t, begin_t, [receiver], end_t))
//     end
//   end
// end
pub fn not_op(not_t: Token, begin_t: Option<Token>, receiver: Option<Node>, end_t: Option<Token>) -> Node {
    wip!();
}

// #
// # Control flow
// #

// # Logical operations: and, or

// def logical_op(type, lhs, op_t, rhs)
//   n(type, [ lhs, rhs ],
//     binary_op_map(lhs, op_t, rhs))
// end
pub fn logical_op(node_type: &str, lhs: Node, op_t: Token, rhs: Node) -> Node {
    wip!();
}

// # Conditionals

// def condition(cond_t, cond, then_t,
//               if_true, else_t, if_false, end_t)
//   n(:if, [ check_condition(cond), if_true, if_false ],
//     condition_map(cond_t, cond, then_t, if_true, else_t, if_false, end_t))
// end
pub fn condition(cond_t: Token, cond: Node, then_t: Token, if_true: Option<Node>, else_t: Option<Token>, if_false: Option<Node>, end_t: Option<Token>) -> Node {
    n_if!( check_condition(cond), if_true, if_false )
}

// def condition_mod(if_true, if_false, cond_t, cond)
//   n(:if, [ check_condition(cond), if_true, if_false ],
//     keyword_mod_map(if_true || if_false, cond_t, cond))
// end
pub fn condition_mod(if_true: Option<Node>, if_false: Option<Node>, cond_t: Token, cond: Node) -> Node {
    wip!();
}

// def ternary(cond, question_t, if_true, colon_t, if_false)
//   n(:if, [ check_condition(cond), if_true, if_false ],
//     ternary_map(cond, question_t, if_true, colon_t, if_false))
// end
pub fn ternary(cond: Node, question_t: Token, if_true: Node, colon_t: Token, if_false: Node) -> Node {
    wip!();
}

// # Case matching

// def when(when_t, patterns, then_t, body)
//   children = patterns << body
//   n(:when, children,
//     keyword_map(when_t, then_t, children, nil))
// end
pub fn when(when_t: Token, patterns: Nodes, then_t: Token, body: Node) -> Node {
    wip!();
}

// def case(case_t, expr, when_bodies, else_t, else_body, end_t)
//   n(:case, [ expr, *(when_bodies << else_body)],
//     condition_map(case_t, expr, nil, nil, else_t, else_body, end_t))
// end

// # Loops

// def loop(type, keyword_t, cond, do_t, body, end_t)
//   n(type, [ check_condition(cond), body ],
//     keyword_map(keyword_t, do_t, nil, end_t))
// end
pub fn build_loop(node_type: &str, keyword_t: Token, cond: Node, do_t: Token, body: Node, end_t: Token) -> Node {
    wip!();
}

// def loop_mod(type, body, keyword_t, cond)
//   if body.type == :kwbegin
//     type = :"#{type}_post"
//   end
// 
//   n(type, [ check_condition(cond), body ],
//     keyword_mod_map(body, keyword_t, cond))
// end
pub fn loop_mod(node_type: &str, body: Node, keyword_t: Token, cond: Node) -> Node {
    wip!();
}

// def for(for_t, iterator, in_t, iteratee,
//         do_t, body, end_t)
//   n(:for, [ iterator, iteratee, body ],
//     for_map(for_t, in_t, do_t, end_t))
// end
pub fn build_for(for_t: Token, iterator: Node, in_t: Token, iteratee: Node, do_t: Token, body: Node, end_t: Token) -> Node {
    wip!();
}

// # Keywords

// def keyword_cmd(type, keyword_t, lparen_t=nil, args=[], rparen_t=nil)
//   if type == :yield && args.count > 0
//     last_arg = args.last
//     if last_arg.type == :block_pass
//       diagnostic :error, :block_given_to_yield, nil, loc(keyword_t), [last_arg.loc.expression]
//     end
//   end
// 
//   n(type, args,
//     keyword_map(keyword_t, lparen_t, args, rparen_t))
// end
pub fn keyword_cmd(node_type: &str, keyword_t: Token, lparen_t: Option<Token>, args: Nodes, rparen_t: Option<Token>) -> Node {
    wip!();
}

// # BEGIN, END

// def preexe(preexe_t, lbrace_t, compstmt, rbrace_t)
//   n(:preexe, [ compstmt ],
//     keyword_map(preexe_t, lbrace_t, [], rbrace_t))
// end
// TODO NOTE
pub fn preexe(preexe_t: Token, lbrace_t: Token, compstmt: Node, rbrace_t: Token) -> Node {
    wip!();
}

// def postexe(postexe_t, lbrace_t, compstmt, rbrace_t)
//   n(:postexe, [ compstmt ],
//     keyword_map(postexe_t, lbrace_t, [], rbrace_t))
// end
// TODO NOTE
pub fn postexe(postexe_t: Token, lbrace_t: Token, compstmt: Node, rbrace_t: Token) -> Node {
    wip!();
}

// # Exception handling

// def rescue_body(rescue_t,
//                 exc_list, assoc_t, exc_var,
//                 then_t, compound_stmt)
//   n(:resbody, [ exc_list, exc_var, compound_stmt ],
//     rescue_body_map(rescue_t, exc_list, assoc_t,
//                     exc_var, then_t, compound_stmt))
// end
// 
// TODO NOTE
//     exclist: Option<Node::Array>
pub fn rescue_body(rescue_t: Token, exc_list: Option<Node>, assoc_t: Option<Token>, exc_var: Option<Node>, then_t: Option<Token>, compound_stmt: Node) -> Node {
    wip!();
}

// def begin_body(compound_stmt, rescue_bodies=[],
//                else_t=nil,    else_=nil,
//                ensure_t=nil,  ensure_=nil)
// 
//   if rescue_bodies.any?
//     if else_t
//       compound_stmt =
//         n(:rescue,
//           [ compound_stmt, *(rescue_bodies + [ else_ ]) ],
//           eh_keyword_map(compound_stmt, nil, rescue_bodies, else_t, else_))
//     else
//       compound_stmt =
//         n(:rescue,
//           [ compound_stmt, *(rescue_bodies + [ nil ]) ],
//           eh_keyword_map(compound_stmt, nil, rescue_bodies, nil, nil))
//     end
//   elsif else_t
//     statements = []
//     if !compound_stmt.nil?
//       if compound_stmt.type == :begin
//         statements += compound_stmt.children
//       else
//         statements.push(compound_stmt)
//       end
//     end
//     statements.push(
//       n(:begin, [ else_ ],
//         collection_map(else_t, [ else_ ], nil)))
//     compound_stmt =
//       n(:begin, statements,
//         collection_map(nil, statements, nil))
//   end
// 
//   if ensure_t
//     compound_stmt =
//       n(:ensure,
//         [ compound_stmt, ensure_ ],
//         eh_keyword_map(compound_stmt, ensure_t, [ ensure_ ], nil, nil))
//   end
// 
//   compound_stmt
// end
pub fn begin_body(  compound_stmt: Option<Node>, rescue_bodies: Nodes,
                    else_t: Option<Token>,       else_: Option<Node>,
                    ensure_t: Option<Token>,     ensure_: Option<Node> ) -> Option<Node> {

    // TODO

    compound_stmt
}

// #
// # Expression grouping
// #

// def compstmt(statements)
//   case
//   when statements.none?
//     nil
//   when statements.one?
//     statements.first
//   else
//     n(:begin, statements,
//       collection_map(nil, statements, nil))
//   end
// end
pub fn compstmt(nodes: Nodes) -> Option<Node> {
    match nodes.len() {
        0 => { None }
        1 => { Some(nodes.get(0).unwrap().clone()) }
        // TODO collection_map
        _ => { Some(Node::Begin(nodes)) }
    }
}

// def begin(begin_t, body, end_t)
//   if body.nil?
//     # A nil expression: `()'.
//     n0(:begin,
//       collection_map(begin_t, nil, end_t))
//   elsif body.type == :mlhs  ||
//        (body.type == :begin &&
//         body.loc.begin.nil? && body.loc.end.nil?)
//     # Synthesized (begin) from compstmt "a; b" or (mlhs)
//     # from multi_lhs "(a, b) = *foo".
//     n(body.type, body.children,
//       collection_map(begin_t, body.children, end_t))
//   else
//     n(:begin, [ body ],
//       collection_map(begin_t, [ body ], end_t))
//   end
// end
pub fn begin(begin_t: Token, body: Option<Node>, end_t: Token) -> Node {
    match body {
        None => { Node::Begin(vec![]) },
        _ => { wip!(); }
    }
}

// def begin_keyword(begin_t, body, end_t)
//   if body.nil?
//     # A nil expression: `begin end'.
//     n0(:kwbegin,
//       collection_map(begin_t, nil, end_t))
//   elsif (body.type == :begin &&
//          body.loc.begin.nil? && body.loc.end.nil?)
//     # Synthesized (begin) from compstmt "a; b".
//     n(:kwbegin, body.children,
//       collection_map(begin_t, body.children, end_t))
//   else
//     n(:kwbegin, [ body ],
//       collection_map(begin_t, [ body ], end_t))
//   end
// end
pub fn begin_keyword(begin_t: Token, body: Option<Node>, end_t: Token) -> Node {
    match body {
        None => { Node::Begin(vec![]) },
        _ => { wip!(); }
    }
}

// #
// # VERIFICATION
// #

// def check_condition(cond)
//   case cond.type
//   when :masgn
//     if @parser.version <= 23
//       diagnostic :error, :masgn_as_condition, nil, cond.loc.expression
//     else
//       cond
//     end
// 
//   when :begin
//     if cond.children.count == 1
//       cond.updated(nil, [
//         check_condition(cond.children.last)
//       ])
//     else
//       cond
//     end
// 
//   when :and, :or, :irange, :erange
//     lhs, rhs = *cond
// 
//     type = case cond.type
//     when :irange then :iflipflop
//     when :erange then :eflipflop
//     end
// 
//     if [:and, :or].include?(cond.type) &&
//            @parser.version == 18
//       cond
//     else
//       cond.updated(type, [
//         check_condition(lhs),
//         check_condition(rhs)
//       ])
//     end
// 
//   when :regexp
//     n(:match_current_line, [ cond ], expr_map(cond.loc.expression))
// 
//   else
//     cond
//   end
// end
// 
// TODO NOTE
fn check_condition(cond: Node) -> Node {
    // TODO
    cond
}

// def check_duplicate_args(args, map={})
//   args.each do |this_arg|
//     case this_arg.type
//     when :arg, :optarg, :restarg, :blockarg,
//          :kwarg, :kwoptarg, :kwrestarg,
//          :shadowarg, :procarg0
// 
//       this_name, = *this_arg
// 
//       that_arg   = map[this_name]
//       that_name, = *that_arg
// 
//       if that_arg.nil?
//         map[this_name] = this_arg
//       elsif arg_name_collides?(this_name, that_name)
//         diagnostic :error, :duplicate_argument, nil,
//                    this_arg.loc.name, [ that_arg.loc.name ]
//       end
// 
//     when :mlhs
//       check_duplicate_args(this_arg.children, map)
//     end
//   end
// end

// def arg_name_collides?(this_name, that_name)
//   case @parser.version
//   when 18
//     this_name == that_name
//   when 19
//     # Ignore underscore.
//     this_name != :_ &&
//       this_name == that_name
//   else
//     # Ignore everything beginning with underscore.
//     this_name && this_name[0] != '_' &&
//       this_name == that_name
//   end
// end

//     #
//     # HELPERS
//     #

//     # Extract a static string from e.g. a regular expression,
//     # honoring the fact that MRI expands interpolations like #{""}
//     # at parse time.
//     def static_string(nodes)
//       nodes.map do |node|
//         case node.type
//         when :str
//           node.children[0]
//         when :begin
//           if (string = static_string(node.children))
//             string
//           else
//             return nil
//           end
//         else
//           return nil
//         end
//       end.join
//     end

//     def static_regexp(parts, options)
//       source = static_string(parts)
//       return nil if source.nil?
// 
//       source = case
//       when options.children.include?(:u)
//         source.encode(Encoding::UTF_8)
//       when options.children.include?(:e)
//         source.encode(Encoding::EUC_JP)
//       when options.children.include?(:s)
//         source.encode(Encoding::WINDOWS_31J)
//       when options.children.include?(:n)
//         source.encode(Encoding::BINARY)
//       else
//         source
//       end
// 
//       Regexp.new(source, (Regexp::EXTENDED if options.children.include?(:x)))
//     end

//     def static_regexp_node(node)
//       if node.type == :regexp
//         parts, options = node.children[0..-2], node.children[-1]
//         static_regexp(parts, options)
//       end
//     end

//     def collapse_string_parts?(parts)
//       parts.one? &&
//           [:str, :dstr].include?(parts.first.type)
//     end
// TODO note
fn is_collapse_string_parts(parts: &Nodes) -> bool {
    if parts.len() == 1 {
        match parts.get(0).unwrap() {
            // TODO emm why can't we just use
            // Node::Str(_str) | Node::DStr(_dstr) => { return true; },
            Node::Str(_str) => { return true; },
            Node::DStr(_dstr) => { return true; },
            _ => {}
        }
    }

    false
}

//     def value(token)
//       token[0]
//     end

//     def string_value(token)
//       unless token[0].valid_encoding?
//         diagnostic(:error, :invalid_encoding, nil, token[1])
//       end
// 
//       token[0]
//     end
fn string_value(token: Token) -> String {
    // TODO encoding stuff

    // TODO refine this after we make all Token has a value
    match token {
        Token::T_STRING(v) | Token::T_STRING_CONTENT(v) | Token::T_SYMBOL(v) => v,
        _ => { panic!("node::string_value: unknown how to handle token {:?}", token); }
    }
}

//     def loc(token)
//       # Pass through `nil`s and return nil for tNL.
//       token[1] if token && token[0]
//     end

//     def diagnostic(type, reason, arguments, location, highlights=[])
//       @parser.diagnostics.process(
//           Diagnostic.new(type, reason, arguments, location, highlights))
// 
//       if type == :error
//         @parser.send :yyerror
//       end
//     end
//   end
