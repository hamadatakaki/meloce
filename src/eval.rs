use std::fmt;

use crate::{
    environment::ExpEnvironment,
    syntax::{Exp, Program},
};

use super::syntax;

#[derive(Clone, Copy)]
pub enum ExpressedValue {
    IntValue(i64),
    BoolValue(bool),
}

impl ExpressedValue {
    pub fn is_int(self) -> bool {
        matches!(self, ExpressedValue::IntValue(_))
    }

    pub fn to_int(self) -> Result<i64, EvalError> {
        match self {
            ExpressedValue::IntValue(x) => Ok(x),
            _ => Err(String::from("fail to cast as int")),
        }
    }

    pub fn is_bool(self) -> bool {
        matches!(self, ExpressedValue::BoolValue(_))
    }

    pub fn to_bool(self) -> Result<bool, EvalError> {
        match self {
            ExpressedValue::BoolValue(x) => Ok(x),
            _ => Err(String::from("fail to cast as bool")),
        }
    }
}

impl fmt::Display for ExpressedValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ExpressedValue::IntValue(x) => write!(f, "{}", x),
            ExpressedValue::BoolValue(b) => write!(f, "{}", b),
        }
    }
}

pub type DenotedValue = ExpressedValue;

pub type EvalError = String;

pub fn error(s: &str) -> EvalError {
    String::from(s)
}

pub fn apply_prim(
    op: syntax::BinOp,
    arg1: ExpressedValue,
    arg2: ExpressedValue,
) -> Result<ExpressedValue, EvalError> {
    match op {
        syntax::BinOp::Plus => {
            if let Ok(i1) = arg1.to_int() {
                if let Ok(i2) = arg2.to_int() {
                    return Ok(ExpressedValue::IntValue(i1 + i2));
                }
            }
            Err(error("Both arguments must be integer: +"))
        }
        syntax::BinOp::Mult => {
            if let Ok(i1) = arg1.to_int() {
                if let Ok(i2) = arg2.to_int() {
                    return Ok(ExpressedValue::IntValue(i1 * i2));
                }
            }
            Err(error("Both arguments must be integer: *"))
        }
        syntax::BinOp::Lt => {
            if let Ok(i1) = arg1.to_int() {
                if let Ok(i2) = arg2.to_int() {
                    return Ok(ExpressedValue::BoolValue(i1 < i2));
                }
            }
            Err(error("Both arguments must be integer: <"))
        }
    }
}

pub struct Evaluator {
    env: ExpEnvironment,
}

impl Evaluator {
    pub fn new(env: ExpEnvironment) -> Self {
        Self { env }
    }

    pub fn eval_exp(&self, exp: Exp) -> Result<ExpressedValue, EvalError> {
        match exp {
            Exp::Var(x) => match self.env.lookup(&x) {
                Ok(&v) => Ok(v),
                Err(_) => Err(format!("Variable not bound: {}", x)),
            },
            Exp::ILit(i) => Ok(ExpressedValue::IntValue(i)),
            Exp::BLit(b) => Ok(ExpressedValue::BoolValue(b)),
            Exp::BinOp(op, exp1, exp2) => {
                let arg1 = self.eval_exp(*exp1)?;
                let arg2 = self.eval_exp(*exp2)?;
                apply_prim(op, arg1, arg2)
            }
            Exp::IfExp(exp1, exp2, exp3) => match self.eval_exp(*exp1)? {
                ExpressedValue::BoolValue(b) => {
                    if b {
                        self.eval_exp(*exp2)
                    } else {
                        self.eval_exp(*exp3)
                    }
                }
                _ => Err(String::from("Test expression must be boolean: if")),
            },
        }
    }

    pub fn eval_decl(
        &self,
        prog: Program,
    ) -> Result<(String, ExpEnvironment, ExpressedValue), EvalError> {
        match prog {
            Program::Exp(e) => {
                let v = self.eval_exp(e)?;
                Ok((String::from("-"), self.env.clone(), v))
            }
        }
    }
}
