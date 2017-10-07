use parser::ast;

use interpreter::interpreter::{Interpreter};
use interpreter::object::{Object, Primitive};
use interpreter::memory::{Refer};

impl Interpreter {
    pub fn eval_expr_binary(&mut self, left_expr: ast::Expr, operator: ast::Operator, right_expr: ast::Expr) -> Refer {
        match operator {
            ast::Operator::Add => return self.eval_expr_binary_addition(left_expr, right_expr),
            ast::Operator::Sub => return self.eval_expr_binary_subtraction(left_expr, right_expr),
            ast::Operator::Mul => return self.eval_expr_binary_multiplication(left_expr, right_expr),
            _ => unimplemented!()
        }
    }

    pub fn eval_expr_binary_addition(&mut self, left_expr: ast::Expr, right_expr: ast::Expr) -> Refer {
        // NOTE since we cant simply do this in rust, we use a `eval_2_exprs`
        // TODO may be improved
        // let left_refer = self.eval_expr(left_expr);
        // let left_value = self.memory.get_value_from_refer(left_refer);
        // let right_refer = self.eval_expr(right_expr);
        // let right_value = self.memory.get_value_from_refer(right_refer);

        let (left_refer, right_refer) = self.eval_2_exprs(left_expr, right_expr);

        let evaled_value = {
            let left_obj = self.memory.get_obj_from_refer(left_refer);
            let right_obj = self.memory.get_obj_from_refer(right_refer);

            Primitive::Number(left_obj.dummy_value + right_obj.dummy_value)
        };

        self.memory.allocate_primitive(evaled_value)
    }

    pub fn eval_expr_binary_subtraction(&mut self, left_expr: ast::Expr, right_expr: ast::Expr) -> Refer {
        let (left_refer, right_refer) = self.eval_2_exprs(left_expr, right_expr);

        let evaled_value = {
            let left_obj = self.memory.get_obj_from_refer(left_refer);
            let right_obj = self.memory.get_obj_from_refer(right_refer);

            Primitive::Number(left_obj.dummy_value - right_obj.dummy_value)
        };

        self.memory.allocate_primitive(evaled_value)
    }

    pub fn eval_expr_binary_multiplication(&mut self, left_expr: ast::Expr, right_expr: ast::Expr) -> Refer {
        let (left_refer, right_refer) = self.eval_2_exprs(left_expr, right_expr);

        let evaled_value = {
            let left_obj = self.memory.get_obj_from_refer(left_refer);
            let right_obj = self.memory.get_obj_from_refer(right_refer);

            Primitive::Number(left_obj.dummy_value * right_obj.dummy_value)
        };

        self.memory.allocate_primitive(evaled_value)
    }
}
