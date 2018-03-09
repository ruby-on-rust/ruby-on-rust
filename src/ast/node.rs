use parser::token::Token;

#[derive(Debug, PartialEq)]
pub enum Node {
    Nil,

    True,
    False,

    Int(isize),

    Str(String),

    Sym(String),

    Ident(Token),
    Assign(Box<Node>, Token, Box<Node>),
    Assignable,
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

// # Strings

// def string(string_t)
//   n(:str, [ string_value(string_t) ],
//     delimited_string_map(string_t))
// end

// def string_internal(string_t)
//   n(:str, [ string_value(string_t) ],
//     unquoted_map(string_t))
// end

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
// TODO INCOMPLETE
pub fn string_compose(parts: Node) -> Node {
    // TODO DUMMY
    if let Node::Str(string_value) = parts {
        return Node::Str(string_value);
    } else {
        panic!("");
    }
}

// def character(char_t)
//   n(:str, [ string_value(char_t) ],
//     prefix_string_map(char_t))
// end

// def __FILE__(__FILE__t)
//   n0(:__FILE__,
//     token_map(__FILE__t))
// end

// # Symbols

// def symbol(symbol_t)
//   n(:sym, [ string_value(symbol_t).to_sym ],
//     prefix_string_map(symbol_t))
// end

// def symbol_internal(symbol_t)
//   n(:sym, [ string_value(symbol_t).to_sym ],
//     unquoted_map(symbol_t))
// end

// def symbol_compose(begin_t, parts, end_t)
//   if collapse_string_parts?(parts)
//     str = parts.first

//     n(:sym, [ str.children.first.to_sym ],
//       collection_map(begin_t, str.loc.expression, end_t))
//   elsif @parser.version == 18 && parts.empty?
//     diagnostic :error, :empty_symbol, nil, loc(begin_t).join(loc(end_t))
//   else
//     n(:dsym, [ *parts ],
//       collection_map(begin_t, parts, end_t))
//   end
// end
