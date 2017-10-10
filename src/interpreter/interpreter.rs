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

    pub fn eval_stmts(&mut self, stmts: ast::Stmts) {
        for stmt in stmts {
            self.context.eval_stmt(stmt);
        }
    }
}
