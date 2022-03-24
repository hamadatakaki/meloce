use std::fmt;

use super::syntax;

pub enum ExpressedValue {
    IntValue(i64),
    BoolValue(bool),
}

impl ExpressedValue {
    pub fn is_int(self) -> bool {
        match self {
            ExpressedValue::IntValue(_) => true,
            _ => false,
        }
    }

    pub fn to_int(self) -> Result<i64, EvalError> {
        match self {
            ExpressedValue::IntValue(x) => Ok(x),
            _ => Err(String::from("fail to cast as int")),
        }
    }

    pub fn is_bool(self) -> bool {
        match self {
            ExpressedValue::BoolValue(_) => true,
            _ => false,
        }
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

// TODO: syntax::Exp をもらって、ExpressedValueを返す関数がimplされたstructをつくる
// pub struct EvalExp { env: Environment }
