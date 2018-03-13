// TODO NOTE
// 
// so when the note in the parser gem says:
// 
// #  * Ragel merges actions. So, if you have `e_lparen = '(' %act` and
// #    `c_lparen = '('` and a lexer action `e_lparen | c_lparen`, the result
// #    _will_ invoke the action `act`.
// #
// #    e_something stands for "something with **e**mbedded action".
// 
// it actually means will _only_ invoke action `act`?
// 
// since, apparently the rule 
//   e_rbrace | e_rparen | ']'
//   => {
// will only invoke the action for e_rbrace, if the e_rbrace matches
// 
// TODO many rules need review
//

use std::collections::HashMap;

use parser::token::Token;

use shared::static_env::StaticEnv;

#[macro_use]
pub mod lexing_state;  use self::lexing_state::LexingState;
#[macro_use]
mod action;            use self::action::Action;
mod input_stream;      use self::input_stream::InputStream;
mod shared_actions;    use self::shared_actions::TSharedActions;
mod machines;
mod matching_patterns;
mod tokens_tables;
mod shared_functions;
mod stack_state;       use self::stack_state::StackState;
mod literal;           use self::literal::Literal;

pub struct Lexer {
    current_state: LexingState, // NOTE like the @cs somehow
    next_state: Option<LexingState>,
    // TODO NOTE simulate fcall
    calling_state: Option<LexingState>,
    // TODO NOTE simulate *stack_pop
    last_state: Option<LexingState>,
    is_breaking: bool,

    tokens_tables: HashMap<&'static str, HashMap<&'static str, Token>>,
    shared_actions: TSharedActions,
    machines: HashMap<LexingState, Vec<Box<Action>>>,

    input_stream: InputStream,

    // stack: Vec<usize>,
    // top: usize,

    pub cond: StackState,
    pub cmdarg: StackState,
    // TODO
    // cond_stack: Vec<StackState>,
    // cmdarg_stack: Vec<StackState>,

    literal_stack: Vec<Literal>,

    // TODO seems like a Ruby 1.9 thing
    paren_nest: usize,
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

    // @in_kwarg
    // # True at the end of "def foo a:"
    in_kwarg: bool,

    static_env: Option<StaticEnv>,

    pub tokens: Vec<Token>,
}

impl Lexer {
    pub fn new(input_string: String) -> Lexer {
        let shared_actions = shared_actions::construct();

        Lexer {
            current_state: LexingState::LineBegin, // NOTE setting value here is no use actually, since every time will pop one from states_stack
            next_state: None,
            calling_state: None,
            last_state: None,
            is_breaking: false,

            tokens_tables: tokens_tables::construct(),

            shared_actions: shared_actions.clone(),
            machines: machines::construct(&shared_actions),

            input_stream: InputStream::new(input_string),

            // stack: vec![],
            // top: 0,

            cond: StackState::new(),
            cmdarg: StackState::new(),

            literal_stack: vec![],

            paren_nest: 0,
            lambda_stack: vec![],

            command_state: false,

            in_kwarg: false,

            static_env: None,

            tokens: Vec::new(),
        }
    }

    // return one token
    // 
    // TODO MAYBE wrap in a Result, instead of Option
    // 
    pub fn advance(&mut self) -> Option<Token> {
        println!("--- lexer: advance ---");

        if !self.tokens.is_empty() {
            return Some(self.tokens.remove(0));
        }

        self.command_state = ( self.current_state == LexingState::ExprValue ) || 
                             ( self.current_state == LexingState::LineBegin );

        self.exec();

        if self.tokens.is_empty() {
            return None;
        } else {
            return Some( self.tokens.remove(0) );
        }
    }

    // match-state-invoke-action loop
    // 
    // exec machine until encounter break
    // 
    fn exec(&mut self) {
        self.is_breaking = false;

        loop {
            println!("\n--- exec looping, current_state: {:?}, next_state: {:?}, calling_state: {:?}, is_breaking: {:?} ---", self.current_state, self.next_state, self.calling_state, self.is_breaking);

            // handle breaking
            if self.is_breaking == true {
                println!("breaking...");
                break;
            }

            // handle state transition
            self.last_state = Some(self.current_state.clone());
            if let Some(calling_state) = self.calling_state.clone() {
                self.current_state = calling_state.clone();
                self.calling_state = None;
            } else {
                if let Some(next_state) = self.next_state.clone() {
                    self.current_state = next_state.clone();
                    self.next_state = None;
                }
            }

            // get actions
            let actions = self.machines.get(&self.current_state).unwrap().clone();

            // find matching action
            let action = self.input_stream.longest_matching_action(&actions).expect("cant match any action");
            println!("matched action: {:?}", action.regex);

            // invoke proc
            let procedure = action.procedure;
            procedure(self);
        }
    }

    // parser will use this method to set lexer's state directly
    pub fn set_state(&mut self, state: LexingState) {
        self.current_state = state;
    }

    fn flag_breaking(&mut self) {
        self.is_breaking = true;
    }

    fn set_next_state(&mut self, state: LexingState) {
        self.next_state = Some(state);
    }

    fn set_calling_state(&mut self, state: LexingState) {
        self.calling_state = Some(state);
    }

    fn emit_token(&mut self, token: Token) {
        println!(">>> emitting token: {:?}", token);

        self.tokens.push(token);
    }

    // emit current slice as token from table
    // TODO naming
    fn emit_token_from_table(&mut self, table_name: &str) {
        let token_str = self.input_stream.current_token().unwrap().clone();

        let tokens_table = self.tokens_tables.get(table_name).unwrap();
        let token = tokens_table.get(token_str.as_str()).expect(&format!("no token {} from tokens_table {}", token_str, table_name));

        println!(">>> emitting token (from table): {:?}", token);

        self.tokens.push((*token).clone());
    }

    fn invoke_proc(&mut self, proc_name: &str) {
        let procedure = self.shared_actions.get(proc_name).expect("no such proc in shared_actions").clone();
        procedure(self);
    }

}
