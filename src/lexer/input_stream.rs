use regex::Regex;

use lexer::action::Action;

pub struct InputStream {
    string: String,
    current_pos: usize
}

impl InputStream {
    pub fn new(string: String) -> InputStream {
        InputStream {
            string,
            current_pos: 0
        }
    }

    // starting from pos
    pub fn longest_matching_action(&mut self, actions: &Vec<Box<Action>>) -> Option<Box<Action>> {

        println!("finding longest matching action...");

        // TODO not that elegant, use Option<Action> instead of
        let mut longest_matched_action_i: isize= -1;
        let mut longest_matched_action_len = 0;
        for (i, action) in actions.iter().enumerate() {

            println!("matching action with regex {:?}", &action.regex);

            match self.match_action_starting_from_pos(&action.regex) {
                None => {},
                Some(len) => {

                    println!("matched something with length: {}", len);

                    if ( len > longest_matched_action_len ) {
                        longest_matched_action_len = len;
                        longest_matched_action_i = ( i as isize );
                    }
                }
            };
        };

        println!("longest_matched_action_len: {}", longest_matched_action_len);
        println!("longest_matched_action_i: {}", longest_matched_action_i);

        match longest_matched_action_i {
            -1 => { None },
            i => { Some(actions.get(i as usize).unwrap().clone()) }
        }
    }

    // return matched length, starting from 1
    fn match_action_starting_from_pos(&mut self, regex: &Regex) -> Option<usize> {

        println!("matching action starting from pos");

        // TODO NOTE
        let sliced_string: String = self.string.char_indices().filter(|&(i, _)| i >= self.current_pos ).map(|(_, e)| e).collect();

        println!("current sliced string: {}", sliced_string);

        let captures = regex.captures(&sliced_string);
        match captures {
            None => None,
            Some(capture) => Some(capture.len())
        }
    }
}
