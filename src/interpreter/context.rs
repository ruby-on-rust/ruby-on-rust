use parser::ast;
use interpreter::object::{Object, Primitive};
use interpreter::scope::{Scope, LocalVar};
use interpreter::memory::{Refer, Memory};

pub struct Context {
    pub memory: Memory,
    pub scopes: Vec<Scope>,
    pub self_refer: Refer
    // TODO
    // pub classes: Classes
}

impl Context {
    pub fn new() -> Context {
        Context {
            memory: Memory::new(),
            scopes: vec![Scope::new()],
            self_refer: None
        }
    }

    pub fn eval_stmt_top_level(&mut self, stmt: ast::StmtTopLevel) {
        println!("### eval_stmt");

        match stmt {
            ast::StmtTopLevel::Expr(expr) => {
                println!("evaluating line as *expression*");
                let evaled_value_refer = self.eval_expr(expr);
                println!("evaluated value: {:?}", evaled_value_refer);
            },
            ast::StmtTopLevel::ClassDefinition(class_def) => {
                unimplemented!()
                // TODO
                // println!("evaluating line as *class definition*");
                // self.classes.def_class(class_def);
            },
            ast::StmtTopLevel::MethodDefinition(method_def) => {
                println!("evaluating line as *method definition*");
                self.def_method(method_def);
            },
        }
    }

    pub fn eval_stmt_within_method(&mut self, stmt: ast::StmtWithinMethod) {
        println!("### eval_stmt");

        match stmt {
            ast::StmtWithinMethod::Expr(expr) => {
                println!("evaluating line as *expression*");
                let evaled_value_refer = self.eval_expr(expr);
                println!("evaluated value: {:?}", evaled_value_refer);
            },
        }
    }

    pub fn eval_expr(&mut self, expr: ast::Expr) -> Refer {
        println!("### eval_expr: {:?}", expr);

        // TODO debugging
        match expr {
            ast::Expr::Number(n) => {
                return self.memory.allocate_primitive(Primitive::Number(n))
            },

            // TODO distinct localvar, etc
            ast::Expr::Identifier(var_name) => {
                return self.fetch_var(var_name)
            },

            ast::Expr::Binary(l, op, r) => {
                return self.eval_expr_binary(*l, op, *r)
            },

            ast::Expr::Assignment(var_name, expr) => {
                return self.eval_expr_assignment(var_name, *expr)
            },

            // _ => {
            //     unimplemented!()
            // }
        }
    }

    // TODO NOTE
    // NOTE since we cant simply do this in rust, we use a `eval_2_exprs`
    // TODO may be improved
    // TODO separate to ... expression.rs?
    pub fn eval_2_exprs(&mut self, expr_1: ast::Expr, expr_2: ast::Expr) -> (Refer, Refer) {
        (self.eval_expr(expr_1), self.eval_expr(expr_2))
    }

    pub fn eval_expr_assignment(&mut self, var_name: String, expr: ast::Expr) -> Refer {
        println!("### eval_expr_assignment: {:?} <- {:?}", var_name, expr);

        let evaled_value_refer = self.eval_expr(expr);

        self.assign_var(var_name, evaled_value_refer);

        return evaled_value_refer
    }

    // TODO
    // distinct localvar, global var, etc, parser-wise
    // return a Result
    pub fn fetch_var(&mut self, var_name: String) -> Refer {
        // TODO assuming local var
        self.current_scope().fetch_local_var_refer(var_name)
    }

    // TODO
    // distinct localvar, global var, etc
    // return a Result
    pub fn assign_var(&mut self, var_name: String, refer: Refer) {
        // TODO assuming local var
        self.current_scope().assign_local_var(var_name, refer);
    }

    // TODO
    // refine
    pub fn def_method(&mut self, method_def: ast::MethodDefinition) {
        unimplemented!();
    }

    fn current_scope(&mut self) -> &mut Scope {
        self.scopes.last_mut().unwrap()
    }
}
