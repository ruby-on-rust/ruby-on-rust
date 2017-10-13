use parser::ast;

use interpreter::context::{Context};

pub struct Interpreter {
    pub context: Context,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            context: Context::new()
        }
    }

    pub fn eval_program(&mut self, program: ast::Program) {
        for stmt in program {
            self.context.eval_stmt_top_level(stmt);
        }
    }
}
