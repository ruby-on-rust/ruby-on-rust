use parser::ast;

#[derive(Debug)]
enum Value {
    Null,
    Number(i64)
}

pub struct Interpreter {
}

impl Interpreter {
    pub fn eval_line(&self, expr: ast::Expr) {
        let value = self.eval_expr(expr);
        println!("evaluated value: {:?}", value);
    }

    pub fn eval_expr(&self, expr: ast::Expr) -> Value {
        match expr {
            ast::Expr::Number(n) => {
                return Value::Number(n)
            },
            ast::Expr::Binary(l, op, r) => {
                return self.eval_expr_binary(*l, op, *r)
            },
            _ => {
                return Value::Null
            }
        }
    }

    pub fn eval_expr_binary(&self, left_expr: ast::Expr, operator: ast::Operator, right_expr: ast::Expr) -> Value {
        match operator {
            ast::Operator::Add => return self.eval_expr_binary_addition(left_expr, right_expr),
            ast::Operator::Sub => return self.eval_expr_binary_substraction(left_expr, right_expr),
            ast::Operator::Mul => return self.eval_expr_binary_multiplication(left_expr, right_expr),
            _ => unimplemented!()
        }

        Value::Null
    }

    pub fn eval_expr_binary_addition(&self, left_expr: ast::Expr, right_expr: ast::Expr) -> Value {
        let left_value = self.eval_expr(left_expr);
        let right_value = self.eval_expr(right_expr);

        match (left_value, right_value) {
            (Value::Number(left_value), Value::Number(right_value)) => { return Value::Number(left_value + right_value) },
            _ => unimplemented!()
        }
    }

    pub fn eval_expr_binary_substraction(&self, left_expr: ast::Expr, right_expr: ast::Expr) -> Value {
        let left_value = self.eval_expr(left_expr);
        let right_value = self.eval_expr(right_expr);

        match (left_value, right_value) {
            (Value::Number(left_value), Value::Number(right_value)) => { return Value::Number(left_value - right_value) },
            _ => unimplemented!()
        }
    }

    pub fn eval_expr_binary_multiplication(&self, left_expr: ast::Expr, right_expr: ast::Expr) -> Value {
        let left_value = self.eval_expr(left_expr);
        let right_value = self.eval_expr(right_expr);

        match (left_value, right_value) {
            (Value::Number(left_value), Value::Number(right_value)) => { return Value::Number(left_value * right_value) },
            _ => unimplemented!()
        }
    }
}
