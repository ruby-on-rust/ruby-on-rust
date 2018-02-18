pub struct StackState {
    stack: Vec<bool>,
}

impl StackState {
    pub fn new() -> StackState {
        StackState {
            stack: vec![]
        }
    }

    pub fn push(&mut self, state: bool) {
        self.stack.push(state);
    }

    pub fn pop(&mut self) -> bool {
        if self.stack.is_empty() {
            false
        } else {
            self.stack.pop().unwrap()
        }
    }
}
