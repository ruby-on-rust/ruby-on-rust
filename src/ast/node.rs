use parser::token::Token;

#[derive(Debug, PartialEq)]
pub enum Node {
    Nil,

    Int(isize),
    Ident(Token),
    Assign(Box<Node>, Token, Box<Node>),
    Assignable,

    None, // NOTE special Node for nothing
}


// def accessible(node)
//   case node.type
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
//
//   else
//     node
//   end
// end
// TODO
pub fn accessible(node: Node) -> Node {
    node
}
