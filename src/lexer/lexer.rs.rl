// TODO
// set starting cs as lexer_en_line_begin

use lexer::input::Input;
use token::token::Token;

%%{
    machine lexer;

    include "_character_classes.rs.rl";
    include "_token_definitions.rs.rl";
    include "_numeric.rs.rl";
    include "_escape_sequence.rs.rl";
    include "_string_and_heredoc.rs.rl";
    include "_interpolation.rs.rl";
    include "_whitespace.rs.rl";
    include "_expression.rs.rl";

    include "_expr_variable.rs.rl";
    include "_expr_fname.rs.rl";
    include "_expr_endfn.rs.rl";
    include "_expr_dot.rs.rl";
    include "_expr_arg.rs.rl";
    include "_expr_cmdarg.rs.rl";
    include "_expr_endarg.rs.rl";
    include "_expr_mid.rs.rl";
    include "_expr_beg.rs.rl";
    include "_expr_labelarg.rs.rl";
    include "_expr_value.rs.rl";
    include "_expr_end.rs.rl";
    include "_leading_dot.rs.rl";
    include "_line_comment.rs.rl";

    # TODO
    line_begin := |*
        w_any;

        # '=begin' ( c_space | c_nl_zlen )
        # => {
        #     @eq_begin_s = @ts
        #     fgoto line_comment;
        # };

        # '__END__' ( c_eol - zlen )
        # => { p = pe - 3 };

        c_any
        => { fhold; fgoto expr_value; };

        # c_eof => do_eof;
    *|;
}%%

%% write data nofinal;

pub struct Lexer {
    input: Input,

    tokens: Vec<Token>,

    // for ragel
    cs: i32,
    p: i32,
    pe: i32,
    ts: i32,
    te: i32,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let pe = input.len(); // TODO this would be wrong with UTF8 chars
        let input = Input::new(input);

        let cs;
        let ts;
        let te;

        %% write init;

        Lexer {
            input,
            tokens: vec![],
            cs, ts, te,
            p: 0,
            pe: pe as i32,
        }
    }

    // TODO DOC
    // return a Token
    pub fn advance(&mut self) -> Token {
        println!("---\nlexer.advance");

        if !self.tokens.is_empty() { return self.tokens.remove(0); }

        let data = self.input.clone();

        // TODO macro
        let mut cs = self.cs;
        let mut p = self.p;
        let mut pe = self.pe;
        let mut ts = self.ts;
        let mut te = self.te;

        // 
        let eof = self.pe;

        %% write exec;

        self.cs = cs;
        self.p = p;
        self.pe = pe;
        self.ts = ts;
        self.te = te;

        if !self.tokens.is_empty() {
            return self.tokens.remove(0);
        } else {
            panic!("toimpl");
        }
    }

    fn emit(&mut self, token: Token) {
        println!("lexer.emit: {:?}", token);
        self.tokens.push(token);
    }
}
