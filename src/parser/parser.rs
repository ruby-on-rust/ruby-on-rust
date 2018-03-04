// https based on github.com/whitequark/parser/blob/2a73841d6da04a5ab9bd270561165fd766722d43/lib/parser/ruby25.y

use lexer::lexing_state::LexingState;
use lexer::Lexer;
use parser::token::Token;
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

    // get a new one if necessary
    fn current_token(&mut self) -> Token {
        if self.tokens.get(self.current_p).is_none() {
            self.tokens.push(self.lexer.advance().expect("no token emitted after lexer.advance()"));
        }

        let token = (*self.tokens.get(self.current_p).expect("no current token for current_p")).clone();

        token
    }

    fn consume_current_token(&mut self) {
        self.current_p += 1;
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

    // TODO IMCOMPLETE
    // fn p_stmts(&mut self) -> Option<Node> {
    //     if let Some(n_stmt) = self.p_stmt() { return Some(n_stmt); }
    //     None
    // }

    //    stmt_or_begin: stmt
    //                 | klBEGIN tLCURLY top_compstmt tRCURLY
    //                     {
    //                       diagnostic :error, :begin_in_method, nil, val[0]
    //                     }
    // TODO IMCOMPLETE
    // fn p_stmt_or_begin(&mut self) -> Option<Node> {
    //     if let Some(n_stmt) = self.p_stmt() { return Some(n_stmt); }
    //     None
    // }

    // TODO IMCOMPLETE
    fn p_stmt(&mut self) -> Option<Node> {
        if let Some(n_expr) = self.p_expr() { return Some(n_expr); }
        None
    }

    // TODO IMCOMPLETE
    fn p_expr(&mut self) -> Option<Node> {
        if let Some(n_arg) = self.p_arg() { return Some(n_arg); }
        None
    }

    // TODO IMCOMPLETE
    fn p_arg(&mut self) -> Option<Node> {
        if let Some(n_primary) = self.p_primary() { return Some(n_primary); }
        None
    }

    // TODO INCOMPLETE
    fn p_primary(&mut self) -> Option<Node> {
        if let Some(n_literal) = self.p_numeric() { return Some(n_literal); }

        None
    }

    //  literal: numeric
    //         | symbol
    //         | dsym
    // TODO INCOMPLETE
    // TODO DUMMY
    fn p_literal(&mut self) -> Option<Node> {
        if let Some(n_numeric) = self.p_numeric() { return Some(n_numeric); }

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
    // TODO IMCOMPLETE
    fn p_numeric(&mut self) -> Option<Node> {
        if let Some(n_simple_numeric) = self.p_simple_numeric() { return Some(n_simple_numeric); }

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
}
