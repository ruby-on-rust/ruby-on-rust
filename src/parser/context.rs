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

macro_rules! wip { () => { panic!("WIP"); }; }

#[derive(PartialEq, Copy, Clone)]
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
        // TODO from string via strum
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
        match self.stack.last() {
            Some(ScopeStateKind::Class) => true,
            _ => false
        }
    }

    // def indirectly_in_def?
    //     @stack.include?(:def) || @stack.include?(:defs)
    //   end
    // end
    pub fn is_indirectly_in_def(&self) -> bool {
        wip!();
    }

    // def class_definition_allowed?
    //   def_index = stack.rindex { |item| [:def, :defs].include?(item) }
    //   sclass_index = stack.rindex(:sclass)
    //   def_index.nil? || (!sclass_index.nil? && sclass_index > def_index)
    // end
    pub fn is_class_definition_allowed(&self) -> bool {
        let def_index = self.stack.iter().rposition(|&scope| scope == ScopeStateKind::Def || scope == ScopeStateKind::Defs );
        let sclass_index = self.stack.iter().rposition(|&scope| scope == ScopeStateKind::SClass );
        def_index.is_none() || ( !sclass_index.is_none() && sclass_index > def_index )
    }

    // alias module_definition_allowed? class_definition_allowed?
    pub fn is_module_definition_allowed(&self) -> bool { self.is_class_definition_allowed() }

    // alias dynamic_const_definition_allowed? class_definition_allowed?
    pub fn is_dynamic_const_definition_allowed(&self) -> bool { self.is_class_definition_allowed() }
}
