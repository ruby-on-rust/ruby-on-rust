use std::collections::HashSet;

macro_rules! wip { () => { panic!("WIP"); }; }

pub struct StaticEnv {
    variables: HashSet<String>,
    stack: Vec<HashSet<String>>,
}

impl StaticEnv {
    //     def reset
    //       @variables = Set[]
    //       @stack     = []
    //     end
    pub fn new() -> StaticEnv {
        StaticEnv {
            variables: HashSet::new(),
            stack: vec![],
        }
    }

    //     def extend_static
    //       @stack.push(@variables)
    //       @variables = Set[]
    // 
    //       self
    //     end
    // TODO NOTE
    pub fn extend_static(&mut self) {
        wip!();
        // self.stack.push();
    }

    //     def extend_dynamic
    //       @stack.push(@variables)
    //       @variables = @variables.dup
    // 
    //       self
    //     end

    //     def unextend
    //       @variables = @stack.pop
    // 
    //       self
    //     end

    //     def declare(name)
    //       @variables.add(name.to_sym)
    // 
    //       self
    //     end
    pub fn declare(&mut self, name: String) {
        wip!();
    }

    //     def declared?(name)
    //       @variables.include?(name.to_sym)
    //     end
    pub fn has_declared(&self, name: String) -> bool {
        self.variables.contains(&name)
    }
}
