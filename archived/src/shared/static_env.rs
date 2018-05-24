// TODO
// move into parser

// module Parser

//   class StaticEnvironment
//     def extend_static
//       @stack.push(@variables)
//       @variables = Set[]

//       self
//     end

//     def extend_dynamic
//       @stack.push(@variables)
//       @variables = @variables.dup

//       self
//     end

//     def unextend
//       @variables = @stack.pop

//       self
//     end

//     def declare(name)
//       @variables.add(name.to_sym)

//       self
//     end
//   end
// end

use std::collections::HashSet;

//       @variables = Set[]
//       @stack     = []
pub struct StaticEnv {
    variables: HashSet<String>,
    stack: Vec<HashSet<String>>,
}

impl StaticEnv {
    pub fn new() -> StaticEnv {
        StaticEnv {
            variables: HashSet::new(),
            stack: vec![],
        }
    }

    //     def declared?(name)
    //       @variables.include?(name.to_sym)
    //     end
    pub fn has_declared(&self, name: String) -> bool {
        self.variables.contains(&name)
    }
}
