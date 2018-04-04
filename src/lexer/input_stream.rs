use regex::Regex;

use lexer::action::Action;

use parser::token::TokenString;

pub struct InputStream {
    string: String,

    // ripped from whitequark/parser
    // 
    // #  * If your input is `foooooooobar` and the rule is:
    // #
    // #       'f' 'o'+
    // #
    // #    the result will be:
    // #
    // #       foooooooobar
    // #       ^ ts=0   ^ p=te=9
    // #
    // 
    // TODO refine naming
    // TODO use a range for ts..te
    // 
    // TODO NOTE p, ts, te are actually more complex
    // 
    pub p: isize,
    pub ts: Option<usize>,
    pub te: Option<usize>,
    pub tm: usize,

    // TODO NOTE
    pub entering_machine: bool,
}

impl InputStream {
    pub fn new(string: String) -> InputStream {
        InputStream {
            string,

            p: 0,
            ts: None,
            te: None,
            tm: 0,

            entering_machine: true,
        }
    }

    // starting from pos
    pub fn longest_matching_action(&mut self, actions: &Vec<Box<Action>>) -> Option<Box<Action>> {

        println!("finding longest matching action..., current p: {}", self.p);
        // println!("actions: {:?}", actions);

        // TODO not that elegant, use Option<Action> instead of
        let mut longest_matched_action_i: Option<usize> = None;
        let mut longest_matched_action_len = -1; // init as -1, since there will be matching result with len 0 (c_eof)

        let starting_pos = ( if self.entering_machine { self.p } else { self.p + 1 } ) as usize;
        let sliced_string: String = self.string.chars().skip(starting_pos).collect();

        for (i, action) in actions.iter().enumerate() {

            // println!("matching action with regex {:?}", &action.regex);

            match self.match_action_starting_from_pos(&sliced_string, &action.regex) {
                None => {},
                Some(len) => {
                    let len = len as isize;

                    // println!("matched something, length: {}, regex: {:?}", len, &action.regex);

                    if ( len > longest_matched_action_len ) {
                        longest_matched_action_len = len as isize;
                        longest_matched_action_i = Some(i);
                    }
                }
            };
        };

        println!("longest_matched_action_len: {}", longest_matched_action_len);
        // println!("longest_matched_action_i: {:?}", longest_matched_action_i);

        match longest_matched_action_i {
            None => { return None; },
            Some(i) => {
                // update p, ts, te
                if self.entering_machine {
                    self.ts = Some(self.p as usize);
                    self.p += longest_matched_action_len - 1;
                    self.te = Some((self.p + 1) as usize);
                } else {
                    self.ts = Some((self.p + 1) as usize);
                    self.p += longest_matched_action_len;
                    self.te = Some((self.p + 1) as usize);
                }

                println!("matched token: {:?}", self.current_token() );
                // println!("current ts {} p {} te {}", self.ts.unwrap(), self.p, self.te.unwrap() );

                return Some(actions.get(i).unwrap().clone());
            }
        }
    }

    // TODO maybe dont need to use a Option for current_token, ts, te
    // TODO renaming current_token -> current_slice
    pub fn current_token(&self) -> Option<String> {
        match ( self.ts, self.te ) {
            ( Some(ts), Some(te) ) => {
                Some( self.string.chars().skip(ts).take(te - ts).collect() )
            },
            _ => None
        }
    }

    // TODO renaming current_slice_as_token_string
    pub fn current_token_string(&self) -> TokenString {
        TokenString::from(self.current_token().unwrap())
    }

    pub fn slice_from_range(&self, start: usize, end: usize) -> String {
        self.string.chars().skip(start).take(end - start).collect()
    }

    pub fn token_string_from_range(&self, ts: usize, te: usize) -> TokenString {
        TokenString::from(self.slice_from_range(ts, te))
    }

    // NOTE fhold in ragel
    // TODO refine
    pub fn hold_current_char(&mut self) {
        // println!("\n>>> invoking fhold");

        // println!("=== debugging fhold invoking: p {:?} ts {:?} te {:?}", self.p, self.ts, self.te );

        // NOTE assuming original p is never 0
        self.p -= 1;

        // println!("=== debugging fhold invoking: p {:?} ts {:?} te {:?}", self.p, self.ts, self.te );
    }

    // TODO
    // 
    // NOTE
    // 
    // for some reason
    // original `p = @ts - 1;`
    // 
    // still not sure about usage of `p` and `@p` in original `lexer.rl`
    // 
    // we're using p = @ts for now
    // 
    // TODO renaming hold_current_slice

    pub fn hold_current_token(&mut self) {
        // println!(">>> hold_current_token invoking, p {:?} ts {:?} te {:?}", self.p, self.ts, self.te);

        match ( self.ts, self.te ) {
            ( Some(ts), Some(te) ) => { self.p = ( ts as isize ) - 1; }
            _ => { panic!("can't hold current slice"); }
        }
    }

    fn match_action_starting_from_pos(&mut self, current_slice: &String, regex: &Regex) -> Option<usize> {

        // println!("\n===\n    matching action starting from pos");

        // println!("    current entering machine: {}", self.entering_machine);
        // println!("    current starting pos: {}", starting_pos);
        // println!("    current sliced string: {}, (len: {})", sliced_string, sliced_string.len());
        // println!("    regex: {:?}", regex);

        let captures = regex.captures(current_slice);
        match captures {
            None => None,
            Some(capture) => {
                let match_ = capture.get(0).unwrap();
                let matched_str = String::from(match_.as_str());
                // println!("    ***** matched str: {:?}", matched_str);
                // println!("    DEBUGGING CAPTURE: capture: {:?}", capture);
                return Some(matched_str.len());
            }
        }

    }

}
