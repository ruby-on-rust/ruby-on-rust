use std::fmt::{Debug, Formatter, Error};

pub enum Expr {
    // Nullary(),

    // Unary(),
    Number(i64),
    Identifier(String),
    // TODO prefixed. minus, bang, etc

    Binary(Box<Expr>, Operator, Box<Expr>),
    Assignment(String, Box<Expr>), // TODO maybe we should represent Identifier as a tuple or sth, instead of a plain String

    // Ternary()
}

impl Debug for Expr {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Expr::*;
        match *self {
            Number(n) => write!(fmt, "{:?}", n),
            Identifier(ref s) => write!(fmt, "{:?}", s),
            Binary(ref l, op, ref r) => write!(fmt, "({:?} {:?} {:?})", l, op, r),
            Assignment(ref id, ref e) => write!(fmt, "({:?} = {:?})", id, e),
            // Error => write!(fmt, "error"),
        }
    }
}

#[derive(Copy, Clone)]
pub enum Operator {
    Mul,
    Div,
    Add,
    Sub,
    // Equal,
}

impl Debug for Operator {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Operator::*;
        match *self {
            Mul => write!(fmt, "*"),
            Div => write!(fmt, "/"),
            Add => write!(fmt, "+"),
            Sub => write!(fmt, "-"),
            // Equal => write!(fmt, "="),
        }
    }
}
