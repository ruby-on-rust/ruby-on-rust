use regex::Regex;

use lexer::action::Action;

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
    p: usize,
    ts: Option<usize>,
    te: Option<usize>,
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

        println!("finding longest matching action... current p {}", self.p);

        // TODO not that elegant, use Option<Action> instead of
        let mut longest_matched_action_i: Option<usize> = None;
        let mut longest_matched_action_len = 0;
        for (i, action) in actions.iter().enumerate() {

            println!("matching action with regex {:?}", &action.regex);

            match self.match_action_starting_from_pos(&action.regex) {
                None => {},
                Some(len) => {

                    println!("matched something with length: {}", len);

                    if ( len > longest_matched_action_len ) {
                        longest_matched_action_len = len;
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
                self.p += longest_matched_action_len;
                self.te = Some(self.p);

                println!("matched token: {:?}", self.current_matched_token() );

                Some(actions.get(i).unwrap().clone())
            }
        }
    }

    pub fn current_matched_token(&self) -> Option<String> {
        match ( self.ts, self.te ) {
            ( Some(ts), Some(te) ) => {
                Some( self.string.chars().skip(ts).take(te - ts).collect() )
            },
            _ => None
        }
    }

    // TODO refine naming
    pub fn simulate_fhold(&mut self) {
        println!("invoking cmd fhold");

        // NOTE assume original p is never 0
        self.p -= 1;
    }

    // basically
    // p = @ts - 1;
    pub fn hold_current_token(&mut self) {
        println!("invoking cmd hold_current_token");

        match ( self.ts, self.te ) {
            ( Some(ts), Some(te) ) => {
                if ts == 0 {
                    self.p = 0;
                }
                else {
                    self.p = ts - 1;
                }
            },
            _ => {
                println!("    no current token");
            }
        }
    }

    // return matched length, starting from 1
    fn match_action_starting_from_pos(&mut self, regex: &Regex) -> Option<usize> {

        println!("    matching action starting from pos");

        let sliced_string: String = self.string.chars().skip(self.p).collect();

        println!("    current sliced string: {}", sliced_string);

        let captures = regex.captures(&sliced_string);
        match captures {
            None => None,
            Some(capture) => Some(capture.len())
        }

    }

}
