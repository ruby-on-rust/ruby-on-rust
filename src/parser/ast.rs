use std::fmt::{Debug, Formatter, Error};

pub enum Expr {
  Number(i64),
  Operation(Box<Expr>, Operator, Box<Expr>)
}

#[derive(Copy, Clone)]
pub enum Operator {
    Mul,
    Div,
    Add,
    Sub,
}

impl Debug for Expr {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Expr::*;
        match *self {
            Number(n) => write!(fmt, "{:?}", n),
            Operation(ref l, op, ref r) => write!(fmt, "({:?} {:?} {:?})", l, op, r),
            // Error => write!(fmt, "error"),
        }
    }
}

impl Debug for Operator {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Operator::*;
        match *self {
            Mul => write!(fmt, "*"),
            Div => write!(fmt, "/"),
            Add => write!(fmt, "+"),
            Sub => write!(fmt, "-"),
        }
    }
}
