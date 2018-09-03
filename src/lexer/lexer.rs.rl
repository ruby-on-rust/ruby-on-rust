// TODO
// set starting cs as lexer_en_line_begin

use lexer::input::Input;
use token::token::Token;

%%{
    machine lexer;

    line_begin := |*
        digit+
        => {
            self.emit(Token::T_INTEGER(1));
        };
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
