use parser::ast;

pub type Refer = Option<u64>;

// TODO make make Value a struct
// TODO implement `primitive`
// TODO implement objects
#[derive(Debug)]
pub enum Value {
    Number(i64)
}

// TODO implement struct Memory
#[derive(Default)]
pub struct Interpreter {
    // TODO REVISIT only make `pub` for `Default` to work
    pub memory: Vec<Value>
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
            ast::Expr::Binary(l, op, r) => {
                return self.eval_expr_binary(*l, op, *r)
            },
            ast::Expr::Assignment(var_name, expr) => {
                unimplemented!()
                // self.eval_expr_assignment(var_name, *expr)
            },
            _ => {
                unimplemented!()
            }
        }
    }

    // TODO NOTE this is 
    pub fn eval_2_exprs(&mut self, expr_1: ast::Expr, expr_2: ast::Expr) -> (Refer, Refer) {
        (self.eval_expr(expr_1), self.eval_expr(expr_2))
    }

    // pub fn eval_expr_assignment(&mut self, var_name: String, expr: ast::Expr) -> Refer {
    //     let evaled_value_refer = self.eval_expr(expr);
    //     let evaled_value_ref = self.get_value_of_refer(evaled_value_refer);
    // }
}
