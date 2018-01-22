pub struct InputStream {
    chars: Vec<char>,
    current_pos: Option<usize>
}

impl InputStream {
    pub fn new(chars: Vec<char>) -> InputStream {
        InputStream {
            chars,
            current_pos: None
        }
    }

    pub fn current(&self) -> char {
        let c = self.chars[self.current_pos.unwrap()];

        // println!("InputStream: getting current lexing char {}({}) at pos {}", c, c.escape_unicode(), (self.current_pos.unwrap()));

        c
    }

    pub fn next(&mut self) -> Option<char> {
        // update current pos
        match self.current_pos {
            None => { self.current_pos = Some(0); }
            Some(pos) => {
                self.current_pos = Some(pos + 1);
            }
        };

        // println!("during next, new pos: {}", self.current_pos.unwrap());
        // println!("{}", self.chars.len());

        if self.current_pos.unwrap() >= self.chars.len() {
            return None
        } else {
            return Some(self.current())
        }
    }

    pub fn put_back(&mut self, char: char) {
        self.chars.push(char);

        self.current_pos = Some(self.current_pos.unwrap() - 1);
    }
}
