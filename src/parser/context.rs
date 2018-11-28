//   # Context of parsing that is represented by a stack of scopes.
//   #
//   # Supported states:
//   # + :class - in the class body (class A; end)
//   # + :sclass - in the singleton class body (class << obj; end)
//   # + :def - in the method body (def m; end)
//   # + :defs - in the singleton method body (def self.m; end)
//   # + :block - in the block body (tap {})
//   # + :lambda - in the lambda body (-> {})
//   #

enum ScopeStateKind {
    Class, SClass, Def, Defs, Block, Lambda
}

pub struct Context {
    stack: Vec<ScopeStateKind>,
}

impl Context {
    //     def initialize
    //       @stack = []
    //       freeze
    //     end
    pub fn new() -> Context {
        Context {
            stack: vec![],
        }
    }

    //     def push(state)
    //       @stack << state
    //     end
    pub fn push(&mut self, state: &str) {
        let state_kind = match state {
            "class" => ScopeStateKind::Class,
            "sclass" => ScopeStateKind::SClass,
            "def" => ScopeStateKind::Def,
            "defs" => ScopeStateKind::Defs,
            "block" => ScopeStateKind::Block,
            "lambda" => ScopeStateKind::Lambda,
            _ => { panic!("unknown type"); }
        };
        self.stack.push(state_kind);
    }

    //     def pop
    //       @stack.pop
    //     end
    pub fn pop(&mut self) {
        self.stack.pop();
    }

    //     def reset
    //       @stack.clear
    //     end

    //     def in_class?
    //       @stack.last == :class
    //     end
    pub fn is_in_class(&self) -> bool {
        match self.stack.last().unwrap() {
            ScopeStateKind::Class => true,
            _ => false
        }
    }

    //     def indirectly_in_def?
    //       @stack.include?(:def) || @stack.include?(:defs)
    //     end
    //   end

    // def class_definition_allowed?
    //   def_index = stack.rindex { |item| [:def, :defs].include?(item) }
    //   sclass_index = stack.rindex(:sclass)
    //    def_index.nil? || (!sclass_index.nil? && sclass_index > def_index)
    // end
    // alias module_definition_allowed? class_definition_allowed?
    // alias dynamic_const_definition_allowed? class_definition_allowed?
}
