// // BASED ON https://github.com/whitequark/parser/blob/2a73841d6da04a5ab9bd270561165fd766722d43/test/test_parser.rb

extern crate ruby_on_rust;

use ruby_on_rust::parser::parser::Parser;
use ruby_on_rust::ast::node::Node;

// helpers
// TODO NOTE
macro_rules! assert_parses {
    ($content:expr, $node:expr) => {
        let mut parser = Parser::new();
        let node = parser.parse($content);
        assert_eq!(node, $node);
    };
}

// TODO NOTE
macro_rules! n_str {
    ($string:expr) => {
        Node::Str(String::from($string))
    };
}

// TODO NOTE
macro_rules! n_sym {
    ($string:expr) => {
        Node::Sym(String::from($string))
    };
}

//   #
//   # Literals
//   #

//   def test_empty_stmt
//     assert_parses(
//       nil,
//       %q{})
//   end
// TODO FAILING

//   def test_nil
//     assert_parses(
//       s(:nil),
//       %q{nil},
//       %q{~~~ expression})
//   end
#[test]
fn test_nil() { assert_parses!("nil", Node::Nil); }

//   def test_nil_expression
//     assert_parses(
//       s(:begin),
//       %q{()},
//       %q{^ begin
//         | ^ end
//         |~~ expression})

//     assert_parses(
//       s(:kwbegin),
//       %q{begin end},
//       %q{~~~~~ begin
//         |      ~~~ end
//         |~~~~~~~~~ expression})
//   end

//   def test_true
//     assert_parses(
//       s(:true),
//       %q{true},
//       %q{~~~~ expression})
//   end

//   def test_false
//     assert_parses(
//       s(:false),
//       %q{false},
//       %q{~~~~~ expression})
//   end

//   def test_int
//     assert_parses(
//       s(:int, 42),
//       %q{42},
//       %q{~~ expression})

//     assert_parses(
//       s(:int, 42),
//       %q{+42},
//       %q{^ operator
//         |~~~ expression})

//     assert_parses(
//       s(:int, -42),
//       %q{-42},
//       %q{^ operator
//         |~~~ expression})
//   end

//   def test_int___LINE__
//     assert_parses(
//       s(:int, 1),
//       %q{__LINE__},
//       %q{~~~~~~~~ expression})
//   end

//   def test_float
//     assert_parses(
//       s(:float, 1.33),
//       %q{1.33},
//       %q{~~~~ expression})

//     assert_parses(
//       s(:float, -1.33),
//       %q{-1.33},
//       %q{^ operator
//         |~~~~~ expression})
//   end

//   def test_rational
//     assert_parses(
//       s(:rational, Rational(42)),
//       %q{42r},
//       %q{~~~ expression},
//       SINCE_2_1)

//     assert_parses(
//       s(:rational, Rational(421, 10)),
//       %q{42.1r},
//       %q{~~~~~ expression},
//       SINCE_2_1)
//   end

//   def test_complex
//     assert_parses(
//       s(:complex, Complex(0, 42)),
//       %q{42i},
//       %q{~~~ expression},
//       SINCE_2_1)

//     assert_parses(
//       s(:complex, Complex(0, Rational(42))),
//       %q{42ri},
//       %q{~~~~ expression},
//       SINCE_2_1)

//     assert_parses(
//       s(:complex, Complex(0, 42.1)),
//       %q{42.1i},
//       %q{~~~~~ expression},
//       SINCE_2_1)

//     assert_parses(
//       s(:complex, Complex(0, Rational(421, 10))),
//       %q{42.1ri},
//       %q{~~~~~~ expression},
//       SINCE_2_1)
//   end
