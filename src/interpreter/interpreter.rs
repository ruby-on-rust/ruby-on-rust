use std::collections::HashMap;

use parser::ast;

use interpreter::memory::{Refer, Value};
use interpreter::scope::{Var};

// TODO implement struct Memory
#[derive(Default)]
pub struct Interpreter {
    // TODO REVISIT only make `pub` for `Default` to work
    pub memory: Vec<Value>,
    pub vars: HashMap<String, Var>
}

impl Interpreter {
    pub fn eval_line(&mut self, expr: ast::Expr) {
        let value = self.eval_expr(expr);
        println!("evaluated value: {:?}", value);
    }

    pub fn eval_expr(&mut self, expr: ast::Expr) -> Refer {
        match expr {
            ast::Expr::Number(n) => {
                return self.memory_allocate(Value::Number(n))
            },

            ast::Expr::Identifier(var_name) => {
                return self.get_var_refer(var_name)
            },

            ast::Expr::Binary(l, op, r) => {
                return self.eval_expr_binary(*l, op, *r)
            },

            ast::Expr::Assignment(var_name, expr) => {
                return self.eval_expr_assignment(var_name, *expr)
            },

            _ => {
                unimplemented!()
            }
        }
    }

    // TODO NOTE
    // NOTE since we cant simply do this in rust, we use a `eval_2_exprs`
    // TODO may be improved
    pub fn eval_2_exprs(&mut self, expr_1: ast::Expr, expr_2: ast::Expr) -> (Refer, Refer) {
        (self.eval_expr(expr_1), self.eval_expr(expr_2))
    }

    pub fn eval_expr_assignment(&mut self, var_name: String, expr: ast::Expr) -> Refer {
        let evaled_value_refer = self.eval_expr(expr);

        self.assign_var(var_name, evaled_value_refer);

        return evaled_value_refer
    }
}
