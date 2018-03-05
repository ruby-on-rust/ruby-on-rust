use parser::token::Token;

#[derive(Debug, PartialEq)]
pub enum Node {
    Nil,

    True,
    False,

    Int(isize),

    Ident(Token),
    Assign(Box<Node>, Token, Box<Node>),
    Assignable,

    None, // NOTE special Node for nothing
}


// def unary_num(unary_t, numeric)
//   value, = *numeric
//   operator_loc = loc(unary_t)

//   case value(unary_t)
//   when '+'
//     value = +value
//   when '-'
//     value = -value
//   end

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
