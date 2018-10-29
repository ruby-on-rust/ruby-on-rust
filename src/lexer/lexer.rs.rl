// TODO
// set starting cs as lexer_en_line_begin

// TODO set #[allow(non_upper_case_globals)] for generated static vars

use std::rc::Rc;
use std::cell::RefCell;
use token::token::Token;
use lexer::literal::Literal;
use lexer::stack_state::StackState;

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
    #
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
    # include "_line_comment.rs.rl";
    include "_line_begin.rs.rl";
}%%

%% write data nofinal;

pub struct Lexer {
    input: String,

    // ragel
    cs: i32,
    p: i32,
    pe: i32,
    ts: i32,
    te: i32,
    tm: i32,
    act: i32,
    stack: [i32; 16],
    top: i32,

    cond: StackState,
    cmdarg: StackState,
    // TODO
    // @cond_stack   = []
    // @cmdarg_stack = []

    // # Lexer state:
    // @token_queue   = []
    // @literal_stack = []
    tokens: Rc<RefCell<Vec<Token>>>,
    pub literal_stack: Vec<RefCell<Literal>>,

    // @eq_begin_s    = nil # location of last encountered =begin
    // @sharp_s       = nil # location of last encountered #

    // @newline_s     = nil # location of last encountered newline

    // @num_base      = nil # last numeric base
    // @num_digits_s  = nil # starting position of numeric digits
    // @num_suffix_s  = nil # starting position of numeric suffix
    // @num_xfrm      = nil # numeric suffix-induced transformation
    // TODO Do we need Optional for these values?
    num_base: usize,
    num_digits_s: i32,
    num_suffix_s: i32,

    // @escape_s      = nil # starting position of current sequence
    // @escape        = nil # last escaped sequence, as string

    // @herebody_s    = nil # starting position of current heredoc line

    // # Ruby 1.9 ->() lambdas emit a distinct token if do/{ is
    // # encountered after a matching closing parenthesis.
    // @paren_nest    = 0
    paren_nest: usize,
    // @lambda_stack  = [],
    lambda_stack: Vec<usize>,

    // # After encountering the closing line of <<~SQUIGGLY_HEREDOC,
    // # we store the indentation level and give it out to the parser
    // # on request. It is not possible to infer indentation level just
    // # from the AST because escape sequences such as `\ ` or `\t` are
    // # expanded inside the lexer, but count as non-whitespace for
    // # indentation purposes.
    // @dedent_level  = nil

    // # If the lexer is in `command state' (aka expr_value)
    // # at the entry to #advance, it will transition to expr_cmdarg
    // # instead of expr_arg at certain points.
    // @command_state = false
    command_state: bool,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        // %% write init;
        let cs = ( lexer_start ) as i32;
        let top = 0;
        let ts = 0;
        let te = 0;
        let act = 0;

        let tm = 0;
        let pe = input.len() as i32;
        let stack = [0; 16];

        Lexer {
            input,

            cs, ts, te, tm,
            stack, top,
            p: 0,
            pe,
            act,

            cond: StackState::new(),
            cmdarg: StackState::new(),

            tokens: Rc::new(RefCell::new(vec![])),
            literal_stack: vec![],

            num_base: 0,
            num_digits_s: 0,
            num_suffix_s: 0,

            paren_nest: 0,
            lambda_stack: vec![],

            command_state: false,
        }
    }

    // TODO DOC
    // return a Token
    pub fn advance(&mut self) -> Option<Token> {
        println!("---\nlexer.advance");

        if !self.tokens.borrow().is_empty() { return Some(self.tokens.borrow_mut().remove(0)); }

        // TODO MAJOR utf8 uncompatible
        let _input = self.input.clone();
        let data = _input.as_bytes();

        // TODO macro
        let mut cs = self.cs;
        let mut p = self.p;
        let mut pe = self.pe;
        let mut ts = self.ts;
        let mut te = self.te;
        let mut tm = self.tm;
        let mut act = self.act;
        let mut stack = self.stack;
        let mut top = self.top;

        // NOTE
        // pe - Data end pointer.
        // This should be initialized to p plus the data length on every run of the machine.
        // In Go, Java and Ruby code this should be initialized to the data length.
        // Seems like rust is same with ruby, since they're languages without `goto`

        let eof = self.pe;

        // @command_state = (@cs == klass.lex_en_expr_value ||
        //                   @cs == klass.lex_en_line_begin)
        self.command_state = ( cs == lexer_en_expr_value || cs == lexer_en_line_begin );

        %% write exec;

        self.cs = cs;
        self.p = p;
        self.pe = pe;
        self.ts = ts;
        self.te = te;
        self.tm = tm;
        self.act = act;
        self.stack = stack;
        self.top = top;

        if self.tokens.borrow().is_empty() {
            return None;
        } else {
            return Some(self.tokens.borrow_mut().remove(0));
        }
    }

    // TODO CRITICAL utf8 uncompatible
    fn current_slice(&self, ts: i32, te: i32) -> String {
        self.input.chars().skip(ts as usize).take( ( te - ts ) as usize ).collect()
    }

    fn current_slice_as_token_from_table(&mut self, table_name: &str, current_slice: String) -> Token {
        match table_name {
            !write token tables matching
            _ => { panic!("unreachable! no such table"); }
        }
    }

    fn emit(&mut self, token: Token) {
        println!("lexer.emit: {:?}", token);
        self.tokens.borrow_mut().push(token);
    }

    // TODO NOTE
    fn arg_or_cmdarg(&self) -> i32 {
        if self.command_state { lexer_en_expr_cmdarg } else { lexer_en_expr_arg }
    }
}
