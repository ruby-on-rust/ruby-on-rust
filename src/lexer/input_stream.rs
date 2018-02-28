use regex::Regex;

use lexer::action::Action;

use parser::parser::TokenString;

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
    pub p: usize,
    pub ts: Option<usize>,
    pub te: Option<usize>,
}

impl InputStream {
    pub fn new(string: String) -> InputStream {
        InputStream {
            string,

            p: 0,
            ts: None,
            te: None,
        }
    }

    // starting from pos
    pub fn longest_matching_action(&mut self, actions: &Vec<Box<Action>>) -> Option<Box<Action>> {

        println!("finding longest matching action..., current p: {}", self.p);

        // TODO not that elegant, use Option<Action> instead of
        let mut longest_matched_action_i: Option<usize> = None;
        let mut longest_matched_action_len = -1; // -1 since there will be matching result with len 0 (c_eof)
        for (i, action) in actions.iter().enumerate() {

            // println!("matching action with regex {:?}", &action.regex);

            match self.match_action_starting_from_pos(&action.regex) {
                None => {},
                Some(len) => {
                    let len = len as isize;

                    // println!("matched something with length: {}", len);

                    if ( len > longest_matched_action_len ) {
                        longest_matched_action_len = len as isize;
                        longest_matched_action_i = Some(i);
                    }
                }
            };
        };

        println!("longest_matched_action_len: {}", longest_matched_action_len);
        println!("longest_matched_action_i: {:?}", longest_matched_action_i);

        match longest_matched_action_i {
            None => { None },
            Some(i) => {
                // update p, ts, te
                self.ts = Some(self.p);
                self.p += longest_matched_action_len as usize;
                self.te = Some(self.p);

                println!("matched token: {:?}", self.current_token() );

                Some(actions.get(i).unwrap().clone())
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
        // println!("\n>>> invoking hold_current_token");

        match ( self.ts, self.te ) {
            ( Some(ts), Some(te) ) => {
                self.p = ts;
            },
            _ => {
                // println!("    no current token");
            }
        }
    }

    // return matched length, starting from 1
    fn match_action_starting_from_pos(&mut self, regex: &Regex) -> Option<usize> {

        // println!("    matching action starting from pos");

        let sliced_string: String = self.string.chars().skip(self.p).collect();

        // println!("    current sliced string: {}, (len: {})\n", sliced_string, sliced_string.len());

        let captures = regex.captures(&sliced_string);
        match captures {
            None => None,
            Some(capture) => {
                let match_ = capture.get(0).unwrap();
                let matched_str = String::from(match_.as_str());
                // println!("    matched str: {:?}", matched_str);
                // println!("    DEBUGGING CAPTURE: capture: {:?}", capture);
                Some(matched_str.len())
            }
        }

    }

    pub fn no_more(&self) -> bool {
        self.p >= self.string.len()
    }
}
