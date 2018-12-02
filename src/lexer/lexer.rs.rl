// TODO
// set starting cs as lexer_en_line_begin

// TODO
// apply #[allow(non_upper_case_globals)] for values

use std::rc::Rc;
use std::cell::RefCell;
use crate::{
    token::token::Token,
    lexer::{
        literal::Literal,
        stack_state::StackState,
        dedenter::Dedenter,
    },
    explainer
};

macro_rules! wip { () => { panic!("WIP"); }; }

macro_rules! explain {
    ( $ ( $ arg : tt ) * ) => {
        let message = format!( $($arg)* );
        explainer::explain("lexer", message);
    };
}

// TODO VENDOR using a static str as return value will cause ragel to run forever.
// https://github.com/ruby-on-rust/ruby-on-rust/issues/3
fn state_name_from_id(state_id: i32) -> String {
    if state_id == lexer_en_interp_words { return String::from("interp_words"); }
    if state_id == lexer_en_interp_string { return String::from("interp_string"); }
    if state_id == lexer_en_plain_words { return String::from("plain_words"); }
    if state_id == lexer_en_plain_string { return String::from("plain_string"); }
    if state_id == lexer_en_interp_backslash_delimited { return String::from("interp_backslash_delimited"); }
    if state_id == lexer_en_plain_backslash_delimited { return String::from("plain_backslash_delimited"); }
    if state_id == lexer_en_interp_backslash_delimited_words { return String::from("interp_backslash_delimited_words"); }
    if state_id == lexer_en_plain_backslash_delimited_words { return String::from("plain_backslash_delimited_words"); }
    if state_id == lexer_en_regexp_modifiers { return String::from("regexp_modifiers"); }
    if state_id == lexer_en_expr_variable { return String::from("expr_variable"); }
    if state_id == lexer_en_expr_fname { return String::from("expr_fname"); }
    if state_id == lexer_en_expr_endfn { return String::from("expr_endfn"); }
    if state_id == lexer_en_expr_dot { return String::from("expr_dot"); }
    if state_id == lexer_en_expr_arg { return String::from("expr_arg"); }
    if state_id == lexer_en_expr_cmdarg { return String::from("expr_cmdarg"); }
    if state_id == lexer_en_expr_endarg { return String::from("expr_endarg"); }
    if state_id == lexer_en_expr_mid { return String::from("expr_mid"); }
    if state_id == lexer_en_expr_beg { return String::from("expr_beg"); }
    if state_id == lexer_en_expr_labelarg { return String::from("expr_labelarg"); }
    if state_id == lexer_en_expr_value { return String::from("expr_value"); }
    if state_id == lexer_en_expr_end { return String::from("expr_end"); }
    if state_id == lexer_en_leading_dot { return String::from("leading_dot"); }
    if state_id == lexer_en_line_begin { return String::from("line_begin"); }
    unreachable!();
}

fn state_id_from_name(state_name: &str) -> i32 {
    match state_name {
        "interp_words" => { lexer_en_interp_words },
        "interp_string" => { lexer_en_interp_string },
        "plain_words" => { lexer_en_plain_words },
        "plain_string" => { lexer_en_plain_string },
        "interp_backslash_delimited" => { lexer_en_interp_backslash_delimited },
        "plain_backslash_delimited" => { lexer_en_plain_backslash_delimited },
        "interp_backslash_delimited_words" => { lexer_en_interp_backslash_delimited_words },
        "plain_backslash_delimited_words" => { lexer_en_plain_backslash_delimited_words },
        "regexp_modifiers" => { lexer_en_regexp_modifiers },
        "expr_variable" => { lexer_en_expr_variable },
        "expr_fname" => { lexer_en_expr_fname },
        "expr_endfn" => { lexer_en_expr_endfn },
        "expr_dot" => { lexer_en_expr_dot },
        "expr_arg" => { lexer_en_expr_arg },
        "expr_cmdarg" => { lexer_en_expr_cmdarg },
        "expr_endarg" => { lexer_en_expr_endarg },
        "expr_mid" => { lexer_en_expr_mid },
        "expr_beg" => { lexer_en_expr_beg },
        "expr_labelarg" => { lexer_en_expr_labelarg },
        "expr_value" => { lexer_en_expr_value },
        "expr_end" => { lexer_en_expr_end },
        "leading_dot" => { lexer_en_leading_dot },
        "line_begin" => { lexer_en_line_begin },
        _ => { unreachable!(); }
    }
}

%%{
    machine lexer;

    variable cs self.cs;

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
    pub cs: i32,
    p: i32,
    pe: i32,
    ts: i32,
    te: i32,
    tm: i32,
    act: i32,
    stack: [i32; 16],
    top: i32,

    pub cond: StackState,
    pub cmdarg: StackState,
    pub cond_stack: Vec<StackState>,
    pub cmdarg_stack: Vec<StackState>,

    // # Lexer state:
    // @token_queue   = []
    // @literal_stack = []
    tokens: Rc<RefCell<Vec<Token>>>,
    pub literal_stack: Vec<RefCell<Literal>>,

    // @eq_begin_s    = nil # location of last encountered =begin
    // @sharp_s       = nil # location of last encountered #

    // @newline_s     = nil # location of last encountered newline
    newline_s: Option<i32>, // TODO do we need an Option here

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

    // TODO are these 2 values ruby19 only?
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
    pub dedent_level: Option<isize>,

    // # If the lexer is in `command state' (aka expr_value)
    // # at the entry to #advance, it will transition to expr_cmdarg
    // # instead of expr_arg at certain points.
    // @command_state = false
    command_state: bool,

    // # True at the end of "def foo a:"
    // @in_kwarg      = false
    pub in_kwarg: bool,

    // # State before =begin / =end block comment
    // @cs_before_block_comment = self.class.lex_en_line_begin
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
            cond_stack: vec![],
            cmdarg_stack: vec![],

            tokens: Rc::new(RefCell::new(vec![])),
            literal_stack: vec![],

            newline_s: None,

            num_base: 0,
            num_digits_s: 0,
            num_suffix_s: 0,

            paren_nest: 0,
            lambda_stack: vec![],

            dedent_level: None,
            command_state: false,

            in_kwarg: false
        }
    }

    // TODO DOC
    // return a Token
    #[allow(unused_parens, unused_assignments, unused_variables)]
    pub fn advance(&mut self) -> Option<Token> {
        explain!("lexer:advance: current_state: {}", state_name_from_id(self.cs));

        if !self.tokens.borrow().is_empty() { return Some(self.tokens.borrow_mut().remove(0)); }

        // TODO MAJOR utf8 uncompatible
        let _input = self.input.clone();
        let data = _input.as_bytes();

        // TODO macro
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
        self.command_state = ( self.cs == lexer_en_expr_value || self.cs == lexer_en_line_begin );

        %% write exec;

        self.p = p;
        self.pe = pe;
        self.ts = ts;
        self.te = te;
        self.tm = tm;
        self.act = act;
        self.stack = stack;
        self.top = top;

        explain!("lexer:advance:advanced current state: {}", state_name_from_id(self.cs));

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
        explain!("lexer.emit: {:?}", token);
        self.tokens.borrow_mut().push(token);
    }

    // TODO NOTE
    fn arg_or_cmdarg(&self) -> i32 {
        if self.command_state { lexer_en_expr_cmdarg } else { lexer_en_expr_arg }
    }

    pub fn set_state(&mut self, state_name: &str) {
        explain!("lexer.set_state: {}", state_name);

        self.cs = state_id_from_name(state_name);
    }
}
