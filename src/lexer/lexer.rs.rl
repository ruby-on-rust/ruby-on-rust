use lexer::input::Input;

%%{
    machine lexer;

    line_begin := |*
        any;

        #any
        #=> { fhold; fgoto expr_value; };

        # c_eof => do_eof;
    *|;
}%%

%% write data nofinal;

pub struct Lexer {
    input: Input,

    // for ragel
    cs: i32,
    p: i32,
    pe: i32,
    ts: i32,
    te: i32,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let input = Input::new(input);

        let cs;
        let ts;
        let te;

        %% write init;

        Lexer {
            input,
            cs, ts, te,
            p: 0,
            pe: 0
        }
    }

    pub fn advance(&mut self) {
        let data = self.input.clone();

        // TODO macro
        let mut cs = self.cs;
        let mut p = self.p;
        let mut pe = self.pe;
        let mut ts = self.ts;
        let mut te = self.te;

        %% write exec;

        self.cs = cs;
        self.p = p;
        self.pe = pe;
        self.ts = ts;
        self.te = te;
    }
}
