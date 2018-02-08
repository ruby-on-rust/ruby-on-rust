use std::collections::HashMap;

use parser::parser::Token;

mod input_stream;      use lexer::input_stream::InputStream;
mod lexing_state;      use lexer::lexing_state::{LexingState};
mod shared_actions;
mod transactions;
mod action;            use lexer::action::{Action};
mod matching_patterns;

pub struct Lexer {
    // TODO CONSTant
    transactions: HashMap<LexingState, Vec<Box<Action>>>,

    input_stream: InputStream,
    state: LexingState,

    is_breaking: bool,

    pub tokens: Vec<Token>,
}

impl Lexer {
    pub fn new(input_string: String) -> Lexer {
        Lexer {
            state: LexingState::LineBegin,
            transactions: transactions::construct(),
            is_breaking: false,

            input_stream: InputStream::new(input_string),

            tokens: Vec::new(),
        }
    }

    // TODO return Result
    pub fn lex(&mut self) {
        loop {
            // TODO advance and advance and advance
            self.advance();
        }
    }

    // return a token
    // 
    // TODO
    // then the current `emit` is not correct
    // every `exec()` should emit a token
    // 
    // TODO wrap in a Result
    // 

    fn advance(&mut self) {
        // TODO token queue

        // println!("--- advance ---");

        // TODO HACKING NOT WORKING not the correct way
        if (self.input_stream.no_more()) {
            println!("no more..., breaking...");
            return;
        }

        // 
        self.exec();
    }

    // match-state-invoke-action loop
    // 
    // exec machine until encounter break
    // 
    fn exec(&mut self) {
        self.is_breaking = false;

        loop {
            println!("\n--- exec looping, state: {:?} ---", self.state);

            if ( self.is_breaking == true ) {
                println!("breaking...");
                break;
            }

            // ===

            // get actions
            let actions = &self.transactions.get(&self.state).unwrap().clone();

            // find matching action
            let action= self.input_stream.longest_matching_action(&actions).expect("cant match any action");
            println!("matched action: {:?}", action.regex);

            // invoke proc
            let procedure = action.procedure;
            procedure(self);
        }
    }

    fn flag_breaking(&mut self) {
        self.is_breaking = true;
    }

    fn emit_token(&mut self, token: Token) {
        println!("emitting token: {:?}", token);

        self.tokens.push(token);
    }

    // 
    // mapping tables of strings literal -> token
    // 
    // 
    // PUNCTUATION = {
    //     '='   => :tEQL,     '&'   => :tAMPER2,  '|'   => :tPIPE,
    //     '!'   => :tBANG,    '^'   => :tCARET,   '+'   => :tPLUS,
    //     '-'   => :tMINUS,   '*'   => :tSTAR2,   '/'   => :tDIVIDE,
    //     '%'   => :tPERCENT, '~'   => :tTILDE,   ','   => :tCOMMA,
    //     ';'   => :tSEMI,    '.'   => :tDOT,     '..'  => :tDOT2,
    //     '...' => :tDOT3,    '['   => :tLBRACK2, ']'   => :tRBRACK,
    //     '('   => :tLPAREN2, ')'   => :tRPAREN,  '?'   => :tEH,
    //     ':'   => :tCOLON,   '&&'  => :tANDOP,   '||'  => :tOROP,
    //     '-@'  => :tUMINUS,  '+@'  => :tUPLUS,   '~@'  => :tTILDE,
    //     '**'  => :tPOW,     '->'  => :tLAMBDA,  '=~'  => :tMATCH,
    //     '!~'  => :tNMATCH,  '=='  => :tEQ,      '!='  => :tNEQ,
    //     '>'   => :tGT,      '>>'  => :tRSHFT,   '>='  => :tGEQ,
    //     '<'   => :tLT,      '<<'  => :tLSHFT,   '<='  => :tLEQ,
    //     '=>'  => :tASSOC,   '::'  => :tCOLON2,  '===' => :tEQQ,
    //     '<=>' => :tCMP,     '[]'  => :tAREF,    '[]=' => :tASET,
    //     '{'   => :tLCURLY,  '}'   => :tRCURLY,  '`'   => :tBACK_REF2,
    //     '!@'  => :tBANG,    '&.'  => :tANDDOT,
    // }

    // PUNCTUATION_BEGIN = {
    //     '&'   => :tAMPER,   '*'   => :tSTAR,    '**'  => :tDSTAR,
    //     '+'   => :tUPLUS,   '-'   => :tUMINUS,  '::'  => :tCOLON3,
    //     '('   => :tLPAREN,  '{'   => :tLBRACE,  '['   => :tLBRACK,
    // }

    // KEYWORDS = {
    //     'if'     => :kIF_MOD,      'unless'   => :kUNLESS_MOD,
    //     'while'  => :kWHILE_MOD,   'until'    => :kUNTIL_MOD,
    //     'rescue' => :kRESCUE_MOD,  'defined?' => :kDEFINED,
    //     'BEGIN'  => :klBEGIN,      'END'      => :klEND,
    // }

    // KEYWORDS_BEGIN = {
    //     'if'     => :kIF,          'unless'   => :kUNLESS,
    //     'while'  => :kWHILE,       'until'    => :kUNTIL,
    //     'rescue' => :kRESCUE,      'defined?' => :kDEFINED,
    // }

    // %w(class module def undef begin end then elsif else ensure case when
    //     for break next redo retry in do return yield super self nil true
    //     false and or not alias __FILE__ __LINE__ __ENCODING__).each do |keyword|
    //     KEYWORDS_BEGIN[keyword] = KEYWORDS[keyword] = :"k#{keyword.upcase}"
    // end

    fn emit_token_from_table(&mut self, table_name: &str) {
        // TODO const, separate

        let keywords: HashMap<&'static str, Token> = vec![
            ( "true", Token::K_TRUE ),
        ].into_iter().collect();

        let tables: HashMap<&'static str, HashMap<&str, Token>> = vec![
            ( "keywords", keywords ),
        ].into_iter().collect();

        // ---
        let token_str = self.input_stream.current_matched_token().unwrap().clone();
        let token = tables.get(table_name).unwrap().get(token_str.as_str()).unwrap();

        self.emit_token(*token);
    }
}
